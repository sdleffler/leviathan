use std::iter::{FromIterator, IntoIterator};
use std::mem;
use std::ops::{AddAssign, SubAssign, Mul, MulAssign, Deref, DerefMut, Index, IndexMut};

use array::storage::*;
use linalg::traits::*;
use num::traits::*;
use typehack::peano::*;
use typehack::dim::*;
use typehack::void::*;


#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct StaticVec<T, N: Nat>
    where N: Link<T>
{
    elems: Storage<T, N>,
}


impl<T: Copy, N: Nat + Link<T>> Copy for StaticVec<T, N>
    where StaticVec<T, N>: Clone,
          Storage<T, N>: Copy
{
}


impl<'a, T: Copy, N: Nat> From<&'a [T]> for StaticVec<T, N>
    where N: Link<T>
{
    fn from(slice: &[T]) -> Self {
        StaticVec { elems: Storage::from_slice(slice) }
    }
}


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DynamicVec<T, N: Dim> {
    size: N,
    elems: Vec<T>,
}


impl<'a, T: Copy> From<&'a [T]> for DynamicVec<T, Dyn> {
    fn from(slice: &[T]) -> Self {
        DynamicVec {
            size: Dyn(slice.len()),
            elems: Vec::from(slice),
        }
    }
}


impl<'a, T: Copy, N: Nat> From<&'a [T]> for DynamicVec<T, N> {
    fn from(slice: &[T]) -> Self {
        DynamicVec {
            size: N::as_data(),
            elems: Vec::from(&slice[..N::as_usize()]),
        }
    }
}


impl<T: Copy> FromIterator<T> for DynamicVec<T, Dyn> {
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item = T>
    {
        let elems: Vec<_> = iter.into_iter().collect();
        DynamicVec {
            size: Dyn(elems.len()),
            elems: elems,
        }
    }
}


impl<T> Deref for StaticVec<T, S<Z>> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        &self.elems[0]
    }
}


impl<T> Deref for DynamicVec<T, S<Z>> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        &self.elems[0]
    }
}


impl<T> DerefMut for StaticVec<T, S<Z>> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        &mut self.elems[0]
    }
}


impl<T> DerefMut for DynamicVec<T, S<Z>> {
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


impl<T> Deref for StaticVec<T, S<S<Z>>> {
    type Target = Vec2View<T>;

    fn deref<'a>(&'a self) -> &'a Vec2View<T> {
        unsafe { mem::transmute::<&'a StaticVec<T, S<S<Z>>>, &'a Vec2View<T>>(self) }
    }
}


impl<T> Deref for DynamicVec<T, S<S<Z>>> {
    type Target = Vec2View<T>;

    fn deref<'a>(&'a self) -> &'a Vec2View<T> {
        unsafe { mem::transmute::<*const T, &'a Vec2View<T>>(self.elems.as_slice().as_ptr()) }
    }
}


impl<T> DerefMut for StaticVec<T, S<S<Z>>> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec2View<T> {
        unsafe { mem::transmute::<&'a mut StaticVec<T, S<S<Z>>>, &'a mut Vec2View<T>>(self) }
    }
}


impl<T> DerefMut for DynamicVec<T, S<S<Z>>> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec2View<T> {
        unsafe {
            mem::transmute::<*mut T, &'a mut Vec2View<T>>(self.elems.as_mut_slice().as_mut_ptr())
        }
    }
}


#[repr(C)]
pub struct Vec3View<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    void: Void,
}


impl<T> Deref for StaticVec<T, S<S<S<Z>>>> {
    type Target = Vec3View<T>;

    fn deref<'a>(&'a self) -> &'a Vec3View<T> {
        unsafe { mem::transmute::<&'a StaticVec<T, S<S<S<Z>>>>, &'a Vec3View<T>>(self) }
    }
}


impl<T> Deref for DynamicVec<T, S<S<S<Z>>>> {
    type Target = Vec3View<T>;

    fn deref<'a>(&'a self) -> &'a Vec3View<T> {
        unsafe { mem::transmute::<*const T, &'a Vec3View<T>>(self.elems.as_slice().as_ptr()) }
    }
}


