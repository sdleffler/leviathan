use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use iter_exact::{ChainExactExt, CollectExactExt};

use geometry::primitive::{Facet, Plane, Point, SimplexSubset};
use linalg::{Scalar, Vect, VectorNorm};
use num::traits::Float;
use typehack::prelude::*;


// TODO: This quickhull implementation is not well-suited for spitting out the resulting point set,
//       unordered, without any faceting. This functionality is reasonable to have.



pub trait QuickHullExt<T: Scalar, D: Dim> {
    fn quick_hull(&self, D) -> ConvexHull<D>;
}


#[derive(Debug, Clone)]
pub struct ConvexHull<D: Dim> {
    pub points: Vec<PointIdx>,
    pub facets: Vec<FacetIndices<D>>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PointIdx(pub usize);

impl From<usize> for PointIdx {
    fn from(i: usize) -> Self {
        PointIdx(i)
    }
}

impl From<PointIdx> for usize {
    fn from(idx: PointIdx) -> Self {
        idx.0
    }
}


struct QhElement<T: Scalar, D: Dim> {
    facet: FacetIndices<D>,
    plane: Plane<T, D>,
    outside: Vec<PointIdx>,
    neighbors: Vec<(QhFacetRef<T, D>, RidgeIndices<D>)>,
    visited: bool,
    dead: bool,
}

type FacetIndices<D: Dim> = Data<PointIdx, D>;
type RidgeIndices<D: Dim> = Data<PointIdx, D::Pred>;

type QhFacetRef<T: Scalar, D: Dim> = Rc<RefCell<QhElement<T, D>>>;


impl<T: Scalar + Float, D: Dim> QuickHullExt<T, D> for [Point<T, D>] {
    fn quick_hull(&self, dim: D) -> ConvexHull<D> {
        debug!("Beginning quickhull with {} points in {} dimensions.",
               self.len(),
               dim.reify());

        let mut pt_indices: Vec<PointIdx> = (0..self.len()).map(PointIdx).collect();

        debug!("Generated vector of {} point indices. Checking for extremes...",
               pt_indices.len());

        // We build an initial simplex by first finding the furthest pair of points in the set.
        // First, we find the points with minimum and maximum coordinates:

        let extremes: Data<PointIdx, D::Double> = {
            let iter = self.iter().enumerate();

            // We store minimums in even elements, and maximums in odd ones.
            let mut initial = Data::from_elem(dim.double(), &PointIdx(0));

            for (i, pt) in iter {
                for j in 0..dim.reify() {
                    if pt[j] < self[initial[2 * j].0][j] {
                        initial[2 * j] = PointIdx(i);
                    }

                    if pt[j] > self[initial[2 * j + 1].0][j] {
                        initial[2 * j + 1] = PointIdx(i);
                    }
                }
            }

            initial
        };

        debug!("Extreme points: {:?}.", extremes);

        // Now, we find the pair of these points which is furthest apart, via brute force, and add
        // those to our initial simplex:

        let mut simplex: SimplexSubset<T, D> = SimplexSubset::with_capacity(dim.succ());

        debug!("Simplex: {:?}", &*simplex);

        let mut simplex_indices: DataVec<PointIdx, D::Succ> = DataVec::with_capacity(dim.succ());

        {
            let mut max_dist = T::zero();
            let mut initial = (PointIdx(0), PointIdx(0));

            for &PointIdx(i) in extremes.iter() {
                for &PointIdx(j) in extremes.iter() {
                    let dist = (&self[i] - &self[j]).norm();
                    if dist > max_dist {
                        initial = (PointIdx(i), PointIdx(j));
                        max_dist = dist;
                    }
                }
            }

            debug!("Furthest points determined to be {:?} and {:?}, with a distance of {:?}.",
                   initial.0,
                   initial.1,
                   max_dist);

            simplex.push(&self[usize::from(initial.0)]);
            simplex_indices.push(initial.0);
            simplex.push(&self[usize::from(initial.1)]);
            simplex_indices.push(initial.1);

            if initial.0 > initial.1 {
                pt_indices.remove(usize::from(initial.0));
                pt_indices.remove(usize::from(initial.1));
            } else {
                pt_indices.remove(usize::from(initial.1));
                pt_indices.remove(usize::from(initial.0));
            }
        }

        debug!("Simplex: {:?}, with {} remaining potentially outside points. Building rest of \
                simplex...",
               &*simplex,
               pt_indices.len());

        // Now, until our simplex is full, we find points far from the simplex and add them in.

        while !simplex.is_full() {
            let max_idx = {
                let mut iter = pt_indices.iter();

                let (mut max_dist, mut max_idx) = {
                    let idx = 0;
                    (simplex.distance(&self[usize::from(pt_indices[idx])], dim), idx)
                };

                for idx in 0..pt_indices.len() {
                    let dist = simplex.distance(&self[usize::from(pt_indices[idx])], dim);
                    if dist > max_dist {
                        max_dist = dist;
                        max_idx = idx;
                    }
                }

                max_idx
            };

            debug!("Furthest point from simplex of {} points determined to be {:?}. Removing \
                    from point index list...",
                   simplex.len(),
                   pt_indices[max_idx]);

            simplex.push(&self[usize::from(pt_indices[max_idx])]);
            simplex_indices.push(pt_indices[max_idx]);

            pt_indices.remove(max_idx);
        }

        // The simplex is a valid convex hull. By taking the average of all its vertices, we are
        // guaranteed an interior point.
        let guaranteed_interior_point: Point<T, D> = {
            let sum: Vect<T, D> = simplex.iter().map(|&p| Vect::from(p.clone())).sum();
            let vect = sum / T::from_usize(simplex.len());
            vect.into()
        };

        debug!("Calculated {:?} to be a guaranteed interior point by averaging all points in the \
                initial simplex.",
               guaranteed_interior_point);

        // We now have our full initial simplex. We can now generate our first few facets from it.
        let mut elements: Vec<QhFacetRef<T, D>> = Vec::with_capacity(dim.succ()
            .reify());

        let d = dim.succ().reify();
        for i in 0..d {
            debug!("Building initial facet {} from simplex (excluding {}th point from simplex).",
                   i,
                   i);

            let facet: FacetIndices<D> =
                (0..i).chain_exact(i + 1..d).map(|j| simplex_indices[j]).collect_exact();

            debug!("Facet points: {:?}. Constructing plane from facet...",
                   facet);

            let mut plane: Plane<T, D> = facet.iter()
                .map(|&PointIdx(i)| self[i].clone())
                .collect_exact::<Facet<T, D>>()
                .into();

            if plane.signed_distance(&guaranteed_interior_point).gt_zero() {
                plane.n = -plane.n;
                debug!("Plane signed distance has the wrong sign, flipping.");
            }

            debug!("Plane constructed from facet: {:?}.", plane);

            let element: QhFacetRef<T, D> = Rc::new(RefCell::new(QhElement {
                facet: facet,
                plane: plane,
                outside: Vec::new(),
                neighbors: Vec::new(),
                visited: false,
                dead: false,
            }));

            debug!("Constructing neighbors for facet ({} neighbors to link.)",
                   elements.len());

            for (j, &ref neighbor) in elements.iter().enumerate() {
                let ridge: RidgeIndices<D> = (0..j)
                    .chain_exact(j + 1..i)
                    .chain_exact(i + 1..d)
                    .map(|i| simplex_indices[i])
                    .collect_exact();

                debug!("Linking {:?} with its neighbor {:?}, with ridge {:?}.",
                       element.borrow().facet,
                       neighbor.borrow().facet,
                       ridge);

                neighbor.borrow_mut().neighbors.push((element.clone(), ridge.clone()));
                element.borrow_mut().neighbors.push((neighbor.clone(), ridge));
            }

            debug!("Facet {} constructed successfully.", i);

            elements.push(element);
        }

        // We must now generate the conflict sets for our first facets. We do so by looping through
        // our facets, and assigning all points above them to their conflict sets.

        debug!("Generating conflict sets... {} points to assign.",
               pt_indices.len());

        for ref element in elements.iter_mut() {
            let mut i = 0;
            while i < pt_indices.len() {
                let PointIdx(idx) = pt_indices[i];

                debug!("Checking point {:?} against the hyperplane of element {:?}. Signed \
                        distance: {:?}",
                       PointIdx(idx),
                       element.borrow().facet,
                       element.borrow().plane.signed_distance(&self[idx]));

                if element.borrow().plane.signed_distance(&self[idx]).gt_zero() {
                    element.borrow_mut().outside.push(pt_indices.swap_remove(i));
                } else {
                    i += 1;
                }
            }
        }

        // Any points remaining in pt_indices are guaranteed to be interior points.

        debug!("Filtering for nonempty facets...");

        let mut nonempty_facets: Vec<_> = elements.iter()
            .cloned()
            .filter(|ref facet| {
                debug!("Facet {:?} has {} conflict points;",
                       facet.borrow().facet,
                       facet.borrow().outside.len());
                !facet.borrow().outside.is_empty()
            })
            .collect();

        debug!("Entering refinement loop:");

        let mut iteration = 0;

        while let Some(facet) = nonempty_facets.pop() {
            debug!("********* ITERATION {} \
                    ********************************************************",
                   iteration);
            iteration += 1;

            debug!("Selected facet with nonempty conflict list (facet: {:?}, {} conflicts.)",
                   facet.borrow().facet,
                   facet.borrow().outside.len());

            // We select the furthest point p of our facet f's outside set.
            let PointIdx(p_idx) = {
                let mut max_dist = T::zero();
                let mut max_idx = 0;
                for (idx, &PointIdx(pt_idx)) in facet.borrow().outside.iter().enumerate() {
                    let dist = facet.borrow().plane.signed_distance(&self[pt_idx]);
                    if dist > max_dist {
                        max_dist = dist;
                        max_idx = idx;
                    }
                }

                facet.borrow_mut().outside.swap_remove(max_idx)
            };

            debug!("Selected {:?} as the furthest point of the conflict set.",
                   PointIdx(p_idx));

            let p = &self[p_idx];

            // We must find the "horizon". This is the set of ridges which form the boundary
            // between the "visible" and "non-visible" facets. We do this by building a subgraph
            // where the nodes are all visible facets; the ridges are then all edges where one node
            // is in the subgraph and the other is not.

            let mut visit_stack = vec![facet.clone()];
            let mut visible = vec![facet.clone()];
            let mut horizon = Vec::new();

            facet.borrow_mut().visited = true;
            facet.borrow_mut().dead = true;

            debug!("Beginning horizon search...");

            while let Some(facet) = visit_stack.pop() {
                debug!("Searching facet {:?}.", facet.borrow().facet);

                for (neighbor, ridge) in
                    facet.borrow().neighbors.iter().map(|&(ref nb, ref ridge)| (nb.clone(), ridge)) {
                    if !neighbor.borrow().visited {
                        debug!("Checking unvisited neighbor {:?}...",
                               neighbor.borrow().facet);

                        neighbor.borrow_mut().visited = true;

                        if neighbor.borrow().plane.signed_distance(p).gt_zero() {
                            debug!("Neighbor found to be visible; pushing to visible set and \
                                    search stack.");
                            neighbor.borrow_mut().dead = true;
                            visible.push(neighbor.clone());
                            visit_stack.push(neighbor);
                        } else {
                            debug!("Neighbor found to not be visible; pushing to horizon set \
                                    with ridge {:?}.",
                                   ridge);
                            horizon.push((neighbor, ridge.clone()));
                        }
                    } else if !neighbor.borrow().dead {
                        debug!("Neighbor revisited, but not visible; pushing to horizon set \
                                with unique ridge {:?}.",
                               ridge);
                        horizon.push((neighbor, ridge.clone()));
                    } else {
                        debug!("Neighbor {:?} already visited and visible... skipping...",
                               neighbor.borrow().facet);
                    }
                }
            }

            debug!("Resetting all horizon `visited` flags.");

            for &(ref neighbor, _) in horizon.iter() {
                neighbor.borrow_mut().visited = false;
            }

            let mut new_facets: Vec<QhFacetRef<T, D>> = Vec::new();

            debug!("Generating new facets from {} horizon ridges.",
                   horizon.len());

            for (neighbor, ridge) in horizon {
                debug!("Generating new facet set...");

                let new_facet: Data<PointIdx, D> = {
                    debug!("Collecting ridge points...");
                    let mut facet: DataVec<PointIdx, D> = ridge.iter().cloned().collect_exact();
                    debug!("Pushing eye point...");
                    facet.push(PointIdx(p_idx));
                    debug!("Converting DataVec into Data..");
                    facet.into()
                };

                debug!("Generating new facet hyperplane...");

                let new_plane = {
                    let mut plane: Plane<T, D> = new_facet.iter()
                        .map(|&PointIdx(i)| self[i].clone())
                        .collect_exact::<Facet<T, D>>()
                        .into();

                    if plane.signed_distance(&guaranteed_interior_point)
                        .gt_zero() {
                        plane.n = -plane.n;
                    }

                    plane
                };

                debug!("Generating (empty) outside set and allocating shared pointer...");

                let new_outside = Vec::new();

                debug!("Generating singleton neighbor set with neighbor {:?} and ridge {:?}...",
                       neighbor.borrow().facet,
                       ridge);

                let mut new_neighbors = vec![(neighbor.clone(), ridge.clone())];

                let new_element_rc = Rc::new(RefCell::new(QhElement {
                    facet: new_facet.clone(),
                    plane: new_plane,
                    outside: new_outside,
                    neighbors: new_neighbors,
                    visited: false,
                    dead: false,
                }));

                debug!("Linking singleton neighbor...");
                neighbor.borrow_mut().neighbors.push((new_element_rc.clone(), ridge));

                elements.push(new_element_rc.clone());

                {
                    let mut new_element = new_element_rc.borrow_mut();

                    'finding_new_neighbors: for &ref facet in new_facets.iter() {
                        let mut skipped = None;

                        debug!("Checking potential neighbor {:?} for {:?}.",
                               facet.borrow().facet,
                               new_element.facet);

                        for (i, ref p_idx) in new_facet.iter().enumerate() {
                            if !facet.borrow().facet.contains(p_idx) {
                                if skipped.is_none() {
                                    skipped = Some(i);
                                } else {
                                    continue 'finding_new_neighbors;
                                }
                            }
                        }

                        let skipped_idx = match skipped {
                            Some(skipped_idx) => skipped_idx,
                            None => continue 'finding_new_neighbors,
                        };

                        debug!("Neighbor discovered, containing all elements but the {}th.",
                               skipped_idx);
                        let new_ridge = new_facet.clone().contract(skipped_idx);

                        debug!("Neighbor discovered: {:?} with ridge {:?}.",
                               facet.borrow().facet,
                               new_ridge);

                        facet.borrow_mut()
                            .neighbors
                            .push((new_element_rc.clone(), new_ridge.clone()));
                        new_element.neighbors.push((facet.clone(), new_ridge));
                    }
                }

                debug!("Generated new facet {:?} with {} neighbors.",
                       new_element_rc.borrow().facet,
                       new_element_rc.borrow().neighbors.len());

                new_facets.push(new_element_rc);
            }

            debug!("Collecting points from facets to be deleted...");

            let mut unassigned_pts = Vec::new();
            for old_facet in visible.into_iter() {
                debug!("Exterminating facet {:?}...", old_facet.borrow().facet);

                // Unlink this old facet from all its neighbors.
                for &(ref old_neighbor, _) in old_facet.borrow().neighbors.iter() {
                    debug!("Unlinking neighbor {:?}...", old_neighbor.borrow().facet);

                    old_neighbor.borrow_mut().neighbors.retain(|&(ref reflexive, _)| {
                        (reflexive.as_ref() as *const _) != (old_facet.as_ref() as *const _)
                    });
                }

                // All references to this facet from other facets should be gone now.
                // We took this facet by value, so it should be dead now as well. No cycles. One
                // reference left.
                let mut old_facet = old_facet.borrow_mut();

                unassigned_pts.extend(old_facet.outside.drain(..));
            }

            debug!("Assigning collected points to new facets... {} points to be assigned.",
                   unassigned_pts.len());

            // For each new facet: we steal unassigned points from facets in V. Then, add facets
            // with newly assigned facets into the nomepty_facets queue. We then push them into the
            // nonempty facet queue.
            for new_facet in new_facets.into_iter() {
                let mut i = 0;
                while i < unassigned_pts.len() {
                    if new_facet.borrow()
                        .plane
                        .signed_distance(&self[unassigned_pts[i].0])
                        .gt_zero() {
                        new_facet.borrow_mut().outside.push(unassigned_pts.swap_remove(i));
                    } else {
                        i += 1;
                    }
                }

                if new_facet.borrow().outside.len() > 0 {
                    nonempty_facets.push(new_facet);
                }
            }
        }

