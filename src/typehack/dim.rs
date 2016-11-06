use typehack::binary::*;


pub trait Dim: Copy {
    fn reify(&self) -> usize;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dyn(pub usize);

impl<N: Nat> Dim for N {
    fn reify(&self) -> usize {
        N::as_usize()
    }
}

impl Dim for Dyn {
    fn reify(&self) -> usize {
        self.0
    }
}


pub trait DimCompat<R: Dim>: Dim {}

impl<S: Dim> DimCompat<S> for S {}
impl<S: Nat> DimCompat<Dyn> for S {}
impl<S: Nat> DimCompat<S> for Dyn {}


pub trait DimMul<R: Dim>: Dim {
    type Result: Dim;

    fn mul(self, rhs: R) -> Self::Result;
}


impl<N: Nat, M: Nat> DimMul<N> for M
    where M: NatMul<N>
{
    type Result = <M as NatMul<N>>::Result;

    fn mul(self, _rhs: N) -> Self::Result {
        Self::Result::as_data()
    }
}

impl<N: Nat> DimMul<Dyn> for N {
    type Result = Dyn;

    fn mul(self, rhs: Dyn) -> Self::Result {
        Dyn(self.reify() * rhs.reify())
    }
}

impl<N: Dim> DimMul<N> for Dyn {
    type Result = Dyn;

    fn mul(self, rhs: N) -> Self::Result {
        Dyn(self.reify() * rhs.reify())
    }
}
