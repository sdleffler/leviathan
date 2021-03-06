use geometry::primitive::Point;
use geometry::shape::SupportMapping;
use linalg::{Dot, Mat, Scalar, Vect, VectorNorm};
use num::traits::Float;
use typehack::prelude::*;


struct DistanceCache<T: Scalar, D: Dim> {
    dims: D,
    simplex: DataVec<Vect<T, D>, D::Succ>,
    dots: Mat<T, D::Succ, D::Succ>,
    deltas: Mat<T, <B1 as DimShl<D::Succ>>::Result, D::Succ>,
    subset: usize,
}


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<T: Scalar + Clone, D: Dim> DistanceCache<T, D>
{
    fn new(d: D) -> DistanceCache<T, D> {
        unsafe {
            DistanceCache {
                dims: d,
                simplex: DataVec::with_capacity(d.succ()),
                dots: Mat::uninitialized(d.succ(), d.succ()),
                deltas: Mat::uninitialized(B1::as_data().shl(d.succ()), d.succ()),
                subset: 0,
            }
        }
    }


    fn free_subset_slot(&self) -> usize {
        (!self.subset).trailing_zeros() as usize
    }


    fn from_barycentric(&self, simplex: &DataVec<Vect<T, D>, D::Succ>) -> Vect<T, D> {
        let delta_total: T = (0..simplex.len())
            .filter(|i| self.subset & (1usize << i) != 0)
            .map(|i| self.deltas[[self.subset, i]].clone())
            .sum();

        (0..simplex.len())
            .filter(|i| self.subset & (1usize << i) != 0)
            .map(|i| {
                simplex[i].clone() * (self.deltas[[self.subset, i]].clone() / delta_total.clone())
            })
            .sum()
    }


    fn nearest(&mut self,
               w_k: Vect<T, D>)
               -> Option<Vect<T, D>> {
        let d = self.dims.reify();
        let r = self.free_subset_slot();

        if self.simplex.len() < d {
            debug!("Building simplex... length: {}", self.simplex.len());
            self.simplex.push(w_k);
            // Now we compute the dot products of w_k with the other members of `W`:
            for (i, y_i) in self.simplex.iter().enumerate() {
                debug!("Computing and caching dot product: {} ⋅ {} ({:?} ⋅ {:?})",
                         r,
                         i,
                         self.simplex[r],
                         y_i);
                self.dots[[i, r]] = self.simplex[r].clone().dot(y_i.clone());
                self.dots[[r, i]] = self.dots[[i, r]].clone();
            }

            // And now, for the hard part. We compute all of the deltas *changed* by the insertion of
            // k. This is actually fairly simple since we don't yet have the entire simplex. We can
            // just compute the new set of deltas with k! This is also very simple to enumerate
            // over. We have k, which is the new ith member of the simplex. We iterate through
            // 2^i..2^(i+1) - 1, giving us *only* the new deltas. (This is equivalent to setting a
            // bit, and then iterating through all of the integers less than 2^i, but with that bit
            // set.)
            debug!("Simplex has {} elements.", self.simplex.len());
            for s in (1usize << r)..(2usize << r) {
                debug!("Updating Δs for subset {:b}...", s);

                for j in (0..self.simplex.len()).filter(|&j| s & (1 << j) != 0) {
                    debug!("Calculating Δ_{}({:b})...", j, s);

                    let s_p = s & !(1 << j);

                    debug!("{:b}∖{:b} = {:b}", s, 1 << j, s_p);

                    let k = s_p.trailing_zeros() as usize; // k = min i, i ∈ Iₛ where Iₛ is now 0..simplex.len() without j.

                    self.deltas[[s, j]] = if s.count_ones() == 1 {
                        T::one()
                    } else {
                        (0..self.simplex.len())
                            .filter(|&i| s_p & (1 << i) != 0)
                            .map(|i| {
                                debug!("+ Δ_{}({:b}) * (y_{} ⋅ y_{} - y_{} ⋅ \
                                          y_{}) == {:?} * ({:?} - {:?})",
                                         i,
                                         s_p,
                                         i,
                                         k,
                                         i,
                                         j,
                                         self.deltas[[s_p, i]],
                                         self.dots[[i, k]],
                                         self.dots[[i, j]]);
                                self.deltas[[s_p, i]].clone() *
                                (self.dots[[i, k]].clone() - self.dots[[i, j]].clone())
                            })
                            .sum()
                    };

                    debug!("Δ_{}({:b}) := {:?}", j, s, self.deltas[[s, j]]);
                }
            }
        } else {
            // The simplex is *full.* The next place to overwrite is indicated by the "next" field of
            // the cache.
            self.simplex.set(r, w_k);
            debug!("Simplex full: discarded vertex: {}.", r);
            // We now compute our dot products, but this time with *all* members:
            for (i, y_i) in self.simplex.iter().enumerate() {
                debug!("Computing and caching dot product: {} ⋅ {} ({:?} ⋅ {:?})",
                         r,
                         i,
                         self.simplex[r],
                         y_i);
                self.dots[[i, r]] = self.simplex[r].clone().dot(y_i.clone());
                self.dots[[r, i]] = self.dots[[i, r]].clone();
            }
            // Now we compute all of the changed deltas. About this next iterator, just trust me - it works.
            //
            // Probably. (Verified with d = 6, r = 3.)
            for s in (0usize..(1 << (d - r))).map(|l| (l << 1) | 1).flat_map(|l| (0..(2 << r)).map(move |i| i | (l << r))) {
                debug!("Updating Δs for subset {:b}...", s);
                for j in (0..self.simplex.len()).filter(|&j| s & (1 << j) != 0) {
                    debug!("Calculating Δ_{}({:b})...", j, s);

                    let s_p = s & !(1 << j);

                    debug!("{:b}∖{:b} = {:b}", s, 1 << j, s_p);

                    let k = s_p.trailing_zeros() as usize; // k = min i, i ∈ Iₛ where Iₛ is now 0..simplex.len() without j.

                    self.deltas[[s, j]] = if s.count_ones() == 1 {
                        T::one()
                    } else {
                        (0..self.simplex.len())
                            .filter(|&i| s_p & (1 << i) != 0)
                            .map(|i| {
                                debug!("+ Δ_{}({:b}) * (y_{} ⋅ y_{} - y_{} ⋅ \
                                          y_{}) == {:?} * ({:?} - {:?})",
                                         i,
                                         s_p,
                                         i,
                                         k,
                                         i,
                                         j,
                                         self.deltas[[s_p, i]],
                                         self.dots[[i, k]],
                                         self.dots[[i, j]]);
                                self.deltas[[s_p, i]].clone() *
                                (self.dots[[i, k]].clone() - self.dots[[i, j]].clone())
                            })
                            .sum()
                    };

                    debug!("Δ_{}({:b}) := {:?}", j, s, self.deltas[[s, j]]);
                }
            }
        }

        // TODO: When break-with-non-unit-value lands in stable Rust, use it here.
        'subsets: for s in (1..(1 << self.simplex.len())).filter(|&s| s & (1 << r) != 0) {
            debug!("Testing subset {:b}...", s);

            for i in 0..self.simplex.len() {
                if s & (1 << i) != 0 {
                    debug!("Testing Δ_{}({:b}), `i` is in `s`; {:?} > 0 to succeed: {}",
                             i,
                             s,
                             self.deltas[[s, i]],
                             self.deltas[[s, i]].gt_zero());
                } else {
                    debug!("Testing Δ_{}({:b}), `i` is not in `s`; {:?} <= 0 to succeed: {}",
                             i,
                             s | (1 << i),
                             self.deltas[[s | (1 << i), i]],
                             self.deltas[[s | (1 << i), i]].lte_zero());
                }

                let i_in_s = s & (1 << i) != 0; // i ∈ Iₛ ?

                if i_in_s && self.deltas[[s, i]].lte_zero() {
                    debug!("Failure! delta({:b}) is not a unique solution: Δ_{}({:b}) <= 0", s, i, s);
                    continue 'subsets;
                } else if !i_in_s && self.deltas[[s | (1 << i), i]].gt_zero() {
                    debug!("Failure! delta({:b}) is not a unique solution: Δ_{}({:b}) > 0", s, i, s | (1 << i));
                    continue 'subsets;
                }
            }

            debug!("Success: subset {:b} is a unique solution.", s);
            self.subset = s;

            return Some(self.from_barycentric(&self.simplex));
        }

        None
    }
}


