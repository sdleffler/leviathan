use std::marker::PhantomData;
use std::mem;
use std::ops::{AddAssign, SubAssign, Mul, Index, IndexMut};
use std::ptr;

use array::storage::*;
use linalg::traits::*;
use num::traits::*;
use typehack::dim::*;
use typehack::peano::*;


// Notation:
// - `N`, `M`, `P`, or `Q` implies a type implementing `Nat`;
// - `N?`, `M?`, `P?`, or `Q?` implies a type implementing `Dim`, and could thus either be a `Nat` or `Dyn`;
// - `Dyn` implies a dynamic dimensionality.
//
// See linalg.todo.


macro_rules! mmul_loop_naive {
    ($lhs:expr, $rhs:expr, $out:expr, $m:expr, $n:expr, $p:expr, $q:expr) => (
        {
            let mut out = $out;

            let n = $n;
            let p = $p;

            debug_assert_eq!(n, p);

            let n = ::std::cmp::min(n, p);

            for i in 0..$m {
                for j in 0..$q {
                    for k in 0..n {
                        out[[i, j]] += $lhs[[i, k]] * $rhs[[k, j]];
                    }
                }
            }

            out
        }
    );
}


macro_rules! madd_inplace {
    (@$layout:ident, $op:tt, $lhs:expr, $rhs:expr, $m:expr, $n:expr, $p:expr, $q:expr) => (
        {
            let m = $m;
            let n = $n;
            let p = $p;
            let q = $q;

            debug_assert_eq!(m, p);
            debug_assert_eq!(n, q);

            let m = ::std::cmp::min(m, p);
            let n = ::std::cmp::min(n, q);

            madd_inplace!(@$layout, $op, $lhs, $rhs, m, n)
        }
    );
    (@col, $op:tt, $lhs:expr, $rhs:expr, $m:expr, $n:expr) => (
        {
            let mut lhs = $lhs;

            for i in 0..$m {
                for j in 0..$n {
                    lhs[[i, j]] $op $rhs[[i, j]];
                }
            }

            lhs
        }
    );
    (@row, $op:tt, $lhs:expr, $rhs:expr, $m:expr, $n:expr) => (
        {
            let mut lhs = $lhs;

            for j in 0..$n {
                for i in 0..$m {
                    lhs[[i, j]] $op $rhs[[i, j]];
                }
            }

            lhs
        }
    );
}


macro_rules! madd_allocating {
    ($op:tt, $lhs:expr, $rhs:expr, $out:expr, $m:expr, $n:expr, $p:expr, $q:expr) => {
        {
            let mut out = $out;

            let m = $m;
            let n = $n;
            let p = $p;
            let q = $q;

            debug_assert_eq!(m, p);
            debug_assert_eq!(n, q);

            let m = ::std::cmp::min(m, p);
            let n = ::std::cmp::min(n, q);

            for i in 0..m {
                for j in 0..n {
                    out[[i, j]] = $lhs[[i, j]] $op $rhs[[i, j]];
                }
            }

            out
        }
    }
}


#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct StaticMat<T: Copy, M: Nat, N: Nat, L: Layout>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    elems: Storage<T, <M as NatMul<N>>::Result>,
    phantom: PhantomData<L>,
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DynamicMat<T: Copy, M: Dim, N: Dim, L: Layout> {
    rows: M,
    cols: N,
    elems: Vec<T>,
    phantom: PhantomData<L>,
}


impl<T: Copy, M: Nat, N: Nat, L: Layout> Copy for StaticMat<T, M, N, L>
    where StaticMat<T, M, N, L>: Clone,
          M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>,
          Storage<T, <M as NatMul<N>>::Result>: Copy
{
}


impl<'a, T: Copy, M: Nat, N: Nat, L: Layout> From<&'a [T]> for StaticMat<T, M, N, L>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    fn from(slice: &[T]) -> Self {
        StaticMat {
            elems: Storage::from_slice(slice),
            phantom: PhantomData,
        }
    }
}


impl<T: Copy, M: Nat, N: Nat, L: Layout> From<T> for StaticMat<T, M, N, L>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    fn from(elem: T) -> Self {
        StaticMat {
            elems: Storage::from_elem(&elem),
            phantom: PhantomData,
        }
    }
}


impl<T: Copy, M: Dim, N: Dim, L: Layout> DynamicMat<T, M, N, L> {
    pub fn from_elem(m: M, n: N, elem: T) -> DynamicMat<T, M, N, L> {
        DynamicMat {
            rows: m,
            cols: n,
            elems: vec![elem; m.reify() * n.reify()],
            phantom: PhantomData,
        }
    }
}


impl<T: Copy, M: Nat, N: Nat, L: Layout> Matrix for StaticMat<T, M, N, L>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    type Rows = M;
    type Cols = N;

    type Scalar = T;
}


impl<T: Copy, M: Dim, N: Dim, L: Layout> Matrix for DynamicMat<T, M, N, L> {
    type Rows = M;
    type Cols = N;

    type Scalar = T;
}


impl<T: Copy, N: Nat, L: Layout> Square for StaticMat<T, N, N, L>
    where N: NatMul<N>,
          <N as NatMul<N>>::Result: Link<T>
{
}


impl<T: Copy, N: Nat, L: Layout> Square for DynamicMat<T, N, N, L> {}


