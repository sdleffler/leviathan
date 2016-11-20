use std::ops::{Add, Sub, Index, IndexMut};

use iter_exact::{CollectExactExt, FromExactSizeIterator};

use linalg::*;
use linalg::algorithm::solve::gaussian::GaussianNullspaceExt;
use num::traits::Float;
use typehack::prelude::*;
use typehack::data;


#[macro_export]
macro_rules! Point {
    ($($xs:expr),* $(,)*) => ($crate::geometry::primitive::Point::from(Vect![$($xs),*]));
}


#[macro_export]
macro_rules! Simplex {
    ($($pts:expr),* $(,)*) => ($crate::geometry::primitive::Simplex { points: data![$($pts),*] });
}


#[macro_export]
macro_rules! Facet {
    ($($pts:expr),* $(,)*) => ($crate::geometry::primitive::Facet { points: data![$($pts),*] });
}


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Point<T: Scalar, N: Dim> {
    vect: Vect<T, N>,
}


impl<T: Copy + Scalar, N: Dim> Copy for Point<T, N> where Vect<T, N>: Copy {}


impl<T: Scalar, N: Dim> Point<T, N> {
    pub fn from_elem(size: N, elem: &T) -> Point<T, N>
        where T: Clone
    {
        Point { vect: Vect::from_elem(size, elem) }
    }


    pub fn size(&self) -> N {
        self.vect.size()
    }
}


impl<T: Scalar, N: Dim> From<Vect<T, N>> for Point<T, N> {
    fn from(vect: Vect<T, N>) -> Point<T, N> {
        Point { vect: vect }
    }
}


impl<T: Scalar, N: Dim> From<Point<T, N>> for Vect<T, N> {
    fn from(pt: Point<T, N>) -> Vect<T, N> {
        pt.vect
    }
}


impl<T: Scalar, N: Dim> Add<Vect<T, N>> for Point<T, N> {
    type Output = Point<T, N>;

    fn add(self, rhs: Vect<T, N>) -> Point<T, N> {
        Point { vect: self.vect + rhs }
    }
}


impl<'a, T: Scalar, N: Dim> Add<Vect<T, N>> for &'a Point<T, N> {
    type Output = Point<T, N>;

    fn add(self, rhs: Vect<T, N>) -> Point<T, N> {
        Point { vect: &self.vect + rhs }
    }
}


impl<'b, T: Scalar, N: Dim> Add<&'b Vect<T, N>> for Point<T, N> {
    type Output = Point<T, N>;

    fn add(self, rhs: &'b Vect<T, N>) -> Point<T, N> {
        Point { vect: self.vect + rhs }
    }
}


impl<'a, 'b, T: Scalar, N: Dim> Add<&'b Vect<T, N>> for &'a Point<T, N> {
    type Output = Point<T, N>;

    fn add(self, rhs: &'b Vect<T, N>) -> Point<T, N> {
        Point { vect: &self.vect + rhs }
    }
}


impl<T: Scalar, N: Dim> Add<Point<T, N>> for Vect<T, N> {
    type Output = Point<T, N>;

    fn add(self, rhs: Point<T, N>) -> Point<T, N> {
        Point { vect: self + rhs.vect }
    }
}


impl<T: Scalar, N: Dim> Sub<Vect<T, N>> for Point<T, N> {
    type Output = Point<T, N>;

    fn sub(self, rhs: Vect<T, N>) -> Point<T, N> {
        Point { vect: self.vect - rhs }
    }
}


impl<T: Scalar, N: Dim> Sub<Point<T, N>> for Vect<T, N> {
    type Output = Point<T, N>;

    fn sub(self, rhs: Point<T, N>) -> Point<T, N> {
        Point { vect: self - rhs.vect }
    }
}


impl<T: Scalar, N: Dim> Sub for Point<T, N> {
    type Output = Vect<T, N>;

    fn sub(self, rhs: Self) -> Vect<T, N> {
        self.vect - rhs.vect
    }
}


impl<'a, T: Clone + Scalar, N: Dim> Sub<&'a Point<T, N>> for Point<T, N> {
    type Output = Vect<T, N>;

    fn sub(self, rhs: &Self) -> Vect<T, N> {
        self.vect - &rhs.vect
    }
}


impl<'a, T: Clone + Scalar, N: Dim> Sub<Point<T, N>> for &'a Point<T, N> {
    type Output = Vect<T, N>;