pub struct GjkInfo<'a,
                   A: ?Sized + SupportMapping + 'a,
                   B: ?Sized + SupportMapping<Scalar = A::Scalar, Dims = A::Dims> + 'a>
{
    cache: DistanceCache<A::Scalar, A::Dims>,
    a: &'a A,
    a_pts: DataVec<Vect<A::Scalar, A::Dims>, Succ<A::Dims>>,
    b: &'a B,
    b_pts: DataVec<Vect<A::Scalar, A::Dims>, Succ<A::Dims>>,
}


impl<'a, A: ?Sized + SupportMapping + 'a, B: ?Sized + SupportMapping<Scalar = A::Scalar, Dims = A::Dims> + 'a> GjkInfo<'a,
                                                                                           A,
                                                                                           B> where A::Scalar: Float {
    pub fn nearest_points(&self) -> (Vect<A::Scalar, A::Dims>, Vect<A::Scalar, A::Dims>) {
        (Vect::from(self.cache.from_barycentric(&self.a_pts)), Vect::from(self.cache.from_barycentric(&self.b_pts)))
    }

    pub fn distance(&self) -> A::Scalar {
        self.cache.from_barycentric(&self.cache.simplex).norm()
    }
}


pub trait GjkExt<B: SupportMapping<Scalar = Self::Scalar, Dims = Self::Dims>>
    : SupportMapping {
    fn gjk<'a>(&'a self, &'a B) -> GjkInfo<'a, Self, B>;
}


