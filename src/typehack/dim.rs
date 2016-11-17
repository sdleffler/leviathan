use std::fmt::Debug;

use typehack::binary::*;


pub trait Dim: Eq + Debug + Copy {
    type Succ: Dim;
    type Pred: Dim;

    type Double: Dim;

    fn compatible(usize) -> bool;
    fn from_usize(usize) -> Self;

    fn reify(&self) -> usize;
    fn succ(self) -> Self::Succ;

    fn double(self) -> Self::Double;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dyn(pub usize);

impl<N: Nat> Dim for N {
    type Succ = N::Succ;
    type Pred = N::Pred;

    type Double = N::Double;

    fn compatible(n: usize) -> bool {
        N::as_usize() == n
    }

    fn from_usize(_: usize) -> Self {
        N::as_data()
    }

    fn reify(&self) -> usize {
        N::as_usize()
    }

    fn succ(self) -> Self::Succ {
        N::Succ::as_data()
    }

    fn double(self) -> Self::Double {
        N::Double::as_data()
    }
}

impl Dim for Dyn {
    type Succ = Dyn;
    type Pred = Dyn;

    type Double = Dyn;

    fn compatible(_: usize) -> bool {
        true
    }

    fn from_usize(n: usize) -> Self {
        Dyn(n)
    }

    fn reify(&self) -> usize {
        self.0
    }

    fn succ(self) -> Self::Succ {
        Dyn(self.0 + 1)
    }

    fn double(self) -> Self::Double {
        Dyn(self.0 * 2)
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


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<N: Dim, M: Dim> DimMul<N> for M {
    default type Result = Dyn;

    default fn mul(self, _: N) -> Self::Result {
        unreachable!();
    }
}

impl<N: Nat, M: Nat> DimMul<N> for M {
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


pub trait DimAdd<R: Dim>: Dim {
    type Result: Dim;

    fn add(self, rhs: R) -> Self::Result;
}


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<N: Dim, M: Dim> DimAdd<N> for M {
    default type Result = ();

    default fn add(self, _: N) -> Self::Result {
        unreachable!();
    }
}

impl<N: Nat, M: Nat> DimAdd<N> for M {
    type Result = <M as NatAdd<N>>::Result;

    fn add(self, _rhs: N) -> Self::Result {
        Self::Result::as_data()
    }
}

impl<N: Nat> DimAdd<Dyn> for N {
    type Result = Dyn;

    fn add(self, rhs: Dyn) -> Self::Result {
        Dyn(self.reify() + rhs.reify())
    }
}

impl<N: Dim> DimAdd<N> for Dyn {
    type Result = Dyn;

    fn add(self, rhs: N) -> Self::Result {
        Dyn(self.reify() + rhs.reify())
    }
}


pub trait DimShl<R: Dim>: Dim {
    type Result: Dim;

    fn shl(self, rhs: R) -> Self::Result;
}


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<N: Dim, M: Dim> DimShl<N> for M {
    default type Result = ();

    default fn shl(self, _rhs: N) -> Self::Result {
        unreachable!();
    }
}

impl<N: Nat, M: Nat> DimShl<N> for M {
    type Result = <M as NatShl<N>>::Result;

    fn shl(self, _rhs: N) -> Self::Result {
        Self::Result::as_data()
    }
}

impl<N: Nat> DimShl<Dyn> for N {
    type Result = Dyn;

    fn shl(self, rhs: Dyn) -> Self::Result {
        Dyn(self.reify() << rhs.reify())
    }
}

impl<N: Dim> DimShl<N> for Dyn {
    type Result = Dyn;

    fn shl(self, rhs: N) -> Self::Result {
        Dyn(self.reify() << rhs.reify())
    }
}
