use std::marker::PhantomData;

mod arith;
pub use self::arith::*;


pub trait Nat {
    fn as_usize() -> usize;
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub struct I<N = ()>(PhantomData<N>);
#[cfg_attr(rustfmt, rustfmt_skip)]
pub struct O<N = ()>(PhantomData<N>);

impl Nat for () {
    fn as_usize() -> usize {
        0
    }
}

impl<N: Nat> Nat for I<N> {
    fn as_usize() -> usize {
        N::as_usize() * 2 + 1
    }
}

impl<N: Nat> Nat for O<N> {
    fn as_usize() -> usize {
        N::as_usize() * 2
    }
}
