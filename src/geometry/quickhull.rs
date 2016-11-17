use std::cmp::Ordering;

use geometry::primitive::{Facet, Point};
use linalg::{Scalar, VectorNorm};
use num::traits::Float;
use typehack::prelude::*;


pub trait QuickHullExt<T: Scalar, D: Dim> {
    fn quick_hull(self) -> Vec<Facet<T, D>>;
}


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<'a, T: 'a + Scalar + Clone + Float, D: 'a + Dim> QuickHullExt<T, D> for &'a [Point<T, D>] {
    fn quick_hull(self) -> Vec<Facet<T, D>> {
        let dim = self[0].size(); // TODO: Better way to find this.

        // We build an initial simplex by first finding the furthest pair of points in the set.
        // First, we find the points with minimum and maximum coordinates:

        let extremes: Data<&'a Point<T, D>, D::Double> = {
            let mut iter = self.iter();

            let fst = match iter.next() {
                Some(pt) => pt,
                None => return Vec::new(),
            };

            // We store minimums in even elements, and maximums in odd ones.
            let mut initial = Data::from_elem(dim.double(), &fst); // &fst here because it expects &&Point, not &Point.

            for pt in iter {
                for i in 0..dim.reify() {
                    if pt[i] < initial[2 * i][i] {
                        initial[2 * i] = pt;
                    }

                    if pt[i] > initial[2 * i + 1][i] {
                        initial[2 * i + 1] = pt;
                    }
                }
            }

            initial
        };

        // Now, we find the pair of these points which is furthest apart, via brute force:

        let furthest: (&'a Point<T, D>, &'a Point<T, D>) = {
            let mut initial = (T::zero(), extremes[0], extremes[0]);

            for &a in extremes.iter() {
                for &b in extremes.iter() {
                    let dist = (a - b).norm();
                    if dist > initial.0 {
                        initial = (dist, a, b);
                    }
                }
            }

            (initial.1, initial.2)
        };
        
        unimplemented!();
    }
}
