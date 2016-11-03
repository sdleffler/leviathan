use std::marker::PhantomData;
use std::ops::Add;

use super::{Nat, S, Z};


impl Add<Z> for Z {
    type Output = Z;

    fn add(self, _rhs: Z) -> Z {
        self
    }
}


impl<N: Nat> Add<Z> for S<N> {
    type Output = S<N>;

    fn add(self, _rhs: Z) -> S<N> {
        self
    }
}


impl<N: Nat> Add<S<N>> for Z {
    type Output = S<N>;

    fn add(self, rhs: S<N>) -> S<N> {
        rhs
    }
}


impl<M: Nat, N: Nat> Add<S<N>> for S<M>
    where M: Add<N>,
          <M as Add<N>>::Output: Nat
{
    type Output = S<S<<M as Add<N>>::Output>>;

    fn add(self, _rhs: S<N>) -> S<S<<M as Add<N>>::Output>> {
        S(PhantomData)
    }
}


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


impl<N: Nat> NatMul<N> for Z {
    type Result = Z;
}


impl<N: Nat, M: Nat> NatMul<M> for S<N>
    where N: NatMul<M>,
          M: NatAdd<<N as NatMul<M>>::Result>
{
    type Result = <M as NatAdd<<N as NatMul<M>>::Result>>::Result;
}


// impl<N: Nat> NatMul<Z> for N {
//     type Result = Z;
// }
//
//
// impl<N: Nat, M: Nat> NatMul<S<M>> for N
//     where N: NatMul<M> + NatAdd<<N as NatMul<M>>::Result>
// {
//     type Result = <N as NatAdd<<N as NatMul<M>>::Result>>::Result;
// }
