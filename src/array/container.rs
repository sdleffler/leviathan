use std::ops::{Index, IndexMut};

use array::storage::*;
use typehack::dim::*;
use typehack::peano::*;
use typehack::tvec::*;


#[macro_export]
macro_rules! Dims {
    ($t:ty $(, $rest:ty)*) => ($crate::typehack::tvec::TCons<$t, Dims![$($rest),*]>);
    () => ($crate::typehack::tvec::TNil);
}


pub trait Dims {
    type Length: Nat;
    type Index: Homogeneous<usize> + TIndex<Self::Length>;

    fn total(&self) -> usize;
    fn is_valid(&self, &Self::Index) -> bool;
    fn flatten(&self, Self::Index) -> usize;
    fn flatten_safe(&self, Self::Index) -> Option<usize>;
    fn expand(&self, usize) -> Self::Index;
}


pub trait StaticDims: Dims {
    type Product: Nat;

    fn static_is_valid(&Self::Index) -> bool;
    fn static_flatten(Self::Index) -> usize;
    fn static_flatten_safe(Self::Index) -> Option<usize>;
    fn static_expand(usize) -> Self::Index;
}


impl<N: Dim, L: Dims> Dims for TCons<N, L> {
    type Length = S<<L as Dims>::Length>;
    type Index = TCons<usize, <L as Dims>::Index>;

    fn total(&self) -> usize {
        self.elem.reify() * self.next.total()
    }

    fn is_valid(&self, idx: &TCons<usize, <L as Dims>::Index>) -> bool {
        idx.elem < self.elem.reify() && self.next.is_valid(&idx.next)
    }

    fn flatten(&self, idx: TCons<usize, <L as Dims>::Index>) -> usize {
        idx.elem * self.next.total() + self.next.flatten(idx.next)
    }

    fn flatten_safe(&self, idx: TCons<usize, <L as Dims>::Index>) -> Option<usize> {
        if self.is_valid(&idx) {
            Some(self.flatten(idx))
        } else {
            None
        }
    }

    fn expand(&self, idx: usize) -> TCons<usize, <L as Dims>::Index> {
        TCons {
            elem: idx / self.next.total(),
            next: self.next.expand(idx % self.next.total()),
        }
    }
}


impl<N: Nat, L: StaticDims> StaticDims for TCons<N, L>
    where N: NatMul<L::Product>
{
    type Product = <N as NatMul<L::Product>>::Result;

    fn static_is_valid(idx: &TCons<usize, <L as Dims>::Index>) -> bool {
        idx.elem < N::as_usize() && L::static_is_valid(&idx.next)
    }

    fn static_flatten(idx: TCons<usize, <L as Dims>::Index>) -> usize {
        idx.elem * L::Product::as_usize() + L::static_flatten(idx.next)
    }

    fn static_flatten_safe(idx: TCons<usize, <L as Dims>::Index>) -> Option<usize> {
        if Self::static_is_valid(&idx) {
            Some(Self::static_flatten(idx))
        } else {
            None
        }
    }

    fn static_expand(idx: usize) -> TCons<usize, <L as Dims>::Index> {
        let product = L::Product::as_usize();
        TCons {
            elem: idx / product,
            next: L::static_expand(idx % product),
        }
    }
}


impl<N: Dim> Dims for TCons<N, TNil> {
    type Length = Z;
    type Index = TCons<usize, TNil>;

    fn total(&self) -> usize {
        self.elem.reify()
    }

    fn is_valid(&self, idx: &TCons<usize, TNil>) -> bool {
        idx.elem < self.elem.reify()
    }

    fn flatten(&self, idx: TCons<usize, TNil>) -> usize {
        idx.elem
    }

    fn flatten_safe(&self, idx: TCons<usize, TNil>) -> Option<usize> {
        if idx.elem < self.elem.reify() {
            Some(idx.elem)
        } else {
            None
        }
    }

    fn expand(&self, idx: usize) -> TCons<usize, TNil> {
        TCons {
            elem: idx,
            next: TNil,
        }
    }
}


impl<N: Nat> StaticDims for TCons<N, TNil> {
    type Product = N;

