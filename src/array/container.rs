use std::ops::{Index, IndexMut};

use typehack::data::*;
use typehack::dim::*;
use typehack::binary::Nat as BNat;
use typehack::binary::I;
use typehack::peano::Nat as PNat;
use typehack::peano::{S, Z};
use typehack::tvec::*;


#[macro_export]
macro_rules! Dims {
    ($t:ty $(, $rest:ty)*) => ($crate::typehack::tvec::TCons<$t, Dims![$($rest),*]>);
    () => ($crate::typehack::tvec::TNil);
}


#[macro_export]
macro_rules! dims {
    (($t:expr) $(, $rest:tt)*) => ($crate::typehack::tvec::TCons { elem: $crate::typehack::dim::Dyn($t), next: dims![$($rest),*] });
    ($t:ident $(, $rest:tt)*) => ($crate::typehack::tvec::TCons { elem: $t::as_data(), next: dims![$($rest),*] });
    () => ($crate::typehack::tvec::TNil);
}


pub trait Dims<T>: Copy {
    type Length: PNat;
    type Index: Homogeneous<usize>;

    type Product: Store<T>;

    fn product(&self) -> Self::Product;

    fn total(&self) -> usize;
    fn is_valid(&self, &Self::Index) -> bool;
    fn flatten(&self, Self::Index) -> usize;
    fn flatten_safe(&self, Self::Index) -> Option<usize>;
    fn expand(&self, usize) -> Self::Index;
}


macro_rules! dims_impl {
    (@TCons) => {
        type Length = S<L::Length>;
        type Index = TCons<usize, L::Index>;

        fn total(&self) -> usize {
            self.elem.reify() * self.next.total()
        }

        fn is_valid(&self, idx: &TCons<usize, L::Index>) -> bool {
            idx.elem < self.elem.reify() && self.next.is_valid(&idx.next)
        }

        fn flatten(&self, idx: TCons<usize, L::Index>) -> usize {
            idx.elem * self.next.total() + self.next.flatten(idx.next)
        }

        fn flatten_safe(&self, idx: TCons<usize, L::Index>) -> Option<usize> {
            if self.is_valid(&idx) {
                Some(self.flatten(idx))
            } else {
                None
            }
        }

        fn expand(&self, idx: usize) -> TCons<usize, L::Index> {
            TCons {
                elem: idx / self.next.total(),
                next: self.next.expand(idx % self.next.total()),
            }
        }
    };
    (@TNil) => {
        type Length = Z;
        type Index = TNil;

        type Product = I;

        fn total(&self) -> usize {
            1
        }

        fn is_valid(&self, _idx: &TNil) -> bool {
            true
        }

        fn flatten(&self, _idx: TNil) -> usize {
            0
        }

        fn flatten_safe(&self, _idx: TNil) -> Option<usize> {
            Some(0)
        }

        fn expand(&self, _idx: usize) -> TNil {
            TNil
        }
    };
}


impl<T, M: BNat + DimMul<N>, N: Dim + Store<T>, L: Dims<T, Product = N>> Dims<T> for TCons<M, L>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Store<T>
{
    type Product = <M as DimMul<N>>::Result;

    fn product(&self) -> Self::Product {
        DimMul::mul(self.elem, self.next.product())
    }

    dims_impl!(@TCons);
}


impl<T, L: Dims<T>> Dims<T> for TCons<Dyn, L>
    where Dyn: DimMul<L::Product>,
          <Dyn as DimMul<L::Product>>::Result: Store<T>
{
    type Product = <Dyn as DimMul<L::Product>>::Result;

    fn product(&self) -> Self::Product {
        DimMul::mul(self.elem, self.next.product())
    }

    dims_impl!(@TCons);
}


impl<T> Dims<T> for TNil {
    fn product(&self) -> Self::Product {
        Self::Product::as_data()
    }

    dims_impl!(@TNil);
}


impl<T, X: Dim> Index<usize> for Array<T, TCons<X, TNil>>
    where TCons<X, TNil>: Dims<T, Index = TCons<usize, TNil>>
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


impl<T, X: Dim> IndexMut<usize> for Array<T, TCons<X, TNil>>
    where TCons<X, TNil>: Dims<T, Index = TCons<usize, TNil>>
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


impl<T, X: Dim, Y: Dim> Index<[usize; 2]> for Array<T, TCons<X, TCons<Y, TNil>>>
    where TCons<X, TCons<Y, TNil>>: Dims<T, Index = TCons<usize, TCons<usize, TNil>>>
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


impl<T, X: Dim, Y: Dim> IndexMut<[usize; 2]> for Array<T, TCons<X, TCons<Y, TNil>>>
    where TCons<X, TCons<Y, TNil>>: Dims<T, Index = TCons<usize, TCons<usize, TNil>>>
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


impl<T, X: Dim, Y: Dim, Z: Dim> Index<[usize; 3]> for Array<T, TCons<X, TCons<Y, TCons<Z, TNil>>>>
    where TCons<X, TCons<Y, TCons<Z, TNil>>>: Dims<T,
                                                   Index = TCons<usize,
                                                                 TCons<usize, TCons<usize, TNil>>>>
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


impl<T, X: Dim, Y: Dim, Z: Dim> IndexMut<[usize; 3]>
    for Array<T, TCons<X, TCons<Y, TCons<Z, TNil>>>>
    where TCons<X, TCons<Y, TCons<Z, TNil>>>: Dims<T,
                                                   Index = TCons<usize,
                                                                 TCons<usize, TCons<usize, TNil>>>>
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


pub struct Array<T, D: Dims<T>>
    where D::Product: Store<T>
{
    dims: D,
    data: Data<T, D::Product>,
}


impl<T, D: Dims<T>> Array<T, D> {
    #[inline]
    pub fn length(&self) -> usize {
        self.dims.total()
    }

    #[inline]
    pub fn get(&self, idx: D::Index) -> Option<&T> {
        self.dims.flatten_safe(idx).map(|i| &self.data[i])
    }

    #[inline]
    pub fn get_mut(&mut self, idx: D::Index) -> Option<&mut T> {
        self.dims.flatten_safe(idx).map(move |i| &mut self.data[i])
    }


    pub fn from_elem(dims: D, elem: &T) -> Self
        where T: Clone
    {
        Array {
            dims: dims,
            data: Data::from_elem(dims.product(), elem),
        }
    }


    pub fn from_fn<F: Fn(D::Index) -> T>(dims: D, f: F) -> Self {
        Array {
            dims: dims,
            data: Data::from_fn(dims.product(), |idx| f(dims.expand(idx))),
        }
    }
}
