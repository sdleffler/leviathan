use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::slice;

use typehack::binary::*;
use typehack::dim::*;


#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node<E, C> {
    e: E,
    l: C,
    r: C,
}


pub trait Raw<E>: Nat {
    type Data;
}


impl<E> Raw<E> for () {
    type Data = ();
}

impl<E, N: Nat> Raw<E> for O<N>
    where N: Raw<E>
{
    type Data = Node<(), <N as Raw<E>>::Data>;
}

impl<E, N: Nat> Raw<E> for I<N>
    where N: Raw<E>
{
    type Data = Node<E, <N as Raw<E>>::Data>;
}


pub trait Store<E>: Dim {
    type Reify;

    unsafe fn uninitialized(self) -> Self::Reify;
    unsafe fn into_slice<'a>(self, &'a Self::Reify) -> &'a [E];
    unsafe fn into_mut_slice<'a>(self, &'a mut Self::Reify) -> &'a mut [E];
}


impl<E> Store<E> for Dyn {
    type Reify = Vec<E>;

    unsafe fn uninitialized(self) -> Self::Reify {
        let mut vec = Vec::with_capacity(self.reify());
        vec.set_len(self.reify());
        vec
    }

    unsafe fn into_slice<'a>(self, r: &'a Self::Reify) -> &'a [E] {
        &r[..self.reify()]
    }

    unsafe fn into_mut_slice<'a>(self, r: &'a mut Self::Reify) -> &'a mut [E] {
        &mut r[..self.reify()]
    }
}


impl<E, N: Nat> Store<E> for N
    where N: Raw<E> + NatSub<B32>
{
    type Reify = Box<N::Data>;

    unsafe fn uninitialized(self) -> Self::Reify {
        Box::new(mem::uninitialized())
    }

    unsafe fn into_slice<'a>(self, r: &'a Box<N::Data>) -> &'a [E] {
        slice::from_raw_parts(mem::transmute::<&N::Data, *const E>(r.as_ref()),
                              self.reify())
    }

    unsafe fn into_mut_slice<'a>(self, r: &mut Box<N::Data>) -> &'a mut [E] {
        slice::from_raw_parts_mut(mem::transmute::<&mut N::Data, *mut E>(r.as_mut()),
                                  self.reify())
    }
}


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<E, N: Nat> Store<E> for N
    where N: Raw<E>
{
    default type Reify = N::Data;

    default unsafe fn uninitialized(self) -> Self::Reify {
        mem::uninitialized()
    }

    default unsafe fn into_slice<'a>(self, p: &'a Self::Reify) -> &'a [E] {
        slice::from_raw_parts(mem::transmute::<&Self::Reify, *const E>(p), self.reify())
    }

    default unsafe fn into_mut_slice<'a>(self, p: &'a mut Self::Reify) -> &'a mut [E] {
        slice::from_raw_parts_mut(mem::transmute::<&mut Self::Reify, *mut E>(p), self.reify())
    }
}


pub struct Data<T, S: Store<T>> {
    pub size: S,
    data: S::Reify,
}


impl<T: Clone, S: Store<T>> Clone for Data<T, S>
    where S::Reify: Clone
{
    fn clone(&self) -> Self {
        Data {
            size: self.size,
            data: self.data.clone(),
        }
    }
}


impl<T, S: Store<T>> Deref for Data<T, S> {
    type Target = [T];

    fn deref<'a>(&'a self) -> &'a [T] {
        unsafe { self.size.into_slice(&self.data) }
    }
}


impl<T, S: Store<T>> DerefMut for Data<T, S> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut [T] {
        unsafe { self.size.into_mut_slice(&mut self.data) }
    }
}


impl<T, S: Store<T>> Data<T, S> {
    pub fn from_elem(s: S, elem: &T) -> Data<T, S>
        where T: Clone
    {
        let mut data: Self;

        unsafe {
            data = Data {
                size: s,
                data: s.uninitialized(),
            };

            for loc in data.iter_mut() {
                ptr::write(loc, elem.clone());
            }
        }

        data
    }


    pub fn from_slice(s: S, slice: &[T]) -> Data<T, S>
        where T: Clone
    {
        assert_eq!(slice.len(), s.reify());

        let mut data: Self;

        unsafe {
            data = Data {
                size: s,
                data: s.uninitialized(),
            };

            for (i, loc) in data.iter_mut().enumerate() {
                ptr::write(loc, slice[i].clone());
            }
        }

        data
    }


    pub fn from_fn<F: Fn(usize) -> T>(s: S, f: F) -> Data<T, S> {
        let mut data: Self;

        unsafe {
            data = Data {
                size: s,
                data: s.uninitialized(),
            };

            for (i, loc) in data.iter_mut().enumerate() {
                ptr::write(loc, f(i));
            }
        }

        data
    }


    pub unsafe fn uninitialized(s: S) -> Data<T, S> {
        Data {
            size: s,
            data: s.uninitialized(),
        }
    }
}


impl<T: PartialEq, N: Store<T>> PartialEq for Data<T, N> {
    fn eq(&self, rhs: &Self) -> bool {
        self.deref() == rhs.deref()
    }
}

impl<T: Eq, N: Store<T>> Eq for Data<T, N> {}


impl<T: fmt::Debug, N: Store<T>> fmt::Debug for Data<T, N> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(fmt)
    }
}


#[cfg(test)]
mod tests {
    use std::mem;

    use typehack::binary::*;

    use super::*;

    #[test]
    fn data_create_p1() {
        let _: Data<i32, B1> = Data::from_elem(B1::as_data(), &0);
        assert_eq!(mem::size_of::<Data<i32, B1>>(), mem::size_of::<i32>());
    }

    #[test]
    fn data_create_p2() {
        let _: Data<i32, B2> = Data::from_elem(B2::as_data(), &0);
        assert_eq!(mem::size_of::<Data<i32, B2>>(), mem::size_of::<i32>() * 2);
    }

    #[test]
    fn data_create_p4() {
        let _: Data<i32, B4> = Data::from_elem(B4::as_data(), &0);
        assert_eq!(mem::size_of::<Data<i32, B4>>(), mem::size_of::<i32>() * 4);
    }

    #[test]
    fn data_create_p8() {
        let _: Data<i32, B8> = Data::from_elem(B8::as_data(), &0);
        assert_eq!(mem::size_of::<Data<i32, B8>>(), mem::size_of::<i32>() * 8);
    }

    #[test]
    fn data_create_p16() {
        let _: Data<i32, B16> = Data::from_elem(B16::as_data(), &0);
        assert_eq!(mem::size_of::<Data<i32, B16>>(), mem::size_of::<i32>() * 16);
    }

    #[test]
    fn data_create_p32() {
        let _: Data<i32, B32> = Data::from_elem(B32::as_data(), &0);
        assert_eq!(mem::size_of::<Data<i32, B32>>(), mem::size_of::<Box<i32>>());
    }

    #[test]
    fn data_create_p64() {
        let _: Data<i32, B64> = Data::from_elem(B64::as_data(), &0);
        assert_eq!(mem::size_of::<Data<i32, B64>>(), mem::size_of::<Box<i32>>());
    }
}