impl<T> DerefMut for StaticVec<T, S<S<S<Z>>>> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec3View<T> {
        unsafe { mem::transmute::<&'a mut StaticVec<T, S<S<S<Z>>>>, &'a mut Vec3View<T>>(self) }
    }
}


impl<T> DerefMut for DynamicVec<T, S<S<S<Z>>>> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec3View<T> {
        unsafe {
            mem::transmute::<*mut T, &'a mut Vec3View<T>>(self.elems.as_mut_slice().as_mut_ptr())
        }
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


impl<T> Deref for StaticVec<T, S<S<S<S<Z>>>>> {
    type Target = Vec4View<T>;

    fn deref<'a>(&'a self) -> &'a Vec4View<T> {
        unsafe { mem::transmute::<&'a StaticVec<T, S<S<S<S<Z>>>>>, &'a Vec4View<T>>(self) }
    }
}


impl<T> Deref for DynamicVec<T, S<S<S<S<Z>>>>> {
    type Target = Vec4View<T>;

    fn deref<'a>(&'a self) -> &'a Vec4View<T> {
        unsafe { mem::transmute::<*const T, &'a Vec4View<T>>(self.elems.as_slice().as_ptr()) }
    }
}


impl<T> DerefMut for StaticVec<T, S<S<S<S<Z>>>>> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec4View<T> {
        unsafe { mem::transmute::<&'a mut StaticVec<T, S<S<S<S<Z>>>>>, &'a mut Vec4View<T>>(self) }
    }
}


impl<T> DerefMut for DynamicVec<T, S<S<S<S<Z>>>>> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec4View<T> {
        unsafe {
            mem::transmute::<*mut T, &'a mut Vec4View<T>>(self.elems.as_mut_slice().as_mut_ptr())
        }
    }
}


impl<T: Copy, N: Nat + Link<T>> Vector for StaticVec<T, N> {
    type Dims = N;

    type Scalar = T;
}


impl<T: Copy, N: Nat + Link<T>> VectorAdd for StaticVec<T, N>
    where T: AddAssign
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        for i in 0..N::as_usize() {
            self.elems[i] += rhs.elems[i];
        }

        self
    }
}


impl<T: Copy, N: Nat + Link<T>> VectorSub for StaticVec<T, N>
    where T: SubAssign
{
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self {
        for i in 0..N::as_usize() {
            self.elems[i] -= rhs.elems[i];
        }

        self
    }
}


impl<T: Copy, N: Nat + Link<T>> VectorHadamard for StaticVec<T, N>
    where T: MulAssign
{
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self {
        for i in 0..N::as_usize() {
            self.elems[i] *= rhs.elems[i];
        }

        self
    }
}


impl<T: Copy, N: Nat + Link<T>> VectorDot for StaticVec<T, N>
    where T: Mul,
          <T as Mul>::Output: Zero + AddAssign
{
    type Output = <T as Mul>::Output;

    fn dot(self, rhs: Self) -> Self::Output {
        let mut accum = <T as Mul>::Output::zero();

        let lhs = &self.elems[..N::as_usize()];
        let rhs = &rhs.elems[..N::as_usize()];

        for i in 0..N::as_usize() {
            accum += lhs[i] * rhs[i];
        }

        accum
    }
}


impl<'a, 'b, T: Copy, N: Nat + Link<T>> VectorDot<&'b StaticVec<T, N>> for &'a StaticVec<T, N>
    where T: Mul,
          <T as Mul>::Output: Zero + AddAssign
{
    type Output = <T as Mul>::Output;

    fn dot(self, rhs: &'b StaticVec<T, N>) -> Self::Output {
        let mut accum = <T as Mul>::Output::zero();

        let lhs = &self.elems[..N::as_usize()];
        let rhs = &rhs.elems[..N::as_usize()];

        for i in 0..N::as_usize() {
            accum += lhs[i] * rhs[i];
        }

        accum
    }
}


impl<T: Copy, N: Nat + Link<T>> Index<usize> for StaticVec<T, N> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        &self.elems[idx]
    }
}


impl<T: Copy, N: Nat + Link<T>> IndexMut<usize> for StaticVec<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        &mut self.elems[idx]
    }
}
