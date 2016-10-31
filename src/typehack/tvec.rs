use std::marker::PhantomData;

use typehack::peano::*;


pub struct TCons<E, N>(PhantomData<(E, N)>);
pub struct TNil;


pub trait TIndex<V>: Nat {
    type Result;
}


impl<E, I: Nat + TIndex<N>, N> TIndex<TCons<E, N>> for S<I> {
    type Result = <I as TIndex<N>>::Result;
}


impl<E, N> TIndex<TCons<E, N>> for Z {
    type Result = E;
}