        debug!("Quickhull finished. Generating filtered point list...");

        elements.retain(|element| !element.borrow().dead);

        let mut pt_list = Vec::new();

        for ref element in elements.iter() {
            for &pt_idx in element.borrow().facet.iter() {
                pt_list.push(pt_idx);
            }
        }

        pt_list.sort();
        pt_list.dedup(); // Is this step necessary?

        // API may change; this little change may be useful.
        //
        // let facets = elements.into_iter()
        //     .filter_map(|element| {
        //         if !element.borrow().dead {
        //             Some(element.borrow()
        //                 .facet
        //                 .clone()
        //                 .into_iter()
        //                 .map(|p_idx| PointIdx(pt_list.binary_search(&p_idx).unwrap()))
        //                 .collect_exact::<FacetIndices<D>>())
        //         } else {
        //             None
        //         }
        //     })
        //     .collect::<Vec<_>>();

        let facets = elements.into_iter()
            .map(|element| {
                element.borrow()
                    .facet
                    .clone()
            })
            .collect::<Vec<_>>();

        ConvexHull {
            points: pt_list,
            facets: facets,
        }
    }
}


#[cfg(test)]
mod tests {
    extern crate env_logger;

    use super::*;

    use typehack::prelude::*;

    macro_rules! assert_edge {
        ($hull:expr, $a:expr => $b:expr) => (assert!($hull.facets.contains(&data![PointIdx($a), PointIdx($b)]) ||
                $hull.facets.contains(&data![PointIdx($b), PointIdx($a)])));
    }

