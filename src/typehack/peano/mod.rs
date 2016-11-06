use std::marker::PhantomData;

use typehack::binary::Nat as BNat;
use typehack::binary::O;

mod arith;
pub use self::arith::*;


pub trait Nat: Copy {
    type AsBinary: BNat;

    fn as_data() -> Self;
    fn as_usize() -> usize;
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Z;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct S<N: Nat>(PhantomData<N>);


impl Nat for Z {
    type AsBinary = O;

    fn as_data() -> Z {
        Z
    }

    fn as_usize() -> usize {
        0
    }
}


impl<N: Nat> Nat for S<N> {
    type AsBinary = <N::AsBinary as BNat>::Succ;

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
pub type P32 = S<P31>;
pub type P33 = S<P32>;
pub type P34 = S<P33>;
pub type P35 = S<P34>;
pub type P36 = S<P35>;
pub type P37 = S<P36>;
pub type P38 = S<P37>;
pub type P39 = S<P38>;
pub type P40 = S<P39>;
pub type P41 = S<P40>;
pub type P42 = S<P41>;
pub type P43 = S<P42>;
pub type P44 = S<P43>;
pub type P45 = S<P44>;
pub type P46 = S<P45>;
pub type P47 = S<P46>;
pub type P48 = S<P47>;
pub type P49 = S<P48>;
pub type P50 = S<P49>;
pub type P51 = S<P50>;
pub type P52 = S<P51>;
pub type P53 = S<P52>;
pub type P54 = S<P53>;
pub type P55 = S<P54>;
pub type P56 = S<P55>;
pub type P57 = S<P56>;
pub type P58 = S<P57>;
pub type P59 = S<P58>;
pub type P60 = S<P59>;
pub type P61 = S<P60>;
pub type P62 = S<P61>;
pub type P63 = S<P62>;
pub type P64 = S<P63>;
