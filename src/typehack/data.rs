use std::cmp;
use std::fmt;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::slice;

use unreachable::UncheckedOptionExt;

use iter_exact::FromExactSizeIterator;

use typehack::binary::*;
use typehack::dim::*;


#[macro_export]
macro_rules! data {
    (@assign $data:ident $n:expr => $x:expr $(, $xs:expr)*) => ($data[$n] = $x; data!(@assign $data ($n + 1) => $($xs),*));
    (@assign $data:ident $n:expr =>) => ();
    (@count $x:expr $(, $xs:expr)*) => (<data!(@count $($xs),*) as $crate::typehack::binary::Nat>::Succ);
    (@count) => ($crate::typehack::binary::O);
    ($($xs:expr),*) => ({
        let mut data: $crate::typehack::data::Data<_, data!(@count $($xs),*)>;
        unsafe {
            data = $crate::typehack::data::Data::uninitialized(
                <data!(@count $($xs),*) as $crate::typehack::binary::Nat>::as_data());
            data!(@assign data 0 => $($xs),*);
        }
        data
    });
}


pub trait DependentClone<T>: Sized {
    fn dependent_clone(&self) -> Self where T: Clone;
}


impl<T> DependentClone<T> for Vec<T> {
    fn dependent_clone(&self) -> Self
        where T: Clone
    {
        self.clone()
    }
}


impl<E, T: DependentClone<E>> DependentClone<E> for Box<T> {
    fn dependent_clone(&self) -> Self
        where E: Clone
    {
        Box::new(self.as_ref().dependent_clone())
    }
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct PopulatedNode<E, C> {
    e: E,
    l: C,
    r: C,
}



#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmptyNode<E, C> {
    e: PhantomData<E>,
    l: C,
    r: C,
}


impl<E, C: DependentClone<E>> DependentClone<E> for PopulatedNode<E, C> {
    fn dependent_clone(&self) -> Self
        where E: Clone
    {
        PopulatedNode {
            e: self.e.clone(),
            l: self.l.dependent_clone(),
            r: self.r.dependent_clone(),
        }
    }
}


impl<E, C: DependentClone<E>> DependentClone<E> for EmptyNode<E, C> {
    fn dependent_clone(&self) -> Self
        where E: Clone
    {
        EmptyNode {
            e: PhantomData,
            l: self.l.dependent_clone(),
            r: self.r.dependent_clone(),
        }
    }
}


pub trait Raw<E>: Nat {
    type Data: DependentClone<E> + Diceable<E, Self>;
}


impl<E> DependentClone<E> for () {
    fn dependent_clone(&self) -> Self {
        ()
    }
}


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<T: Nat, E> Raw<E> for T {
    default type Data = ();
}


impl<E> Raw<E> for P {
    type Data = ();
}

impl<E, N: Nat> Raw<E> for O<N> {
    type Data = EmptyNode<E, <N as Raw<E>>::Data>;
}

impl<E, N: Nat> Raw<E> for I<N> {
    type Data = PopulatedNode<E, <N as Raw<E>>::Data>;
}


pub trait Diceable<E, D: Dim> {
    unsafe fn uninitialized(D) -> Self;
    unsafe fn forget_internals(self);

    unsafe fn into_slice<'a>(&'a self, D) -> &'a [E];
    unsafe fn into_mut_slice<'a>(&'a mut self, D) -> &'a mut [E];
}

impl<E, D: Dim> Diceable<E, D> for () {
    unsafe fn uninitialized(dim: D) -> () {
        () // It's a fscking unit.
    }

    unsafe fn forget_internals(mut self) {} // It's a fscking unit.

    unsafe fn into_slice<'a>(&'a self, dim: D) -> &'a [E] {
        &[]
    }

    unsafe fn into_mut_slice<'a>(&'a mut self, dim: D) -> &'a mut [E] {
        &mut []
    }
}