    macro_rules! assert_face {
        ($hull:expr, $a:expr => $b:expr => $c:expr) => (
            assert!($hull.facets.contains(&data![PointIdx($a), PointIdx($b), PointIdx($c)]) ||
                    $hull.facets.contains(&data![PointIdx($a), PointIdx($c), PointIdx($b)]) ||
                    $hull.facets.contains(&data![PointIdx($b), PointIdx($a), PointIdx($c)]) ||
                    $hull.facets.contains(&data![PointIdx($b), PointIdx($c), PointIdx($a)]) ||
                    $hull.facets.contains(&data![PointIdx($c), PointIdx($a), PointIdx($b)]) ||
                    $hull.facets.contains(&data![PointIdx($c), PointIdx($b), PointIdx($a)])));
    }

    macro_rules! assert_pts {
        ($hull:expr, $($p:expr),*) => ($(assert!($hull.points.contains(&PointIdx($p)));)*);
    }

    macro_rules! assert_not_pts {
        ($hull:expr, $($p:expr),*) => ($(assert!(!$hull.points.contains(&PointIdx($p)));)*);
    }

    #[test]
    fn qhull_2d_trivial_nondegenerate_1() {
        let _ = env_logger::init();

        let triangle = vec![Point![1., 2.], Point![2., 3.], Point![4., 1.]];
        let hull = triangle.quick_hull(B2::as_data());
    }

