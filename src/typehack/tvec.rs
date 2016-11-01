use std::mem;
use std::ops::{Deref, DerefMut};
use std::slice;

use typehack::peano::*;


#[repr(C)]
pub struct TCons<E, N> {
    pub elem: E,
    pub next: N,
}
pub struct TNil;


pub trait TIndex<N: Nat> {
    type Result;

    fn value(&self) -> &Self::Result;
}


impl<E, I: Nat, N: TIndex<I>> TIndex<S<I>> for TCons<E, N> {
    type Result = <N as TIndex<I>>::Result;

    fn value(&self) -> &Self::Result {
        self.next.value()
    }
}


impl<E, N> TIndex<Z> for TCons<E, N> {
    type Result = E;

    fn value(&self) -> &Self::Result {
        &self.elem
    }
}


pub trait Homogeneous<E> {
    type Length: Nat;
}


impl<E, N: Homogeneous<E>> Homogeneous<E> for TCons<E, N> {
    type Length = S<N::Length>;
}

impl<E> Homogeneous<E> for TNil {
    type Length = Z;
}


impl<E, N: Homogeneous<E>> Deref for TCons<E, N> {
    type Target = [E];

    fn deref<'a>(&'a self) -> &'a [E] {
        unsafe { slice::from_raw_parts(mem::transmute::<&'a Self, *const E>(self),
                              <Self as Homogeneous<E>>::Length::as_usize()) }
    }
}


impl<E, N: Homogeneous<E>> DerefMut for TCons<E, N> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut [E] {
        unsafe { slice::from_raw_parts_mut(mem::transmute::<&'a mut Self, *mut E>(self),
                              <Self as Homogeneous<E>>::Length::as_usize()) }
    }
}
