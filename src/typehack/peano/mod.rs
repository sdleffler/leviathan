use std::marker::PhantomData;


mod arith;
pub use self::arith::*;


pub trait Nat {
    fn as_usize() -> usize;
}


pub struct Z;
pub struct S<N: Nat>(PhantomData<N>);


impl Nat for Z {
    fn as_usize() -> usize {
        0
    }
}


impl<N: Nat> Nat for S<N> {
    fn as_usize() -> usize {
        1 + N::as_usize()
    }
}


#[macro_export]
macro_rules! peano_count {
    ($elem:expr $(, $rest:expr)*) => ($crate::peano::S<peano_count!($($rest),*)>);
    () => ($crate::peano::Z);
}
