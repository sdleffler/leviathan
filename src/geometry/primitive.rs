use std::ops::{Add, Sub, Index, IndexMut};

use iter_exact::{ChainExactExt, CollectExactExt, FromExactSizeIterator};

use linalg::*;
use linalg::solve::gaussian::GaussianNullspaceExt;
use num::traits::Float;
use typehack::data::*;
use typehack::dim::{Dim, DimMul};


#[macro_export]
macro_rules! Point {
    ($($xs:expr),* $(,)*) => (Point { vect: Vect![$($xs),*] });
}


#[macro_export]
macro_rules! Simplex {
    ($($pts:expr),* $(,)*) => (Simplex { points: data![$($pts),*] });
}


#[macro_export]
macro_rules! Facet {
    ($($pts:expr),* $(,)*) => (Facet { points: data![$($pts),*] });
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
    pub fn signed_distance(&self, p: Point<T, N>) -> T {
        (p - &self.p0).component(self.n.clone())
    }
}


impl<T: Clone + Scalar, N: Dim> From<Facet<T, N>> for Plane<T, N> {
    fn from(facet: Facet<T, N>) -> Plane<T, N> {
        let mut iter = facet.into_iter();
        let p0 = iter.next().unwrap();

        let mat = Mat::<T, N::Pred, N, Row>::from_rows(iter.map(|p| p - p0.clone()));
        let n = mat.ge_null_elem();

        Plane { n: n, p0: p0 }
    }
}


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Simplex<T: Scalar, N: Dim> {
    points: Data<Point<T, N>, N::Succ>,
}


impl<T: Copy + Scalar, N: Dim> Copy for Simplex<T, N> where Data<Point<T, N>, N::Succ>: Copy {}


impl<T: Clone + Scalar, N: Dim> Simplex<T, N> {
    pub fn dims(&self) -> N::Succ {
        self.points.size
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
    type IntoIter = IntoIter<Point<T, N>, N>;

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