    #[test]
    fn qhull_2d_trivial_nondegenerate_2() {
        let _ = env_logger::init();

        let triangle = vec![Point![0., -2.], Point![-2., 3.], Point![2., 4.]];
        let hull = triangle.quick_hull(B2::as_data());
    }

    #[test]
    fn qhull_2d_nontrivial_nondegenerate_1() {
        let _ = env_logger::init();

        // No interior points.

        let points = vec![Point![0., -2.], Point![-1., 3.], Point![-2., 0.], Point![2., 2.]];
        let hull = points.quick_hull(B2::as_data());

        assert_edge!(hull, 1 => 3);
        assert_edge!(hull, 1 => 2);
        assert_edge!(hull, 0 => 2);
        assert_edge!(hull, 0 => 3);
    }

    #[test]
    fn qhull_2d_nontrivial_nondegenerate_2() {
        let _ = env_logger::init();

        // No interior points.

        let points = vec![Point![0., -2.],
                          Point![2., -1.],
                          Point![1., 1.],
                          Point![3., 2.],
                          Point![1., 3.],
                          Point![0., 4.],
                          Point![-1., 3.],
                          Point![-3., 2.],
                          Point![-1., 1.],
                          Point![-2., 0.]];
        let hull = points.quick_hull(B2::as_data());

        debug!("hull: {:?}", hull);

        assert_edge!(hull, 0 => 1);
        assert_edge!(hull, 1 => 3);
        assert_edge!(hull, 3 => 5);
        assert_edge!(hull, 5 => 7);
        assert_edge!(hull, 7 => 9);
        assert_edge!(hull, 9 => 0);
    }

