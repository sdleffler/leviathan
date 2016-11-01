use std::mem;
use std::ops::{AddAssign, SubAssign, Mul, MulAssign, Deref, DerefMut, Index, IndexMut};

use array::storage::*;
use num::traits::*;
use typehack::peano::*;
use typehack::void::*;
use vector::traits::*;


#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct DenseVec<T, N: Nat>
    where N: Link<T>
{
    data: Storage<T, N>,
}


impl<T> Deref for DenseVec<T, S<Z>> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        unsafe { mem::transmute::<&'a DenseVec<T, S<Z>>, &'a T>(self) }
    }
}


impl<T> DerefMut for DenseVec<T, S<Z>> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { mem::transmute::<&'a mut DenseVec<T, S<Z>>, &'a mut T>(self) }
    }
}


#[repr(C)]
pub struct Vec2View<T> {
    pub x: T,
    pub y: T,
    void: Void,
}


impl<T> Deref for DenseVec<T, S<S<Z>>> {
    type Target = Vec2View<T>;

    fn deref<'a>(&'a self) -> &'a Vec2View<T> {
        unsafe { mem::transmute::<&'a DenseVec<T, S<S<Z>>>, &'a Vec2View<T>>(self) }
    }
}


impl<T> DerefMut for DenseVec<T, S<S<Z>>> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec2View<T> {
        unsafe { mem::transmute::<&'a mut DenseVec<T, S<S<Z>>>, &'a mut Vec2View<T>>(self) }
    }
}


#[repr(C)]
pub struct Vec3View<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    void: Void,
}


impl<T> Deref for DenseVec<T, S<S<S<Z>>>> {
    type Target = Vec3View<T>;

    fn deref<'a>(&'a self) -> &'a Vec3View<T> {
        unsafe { mem::transmute::<&'a DenseVec<T, S<S<S<Z>>>>, &'a Vec3View<T>>(self) }
    }
}


impl<T> DerefMut for DenseVec<T, S<S<S<Z>>>> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec3View<T> {
        unsafe { mem::transmute::<&'a mut DenseVec<T, S<S<S<Z>>>>, &'a mut Vec3View<T>>(self) }
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


impl<T> Deref for DenseVec<T, S<S<S<S<Z>>>>> {
    type Target = Vec4View<T>;

    fn deref<'a>(&'a self) -> &'a Vec4View<T> {
        unsafe { mem::transmute::<&'a DenseVec<T, S<S<S<S<Z>>>>>, &'a Vec4View<T>>(self) }
    }
}


impl<T> DerefMut for DenseVec<T, S<S<S<S<Z>>>>> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec4View<T> {
        unsafe { mem::transmute::<&'a mut DenseVec<T, S<S<S<S<Z>>>>>, &'a mut Vec4View<T>>(self) }
    }
}


impl<T: Copy, N: Nat + Link<T>> Copy for DenseVec<T, N>
    where DenseVec<T, N>: Clone,
          Storage<T, N>: Copy
{
}


impl<'a, T: Copy, N: Nat> From<&'a [T]> for DenseVec<T, N>
    where N: Link<T>
{
    fn from(slice: &[T]) -> Self {
        DenseVec { data: Storage::from_slice(slice) }
    }
}


impl<T: Copy, N: Nat + Link<T>> Vector for DenseVec<T, N> {
    type Dims = N;

    type Scalar = T;
}


impl<T: Copy, N: Nat + Link<T>> VectorAdd for DenseVec<T, N>
    where T: AddAssign
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        for i in 0..N::as_usize() {
            self.data[i] += rhs.data[i];
        }

        self
    }
}


impl<T: Copy, N: Nat + Link<T>> VectorSub for DenseVec<T, N>
    where T: SubAssign
{
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self {
        for i in 0..N::as_usize() {
            self.data[i] -= rhs.data[i];
        }

        self
    }
}


impl<T: Copy, N: Nat + Link<T>> VectorHadamard for DenseVec<T, N>
    where T: MulAssign
{
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self {
        for i in 0..N::as_usize() {
            self.data[i] *= rhs.data[i];
        }

        self
    }
}


impl<T: Copy, N: Nat + Link<T>> VectorDot for DenseVec<T, N>
    where T: Mul,
          <T as Mul>::Output: Zero + AddAssign
{
    type Output = <T as Mul>::Output;

    fn dot(self, rhs: Self) -> Self::Output {
        let mut accum = <T as Mul>::Output::zero();

        let lhs = &self.data[..N::as_usize()];
        let rhs = &rhs.data[..N::as_usize()];

        for i in 0..N::as_usize() {
            accum += lhs[i] * rhs[i];
        }

        accum
    }
}


impl<'a, 'b, T: Copy, N: Nat + Link<T>> VectorDot<&'b DenseVec<T, N>> for &'a DenseVec<T, N>
    where T: Mul,
          <T as Mul>::Output: Zero + AddAssign
{
    type Output = <T as Mul>::Output;

    fn dot(self, rhs: &'b DenseVec<T, N>) -> Self::Output {
        let mut accum = <T as Mul>::Output::zero();

        let lhs = &self.data[..N::as_usize()];
        let rhs = &rhs.data[..N::as_usize()];

        for i in 0..N::as_usize() {
            accum += lhs[i] * rhs[i];
        }

        accum
    }
}


impl<T: Copy, N: Nat + Link<T>> Index<usize> for DenseVec<T, N> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        &self.data[idx]
    }
}


impl<T: Copy, N: Nat + Link<T>> IndexMut<usize> for DenseVec<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        &mut self.data[idx]
    }
}
