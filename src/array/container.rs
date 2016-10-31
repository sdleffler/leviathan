use typehack::peano::*;
use typehack::tvec::*;

use array::index::*;


pub trait Dims {
    type Length: Nat;
    type Index: At<Self::Length>;

    fn total() -> usize;
}


impl<M: Nat, N: Nat, L> Dims for TCons<M, TCons<N, L>>
    where TCons<N, L>: Dims
{
    type Length = S<<TCons<N, L> as Dims>::Length>;
    type Index = CCons<<TCons<N, L> as Dims>::Index>;

    fn total() -> usize {
        M::as_usize() * TCons::<N, L>::total()
    }
}


impl<M: Nat> Dims for TCons<M, TNil> {
    type Length = Z;
    type Index = CNil;

    fn total() -> usize {
        M::as_usize()
    }
}


pub trait DynArray<T, D: Nat, I: At<D>> {
    fn length(&self) -> usize;

    fn get(&self, idx: I) -> Option<&T>;
    fn get_mut(&mut self, idx: I) -> Option<&mut T>;
}
