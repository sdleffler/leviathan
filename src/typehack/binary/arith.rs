use super::{Nat, I, O};

pub trait NatAdd<N: Nat>: Nat {
    type Result: Nat;
}

impl<N: Nat> NatAdd<N> for () {
    type Result = N;
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
