use std::fmt::Debug;
use std::marker::PhantomData;


pub mod arith;
pub use self::arith::*;


#[macro_export]
macro_rules! binary_type {
    (0 $($t:tt)*) => ([$($t)*] O);
    (1 $($t:tt)*) => ([$($t)*] I);
    ([0 $($t:tt)*] $($cons:tt)*) => (binary_type!([$($t)*] $($cons)* < O) >);
    ([1 $($t:tt)*] $($cons:tt)*) => (binary_type!([$($t)*] $($cons)* < I) >);
    ([] $($cons:tt)*) => ($($cons)*);
}


pub trait Nat: Eq + Debug + Copy + Default {
    type Succ: Nat;
    type Pred: Nat;

    type Double: Nat;
    type DoublePlusOne: Nat;

    type ShrOnce: Nat;

    fn as_usize() -> usize;
    fn as_data() -> Self;
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct I<N = P>(PhantomData<N>);
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct O<N = P>(PhantomData<N>);
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct P;


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<T: Copy + Eq + Debug + Default> Nat for T {
    default type Succ = I;
    default type Pred = T;

    default type Double = O;
    default type DoublePlusOne = O;

    default type ShrOnce = T;

    default fn as_usize() -> usize {
        0
    }

    default fn as_data() -> Self {
        Self::default()
    }
}

/// The point type's Nat implementation is a sort of fixed point for `Double`, `DoublePlusOne`, and `Pred`.
/// Remember, P *is not zero!*
#[cfg_attr(rustfmt, rustfmt_skip)]
impl Nat for P {
    type Succ = I;
    type Pred = P;

    type Double = O;
    type DoublePlusOne = O;

    type ShrOnce = P;

    fn as_usize() -> usize {
        0
    }

    fn as_data() -> Self {
        P
    }
}

impl<N: Nat> Nat for I<N> {
    type Succ = O<N::Succ>;
    type Pred = O<N>;

    type Double = O<I<N>>;
    type DoublePlusOne = I<I<N>>;

    type ShrOnce = N;

    fn as_usize() -> usize {
        N::as_usize() * 2 + 1
    }

    fn as_data() -> Self {
        I(PhantomData::<N>)
    }
}

impl<N: Nat> Nat for O<N> {
    type Succ = I<N>;
    type Pred = <N::Pred as Nat>::DoublePlusOne;

    type Double = O<O<N>>;
    type DoublePlusOne = I<O<N>>;

    type ShrOnce = N;

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
pub type B6 = O<I<I>>;
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_as_usize() {
        assert_eq!(B0::as_usize(), 0);
        assert_eq!(B1::as_usize(), 1);
        assert_eq!(B2::as_usize(), 2);
    }


    #[test]
    fn binary_add() {
        assert_eq!(<B0 as NatAdd<B0>>::Result::as_usize(), 0);
        assert_eq!(<B0 as NatAdd<B1>>::Result::as_usize(), 1);
        assert_eq!(<B1 as NatAdd<B0>>::Result::as_usize(), 1);
        assert_eq!(<B1 as NatAdd<B1>>::Result::as_usize(), 2);
        assert_eq!(<B3 as NatAdd<B2>>::Result::as_usize(), 5);
    }


    #[test]
    fn binary_sub() {
        assert_eq!(<B0 as NatSub<B0>>::Result::as_usize(), 0);
        assert_eq!(<B1 as NatSub<B0>>::Result::as_usize(), 1);
        assert_eq!(<B1 as NatSub<B1>>::Result::as_usize(), 0);
        assert_eq!(<B2 as NatSub<B1>>::Result::as_usize(), 1);
        assert_eq!(<B4 as NatSub<B1>>::Result::as_usize(), 3);
        assert_eq!(<B4 as NatSub<B2>>::Result::as_usize(), 2);
    }


    #[test]
    fn binary_mul() {
        assert_eq!(<B0 as NatMul<B0>>::Result::as_usize(), 0);
        assert_eq!(<B0 as NatMul<B1>>::Result::as_usize(), 0);
        assert_eq!(<B1 as NatMul<B0>>::Result::as_usize(), 0);
        assert_eq!(<B1 as NatMul<B1>>::Result::as_usize(), 1);
        assert_eq!(<B1 as NatMul<B2>>::Result::as_usize(), 2);
        assert_eq!(<B2 as NatMul<B1>>::Result::as_usize(), 2);
        assert_eq!(<B2 as NatMul<B2>>::Result::as_usize(), 4);
        assert_eq!(<B4 as NatMul<B3>>::Result::as_usize(), 12);
    }