    #[test]
    fn qhull_2d_nontrivial_nondegenerate_3() {
        let _ = env_logger::init();

        // No interior points.

        let points = vec![Point![0.3215348546593775, 0.03629583077160248], // 0
                          Point![0.02402358131857918, -0.2356728797179394], // 1
                          Point![0.04590851212470659, -0.4156409924995536], // 2
                          Point![0.3218384001607433, 0.1379850698988746], // 3
                          Point![0.11506479756447, -0.1059521474930943], // 4
                          Point![0.2622539999543261, -0.29702873322836], // 5
                          Point![-0.161920957418085, -0.4055339716426413], // 6
                          Point![0.1905378631228002, 0.3698601009043493], // 7
                          Point![0.2387090918968516, -0.01629827079949742], // 8
                          Point![0.07495888748668034, -0.1659825110491202], // 9
                          Point![0.3319341836794598, -0.1821814101954749], // 10
                          Point![0.07703635755650362, -0.2499430638271785], // 11
                          Point![0.2069242999022122, -0.2232970760420869], // 12
                          Point![0.04604079532068295, -0.1923573186549892], // 13
                          Point![0.05054295812784038, 0.4754929463150845], // 14
                          Point![-0.3900589168910486, 0.2797829520700341], // 15
                          Point![0.3120693385713448, -0.0506329867529059], // 16
                          Point![0.01138812723698857, 0.4002504701728471], // 17
                          Point![0.009645149586391732, 0.1060251100976254], // 18
                          Point![-0.03597933197019559, 0.2953639456959105], // 19
                          Point![0.1818290866742182, 0.001454397571696298], // 20
                          Point![0.444056063372694, 0.2502497166863175], // 21
                          Point![-0.05301752458607545, -0.06553921621808712], // 22
                          Point![0.4823896228171788, -0.4776170002088109], // 23
                          Point![-0.3089226845734964, -0.06356112199235814], // 24
                          Point![-0.271780741188471, 0.1810810595574612], // 25
                          Point![0.4293626522918815, 0.2980897964891882], // 26
                          Point![-0.004796652127799228, 0.382663812844701], // 27
                          Point![0.430695573269106, -0.2995073500084759], // 28
                          Point![0.1799668387323309, -0.2973467472915973], // 29
                          Point![0.4932166845474547, 0.4928094162538735], // 30
                          Point![-0.3521487911717489, 0.4352656197131292], // 31
                          Point![-0.4907368011686362, 0.1865826865533206], // 32
                          Point![-0.1047924716070224, -0.247073392148198], // 33
                          Point![0.4374961861758457, -0.001606279519951237], // 34
                          Point![0.003256207800708899, -0.2729194320486108], // 35
                          Point![0.04310378203457577, 0.4452604050238248], // 36
                          Point![0.4916198379282093, -0.345391701297268], // 37
                          Point![0.001675087028811806, 0.1531837672490476], // 38
                          Point![-0.4404289572876217, -0.2894855991839297]]; /* 39 */
        let hull = points.quick_hull(B2::as_data());

        debug!("hull: {:?}", hull);
        assert_pts!(hull, 6, 14, 23, 30, 31, 32, 37, 39);
    }