impl<E, D: Dim> Diceable<E, D> for Vec<E> {
    unsafe fn uninitialized(dim: D) -> Vec<E> {
        // We can create a Vec with uninitialized memory by asking for a given capacity, and
        // then manually setting its length.
        let mut vec = Vec::with_capacity(dim.reify());
        vec.set_len(dim.reify());
        vec
    }

    unsafe fn forget_internals(mut self) {
        // Similarly to creating an uninitialized Vec, we can cause the Vec to forget its
        // internals by setting its length to zero. The memory it holds will still be freed
        // correctly, but without running destructors on the elements inside, which is
        // exactly what we want.
        self.set_len(0);
    }

    unsafe fn into_slice<'a>(&'a self, dim: D) -> &'a [E] {
        &self[..dim.reify()]
    }

    unsafe fn into_mut_slice<'a>(&'a mut self, dim: D) -> &'a mut [E] {
        &mut self[..dim.reify()]
    }
}

impl<E, D: Nat> Diceable<E, D> for EmptyNode<E, <D::ShrOnce as Raw<E>>::Data> {
    unsafe fn uninitialized(_: D) -> Self {
        mem::uninitialized()
    }

    unsafe fn forget_internals(self) {
        mem::forget(self);
    }

    unsafe fn into_slice<'a>(&'a self, _: D) -> &'a [E] {
        slice::from_raw_parts(mem::transmute::<&Self, *const E>(self), D::as_usize())
    }

    unsafe fn into_mut_slice<'a>(&'a mut self, _: D) -> &'a mut [E] {
        slice::from_raw_parts_mut(mem::transmute::<&mut Self, *mut E>(self), D::as_usize())
    }
}

impl<E, D: Nat> Diceable<E, D> for PopulatedNode<E, <D::ShrOnce as Raw<E>>::Data> {
    unsafe fn uninitialized(_: D) -> Self {
        mem::uninitialized()
    }

    unsafe fn forget_internals(self) {
        mem::forget(self);
    }

    unsafe fn into_slice<'a>(&'a self, _: D) -> &'a [E] {
        slice::from_raw_parts(mem::transmute::<&Self, *const E>(self), D::as_usize())
    }

    unsafe fn into_mut_slice<'a>(&'a mut self, _: D) -> &'a mut [E] {
        slice::from_raw_parts_mut(mem::transmute::<&mut Self, *mut E>(self), D::as_usize())
    }
}

impl<E, D: Nat, T> Diceable<E, D> for Box<T> {
    unsafe fn uninitialized(_: D) -> Self {
        Box::new(mem::uninitialized())
    }

    unsafe fn forget_internals(self) {
        let unboxed = *self;
        mem::forget(unboxed)
    }

    unsafe fn into_slice<'a>(&'a self, _: D) -> &'a [E] {
        slice::from_raw_parts(mem::transmute::<&T, *const E>(self.as_ref()), D::as_usize())
    }

    unsafe fn into_mut_slice<'a>(&'a mut self, _: D) -> &'a mut [E] {
        slice::from_raw_parts_mut(mem::transmute::<&mut T, *mut E>(self.as_mut()),
                                  D::as_usize())
    }
}


pub trait Size<E>: Dim {
    type Reify: DependentClone<E> + Diceable<E, Self>;
}


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<E, D: Dim> Size<E> for D {
    default type Reify = Vec<E>;
}


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<E> Size<E> for Dyn {
    type Reify = Vec<E>;
}


impl<E, N: Nat> Size<E> for N
    where N: NatSub<B32>
{
    type Reify = Box<<N as Raw<E>>::Data>;
}


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<E, N: Nat> Size<E> for N
{
    default type Reify = <N as Raw<E>>::Data;
}


#[derive(Copy)]
pub struct Data<T, S: Size<T>> {
    pub size: S,
    data: S::Reify,
}


impl<T: Clone, S: Size<T>> Clone for Data<T, S> {
    fn clone(&self) -> Self {
        Data {
            size: self.size,
            data: self.data.dependent_clone(),
        }
    }
}


