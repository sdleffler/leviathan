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