    #[test]
    fn qhull_3d_nontrivial_nondegenerate_1() {
        let _ = env_logger::init();

        let tetrahedron = vec![Point![1., 1., 1.],
                               Point![2., 3., 1.],
                               Point![-2., 0., 1.],
                               Point![1., 4., 2.],
                               Point![2., 1., 2.]];
        let hull = tetrahedron.quick_hull(B3::as_data());

        debug!("hull: {:?}", hull);
        assert_pts!(hull, 0, 1, 2, 3, 4);

        assert_face!(hull, 1 => 3 => 2);
        assert_face!(hull, 0 => 1 => 2);
        assert_face!(hull, 3 => 4 => 2);
        assert_face!(hull, 4 => 0 => 2);
        assert_face!(hull, 4 => 3 => 1);
        assert_face!(hull, 0 => 4 => 1);
    }

    #[test]
    fn qhull_3d_nontrivial_nondegenerate_2() {
        let _ = env_logger::init();

        let tetrahedron = vec![Point![1., 1., 1.],
                               Point![2., 3., 1.],
                               Point![-2., 0., 1.],
                               Point![1., 4., 2.],
                               Point![0.5, 3., 1.7]];
        let hull = tetrahedron.quick_hull(B3::as_data());

        debug!("hull: {:?}", hull);
        assert_pts!(hull, 0, 1, 2, 3);
        assert_not_pts!(hull, 4);

        assert_face!(hull, 1 => 3 => 2);
        assert_face!(hull, 0 => 1 => 2);
        assert_face!(hull, 3 => 0 => 2);
        assert_face!(hull, 0 => 3 => 1);
    }

    #[test]
    fn qhull_3d_nontrivial_nondegenerate_3() {
        let _ = env_logger::init();

        let tetrahedron = vec![Point![0.346987, 0.594300, 0.395053],
                               Point![0.472077, 0.063314, 0.029606],
                               Point![0.606915, 0.641988, 0.167560],
                               Point![0.554433, 0.549847, 0.032239],
                               Point![0.118838, 0.496147, 0.367041]];
        let hull = tetrahedron.quick_hull(B3::as_data());

        debug!("hull: {:?}", hull);
        assert_pts!(hull, 0, 1, 2, 3, 4);
        // assert_not_pts!(hull);

        assert_face!(hull, 2 => 0 => 4);
        assert_face!(hull, 0 => 1 => 4);
        assert_face!(hull, 1 => 0 => 2);
        assert_face!(hull, 3 => 2 => 4);
        assert_face!(hull, 1 => 3 => 4);
        assert_face!(hull, 3 => 1 => 2);
    }

