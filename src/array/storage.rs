use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
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


#[derive(Copy, Clone)]
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


impl<T: Clone, D: Nat + Link<T>> Clone for Storage<T, D> {
    fn clone(&self) -> Self {
        let mut data: Self;

        unsafe {
            data = mem::uninitialized();

            for (i, v) in self.iter().enumerate() {
                ptr::write(&mut data[i], v.clone());
            }
        }

        data
    }
}


impl<T: Copy, D: Nat + Link<T>> Copy for Storage<T, D> where D::Reify: Copy {}


impl<T, D: Nat + Link<T>> Storage<T, D> {
    pub fn new(data: D::Reify) -> Storage<T, D> {
        Storage { data: data }
    }


    pub fn from_elem(elem: &T) -> Storage<T, D>
        where T: Clone
    {
        let mut storage: Self;

        unsafe {
            storage = mem::uninitialized();

            for loc in storage.iter_mut() {
                ptr::write(loc, elem.clone());
            }
        }

        storage
    }


    pub fn from_slice(slice: &[T]) -> Storage<T, D>
        where T: Clone
    {
        assert_eq!(slice.len(), D::as_usize());

        let mut storage: Self;

        unsafe {
            storage = mem::uninitialized();

            for (i, loc) in storage.iter_mut().enumerate() {
                ptr::write(loc, slice[i].clone());
            }
        }

        storage
    }


    pub fn from_fn<F: Fn(usize) -> T>(f: F) -> Storage<T, D> {
        let mut storage: Self;

        unsafe {
            storage = mem::uninitialized();

            for (i, loc) in storage.iter_mut().enumerate() {
                ptr::write(loc, f(i));
            }
        }

        storage
    }
}


impl<T, D: Nat + Link<T>> Deref for Storage<T, D> {
    type Target = [T];

    fn deref<'a>(&'a self) -> &'a [T] {
        unsafe {
            slice::from_raw_parts(mem::transmute::<*const Storage<T, D>,
                                                   *const T>(self as *const Storage<T, D>),
                                  D::as_usize())
        }
    }
}


impl<T, D: Nat + Link<T>> DerefMut for Storage<T, D> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut [T] {
        unsafe {
            slice::from_raw_parts_mut(mem::transmute::<*mut Storage<T, D>,
                                                       *mut T>(self as *mut Storage<T, D>),
                                      D::as_usize())
        }
    }
}


impl<T: fmt::Debug, D: Nat + Link<T>> fmt::Debug for Storage<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(f)
    }
}


impl<T: PartialEq, D: Nat + Link<T>> PartialEq for Storage<T, D> {
    fn eq(&self, rhs: &Self) -> bool {
        self.deref() == rhs.deref()
    }
}


impl<T: Eq, D: Nat + Link<T>> Eq for Storage<T, D> {}