    fn static_is_valid(idx: &TCons<usize, TNil>) -> bool {
        idx.elem < N::as_usize()
    }

    fn static_flatten(idx: TCons<usize, TNil>) -> usize {
        idx.elem
    }

    fn static_flatten_safe(idx: TCons<usize, TNil>) -> Option<usize> {
        if Self::static_is_valid(&idx) {
            Some(Self::static_flatten(idx))
        } else {
            None
        }
    }

    fn static_expand(idx: usize) -> TCons<usize, TNil> {
        TCons {
            elem: idx,
            next: TNil,
        }
    }
}


pub trait Array<T, D: Dims> {
    fn length(&self) -> usize;

    fn get(&self, idx: D::Index) -> Option<&T>;
    fn get_mut(&mut self, idx: D::Index) -> Option<&mut T>;
}


pub struct StaticArray<T, D: StaticDims>
    where D::Product: Link<T>
{
    data: Storage<T, D::Product>,
}


impl<T, D: StaticDims> StaticArray<T, D>
    where D::Product: Link<T>
{
    pub fn from_elem(elem: &T) -> Self
        where T: Clone
    {
        StaticArray { data: Storage::from_elem(elem) }
    }

    pub fn from_fn<F: Fn(D::Index) -> T>(f: F) -> Self {
        StaticArray { data: Storage::from_fn(|i| f(D::static_expand(i))) }
    }
}


impl<T, X: Nat> Index<usize> for StaticArray<T, TCons<X, TNil>>
    where <TCons<X, TNil> as StaticDims>::Product: Link<T>
{
    type Output = T;

    #[inline]
    fn index(&self, idx: usize) -> &T {
        self.get(TCons {
                elem: idx,
                next: TNil,
            })
            .unwrap()
    }
}


impl<T, X: Nat> IndexMut<usize> for StaticArray<T, TCons<X, TNil>>
    where <TCons<X, TNil> as StaticDims>::Product: Link<T>
{
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut T {
        self.get_mut(TCons {
                elem: idx,
                next: TNil,
            })
            .unwrap()
    }
}


impl<T, X: Nat + NatMul<Y>, Y: Nat> Index<[usize; 2]> for StaticArray<T, TCons<X, TCons<Y, TNil>>>
    where <TCons<X, TCons<Y, TNil>> as StaticDims>::Product: Link<T>
{
    type Output = T;

    #[inline]
    fn index(&self, idx: [usize; 2]) -> &T {
        self.get(TCons {
                elem: idx[0],
                next: TCons {
                    elem: idx[1],
                    next: TNil,
                },
            })
            .unwrap()
    }
}


impl<T, X: Nat + NatMul<Y>, Y: Nat> IndexMut<[usize; 2]> for StaticArray<T, TCons<X, TCons<Y, TNil>>>
    where <TCons<X, TCons<Y, TNil>> as StaticDims>::Product: Link<T>
{
    #[inline]
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut T {
        self.get_mut(TCons {
                elem: idx[0],
                next: TCons {
                    elem: idx[1],
                    next: TNil,
                },
            })
            .unwrap()
    }
}


impl<T, X: Nat + NatMul<<Y as NatMul<Z>>::Result>, Y: Nat + NatMul<Z>, Z: Nat> Index<[usize; 3]>
    for StaticArray<T, TCons<X, TCons<Y, TCons<Z, TNil>>>>
    where <TCons<X, TCons<Y, TCons<Z, TNil>>> as StaticDims>::Product: Link<T>
{
    type Output = T;

    #[inline]
    fn index(&self, idx: [usize; 3]) -> &T {
        self.get(TCons {
                elem: idx[0],
                next: TCons {
                    elem: idx[1],
                    next: TCons {
                        elem: idx[2],
                        next: TNil,
                    },
                },
            })
            .unwrap()
    }
}