    fn sub(self, rhs: Point<T, N>) -> Vect<T, N> {
        &self.vect - rhs.vect
    }
}


impl<'a, 'b, T: Clone + Scalar, N: Dim> Sub<&'b Point<T, N>> for &'a Point<T, N> {
    type Output = Vect<T, N>;

    fn sub(self, rhs: &'b Point<T, N>) -> Vect<T, N> {
        &self.vect - &rhs.vect
    }
}


impl<T: Scalar, N: Dim> Index<usize> for Point<T, N> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        &self.vect[idx]
    }
}


impl<T: Scalar, N: Dim> IndexMut<usize> for Point<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        &mut self.vect[idx]
    }
}


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Plane<T: Scalar, N: Dim> {
    pub n: Vect<T, N>,
    pub p0: Point<T, N>,
}


impl<T: Copy + Scalar, N: Dim> Copy for Plane<T, N> where Vect<T, N>: Copy {}


impl<T: Clone + Scalar + Float, N: Dim> Plane<T, N>
    where Vect<T, N>: Clone
{
    pub fn signed_distance(&self, p: &Point<T, N>) -> T {
        (p - &self.p0).component(self.n.clone())
    }
}


impl<T: Clone + Scalar, N: Dim> From<Facet<T, N>> for Plane<T, N> {
    default fn from(facet: Facet<T, N>) -> Plane<T, N> {
        let mut iter = facet.into_iter();
        let p0 = iter.next().unwrap();

        let mat = Mat::<T, N::Pred, N, Row>::from_rows(iter.map(|p| p - p0.clone()));
        let n = mat.ge_null_elem();

        Plane { n: n, p0: p0 }
    }
}

impl<T: Clone + Scalar> From<Facet<T, B3>> for Plane<T, B3> {
    fn from(facet: Facet<T, B3>) -> Plane<T, B3> {
        let mut iter = facet.into_iter();
        let p0 = iter.next().unwrap();
        let p1 = iter.next().unwrap();
        let p2 = iter.next().unwrap();

        let a = p1 - p0.clone();
        let b = p2 - p0.clone();

        // We perform the cross product of `a` and `b` to get the normal vector.

        Plane {
            n: Vect![a[1].clone() * b[2].clone() - a[2].clone() * b[1].clone(),
                     a[2].clone() * b[0].clone() - a[0].clone() * b[2].clone(),
                     a[0].clone() * b[1].clone() - a[1].clone() * b[0].clone()],
            p0: p0,
        }
    }
}


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Simplex<T: Scalar, N: Dim> {
    points: Data<Point<T, N>, N::Succ>,
}


impl<T: Copy + Scalar, N: Dim> Copy for Simplex<T, N> where Data<Point<T, N>, N::Succ>: Copy {}


impl<T: Clone + Scalar, N: Dim> Simplex<T, N> {
    pub fn dims(&self) -> N::Succ {
        self.points.size()
    }
}


// impl<T: Clone + Scalar, N: Dim> Simplex<T, N> {
//     pub fn facets(&self) -> SimplexFacets<T, N> {
//         (0..self.dims().reify())
//             .map(|i| {
//                 (0..i)
//                     .map(|j| self.points[j].clone())
//                     .chain_exact((i + 1..self.dims().reify()).map(|j| self.points[j].clone()))
//                     .collect_exact()
//             })
//             .collect_exact()
//     }
// }


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Facet<T: Scalar, N: Dim> {
    points: Data<Point<T, N>, N>,
}


impl<T: Copy + Scalar, N: Dim> Copy for Facet<T, N> where Data<Point<T, N>, N>: Copy {}


impl<T: Scalar, N: Dim> IntoIterator for Facet<T, N> {
    type Item = Point<T, N>;
    type IntoIter = data::IntoIter<Point<T, N>, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
    }
}


impl<T: Scalar, N: Dim> FromExactSizeIterator<Point<T, N>> for Facet<T, N> {
    fn from_exact_size_iter<I: IntoIterator<Item = Point<T, N>>>(iter: I) -> Self
        where I::IntoIter: ExactSizeIterator
    {
        Facet { points: iter.into_iter().collect_exact() }
    }
}


impl<T: Scalar, N: Dim> Index<usize> for Facet<T, N> {
    type Output = Point<T, N>;

