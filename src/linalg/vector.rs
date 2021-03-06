use std::cmp;
use std::iter::{FromIterator, Sum};
use std::mem;
use std::ops::{Add, Sub, Mul, MulAssign, Div, Neg, Deref, DerefMut, Index, IndexMut};
use std::ptr;

use void::Void;

use iter_exact::{CollectExactExt, FromExactSizeIterator};
use linalg::matrix::*;
use linalg::traits::*;
use num::traits::*;
use typehack::binary::*;
use typehack::data::*;
use typehack::dim::*;


#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct DenseVec<T: Scalar, N: Size<T>> {
    elems: Data<T, N>,
}


impl<T: Copy + Scalar, N: Size<T>> Copy for DenseVec<T, N> where Data<T, N>: Copy {}


impl<T: Scalar, N: Size<T>> DenseVec<T, N> {
    pub fn as_column<L: Layout>(self) -> DenseMat<T, N, I, L>
        where N: DimMul<I, Result = N>
    {
        DenseMat::from_data(self.elems.size(), I::as_data(), self.elems)
    }


    pub fn as_row<L: Layout>(self) -> DenseMat<T, I, N, L>
        where I: DimMul<N, Result = N>
    {
        DenseMat::from_data(I::as_data(), self.elems.size(), self.elems)
    }


    pub fn from_elem(size: N, elem: &T) -> Self
        where T: Clone
    {
        DenseVec { elems: Data::from_elem(size, elem) }
    }


    pub fn from_fn<F: Fn(usize) -> T>(size: N, f: F) -> Self {
        DenseVec { elems: Data::from_fn(size, f) }
    }


    pub fn from_data(data: Data<T, N>) -> Self {
        DenseVec { elems: data }
    }


    pub fn len(&self) -> usize {
        self.elems.len()
    }


    pub fn size(&self) -> N {
        self.elems.size()
    }


    pub fn as_slice(&self) -> &[T] {
        self.elems.deref()
    }


    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.elems.deref_mut()
    }


    pub fn augment<M: Size<T>, P: Size<T>>(self, rhs: DenseVec<T, M>) -> DenseVec<T, P>
        where N: DimAdd<M, Result = P>
    {
        let mut result;

        unsafe {
            result = DenseVec { elems: Data::uninitialized(self.size().add(rhs.size())) };
            let slen = self.len();

            for (i, elem) in self.elems.into_iter().enumerate() {
                ptr::write(&mut result[i], elem);
            }

            for (j, elem) in rhs.elems.into_iter().enumerate() {
                ptr::write(&mut result[slen + j], elem);
            }
        }

        result
    }


    pub fn zero_extend<M: Size<T>>(self, size: M) -> DenseVec<T, M> {
        let mut result;

        unsafe {
            result = DenseVec { elems: Data::uninitialized(size) };
            let slen = self.len();

            assert!(slen <= size.reify());

            for (i, elem) in self.elems.into_iter().enumerate() {
                ptr::write(&mut result[i], elem);
            }

            for i in slen..size.reify() {
                ptr::write(&mut result[i], T::zero());
            }
        }

        result
    }
}


impl<T: Scalar, N: Size<T>> From<Data<T, N>> for DenseVec<T, N> {
    fn from(data: Data<T, N>) -> Self {
        DenseVec { elems: data }
    }
}


impl<'a, T: Clone + Scalar, N: Size<T>> From<&'a [T]> for DenseVec<T, N> {
    fn from(slice: &[T]) -> Self {
        DenseVec { elems: Data::from_slice(N::from_usize(slice.len()), slice) }
    }
}


impl<T: Scalar> Deref for DenseVec<T, B1> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        &self.elems[0]
    }
}


impl<T: Scalar> DerefMut for DenseVec<T, B1> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        &mut self.elems[0]
    }
}


#[repr(C)]
pub struct Vec2View<T> {
    pub x: T,
    pub y: T,
    void: Void,
}


impl<T: Scalar> Deref for DenseVec<T, B2> {
    type Target = Vec2View<T>;

    fn deref<'a>(&'a self) -> &'a Vec2View<T> {
        unsafe { mem::transmute::<&'a DenseVec<T, B2>, &'a Vec2View<T>>(self) }
    }
}


impl<T: Scalar> DerefMut for DenseVec<T, B2> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec2View<T> {
        unsafe { mem::transmute::<&'a mut DenseVec<T, B2>, &'a mut Vec2View<T>>(self) }
    }
}


#[repr(C)]
pub struct Vec3View<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    void: Void,
}


impl<T: Scalar> Deref for DenseVec<T, B3> {
    type Target = Vec3View<T>;