impl<T, X: Nat + NatMul<<Y as NatMul<Z>>::Result>, Y: Nat + NatMul<Z>, Z: Nat> IndexMut<[usize; 3]>
    for StaticArray<T, TCons<X, TCons<Y, TCons<Z, TNil>>>>
    where <TCons<X, TCons<Y, TCons<Z, TNil>>> as StaticDims>::Product: Link<T>
{
    #[inline]
    fn index_mut(&mut self, idx: [usize; 3]) -> &mut T {
        self.get_mut(TCons {
                elem: idx[0],
                next: TCons {
                    elem: idx[1],
                    next: TCons {
                        elem: idx[2],
                        next: TNil,
                    },
                },
            })
            .unwrap()
    }
}


impl<T, D: StaticDims> Array<T, D> for StaticArray<T, D>
    where D::Product: Link<T>
{
    #[inline]
    fn length(&self) -> usize {
        D::Product::as_usize()
    }

    #[inline]
    fn get(&self, idx: D::Index) -> Option<&T> {
        D::static_flatten_safe(idx).map(|i| &self.data[i])
    }

    #[inline]
    fn get_mut(&mut self, idx: D::Index) -> Option<&mut T> {
        D::static_flatten_safe(idx).map(move |i| &mut self.data[i])
    }
}


pub struct DynamicArray<T, D: Dims> {
    dims: D,
    data: Vec<T>,
}


impl<T, D: Dims> Array<T, D> for DynamicArray<T, D> {
    #[inline]
    fn length(&self) -> usize {
        self.dims.total()
    }

    #[inline]
    fn get(&self, idx: D::Index) -> Option<&T> {
        self.dims.flatten_safe(idx).map(|i| &self.data[i])
    }

    #[inline]
    fn get_mut(&mut self, idx: D::Index) -> Option<&mut T> {
        self.dims.flatten_safe(idx).map(move |i| &mut self.data[i])
    }
}


impl<T, X: Dim> Index<usize> for DynamicArray<T, TCons<X, TNil>> {
    type Output = T;

    #[inline]
    fn index(&self, idx: usize) -> &T {
        self.get(TCons {
                elem: idx,
                next: TNil,
            })
            .unwrap()
    }
}


impl<T, X: Dim> IndexMut<usize> for DynamicArray<T, TCons<X, TNil>> {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut T {
        self.get_mut(TCons {
                elem: idx,
                next: TNil,
            })
            .unwrap()
    }
}


impl<T, X: Dim, Y: Dim> Index<[usize; 2]> for DynamicArray<T, TCons<X, TCons<Y, TNil>>> {
    type Output = T;

    #[inline]
    fn index(&self, idx: [usize; 2]) -> &T {
        self.get(TCons {
                elem: idx[0],
                next: TCons {
                    elem: idx[1],
                    next: TNil,
                },
            })
            .unwrap()
    }
}


impl<T, X: Dim, Y: Dim> IndexMut<[usize; 2]> for DynamicArray<T, TCons<X, TCons<Y, TNil>>> {
    #[inline]
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut T {
        self.get_mut(TCons {
                elem: idx[0],
                next: TCons {
                    elem: idx[1],
                    next: TNil,
                },
            })
            .unwrap()
    }
}


impl<T, X: Dim, Y: Dim, Z: Dim> Index<[usize; 3]>
    for DynamicArray<T, TCons<X, TCons<Y, TCons<Z, TNil>>>> {
    type Output = T;

    #[inline]
    fn index(&self, idx: [usize; 3]) -> &T {
        self.get(TCons {
                elem: idx[0],
                next: TCons {
                    elem: idx[1],
                    next: TCons {
                        elem: idx[2],
                        next: TNil,
                    },
                },
            })
            .unwrap()
    }
}


impl<T, X: Dim, Y: Dim, Z: Dim> IndexMut<[usize; 3]>
    for DynamicArray<T, TCons<X, TCons<Y, TCons<Z, TNil>>>> {
    #[inline]
    fn index_mut(&mut self, idx: [usize; 3]) -> &mut T {
        self.get_mut(TCons {
                elem: idx[0],
                next: TCons {
                    elem: idx[1],
                    next: TCons {
                        elem: idx[2],
                        next: TNil,
                    },
                },
            })
            .unwrap()
    }
}
