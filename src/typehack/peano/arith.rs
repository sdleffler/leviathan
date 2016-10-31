use super::{Nat, S, Z};

pub trait NatAdd<N: Nat>: Nat {
    type Result: Nat;
}


impl<N: Nat> NatAdd<Z> for N {
    type Result = N;
}


impl<N: Nat, M: Nat> NatAdd<S<M>> for N
    where S<N>: NatAdd<M>
{
    type Result = <S<N> as NatAdd<M>>::Result;
}


pub trait NatSub<N: Nat>: Nat {
    type Result: Nat;
}


impl<N: Nat> NatSub<Z> for N {
    type Result = N;
}


impl<N: Nat, M: Nat> NatSub<S<M>> for S<N>
    where N: NatSub<M>
{
    type Result = <N as NatSub<M>>::Result;
}


pub trait NatMul<N: Nat>: Nat {
    type Result: Nat;
}


impl<N: Nat> NatMul<Z> for N {
    type Result = Z;
}


impl<N: Nat, M: Nat> NatMul<S<M>> for N
    where N: NatMul<M> + NatAdd<<N as NatMul<M>>::Result>
{
    type Result = <N as NatAdd<<N as NatMul<M>>::Result>>::Result;
}
