use std::ops::{Add, Sub, Index, IndexMut};

use linalg::*;
use num::traits::Float;
use typehack::data::*;


#[derive(PartialEq, Eq, Debug)]
pub struct Point<T: Scalar, N: Size<T>> {
    vect: Vect<T, N>,
}


impl<T: Clone + Scalar, N: Size<T>> Clone for Point<T, N>
    where Vect<T, N>: Clone
{
    fn clone(&self) -> Self {
        Point { vect: self.vect.clone() }
    }
}


impl<T: Copy + Scalar, N: Size<T>> Copy for Point<T, N> where Vect<T, N>: Copy {}


impl<T: Scalar, N: Size<T>> Point<T, N> {
    pub fn from_elem(size: N, elem: &T) -> Point<T, N>
        where T: Clone
    {
        Point { vect: Vect::from_elem(size, elem) }
    }


    pub fn len(&self) -> N {
        self.vect.len()
    }
}


impl<T: Scalar, N: Size<T>> Add<Vect<T, N>> for Point<T, N> {
    type Output = Point<T, N>;

    fn add(self, rhs: Vect<T, N>) -> Point<T, N> {
        Point { vect: self.vect + rhs }
    }
}


impl<T: Scalar, N: Size<T>> Add<Point<T, N>> for Vect<T, N> {
    type Output = Point<T, N>;

    fn add(self, rhs: Point<T, N>) -> Point<T, N> {
        Point { vect: self + rhs.vect }
    }
}


impl<T: Scalar, N: Size<T>> Sub<Vect<T, N>> for Point<T, N> {
    type Output = Point<T, N>;

    fn sub(self, rhs: Vect<T, N>) -> Point<T, N> {
        Point { vect: self.vect - rhs }
    }
}


impl<T: Scalar, N: Size<T>> Sub<Point<T, N>> for Vect<T, N> {
    type Output = Point<T, N>;

    fn sub(self, rhs: Point<T, N>) -> Point<T, N> {
        Point { vect: self - rhs.vect }
    }
}


impl<T: Scalar, N: Size<T>> Sub for Point<T, N> {
    type Output = Vect<T, N>;

    fn sub(self, rhs: Self) -> Vect<T, N> {
        self.vect - rhs.vect
    }
}


impl<'a, T: Clone + Scalar, N: Size<T>> Sub<&'a Point<T, N>> for Point<T, N> {
    type Output = Vect<T, N>;

    fn sub(self, rhs: &Self) -> Vect<T, N> {
        self.vect - &rhs.vect
    }
}


impl<'a, T: Clone + Scalar, N: Size<T>> Sub<Point<T, N>> for &'a Point<T, N> {
    type Output = Vect<T, N>;

    fn sub(self, rhs: Point<T, N>) -> Vect<T, N> {
        &self.vect - rhs.vect
    }
}


impl<T: Scalar, N: Size<T>> Index<usize> for Point<T, N> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        &self.vect[idx]
    }
}


impl<T: Scalar, N: Size<T>> IndexMut<usize> for Point<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        &mut self.vect[idx]
    }
}


#[derive(PartialEq, Eq, Debug)]
pub struct Plane<T: Scalar, N: Size<T>> {
    n: Vect<T, N>,
    p0: Point<T, N>,
}


impl<T: Clone + Scalar, N: Size<T>> Clone for Plane<T, N>
    where Vect<T, N>: Clone
{
    fn clone(&self) -> Self {
        Plane {
            n: self.n.clone(),
            p0: self.p0.clone(),
        }
    }
}


impl<T: Copy + Scalar, N: Size<T>> Copy for Plane<T, N> where Vect<T, N>: Copy {}


impl<T: Clone + Scalar + Float, N: Size<T>> Plane<T, N>
    where Vect<T, N>: Clone
{
    pub fn signed_distance(&self, p: Point<T, N>) -> T {
        (p - &self.p0).component(self.n.clone())
    }
}