impl<T: Clone + Scalar + Float + From<f64>, A, B> GjkExt<B> for A
    where A: SupportMapping<Scalar = T>,
          B: SupportMapping<Scalar = T, Dims = A::Dims>
{
    fn gjk<'a>(&'a self, b: &'a B) -> GjkInfo<'a, Self, B> {
        let epsilon = 0.00001;

        let a = self;

        assert_eq!(a.dims(), b.dims());

        let mut a_pts: DataVec<Vect<T, A::Dims>, _> = DataVec::with_capacity(a.dims().succ());
        let mut b_pts: DataVec<Vect<T, B::Dims>, _> = DataVec::with_capacity(b.dims().succ());

        let mut cache = DistanceCache::new(self.dims());

        let (mut supp_a, mut supp_b) = {
            let v0 = a.interior_point() - b.interior_point();
            (a.support(&v0), b.support(&-v0))
        };

        a_pts.push(supp_a.clone().into());
        b_pts.push(supp_b.clone().into());

        let mut prev_nearest: Option<Vect<T, A::Dims>> = None;

        while let Some(nearest) = cache.nearest(supp_a - supp_b) {
            if let Some(prev_nearest) = prev_nearest.take() {
                if (prev_nearest - nearest.clone()).norm() < epsilon.into() {
                    break;
                }
            }

            {
                let search_dir = &nearest;

                // These probably look like their signs are flipped opposite to how they should be,
                // but we're not negating the search direction; so, less sign flips.
                supp_a = a.support(&-search_dir);
                supp_b = b.support(search_dir);
            }

            a_pts.set(cache.free_subset_slot(), supp_a.clone().into());
            b_pts.set(cache.free_subset_slot(), supp_b.clone().into());

            prev_nearest = Some(nearest);
        }

        return GjkInfo {
            cache: cache,
            a: a,
            a_pts: a_pts,
            b: b,
            b_pts: b_pts,
        };
    }
}


