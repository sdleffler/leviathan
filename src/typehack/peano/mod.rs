use std::marker::PhantomData;


mod arith;
pub use self::arith::*;


pub trait Nat {
    fn as_data() -> Self;
    fn as_usize() -> usize;
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Z;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct S<N: Nat>(PhantomData<N>);


impl Nat for Z {
    fn as_data() -> Z {
        Z
    }

    fn as_usize() -> usize {
        0
    }
}


impl<N: Nat> Nat for S<N> {
    fn as_data() -> S<N> {
        S(PhantomData::<N>)
    }

    fn as_usize() -> usize {
        1 + N::as_usize()
    }
}


#[macro_export]
macro_rules! peano_count {
    ($elem:expr $(, $rest:expr)*) => ($crate::typehack::peano::S<peano_count!($($rest),*)>);
    () => ($crate::typehack::peano::Z);
}


pub type P0 = Z;
pub type P1 = S<P0>;
pub type P2 = S<P1>;
pub type P3 = S<P2>;
pub type P4 = S<P3>;
pub type P5 = S<P4>;
pub type P6 = S<P5>;
pub type P7 = S<P6>;
pub type P8 = S<P7>;
pub type P9 = S<P8>;
pub type P10 = S<P9>;
pub type P11 = S<P10>;
pub type P12 = S<P11>;
pub type P13 = S<P12>;
pub type P14 = S<P13>;
pub type P15 = S<P14>;
pub type P16 = S<P15>;
pub type P17 = S<P16>;
pub type P18 = S<P17>;
pub type P19 = S<P18>;
pub type P20 = S<P19>;
pub type P21 = S<P20>;
pub type P22 = S<P21>;
pub type P23 = S<P22>;
pub type P24 = S<P23>;
pub type P25 = S<P24>;
pub type P26 = S<P25>;
pub type P27 = S<P26>;
pub type P28 = S<P27>;
pub type P29 = S<P28>;
pub type P30 = S<P29>;
pub type P31 = S<P30>;