    fn index(&self, idx: usize) -> &Point<T, N> {
        &self.points[idx]
    }
}


impl<T: Scalar, N: Dim> IndexMut<usize> for Facet<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut Point<T, N> {
        &mut self.points[idx]
    }
}


pub type SimplexSubset<'a, T: Scalar, N: Dim> = DataVec<&'a Point<T, N>, N::Succ>;

impl<'a, T: Scalar + Float, N: Dim> SimplexSubset<'a, T, N> {
    pub fn distance(&self, p: &Point<T, N>, dim: N) -> T {
        // This nearest-point-on-simplex-subset-to-origin routine is torn straight from GJK.
        // We subtract `pt` from every point of our simplex so that it becomes the origin. Problem
        // solved?

        let simplex: DataVec<Vect<T, N>, N::Succ> =
            self.clone().into_iter().map(|&y| y - p).collect_exact();

        let d = dim.reify();

        let dots: Mat<T, _, _> = unsafe {
            let mut dots: Mat<T, _, _> = Mat::uninitialized(dim.succ(), dim.succ());

            for (i, y_i) in simplex.iter().enumerate() {
                for (j, y_j) in (i..).zip(&simplex[i..]) {
                    dots[[i, j]] = y_j.clone().dot(y_i.clone());
                    dots[[j, i]] = dots[[i, j]].clone();
                }
            }

            dots
        };

        let deltas: Mat<T, _, _> = unsafe {
            let mut deltas: Mat<T, _, _> = Mat::uninitialized(B1::as_data().shl(dim.succ()),
                                                              dim.succ());

            for s in 1usize..(1 << d) {
                for j in (0..simplex.len()).filter(|&j| s & (1 << j) != 0) {
                    let s_p = s & !(1 << j);
                    let k = s_p.trailing_zeros() as usize; // k = min i, i ∈ Iₛ where Iₛ is now 0..simplex.len() without j.

                    deltas[[s, j]] = if s.count_ones() == 1 {
                        T::one()
                    } else {
                        (0..simplex.len())
                            .filter(|&i| s_p & (1 << i) != 0)
                            .map(|i| {
                                deltas[[s_p, i]].clone() *
                                (dots[[i, k]].clone() - dots[[i, j]].clone())
                            })
                            .sum()
                    };
                }
            }

            deltas
        };

        // TODO: When break-with-non-unit-value lands in stable Rust, use it here.
        'subsets: for s in 1..(1 << simplex.len()) {
            for i in 0..simplex.len() {
                let i_in_s = s & (1 << i) != 0; // i ∈ Iₛ ?

                if (i_in_s && deltas[[s, i]].lte_zero()) ||
                   (!i_in_s && deltas[[s | (1 << i), i]].gt_zero()) {
                    continue 'subsets;
                }
            }

            let delta_total: T = (0..simplex.len())
                .filter(|i| s & (1usize << i) != 0)
                .map(|i| deltas[[s, i]].clone())
                .sum();

            let offset: Vect<T, N> = (0..simplex.len())
                .filter(|i| s & (1usize << i) != 0)
                .map(|i| simplex[i].clone() * (deltas[[s, i]].clone() / delta_total.clone()))
                .sum();

            return offset.norm();
        }

        panic!("Numerical instability has caused the algorithm to progress to this point. \
                Someone needs to go find Sean and lock him in the basement until he tears the \
                backup procedure out of GJK and sticks it in here too. Or we could just try \
                returning zer. It's probably close enough not to matter.");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facet_to_plane_2d_1() {
        let facet = Facet![Point![0., 1.], Point![1., 0.]];
        let plane: Plane<_, _> = facet.into();

        assert!(plane.signed_distance(&Point![0., 0.]) * plane.signed_distance(&Point![1., 1.]) <
                0.);
    }

    #[test]
    fn facet_to_plane_3d_1() {
        let facet = Facet![Point![0.472077, 0.063314, 0.029606],
                           Point![0.606915, 0.641988, 0.167560],
                           Point![0.118838, 0.496147, 0.367041]];
        let plane: Plane<_, _> = facet.into();

        assert!(plane.signed_distance(&Point![0.38620424999999997, 0.44893725000000007, 0.239815]) *
                plane.signed_distance(&Point![0.554433, 0.549847, 0.032239]) < 0.);
    }
}
