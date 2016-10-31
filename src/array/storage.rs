use std;
use std::ops::{Deref, DerefMut};
use std::slice;

use typehack::peano::*;


#[macro_export]
macro_rules! pairs {
    ($elem:expr $(, $rest:expr)*) => ($crate::array::storage::Pair { l: $elem, r: pairs!($($rest),*) });
    () => (());
}


#[macro_export]
macro_rules! storage {
    ($($elem:expr),*) => ($crate::array::storage::Storage::<_, peano_count!($($elem),*)>::new(pairs!($($elem),*)));
}


#[repr(C)]
pub struct Pair<T, U> {
    pub l: T,
    pub r: U,
}


pub trait Link<T> {
    type Reify;
}


impl<T> Link<T> for Z {
    type Reify = ();
}


impl<T, N: Nat + Link<T>> Link<T> for S<N> {
    type Reify = Pair<T, N::Reify>;
}


#[repr(C)]
pub struct Storage<T, D: Nat + Link<T>> {
    data: D::Reify,
}


impl<T, D: Nat + Link<T>> Storage<T, D> {
    pub fn new(data: D::Reify) -> Storage<T, D> {
        Storage { data: data }
    }
}


impl<T, D: Nat + Link<T>> Deref for Storage<T, D> {
    type Target = [T];

    fn deref<'a>(&'a self) -> &'a [T] {
        unsafe {
            slice::from_raw_parts(std::mem::transmute::<*const Storage<T, D>,
                                                        *const T>(self as *const Storage<T, D>),
                                  D::as_usize())
        }
    }
}


impl<T, D: Nat + Link<T>> DerefMut for Storage<T, D> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut [T] {
        unsafe {
            slice::from_raw_parts_mut(std::mem::transmute::<*mut Storage<T, D>,
                                                            *mut T>(self as *mut Storage<T, D>),
                                      D::as_usize())
        }
    }
}
