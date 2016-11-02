use typehack::peano::*;


pub trait Dim {
    fn reify(&self) -> usize;
}

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