    fn deref<'a>(&'a self) -> &'a Vec3View<T> {
        unsafe { mem::transmute::<&'a DenseVec<T, B3>, &'a Vec3View<T>>(self) }
    }
}


impl<T: Scalar> DerefMut for DenseVec<T, B3> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec3View<T> {
        unsafe { mem::transmute::<&'a mut DenseVec<T, B3>, &'a mut Vec3View<T>>(self) }
    }
}


#[repr(C)]
pub struct Vec4View<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
    void: Void,
}


impl<T: Scalar> Deref for DenseVec<T, B4> {
    type Target = Vec4View<T>;

    fn deref<'a>(&'a self) -> &'a Vec4View<T> {
        unsafe { mem::transmute::<&'a DenseVec<T, B4>, &'a Vec4View<T>>(self) }
    }
}


impl<T: Scalar> DerefMut for DenseVec<T, B4> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec4View<T> {
        unsafe { mem::transmute::<&'a mut DenseVec<T, B4>, &'a mut Vec4View<T>>(self) }
    }
}


impl<T: Scalar, N: Size<T>> Vector for DenseVec<T, N> {
    type Dims = N;

    type Scalar = T;

    fn size(&self) -> N {
        self.elems.size()
    }
}


impl<'a, T: Scalar, N: Size<T>> Vector for &'a DenseVec<T, N> {
    type Dims = N;

    type Scalar = T;

    fn size(&self) -> N {
        self.elems.size()
    }
}


impl<T: Scalar, N: Size<T>> Add for DenseVec<T, N> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        assert_eq!(self.elems.size(), rhs.elems.size());

        let n = cmp::min(self.elems.len(), rhs.elems.len());

        unsafe {
            for i in 0..n {
                self.elems[i] += ptr::read(&rhs.elems[i]);
            }

            mem::forget(rhs);
        }

        self
    }
}


impl<'a, T: Clone + Scalar, N: Size<T>> Add<&'a DenseVec<T, N>> for DenseVec<T, N> {
    type Output = Self;

    fn add(mut self, rhs: &'a Self) -> Self::Output {
        assert_eq!(self.elems.size(), rhs.elems.size());

        let n = cmp::min(self.elems.len(), rhs.elems.len());

        for i in 0..n {
            self.elems[i] += rhs.elems[i].clone();
        }

        self
    }
}


impl<'a, T: Clone + Scalar, N: Size<T>> Add<DenseVec<T, N>> for &'a DenseVec<T, N> {
    type Output = DenseVec<T, N>;

    fn add(self, mut rhs: DenseVec<T, N>) -> Self::Output {
        assert_eq!(self.elems.size(), rhs.elems.size());

        let n = cmp::min(self.elems.len(), rhs.elems.len());

        for i in 0..n {
            rhs.elems[i] += self.elems[i].clone();
        }

        rhs
    }
}


impl<'a, 'b, T: Clone + Scalar, N: Size<T>> Add<&'b DenseVec<T, N>> for &'a DenseVec<T, N> {
    type Output = DenseVec<T, N>;

    fn add(self, rhs: &'b DenseVec<T, N>) -> Self::Output {
        assert_eq!(self.elems.size(), rhs.elems.size());

        DenseVec::from_fn(self.elems.size(),
                          |i| self.elems[i].clone() + rhs.elems[i].clone())
    }
}


impl<T: Scalar, N: Size<T>> Sub for DenseVec<T, N> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self {
        assert_eq!(self.elems.size(), rhs.elems.size());

        let n = cmp::min(self.elems.len(), rhs.elems.len());

        unsafe {
            for i in 0..n {
                self.elems[i] -= ptr::read(&rhs.elems[i]);
            }

            mem::forget(rhs);
        }

        self
    }
}


impl<'a, T: Clone + Scalar, N: Size<T>> Sub<&'a DenseVec<T, N>> for DenseVec<T, N> {
    type Output = Self;

    fn sub(mut self, rhs: &'a Self) -> Self::Output {
        assert_eq!(self.elems.size(), rhs.elems.size());

        let n = cmp::min(self.elems.len(), rhs.elems.len());

        for i in 0..n {
            self.elems[i] -= rhs.elems[i].clone();
        }

        self
    }
}


impl<'a, T: Clone + Scalar, N: Size<T>> Sub<DenseVec<T, N>> for &'a DenseVec<T, N> {
    type Output = DenseVec<T, N>;

    fn sub(self, mut rhs: DenseVec<T, N>) -> Self::Output {
        assert_eq!(self.elems.size(), rhs.elems.size());

        let n = cmp::min(self.elems.len(), rhs.elems.len());

        for i in 0..n {
            rhs.elems[i] -= self.elems[i].clone();
        }

        rhs
    }
}


