use super::{Nat, I, O};


pub trait NatAdd<N: Nat>: Nat {
    type Result: Nat;
}

impl NatAdd<()> for () {
    type Result = ();
}

impl<N: Nat> NatAdd<O<N>> for () {
    type Result = O<N>;
}

impl<N: Nat> NatAdd<I<N>> for () {
    type Result = I<N>;
}

impl<N: Nat> NatAdd<()> for O<N> {
    type Result = O<N>;
}

impl<N: Nat> NatAdd<()> for I<N> {
    type Result = I<N>;
}

impl<M: Nat, N: Nat> NatAdd<O<N>> for O<M>
    where M: NatAdd<N>
{
    type Result = O<<M as NatAdd<N>>::Result>;
}

impl<M: Nat, N: Nat> NatAdd<I<N>> for O<M>
    where M: NatAdd<N>
{
    type Result = I<<M as NatAdd<N>>::Result>;
}

impl<M: Nat, N: Nat> NatAdd<O<N>> for I<M>
    where M: NatAdd<N>
{
    type Result = I<<M as NatAdd<N>>::Result>;
}

impl<M: Nat, N: Nat> NatAdd<I<N>> for I<M>
    where M: NatAdd<N>,
          <M as NatAdd<N>>::Result: NatAdd<I<O>>
{
    type Result = O<<<M as NatAdd<N>>::Result as NatAdd<I<O>>>::Result>;
}


pub trait NatSub<N: Nat>: Nat {
    type Result: Nat;
}


impl<N: Nat> NatSub<()> for N {
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
    where M: NatSub<I>,
          <M as NatSub<I>>::Result: NatSub<N>
{
    type Result = I<<<M as NatSub<I>>::Result as NatSub<N>>::Result>;
}


pub trait NatMul<N: Nat>: Nat {
    type Result: Nat;
}


impl<N: Nat> NatMul<N> for () {
    type Result = ();
}

impl<M: Nat, N: Nat> NatMul<N> for O<M>
    where M: NatMul<O<N>>
{
    type Result = <M as NatMul<O<N>>>::Result;
}

impl<M: Nat, N: Nat> NatMul<N> for I<M>
    where M: NatMul<O<N>>,
          N: NatAdd<<M as NatMul<O<N>>>::Result>
{
    type Result = <N as NatAdd<<M as NatMul<O<N>>>::Result>>::Result;
}


pub trait NatShl<N: Nat>: Nat {
    type Result: Nat;
}


impl<N: Nat> NatShl<()> for N {
    type Result = N;
}

impl<M: Nat, N: Nat> NatShl<O<N>> for M
    where M: NatShl<N>,
          <M as NatShl<N>>::Result: NatShl<N>
{
    type Result = <<M as NatShl<N>>::Result as NatShl<N>>::Result;
}

impl<M: Nat, N: Nat> NatShl<I<N>> for M
    where M: NatShl<N>,
          <M as NatShl<N>>::Result: NatShl<N>
{
    type Result = O<<<M as NatShl<N>>::Result as NatShl<N>>::Result>;
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