impl<T, S: Size<T>> Deref for Data<T, S> {
    type Target = [T];

    fn deref<'a>(&'a self) -> &'a [T] {
        unsafe { self.data.into_slice(self.size) }
    }
}


impl<T, S: Size<T>> DerefMut for Data<T, S> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut [T] {
        unsafe { self.data.into_mut_slice(self.size) }
    }
}


impl<T, S: Size<T>> Data<T, S> {
    pub fn from_elem(s: S, elem: &T) -> Data<T, S>
        where T: Clone
    {
        let mut data: Self;

        unsafe {
            data = Data {
                size: s,
                data: S::Reify::uninitialized(s),
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
                data: S::Reify::uninitialized(s),
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
                data: S::Reify::uninitialized(s),
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
            data: S::Reify::uninitialized(s),
        }
    }

    pub fn extend(self, t: T) -> Data<T, S::Succ> {
        let mut data: Data<T, S::Succ>;
        let s = self.size.succ();

        unsafe {
            data = Data::uninitialized(s);

            for (i, val) in self.into_iter().enumerate() {
                ptr::write(&mut data[i], val);
            }
        }

        data
    }

    pub fn contract(self, idx: usize) -> Data<T, S::Pred> {
        let len = self.size.reify();
        assert!(idx < len);

        let mut data: Data<T, S::Pred>;
        let s = self.size.pred();

        unsafe {
            data = Data::uninitialized(s);

            for i in 0..idx {
                ptr::write(&mut data[i], ptr::read(&self[i]));
            }

            for i in idx..len {
                ptr::write(&mut data[i], ptr::read(&self[i + 1]));
            }

            self.forget();
        }

        data
    }

    pub unsafe fn forget(self) {
        self.data.forget_internals();
    }
}


impl<T: PartialEq, N: Size<T>> PartialEq for Data<T, N> {
    fn eq(&self, rhs: &Self) -> bool {
        self.deref() == rhs.deref()
    }
}

impl<T: Eq, N: Size<T>> Eq for Data<T, N> {}


impl<T: fmt::Debug, N: Size<T>> fmt::Debug for Data<T, N> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(fmt)
    }
}


impl<T, N: Size<T>> IntoIterator for Data<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            data: Some(self),
            idx: 0,
        }
    }
}


pub struct IntoIter<T, N: Size<T>> {
    data: Option<Data<T, N>>,
    idx: usize,
}


impl<T, N: Size<T>> Iterator for IntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        unsafe {
            let data = self.data.as_ref().unchecked_unwrap();
            if self.idx < data.size.reify() {
                let idx = self.idx;
                self.idx += 1;
                Some(ptr::read(&self.data.as_ref().unchecked_unwrap()[idx]))
            } else {
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unsafe {
            let data = self.data.as_ref().unchecked_unwrap();
            let remaining = data.size.reify() - self.idx;
            (remaining, Some(remaining))
        }
    }
}


impl<T, N: Size<T>> ExactSizeIterator for IntoIter<T, N> {}


impl<T, N: Size<T>> Drop for IntoIter<T, N> {
    fn drop(&mut self) {
        unsafe {
            {
                let data = self.data.as_ref().unchecked_unwrap();
                for i in self.idx..data.size.reify() {
                    mem::drop(ptr::read(&data[i]));
                }
            }
            self.data.take().unchecked_unwrap().forget()
        }
    }
}


impl<T> FromIterator<T> for Data<T, Dyn> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let vec: Vec<T> = iter.into_iter().collect();
        Data {
            size: Dyn(vec.len()),
            data: vec,
        }
    }
}