impl<'a, 'b, T: Clone + Scalar, N: Size<T>> Sub<&'b DenseVec<T, N>> for &'a DenseVec<T, N> {
    type Output = DenseVec<T, N>;

    fn sub(self, rhs: &'b DenseVec<T, N>) -> Self::Output {
        assert_eq!(self.elems.size(), rhs.elems.size());

        DenseVec::from_fn(self.elems.size(),
                          |i| self.elems[i].clone() - rhs.elems[i].clone())
    }
}


impl<T: Scalar, N: Size<T>> Mul for DenseVec<T, N> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self {
        assert_eq!(self.elems.size(), rhs.elems.size());

        let n = cmp::min(self.elems.len(), rhs.elems.len());

        unsafe {
            for i in 0..n {
                self.elems[i] *= ptr::read(&rhs.elems[i]);
            }

            mem::forget(rhs);
        }

        self
    }
}


impl<'a, 'b, T: Clone + Scalar, N: Size<T>> Mul<&'b DenseVec<T, N>> for &'a DenseVec<T, N> {
    type Output = DenseVec<T, N>;

    fn mul(self, rhs: &'b DenseVec<T, N>) -> Self::Output {
        assert_eq!(self.elems.size(), rhs.elems.size());

        DenseVec::from_fn(self.elems.size(),
                          |i| self.elems[i].clone() * rhs.elems[i].clone())
    }
}


impl<T: Clone + Scalar, N: Size<T>> MulAssign<T> for DenseVec<T, N> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..self.elems.len() {
            self.elems[i] *= rhs.clone();
        }
    }
}


impl<T: Clone + Scalar, N: Size<T>> Mul<T> for DenseVec<T, N> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self {
        self *= rhs;
        self
    }
}


impl<'a, T: Clone + Scalar, N: Size<T>> Mul<&'a T> for DenseVec<T, N> {
    type Output = Self;

    fn mul(mut self, rhs: &'a T) -> Self::Output {
        for i in 0..self.elems.len() {
            self.elems[i] *= rhs.clone();
        }

        self
    }
}


impl<'a, T: Clone + Scalar, N: Size<T>> Mul<T> for &'a DenseVec<T, N> {
    type Output = DenseVec<T, N>;

    fn mul(self, rhs: T) -> Self::Output {
        DenseVec::from_fn(self.elems.size(), |i| self.elems[i].clone() * rhs.clone())
    }
}


impl<'a, 'b, T: Clone + Scalar, N: Size<T>> Mul<&'b T> for &'a DenseVec<T, N> {
    type Output = DenseVec<T, N>;

    fn mul(self, rhs: &'b T) -> Self::Output {
        DenseVec::from_fn(self.elems.size(), |i| self.elems[i].clone() * rhs.clone())
    }
}


impl<T: Clone + Scalar, N: Size<T>> Div<T> for DenseVec<T, N> {
    type Output = Self;

    fn div(mut self, rhs: T) -> Self {
        for i in 0..self.elems.len() {
            self.elems[i] /= rhs.clone();
        }

        self
    }
}


impl<'a, T: Clone + Scalar, N: Size<T>> Div<&'a T> for DenseVec<T, N> {
    type Output = Self;

    fn div(mut self, rhs: &'a T) -> Self::Output {
        for i in 0..self.elems.len() {
            self.elems[i] /= rhs.clone();
        }

        self
    }
}


impl<'a, T: Clone + Scalar, N: Size<T>> Div<T> for &'a DenseVec<T, N> {
    type Output = DenseVec<T, N>;

    fn div(self, rhs: T) -> Self::Output {
        DenseVec::from_fn(self.elems.size(), |i| self.elems[i].clone() / rhs.clone())
    }
}


impl<'a, 'b, T: Clone + Scalar, N: Size<T>> Div<&'b T> for &'a DenseVec<T, N> {
    type Output = DenseVec<T, N>;

    fn div(self, rhs: &'b T) -> Self::Output {
        DenseVec::from_fn(self.elems.size(), |i| self.elems[i].clone() / rhs.clone())
    }
}


impl<T: Scalar, N: Size<T>> Neg for DenseVec<T, N> {
    type Output = DenseVec<T, N>;

    fn neg(mut self) -> Self::Output {
        unsafe {
            for elem in self.elems.iter_mut() {
                ptr::write(elem, -ptr::read(elem));
            }
        }

        self
    }
}


impl<'a, T: Clone + Scalar, N: Size<T>> Neg for &'a DenseVec<T, N> {
    type Output = DenseVec<T, N>;

    fn neg(self) -> Self::Output {
        DenseVec::from_fn(self.elems.size(), |i| -self.elems[i].clone())
    }
}


