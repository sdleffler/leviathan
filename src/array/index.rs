use typehack::peano::*;


pub type Idx = usize;


#[repr(C)]
pub struct CCons<N> {
    idx: Idx,
    next: N,
}


#[repr(C)]
pub struct CNil {
    idx: Idx,
}


pub trait At<I: Nat> {
    fn at(&self) -> Idx;
}


impl At<Z> for CNil {
    fn at(&self) -> Idx {
        self.idx
    }
}


impl<N> At<Z> for CCons<N> {
    fn at(&self) -> Idx {
        self.idx
    }
}


impl<I: Nat, N: At<I>> At<S<I>> for CCons<N> {
    fn at(&self) -> Idx {
        self.next.at()
    }
}


pub type Idx1 = CNil;
pub type Idx2 = CCons<Idx1>;
pub type Idx3 = CCons<Idx2>;
pub type Idx4 = CCons<Idx3>;
pub type Idx5 = CCons<Idx4>;
pub type Idx6 = CCons<Idx5>;
pub type Idx7 = CCons<Idx6>;
pub type Idx8 = CCons<Idx7>;
pub type Idx9 = CCons<Idx8>;


macro_rules! i {
    ($idx:expr $(, $rest:expr)*) => (CCons { idx: $idx, next: i![$($rest),*] });
    () => (());
}