impl<T, N: Dim> FromExactSizeIterator<T> for Data<T, N> {
    fn from_exact_size_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
        where I::IntoIter: ExactSizeIterator
    {
        let iter = iter.into_iter();

        let size = iter.len();

        assert!(N::compatible(size));

        let mut out: Data<T, N>;

        unsafe {
            out = Data::uninitialized(N::from_usize(size));
            for (i, t) in iter.enumerate() {
                ptr::write(&mut out[i], t);
            }
        }

        out
    }
}


#[derive(Clone)]
pub struct DataVec<T, N: Size<T>> {
    data: Option<Data<T, N>>,
    len: usize,
}


impl<T, N: Dim> From<DataVec<T, N>> for Data<T, N> {
    fn from(mut dvec: DataVec<T, N>) -> Data<T, N> {
        unsafe { dvec.data.take().unchecked_unwrap() }
    }
}


impl<T, N: Size<T>> Drop for DataVec<T, N> {
    fn drop(&mut self) {
        unsafe {
            if self.data.is_some() {
                let data = self.data.take().unchecked_unwrap();
                for i in 0..self.len {
                    mem::drop(ptr::read(&data[i]));
                }
            }
        }
    }
}


impl<T, N: Nat + Size<T>> DataVec<T, N> {
    pub fn new() -> DataVec<T, N> {
        DataVec {
            data: Some(unsafe { Data::uninitialized(N::as_data()) }),
            len: 0,
        }
    }
}


impl<T, N: Size<T>> DataVec<T, N> {
    pub fn with_capacity(len: N) -> DataVec<T, N> {
        DataVec {
            data: Some(unsafe { Data::uninitialized(len) }),
            len: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        unsafe {
            let data = self.data.as_mut().unchecked_unwrap();
            assert!(self.len < data.size.reify());
            data[self.len] = elem;
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> T {
        unsafe {
            let data = self.data.as_mut().unchecked_unwrap();
            assert!(self.len > 0);
            let elem = ptr::read(&data[self.len]);
            self.len -= 1;
            elem
        }
    }

    pub fn set(&mut self, idx: usize, elem: T) {
        unsafe {
            let data = self.data.as_mut().unchecked_unwrap();
            assert!(idx <= self.len && idx < data.size.reify());
            data[idx] = elem;
            self.len = cmp::max(self.len, idx + 1);
        }
    }

    pub fn insert(&mut self, idx: usize, elem: T) {
        unsafe {
            let data = self.data.as_mut().unchecked_unwrap();
            assert!(idx <= self.len && idx < data.size.reify());
            for i in idx..self.len {
                ptr::write(&mut data[i + 1], ptr::read(&data[i]));
            }
            ptr::write(&mut data[idx], elem);
            self.len = self.len + 1;
        }
    }

    pub fn sorted_insert(&mut self, elem: T)
        where T: Ord
    {
        match self.binary_search(&elem) {
            Ok(_) => {}
            Err(idx) => self.insert(idx, elem),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn size(&self) -> N {
        unsafe { self.data.as_ref().unchecked_unwrap().size }
    }

    pub fn is_full(&self) -> bool {
        self.len >= unsafe { self.data.as_ref().unchecked_unwrap().size.reify() }
    }
}


impl<T, N: Size<T>> FromExactSizeIterator<T> for DataVec<T, N> {
    fn from_exact_size_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
        where I::IntoIter: ExactSizeIterator
    {
        let iter = iter.into_iter();

        let size = iter.len();

        assert!(size <= N::from_usize(size).reify());

        unsafe {
            let mut data: Data<T, N>;

            data = Data::uninitialized(N::from_usize(size));
            for (i, t) in iter.enumerate() {
                ptr::write(&mut data[i], t);
            }

            DataVec {
                data: Some(data),
                len: size,
            }
        }
    }
}


impl<T, N: Size<T>> Deref for DataVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &(unsafe { self.data.as_ref().unchecked_unwrap() })[0..self.len]
    }
}


impl<T, N: Size<T>> DerefMut for DataVec<T, N> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut (unsafe { self.data.as_mut().unchecked_unwrap() })[0..self.len]
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
