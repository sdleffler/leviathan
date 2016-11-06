use std::marker::PhantomData;

mod arith;
pub use self::arith::*;


#[macro_export]
macro_rules! binary_type {
    (0 $($t:tt)*) => ([$($t)*] O);
    (1 $($t:tt)*) => ([$($t)*] I);
    ([0 $($t:tt)*] $($cons:tt)*) => (binary_type!([$($t)*] $($cons)* < O) >);
    ([1 $($t:tt)*] $($cons:tt)*) => (binary_type!([$($t)*] $($cons)* < I) >);
    ([] $($cons:tt)*) => ($($cons)*);
}


pub trait Nat: Copy {
    type Succ: Nat;

    fn as_usize() -> usize;
    fn as_data() -> Self;
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct I<N = ()>(PhantomData<N>);
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct O<N = ()>(PhantomData<N>);

impl Nat for () {
    type Succ = I;

    fn as_usize() -> usize {
        0
    }

    fn as_data() -> Self {
        ()
    }
}

impl<N: Nat> Nat for I<N> {
    type Succ = O<N::Succ>;

    fn as_usize() -> usize {
        N::as_usize() * 2 + 1
    }

    fn as_data() -> Self {
        I(PhantomData::<N>)
    }
}

impl<N: Nat> Nat for O<N> {
    type Succ = I<N>;

    fn as_usize() -> usize {
        N::as_usize() * 2
    }

    fn as_data() -> Self {
        O(PhantomData::<N>)
    }
}


pub type B0 = O;
pub type B1 = I;
pub type B2 = O<I>;
pub type B3 = I<I>;
pub type B4 = O<O<I>>;
pub type B5 = I<O<I>>;
pub type B6 = I<I<O>>;
pub type B7 = I<I<I>>;
pub type B8 = O<O<O<I>>>;
pub type B9 = I<O<O<I>>>;
pub type B10 = O<I<O<I>>>;
pub type B11 = I<I<O<I>>>;
pub type B12 = O<O<I<I>>>;
pub type B13 = I<O<I<I>>>;
pub type B14 = O<I<I<I>>>;
pub type B15 = I<I<I<I>>>;
pub type B16 = O<O<O<O<I>>>>;
pub type B17 = I<O<O<O<I>>>>;
pub type B18 = O<I<O<O<I>>>>;
pub type B19 = I<I<O<O<I>>>>;
pub type B20 = O<O<I<O<I>>>>;
pub type B21 = I<O<I<O<I>>>>;
pub type B22 = O<I<I<O<I>>>>;
pub type B23 = I<I<I<O<I>>>>;
pub type B24 = O<O<O<I<I>>>>;
pub type B25 = I<O<O<I<I>>>>;
pub type B26 = O<I<O<I<I>>>>;
pub type B27 = I<I<O<I<I>>>>;
pub type B28 = O<O<I<I<I>>>>;
pub type B29 = I<O<I<I<I>>>>;
pub type B30 = O<I<I<I<I>>>>;
pub type B31 = I<I<I<I<I>>>>;
pub type B32 = O<O<O<O<O<I>>>>>;
pub type B64 = O<O<O<O<O<O<I>>>>>>;
pub type B128 = O<O<O<O<O<O<O<I>>>>>>>;
pub type B256 = O<O<O<O<O<O<O<O<I>>>>>>>>;
pub type B512 = O<O<O<O<O<O<O<O<O<I>>>>>>>>>;