    #[test]
    fn binary_shl() {
        assert_eq!(<B0 as NatShl<B2>>::Result::as_usize(), 0);
        assert_eq!(<B1 as NatShl<B0>>::Result::as_usize(), 1);
        assert_eq!(<B1 as NatShl<B1>>::Result::as_usize(), 2);
        assert_eq!(<B1 as NatShl<B2>>::Result::as_usize(), 4);
        assert_eq!(<B1 as NatShl<B3>>::Result::as_usize(), 8);
        assert_eq!(<B2 as NatShl<B0>>::Result::as_usize(), 2);
        assert_eq!(<B2 as NatShl<B1>>::Result::as_usize(), 4);
        assert_eq!(<B2 as NatShl<B2>>::Result::as_usize(), 8);
        assert_eq!(<B2 as NatShl<B3>>::Result::as_usize(), 16);
        assert_eq!(<B3 as NatShl<B0>>::Result::as_usize(), 3);
        assert_eq!(<B3 as NatShl<B1>>::Result::as_usize(), 6);
        assert_eq!(<B3 as NatShl<B2>>::Result::as_usize(), 12);
        assert_eq!(<B3 as NatShl<B3>>::Result::as_usize(), 24);
    }


    #[test]
    fn binary_shl_flipped() {
        assert_eq!(<B2 as NatShlFlipped<B0>>::Result::as_usize(), 0);
        assert_eq!(<B0 as NatShlFlipped<B1>>::Result::as_usize(), 1);
        assert_eq!(<B1 as NatShlFlipped<B1>>::Result::as_usize(), 2);
        assert_eq!(<B2 as NatShlFlipped<B1>>::Result::as_usize(), 4);
        assert_eq!(<B3 as NatShlFlipped<B1>>::Result::as_usize(), 8);
        assert_eq!(<B0 as NatShlFlipped<B2>>::Result::as_usize(), 2);
        assert_eq!(<B1 as NatShlFlipped<B2>>::Result::as_usize(), 4);
        assert_eq!(<B2 as NatShlFlipped<B2>>::Result::as_usize(), 8);
        assert_eq!(<B3 as NatShlFlipped<B2>>::Result::as_usize(), 16);
        assert_eq!(<B0 as NatShlFlipped<B3>>::Result::as_usize(), 3);
        assert_eq!(<B1 as NatShlFlipped<B3>>::Result::as_usize(), 6);
        assert_eq!(<B2 as NatShlFlipped<B3>>::Result::as_usize(), 12);
        assert_eq!(<B3 as NatShlFlipped<B3>>::Result::as_usize(), 24);
    }


    #[test]
    fn binary_succ() {
        assert_eq!(<B0 as Nat>::Succ::as_usize(), 1);
        assert_eq!(<B1 as Nat>::Succ::as_usize(), 2);
        assert_eq!(<B2 as Nat>::Succ::as_usize(), 3);
        assert_eq!(<B3 as Nat>::Succ::as_usize(), 4);
        assert_eq!(<B4 as Nat>::Succ::as_usize(), 5);
        assert_eq!(<B5 as Nat>::Succ::as_usize(), 6);
        assert_eq!(<B6 as Nat>::Succ::as_usize(), 7);
        assert_eq!(<B7 as Nat>::Succ::as_usize(), 8);
        assert_eq!(<B8 as Nat>::Succ::as_usize(), 9);
        assert_eq!(<B9 as Nat>::Succ::as_usize(), 10);
        assert_eq!(<B10 as Nat>::Succ::as_usize(), 11);
        assert_eq!(<B11 as Nat>::Succ::as_usize(), 12);
        assert_eq!(<B12 as Nat>::Succ::as_usize(), 13);
        assert_eq!(<B13 as Nat>::Succ::as_usize(), 14);
        assert_eq!(<B14 as Nat>::Succ::as_usize(), 15);
        assert_eq!(<B15 as Nat>::Succ::as_usize(), 16);
    }


    #[test]
    fn binary_pred() {
        assert_eq!(<B0 as Nat>::Pred::as_usize(), 0);
        assert_eq!(<B1 as Nat>::Pred::as_usize(), 0);
        assert_eq!(<B2 as Nat>::Pred::as_usize(), 1);
        assert_eq!(<B3 as Nat>::Pred::as_usize(), 2);
        assert_eq!(<B4 as Nat>::Pred::as_usize(), 3);
        assert_eq!(<B5 as Nat>::Pred::as_usize(), 4);
        assert_eq!(<B6 as Nat>::Pred::as_usize(), 5);
        assert_eq!(<B7 as Nat>::Pred::as_usize(), 6);
        assert_eq!(<B8 as Nat>::Pred::as_usize(), 7);
        assert_eq!(<B9 as Nat>::Pred::as_usize(), 8);
        assert_eq!(<B10 as Nat>::Pred::as_usize(), 9);
        assert_eq!(<B11 as Nat>::Pred::as_usize(), 10);
        assert_eq!(<B12 as Nat>::Pred::as_usize(), 11);
        assert_eq!(<B13 as Nat>::Pred::as_usize(), 12);
        assert_eq!(<B14 as Nat>::Pred::as_usize(), 13);
        assert_eq!(<B15 as Nat>::Pred::as_usize(), 14);
    }
}
