use super::{Nat, P, I, O};


pub trait NatAdd<N: Nat>: Nat {
    type Result: Nat;
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl<M: Nat, N: Nat> NatAdd<N> for M {
    default type Result = N;
}

impl<N: Nat> NatAdd<P> for O<N> {
    type Result = O<N>;
}

impl<N: Nat> NatAdd<P> for I<N> {
    type Result = I<N>;
}

impl<N: Nat> NatAdd<O<N>> for P {
    type Result = O<N>;
}

impl<N: Nat> NatAdd<I<N>> for P {
    type Result = I<N>;
}

impl<M: Nat, N: Nat> NatAdd<O<N>> for O<M> {
    type Result = O<<M as NatAdd<N>>::Result>;
}

impl<M: Nat, N: Nat> NatAdd<I<N>> for O<M> {
    type Result = I<<M as NatAdd<N>>::Result>;
}

impl<M: Nat, N: Nat> NatAdd<O<N>> for I<M> {
    type Result = I<<M as NatAdd<N>>::Result>;
}

impl<M: Nat, N: Nat> NatAdd<I<N>> for I<M> {
    type Result = O<<<M as NatAdd<N>>::Result as Nat>::Succ>;
}


pub trait NatSub<N: Nat>: Nat {
    type Result: Nat;
}


impl<N: Nat> NatSub<P> for N {
    type Result = N;
}

impl<M: Nat, N: Nat> NatSub<O<N>> for O<M>
    where M: NatSub<N>
{
    type Result = O<<M as NatSub<N>>::Result>;
}

impl<M: Nat, N: Nat> NatSub<O<N>> for I<M>
    where M: NatSub<N>
{
    type Result = I<<M as NatSub<N>>::Result>;
}

impl<M: Nat, N: Nat> NatSub<I<N>> for I<M>
    where M: NatSub<N>
{
    type Result = O<<M as NatSub<N>>::Result>;
}

impl<M: Nat, N: Nat> NatSub<I<N>> for O<M>
    where <M as Nat>::Pred: NatSub<N>
{
    type Result = I<<<M as Nat>::Pred as NatSub<N>>::Result>;
}


pub trait NatMul<N: Nat>: Nat {
    type Result: Nat;
}


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<M: Nat, N: Nat> NatMul<N> for M {
    default type Result = M;
}

impl<N: Nat> NatMul<N> for P {
    type Result = P;
}

impl<M: Nat, N: Nat> NatMul<N> for O<M> {
    type Result = <M as NatMul<O<N>>>::Result;
}

impl<M: Nat, N: Nat> NatMul<N> for I<M> {
    type Result = <N as NatAdd<<M as NatMul<O<N>>>::Result>>::Result;
}


pub trait NatShl<N: Nat>: Nat {
    type Result: Nat;
}


impl<M: Nat, N: Nat> NatShl<N> for M {
    type Result = <N as NatShlFlipped<M>>::Result;
}


pub trait NatShlFlipped<N: Nat>: Nat {
    type Result: Nat;
}


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<M: Nat, N: Nat> NatShlFlipped<N> for M {
    default type Result = N;
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl<N: Nat> NatShlFlipped<N> for P {
    default type Result = N;
}

impl<M: Nat, N: Nat> NatShlFlipped<M> for O<N> {
    type Result = <N as NatShlFlipped<<N as NatShlFlipped<M>>::Result>>::Result;
}

impl<M: Nat, N: Nat> NatShlFlipped<M> for I<N> {
    type Result = O<<N as NatShlFlipped<<N as NatShlFlipped<M>>::Result>>::Result>;
}


pub trait NatRem<N: Nat>: Nat {
    type Result: Nat;
}


impl<N: Nat, M: Nat> NatRem<N> for M
    where M: NatSub<N>
{
    type Result = <M as NatSub<N>>::Result;
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl<N: Nat, M: Nat> NatRem<N> for M {
    default type Result = M;
}