#[cfg(test)]
mod tests {
    extern crate env_logger;

    use super::{DistanceCache, GjkExt};

    use geometry::shape::Polygon;
    use linalg::{Scalar, VectorNorm};
    use typehack::binary::{B2, Nat};


    #[test]
    fn gjk_triangle_triangle_1() {
        let _ = env_logger::init();

        let triangle_a = unsafe {
            Polygon::from_raw_vertices(B2::as_data(),
                                       vec![Point![1., 2.], Point![4., 1.], Point![2., 4.]])
        };

        let triangle_b = unsafe {
            Polygon::from_raw_vertices(B2::as_data(),
                                       vec![Point![0., -1.], Point![-3., -2.], Point![-1., 1.]])
        };

        let (nearest_a, nearest_b) = triangle_a.gjk(&triangle_b).nearest_points();

        debug!("Nearest point on a: {:?}, nearest point on b: {:?}.",
               nearest_a,
               nearest_b);

        assert!((nearest_a - Vect![1., 2.]).norm().eq_zero());
        assert!((nearest_b - Vect![-1., 1.]).norm().eq_zero());
    }


    #[test]
    fn distance_subalgorithm_trivial_1() {
        let _ = env_logger::init();

        let mut cache = DistanceCache::new(B2::as_data());

        let p0 = cache.nearest(Vect![3., 0.]).unwrap();
        debug!("p0: {:?}", p0);
        assert!((p0 - Vect![3., 0.]).norm() < 0.00001);

        let p1 = cache.nearest(Vect![0., 3.]).unwrap();
        debug!("p1: {:?}", p1);
        assert!((p1 - Vect![1.5, 1.5]).norm() < 0.00001);

        let p2 = cache.nearest(Vect![1., 1.]).unwrap();
        debug!("p2: {:?}", p2);
        assert!((p2 - Vect![1., 1.]).norm() < 0.00001);
    }


    #[test]
    fn distance_subalgorithm_trivial_2() {
        let _ = env_logger::init();

        let mut cache = DistanceCache::new(B2::as_data());

        let p0 = cache.nearest(Vect![3., 0.]).unwrap();
        debug!("p0: {:?}", p0);
        assert!((p0 - Vect![3., 0.]).norm() < 0.00001);

        let p1 = cache.nearest(Vect![2., 0.]).unwrap();
        debug!("p0: {:?}", p1);
        assert!((p1 - Vect![2., 0.]).norm() < 0.00001);

        let p2 = cache.nearest(Vect![1., -1.]).unwrap();
        debug!("p0: {:?}", p2);
        assert!((p2 - Vect![1., -1.]).norm() < 0.00001);
    }


    #[test]
    fn distance_subalgorithm_nontrivial_1() {
        let _ = env_logger::init();

        let mut cache = DistanceCache::new(B2::as_data());

        let p0 = cache.nearest(Vect![3., 0.]).unwrap();
        debug!("p0: {:?}", p0);
        assert!((p0 - Vect![3., 0.]).norm() < 0.00001);

        let p1 = cache.nearest(Vect![0., 3.]).unwrap();
        debug!("p1: {:?}", p1);
        assert!((p1 - Vect![1.5, 1.5]).norm() < 0.00001);

        let p2 = cache.nearest(Vect![2., 0.]).unwrap();
        debug!("p2: {:?}", p2);
        assert!((p2 - Vect![18. / 13., 12. / 13.]).norm() < 0.00001);

        let p3 = cache.nearest(Vect![0., 2.]).unwrap();
        debug!("p3: {:?}", p3);
        assert!((p3 - Vect![1., 1.]).norm() < 0.00001);
    }
}
