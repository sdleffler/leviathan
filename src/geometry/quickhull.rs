use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use iter_exact::{ChainExactExt, CollectExactExt};

use geometry::primitive::{Facet, Plane, Point, SimplexSubset};
use linalg::{Scalar, Vect, VectorNorm};
use num::traits::Float;
use typehack::prelude::*;


pub trait QuickHullExt<T: Scalar, D: Dim> {
    fn quick_hull(&self, D) -> Vec<FacetIndices<D>>;
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PointIdx(usize);

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


pub struct ConvexHull<T: Scalar, D: Dim> {
    points: Vec<Point<T, D>>,
    facets: Vec<FacetIndices<D>>,
}


impl<T: Scalar + Float, D: Dim> QuickHullExt<T, D> for [Point<T, D>] {
    fn quick_hull(&self, dim: D) -> Vec<FacetIndices<D>> {
        debug!("Beginning quickhull with {} points in {} dimensions.",
               self.len(),
               dim.reify());

        // By taking the average of the point cloud, we're guaranteed an interior point.
        let guaranteed_interior_point: Point<T, D> = {
            let sum: Vect<T, D> = self.iter().cloned().map(Vect::from).sum();
            let vect = sum / T::from_usize(self.len());
            vect.into()
        };

        debug!("Calculated {:?} to be a guaranteed interior point by averaging all points in the \
                hull.",
               guaranteed_interior_point);

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

        debug!("{:?} remaining potentially outside points. Building rest of simplex...",
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

        debug!("Generating conflict sets...");

        for ref element in elements.iter_mut() {
            let mut i = 0;
            while i < pt_indices.len() {
                let PointIdx(idx) = pt_indices[i];
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

            debug!("Beginning horizon search...");

            while let Some(facet) = visit_stack.pop() {
                debug!("Searching facet {:?}.", facet.borrow().facet);

                for (neighbor, ridge) in
                    facet.borrow().neighbors.iter().map(|&(ref nb, ref ridge)| (nb.clone(), ridge)) {
                    if !neighbor.borrow().visited {
                        debug!("Checking unvisited neighbor {:?}...",
                               neighbor.borrow().facet);

                        if neighbor.borrow().plane.signed_distance(p).gt_zero() {
                            debug!("Neighbor found to be visible; pushing to visible set and \
                                    search stack.");
                            visible.push(neighbor.clone());
                            visit_stack.push(neighbor);
                        } else {
                            debug!("Neighbor found to not be visible; pushing to horizon set \
                                    with ridge {:?}.",
                                   ridge);
                            horizon.push((neighbor, ridge.clone()));
                        }
                    } else {
                        debug!("Neighbor {:?} already visited... skipping...",
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

                debug!("Calculating new neighbors...");

                {
                    let mut new_element = new_element_rc.borrow_mut();

                    'finding_new_neighbors: for &ref facet in new_facets.iter() {
                        let mut skipped = None;

                        for (i, ref p_idx) in new_facet.iter().enumerate() {
                            if !facet.borrow().facet.contains(p_idx) && skipped.is_none() {
                                skipped = Some(i);
                            } else if skipped.is_some() {
                                continue 'finding_new_neighbors;
                            }
                        }

                        let skipped_idx = skipped.unwrap();
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
                old_facet.dead = true;
            }

            debug!("Assigning collected points to new facets...");

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

        debug!("Quickhull finished.");

        elements.into_iter()
            .filter_map(|element| {
                if !element.borrow().dead {
                    Some(element.borrow()
                        .facet
                        .clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}


#[cfg(test)]
mod tests {
    extern crate env_logger;

    use super::*;

    use typehack::prelude::*;

    macro_rules! assert_edge {
        ($hull:expr, $a:expr => $b:expr) => (assert!($hull.contains(&data![PointIdx($a), PointIdx($b)]) ||
                $hull.contains(&data![PointIdx($b), PointIdx($a)])));
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
}