impl<T: Scalar, N: Size<T>> Dot for DenseVec<T, N> {
    fn dot(self, rhs: Self) -> T {
        assert_eq!(self.elems.size(), rhs.elems.size());

        let n = cmp::min(self.elems.len(), rhs.elems.len());
        let mut accum = T::zero();

        unsafe {
            {
                let lhs = &self.elems[..n];
                let rhs = &rhs.elems[..n];

                for i in 0..n {
                    accum += ptr::read(&lhs[i]) * ptr::read(&rhs[i]);
                }
            }

            mem::forget(self);
            mem::forget(rhs);
        }

        accum
    }
}


impl<'a, T: Clone + Scalar, N: Size<T>> Dot<&'a DenseVec<T, N>> for DenseVec<T, N> {
    fn dot(self, rhs: &'a DenseVec<T, N>) -> T {
        assert_eq!(self.elems.size(), rhs.elems.size());

        let n = cmp::min(self.elems.len(), rhs.elems.len());
        let mut accum = T::zero();

        unsafe {
            {
                let lhs = &self.elems[..n];
                let rhs = &rhs.elems[..n];

                for i in 0..n {
                    accum += lhs[i].clone() * ptr::read(&rhs[i]);
                }
            }

            mem::forget(rhs);
        }

        accum
    }
}


impl<'a, T: Clone + Scalar, N: Size<T>> Dot<DenseVec<T, N>> for &'a DenseVec<T, N> {
    fn dot(self, rhs: DenseVec<T, N>) -> T {
        assert_eq!(self.elems.size(), rhs.elems.size());

        let n = cmp::min(self.elems.len(), rhs.elems.len());
        let mut accum = T::zero();

        unsafe {
            {
                let lhs = &self.elems[..n];
                let rhs = &rhs.elems[..n];

                for i in 0..n {
                    accum += ptr::read(&lhs[i]) * rhs[i].clone();
                }
            }

            mem::forget(self);
        }

        accum
    }
}


impl<'a, 'b, T: Clone + Scalar, N: Size<T>> Dot<&'b DenseVec<T, N>> for &'a DenseVec<T, N> {
    fn dot(self, rhs: &'b DenseVec<T, N>) -> T {
        assert_eq!(self.elems.size(), rhs.elems.size());

        let n = cmp::min(self.elems.len(), rhs.elems.len());
        let mut accum = T::zero();

        let lhs = &self.elems[..n];
        let rhs = &rhs.elems[..n];

        for i in 0..n {
            accum += lhs[i].clone() * rhs[i].clone();
        }

        accum
    }
}


impl<T: Clone + Scalar + Float, N: Size<T>> VectorNorm for DenseVec<T, N> {
    fn norm(&self) -> T {
        let mut accum = T::zero();

        for elem in self.elems.iter() {
            accum += elem.clone() * elem.clone();
        }

        accum.sqrt()
    }
}


impl<T: Clone + Scalar, N: Size<T>> DenseVec<T, N> {
    pub fn project(self, rhs: Self) -> Self {
        let a_dot_b = self.dot(&rhs);
        let b_dot_b = (&rhs).dot(&rhs);
        rhs * (a_dot_b / b_dot_b)
    }


    pub fn squared(self) -> T {
        let mut accum = T::zero();

        for elem in self.elems {
            let clone = elem.clone();
            accum += elem * clone;
        }

        accum
    }
}


impl<T: Clone + Scalar + Float, N: Size<T>> DenseVec<T, N> {
    pub fn component(self, rhs: Self) -> T {
        let norm = rhs.norm();
        self.dot(rhs) / norm
    }
}


impl<T: Scalar, N: Size<T>> Index<usize> for DenseVec<T, N> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        &self.elems[idx]
    }
}


impl<T: Scalar, N: Size<T>> IndexMut<usize> for DenseVec<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        &mut self.elems[idx]
    }
}


impl<T: Scalar, N: Size<T>> IntoIterator for DenseVec<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.elems.into_iter()
    }
}


impl<T: Scalar> FromIterator<T> for DenseVec<T, Dyn> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        DenseVec { elems: iter.into_iter().collect() }
    }
}


impl<T: Scalar, N: Size<T>> FromExactSizeIterator<T> for DenseVec<T, N> {
    fn from_exact_size_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
        where I::IntoIter: ExactSizeIterator
    {
        DenseVec { elems: iter.into_iter().collect_exact() }
    }
}


impl<T: Scalar, N: Size<T>> Sum for DenseVec<T, N> {
    fn sum<I: Iterator<Item = DenseVec<T, N>>>(mut iter: I) -> DenseVec<T, N> {
        if let Some(first) = iter.next() {
            iter.fold(first, |a, b| a + b)
        } else {
            DenseVec::from_fn(N::from_usize(0), |_| T::zero())
        }
    }
}