impl<T: Copy, M: Nat, N: Nat, L: Layout> MatrixTranspose<StaticMat<T, N, M, L::Alternate>> for StaticMat<T, M, N, L>
    where M: NatMul<N>,
    N: NatMul<M, Result = <M as NatMul<N>>::Result>,
          <M as NatMul<N>>::Result: Link<T>
{
    fn transpose(self) -> StaticMat<T, N, M, L::Alternate> {
        StaticMat { elems: self.elems, phantom: PhantomData }
    }
}


impl<T: Copy, M: Nat, N: Nat> MatrixAdd for StaticMat<T, M, N, RowMajor>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>,
          T: AddAssign
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        madd_inplace!(@row, +=, self, rhs, M::as_usize(), N::as_usize())
    }
}


impl<T: Copy, M: Nat, N: Nat> MatrixAdd for StaticMat<T, M, N, ColMajor>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>,
          T: AddAssign
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        madd_inplace!(@col, +=, self, rhs, M::as_usize(), N::as_usize())
    }
}


impl<T: Copy, M: Nat, N: Nat> MatrixSub for StaticMat<T, M, N, RowMajor>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>,
          T: SubAssign
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        madd_inplace!(@row, -=, self, rhs, M::as_usize(), N::as_usize())
    }
}


impl<T: Copy, M: Nat, N: Nat> MatrixSub for StaticMat<T, M, N, ColMajor>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>,
          T: SubAssign
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        madd_inplace!(@col, -=, self, rhs, M::as_usize(), N::as_usize())
    }
}


impl<T: Copy, N: Nat, L: Layout> MatrixIdentity for StaticMat<T, N, N, L>
    where StaticMat<T, N, N, L>: IndexMut<[usize; 2], Output = T>,
          N: NatMul<N>,
          <N as NatMul<N>>::Result: Link<T>,
          T: Zero + One
{
    fn eye() -> Self {
        let mut res = StaticMat {
            elems: Storage::from_elem(&T::zero()),
            phantom: PhantomData,
        };

        for i in 0..N::as_usize() {
            res[[i, i]] = T::one();
        }

        res
    }
}


impl<T: Copy, N: Nat, L: Layout> MatrixIdentity for DynamicMat<T, N, N, L>
    where DynamicMat<T, N, N, L>: IndexMut<[usize; 2], Output = T>,
          T: Zero + One
{
    fn eye() -> Self {
        let mut res = DynamicMat {
            rows: N::as_data(),
            cols: N::as_data(),
            elems: vec![T::zero(); N::as_usize() * N::as_usize()],
            phantom: PhantomData,
        };

        for i in 0..N::as_usize() {
            res[[i, i]] = T::one();
        }

        res
    }
}


impl<T: Copy, M: Nat, N: Nat> Index<[usize; 2]> for StaticMat<T, M, N, RowMajor>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    type Output = T;

    #[inline]
    fn index(&self, idx: [usize; 2]) -> &T {
        assert!(idx[0] < N::as_usize());

        &self.elems[idx[0] + idx[1] * N::as_usize()]
    }
}


impl<T: Copy, M: Nat, N: Nat> Index<[usize; 2]> for StaticMat<T, M, N, ColMajor>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    type Output = T;

    #[inline]
    fn index(&self, idx: [usize; 2]) -> &T {
        assert!(idx[1] < M::as_usize());

        &self.elems[idx[0] * M::as_usize() + idx[1]]
    }
}


impl<T: Copy, M: Nat, N: Nat> IndexMut<[usize; 2]> for StaticMat<T, M, N, RowMajor>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    #[inline]
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut T {
        assert!(idx[1] < M::as_usize());

        &mut self.elems[idx[0] * M::as_usize() + idx[1]]
    }
}


impl<T: Copy, M: Nat, N: Nat> IndexMut<[usize; 2]> for StaticMat<T, M, N, ColMajor>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    #[inline]
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut T {
        assert!(idx[0] < N::as_usize());

        &mut self.elems[idx[0] + idx[1] * N::as_usize()]
    }
}


impl<T: Copy, M: Dim, N: Dim> Index<[usize; 2]> for DynamicMat<T, M, N, RowMajor> {
    type Output = T;

    #[inline]
    fn index(&self, idx: [usize; 2]) -> &T {
        assert!(idx[1] < self.rows.reify());

        &self.elems[idx[0] * self.rows.reify() + idx[1]]
    }
}


impl<T: Copy, M: Dim, N: Dim> Index<[usize; 2]> for DynamicMat<T, M, N, ColMajor> {
    type Output = T;

    #[inline]
    fn index(&self, idx: [usize; 2]) -> &T {
        assert!(idx[0] < self.cols.reify());

        &self.elems[idx[0] + idx[1] * self.cols.reify()]
    }
}


impl<T: Copy, M: Dim, N: Dim> IndexMut<[usize; 2]> for DynamicMat<T, M, N, RowMajor> {
    #[inline]
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut T {
        assert!(idx[1] < self.rows.reify());

        &mut self.elems[idx[0] * self.rows.reify() + idx[1]]
    }
}


impl<T: Copy, M: Dim, N: Dim> IndexMut<[usize; 2]> for DynamicMat<T, M, N, ColMajor> {
    #[inline]
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut T {
        assert!(idx[0] < self.cols.reify());

        &mut self.elems[idx[0] + idx[1] * self.cols.reify()]
    }
}