    #[test]
    fn qhull_3d_nontrivial_nondegenerate_4() {
        let _ = env_logger::init();

        let tetrahedron = vec![Point![0.177014, 0.572769, 0.201412],
                               Point![0.064319, 0.555407, 0.114194],
                               Point![0.494991, 0.666792, 0.947249],
                               Point![0.046340, 0.320490, 0.377621],
                               Point![0.946863, 0.737976, 0.371916],
                               Point![0.829540, 0.636103, 0.085375]];

        let hull = tetrahedron.quick_hull(B3::as_data());

        debug!("hull: {:?}", hull);
        assert_pts!(hull, 1, 2, 3, 4, 5);
        assert_not_pts!(hull, 0);

        assert_face!(hull, 2 => 4 => 3);
        assert_face!(hull, 4 => 5 => 3);
        assert_face!(hull, 1 => 2 => 3);
        assert_face!(hull, 5 => 1 => 3);
        assert_face!(hull, 2 => 1 => 4);
        assert_face!(hull, 1 => 5 => 4);
    }

    #[test]
    fn qhull_3d_nontrivial_nondegenerate_5() {
        let _ = env_logger::init();

        let points = vec![Point![0.3215426810286406, 0.1678336189760208, -0.2203710966001927], /* 0 */
                          Point![0.2229772524190855, -0.4213242506806965, -0.1966818060695024], /* 1 */
                          Point![0.3688830163971363, -0.1831502133823468, -0.2056387967482571], /* 2 */
                          Point![-0.1712592515826777, -0.3542439228428937, 0.2223876390814666], /* 3 */
                          Point![-0.3309556113844324, -0.370961861099081, 0.2439994981922204], /* 4 */
                          Point![-0.1004397059794885, -0.09014152417903909, -0.008600084584765189], /* 5 */
                          Point![0.458374538420117, -0.09914027349943322, -0.2505798421339875], /* 6 */
                          Point![-0.4954086979808367, -0.3339869997780649, -0.3195065691317492], /* 7 */
                          Point![0.053091190339151, 0.3036317017894533, 0.1380056861210668], /* 8 */
                          Point![0.4615616439483703, 0.4665423151725366, 0.1766835406205464], /* 9 */
                          Point![-0.4797380864431505, 0.0419809916447671, -0.4254776681079321], /* 10 */
                          Point![-0.003168473023146823, -0.2525299883005488, -0.27151530400991], /* 11 */
                          Point![-0.3577162826971303, -0.1375644040643837, -0.04494194644032229], /* 12 */
                          Point![-0.3392973838740004, 0.4288679723896719, -0.01599531622230571], /* 13 */
                          Point![0.1667164640191164, 0.003605551555385444, -0.4014989499947977], /* 14 */
                          Point![0.00714666676441833, 0.1140243407469469, 0.407090128778564], /* 15 */
                          Point![-0.03621271768232132, 0.3728502838619522, 0.4947140370446388], /* 16 */
                          Point![-0.3411871756810576, -0.3328629143842151, -0.4270033635450559], /* 17 */
                          Point![0.3544683273457627, -0.450828987127942, -0.0827870439577727], /* 18 */
                          Point![-0.4018510635028137, 0.08917494033386464, -0.2367824197158054], /* 19 */
                          Point![0.3978697768392692, -0.002667689232777493, 0.1641431727112673], /* 20 */
                          Point![-0.245701439441835, 0.495905311308713, -0.3194406286994373], /* 21 */
                          Point![0.161352035739787, -0.1563404972258401, 0.3852604361113724], /* 22 */
                          Point![0.07214279572678994, -0.4960366976410492, 0.1112227161519441], /* 23 */
                          Point![0.3201855824516951, 0.359077846965825, 0.02136723140381946], /* 24 */
                          Point![0.1190541238701475, -0.05734495917087884, 0.2032677509852384], /* 25 */
                          Point![0.3210853052521919, 0.4807189479290684, 0.4433501688235907], /* 26 */
                          Point![0.3862800354941562, 0.2085496142586224, 0.09336129957191763], /* 27 */
                          Point![0.1233572616459404, 0.265491605052251, 0.117400122450106], /* 28 */
                          Point![0.1438531872293476, -0.2594872752758556, -0.2026374435076839], /* 29 */
                          Point![0.2724846394476338, -0.3506708492996831, 0.2750346518820475], /* 30 */
                          Point![-0.4926118841325975, -0.3279366743079728, 0.3683135596740186], /* 31 */
                          Point![0.2459906458351674, 0.3647787136629026, -0.1641662355178652], /* 32 */
                          Point![-0.141922976953837, -0.2994764654892278, -0.3009570467294725], /* 33 */
                          Point![-0.1850859398814719, 0.2606059478228967, 0.004159106876849283], /* 34 */
                          Point![-0.09789466634196664, -0.3156603563722785, -0.303610991503681], /* 35 */
                          Point![0.2100642609503719, -0.4499717643018549, 0.3245569875692548], /* 36 */
                          Point![-0.1707163766685095, -0.2301452446078371, -0.05112823569320907], /* 37 */
                          Point![-0.312260808713977, -0.1674135249735914, 0.2808831662692904], /* 38 */
                          Point![-0.1966306233747216, 0.2291105671125563, -0.3387042454804333]]; /* 39 */

        // 0.3215426810286406 0.1678336189760208 -0.2203710966001927 // 0
        // 0.2229772524190855 -0.4213242506806965 -0.1966818060695024 // 1
        // 0.458374538420117 -0.09914027349943322 -0.2505798421339875 // 6
        // -0.4954086979808367 -0.3339869997780649 -0.3195065691317492 // 7
        // 0.4615616439483703 0.4665423151725366 0.1766835406205464 // 9
        // -0.4797380864431505 0.0419809916447671 -0.4254776681079321 // 10
        // -0.3392973838740004 0.4288679723896719 -0.01599531622230571 // 13
        // 0.1667164640191164 0.003605551555385444 -0.4014989499947977 // 14
        // -0.03621271768232132 0.3728502838619522 0.4947140370446388 // 16
        // -0.3411871756810576 -0.3328629143842151 -0.4270033635450559 // 17
        // 0.3544683273457627 -0.450828987127942 -0.0827870439577727 // 18
        // 0.3978697768392692 -0.002667689232777493 0.1641431727112673 // 20
        // -0.245701439441835 0.495905311308713 -0.3194406286994373 // 21
        // 0.161352035739787 -0.1563404972258401 0.3852604361113724 // 22
        // 0.07214279572678994 -0.4960366976410492 0.1112227161519441 // 23
        // 0.3210853052521919 0.4807189479290684 0.4433501688235907 // 24
        // 0.2724846394476338 -0.3506708492996831 0.2750346518820475 // 30
        // -0.4926118841325975 -0.3279366743079728 0.3683135596740186 // 31
        // 0.2459906458351674 0.3647787136629026 -0.1641662355178652 // 32
        // 0.2100642609503719 -0.4499717643018549 0.3245569875692548 // 36

        let hull = points.quick_hull(B3::as_data());

        debug!("hull: {:?}", hull);

        assert_face!(hull, 31 =>  23 =>   7);
        assert_face!(hull, 23 =>  17 =>   7);
        assert_face!(hull, 1  => 17  => 23);
        assert_face!(hull, 21 =>  32 =>  9);
        assert_face!(hull, 31 =>  13 =>  16);
        assert_face!(hull, 26 =>  21 =>  9);
        assert_face!(hull, 13 =>  26 =>  16);
        assert_face!(hull, 26 =>  13 =>  21);
        assert_face!(hull, 6  => 20  => 9);
        assert_face!(hull, 20 =>  26 =>  9);
        assert_face!(hull, 26 =>  20 =>  30);
        assert_face!(hull, 36 =>  26 =>  30);
        assert_face!(hull, 31 =>  36 =>  23);
        assert_face!(hull, 17 =>  10 =>   7);
        assert_face!(hull, 13 =>  10 =>  21);
        assert_face!(hull, 10 =>  31 =>   7);
        assert_face!(hull, 10 =>  13 =>  31);
        assert_face!(hull, 14 =>  32 =>  21);
        assert_face!(hull, 10 =>  14 =>  21);
        assert_face!(hull, 14 =>  10 =>  17);
        assert_face!(hull, 14 =>   1 =>   6);
        assert_face!(hull, 14 =>  17 =>   1);
        assert_face!(hull, 22 =>  31 =>  16);
        assert_face!(hull, 22 =>  36 =>  31);
        assert_face!(hull, 26 =>  22 =>  16);
        assert_face!(hull, 36 =>  22 =>  26);
        assert_face!(hull, 1  => 18  =>  6);
        assert_face!(hull, 18 =>  36 =>  30);
        assert_face!(hull, 18 =>   1 =>  23);
        assert_face!(hull, 36 =>  18 =>  23);
        assert_face!(hull, 18 =>  20 =>   6);
        assert_face!(hull, 20 =>  18 =>  30);
        assert_face!(hull, 0  => 14  =>  6);
        assert_face!(hull, 14 =>   0 =>  32);
        assert_face!(hull, 0  =>  6  => 9);
        assert_face!(hull, 32 =>   0 =>  9);
    }
}
