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
    ($op:tt, $lhs:expr, $rhs:expr, $m:expr, $n:expr) => (madd_inplace!($op, $lhs, $rhs, $m, $n, $m, $n));
    ($op:tt, $lhs:expr, $rhs:expr, $m:expr, $n:expr, $p:expr, $q:expr) => (
        {
            let mut lhs = $lhs;

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
pub struct StaticMat<T: Copy, M: Nat, N: Nat>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    elems: Storage<T, <M as NatMul<N>>::Result>,
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DynamicMat<T: Copy, M: Dim, N: Dim> {
    rows: M,
    cols: N,
    elems: Vec<T>,
}


impl<T: Copy, M: Nat, N: Nat> Copy for StaticMat<T, M, N>
    where StaticMat<T, M, N>: Clone,
          M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>,
          Storage<T, <M as NatMul<N>>::Result>: Copy
{
}


impl<'a, T: Copy, M: Nat, N: Nat> From<&'a [T]> for StaticMat<T, M, N>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    fn from(slice: &[T]) -> Self {
        StaticMat { elems: Storage::from_slice(slice) }
    }
}


impl<T: Copy, M: Nat, N: Nat> From<T> for StaticMat<T, M, N>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    fn from(elem: T) -> Self {
        StaticMat { elems: Storage::from_elem(&elem) }
    }
}


impl<T: Copy, M: Nat, N: Nat> Matrix for StaticMat<T, M, N>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    type Rows = M;
    type Cols = N;

    type Scalar = T;
}


impl<T: Copy, N: Nat> Square for StaticMat<T, N, N>
    where N: NatMul<N>,
          <N as NatMul<N>>::Result: Link<T>
{
}


impl<T: Copy, M: Nat, N: Nat> MatrixTranspose for StaticMat<T, M, N>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>,
          N: NatMul<M, Result = <M as NatMul<N>>::Result>
{
    type Output = StaticMat<T, N, M>;

    fn transpose(self) -> StaticMat<T, N, M> {
        let mut res: Self::Output; // Transposing in-place is more work than it's worth.

        unsafe {
            res = mem::uninitialized(); // Give ourselves some scratch space.

            for i in 0..M::as_usize() {
                for j in 0..N::as_usize() {
                    ptr::write(&mut res.elems[j + i * M::as_usize()],
                               self.elems[i + j * N::as_usize()]);
                }
            }

            // `res` should now be fully initialized.
        }

        res
    }
}


impl<T: Copy, M: Nat, N: Nat> MatrixAdd for StaticMat<T, M, N>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>,
          T: AddAssign
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        madd_inplace!(+=, self, rhs, M::as_usize(), N::as_usize())
    }
}


impl<T: Copy, M: Nat, N: Nat> MatrixSub for StaticMat<T, M, N>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>,
          T: SubAssign
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        madd_inplace!(-=, self, rhs, M::as_usize(), N::as_usize())
    }
}


impl<T: Copy, M: Nat, N: Nat, P: Nat> MatrixMul<StaticMat<T, N, P>> for StaticMat<T, M, N>
    where M: NatMul<N> + NatMul<P>,
          N: NatMul<P>,
          <M as NatMul<N>>::Result: Link<T>,
          <N as NatMul<P>>::Result: Link<T>,
          <M as NatMul<P>>::Result: Link<T>,
          T: AddAssign + Mul<Output = T> + Zero
{
    type Output = StaticMat<T, M, P>;

    fn mul(self, rhs: StaticMat<T, N, P>) -> StaticMat<T, M, P> {
        mmul_loop_naive!(self,
                         rhs,
                         StaticMat::from(T::zero()),
                         M::as_usize(),
                         N::as_usize(),
                         N::as_usize(),
                         P::as_usize())
    }
}


impl<T: Copy, N: Nat> MatrixIdentity for StaticMat<T, N, N>
    where N: NatMul<N>,
          <N as NatMul<N>>::Result: Link<T>,
          T: Zero + One
{
    fn eye() -> Self {
        let mut res = StaticMat { elems: Storage::from_elem(&T::zero()) };

        for i in 0..N::as_usize() {
            res[[i, i]] = T::one();
        }

        res
    }
}


impl<T: Copy, M: Nat, N: Nat> Index<[usize; 2]> for StaticMat<T, M, N>
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


impl<T: Copy, M: Nat, N: Nat> IndexMut<[usize; 2]> for StaticMat<T, M, N>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    #[inline]
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut T {
        assert!(idx[0] < N::as_usize());

        &mut self.elems[idx[0] + idx[1] * N::as_usize()]
    }
}


impl<T: Copy, M: Dim, N: Dim> Index<[usize; 2]> for DynamicMat<T, M, N> {
    type Output = T;

    #[inline]
    fn index(&self, idx: [usize; 2]) -> &T {
        &self.elems[idx[0] + idx[1] * self.cols.reify()]
    }
}


impl<T: Copy, M: Dim, N: Dim> IndexMut<[usize; 2]> for DynamicMat<T, M, N> {
    #[inline]
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut T {
        &mut self.elems[idx[0] + idx[1] * self.cols.reify()]
    }
}


impl<T: Copy> DynamicMat<T, Dyn, Dyn> {
    pub fn from_elem(m: usize, n: usize, elem: T) -> DynamicMat<T, Dyn, Dyn> {
        DynamicMat {
            rows: Dyn(m),
            cols: Dyn(n),
            elems: vec![elem; m * n],
        }
    }
}


impl<T: Copy, M: Dim + Nat> DynamicMat<T, M, Dyn> {
    pub fn from_elem(n: usize, elem: T) -> DynamicMat<T, M, Dyn> {
        DynamicMat {
            rows: M::as_data(),
            cols: Dyn(n),
            elems: vec![elem; M::as_usize() * n],
        }
    }
}


impl<T: Copy, N: Dim + Nat> DynamicMat<T, Dyn, N> {
    pub fn from_elem(m: usize, elem: T) -> DynamicMat<T, Dyn, N> {
        DynamicMat {
            rows: Dyn(m),
            cols: N::as_data(),
            elems: vec![elem; m * N::as_usize()],
        }
    }
}


impl<T: Copy, M: Dim + Nat, N: Dim + Nat> DynamicMat<T, M, N> {
    pub fn from_elem(elem: T) -> DynamicMat<T, M, N> {
        DynamicMat {
            rows: M::as_data(),
            cols: N::as_data(),
            elems: vec![elem; M::as_usize() * N::as_usize()],
        }
    }
}


impl<T: Copy, M: Dim, N: Dim> Matrix for DynamicMat<T, M, N> {
    type Rows = M;
    type Cols = N;

    type Scalar = T;
}


// Matrix multiplication: DynamicMat times DynamicMat.

impl<T: Copy, M: Dim + Nat, N: Dim, P: Dim, Q: Dim + Nat> MatrixMul<DynamicMat<T, P, Q>>
    for DynamicMat<T, M, N>
    where N: DimCompat<P>,
          M: NatMul<Q>,
          <M as NatMul<Q>>::Result: Link<T>,
          T: Mul<Output = T> + AddAssign + Zero
{
    type Output = StaticMat<T, M, Q>;

    fn mul(self, rhs: DynamicMat<T, P, Q>) -> Self::Output {
        mmul_loop_naive!(self,
                       rhs,
                       StaticMat::from(T::zero()),
                       M::as_usize(),
                       self.cols.reify(),
                       rhs.rows.reify(),
                       Q::as_usize())
    }
}


impl<T: Copy, N: Dim, P: Dim, Q: Dim + Nat> MatrixMul<DynamicMat<T, P, Q>> for DynamicMat<T, Dyn, N>
    where N: DimCompat<P>,
          T: Mul<Output = T> + AddAssign + Zero
{
    type Output = DynamicMat<T, Dyn, Q>;

    fn mul(self, rhs: DynamicMat<T, P, Q>) -> Self::Output {
        let m = self.rows.reify();

        mmul_loop_naive!(self,
                         rhs,
                         Self::Output::from_elem(m, T::zero()),
                         m,
                         self.cols.reify(),
                         rhs.rows.reify(),
                         Q::as_usize())
    }
}


impl<T: Copy, M: Dim + Nat, N: Dim, P: Dim> MatrixMul<DynamicMat<T, P, Dyn>> for DynamicMat<T, M, N>
    where N: DimCompat<P>,
          T: Mul<Output = T> + AddAssign + Zero
{
    type Output = DynamicMat<T, M, Dyn>;

    fn mul(self, rhs: DynamicMat<T, P, Dyn>) -> Self::Output {
        let q = rhs.cols.reify();

        mmul_loop_naive!(self,
                         rhs,
                         Self::Output::from_elem(q, T::zero()),
                         M::as_usize(),
                         self.cols.reify(),
                         rhs.rows.reify(),
                         q)
    }
}


impl<T: Copy, N: Dim, P: Dim> MatrixMul<DynamicMat<T, P, Dyn>> for DynamicMat<T, Dyn, N>
    where N: DimCompat<P>,
          T: Mul<Output = T> + AddAssign + Zero
{
    type Output = DynamicMat<T, Dyn, Dyn>;

    fn mul(self, rhs: DynamicMat<T, P, Dyn>) -> Self::Output {
        let m = self.rows.reify();
        let q = rhs.cols.reify();

        mmul_loop_naive!(self,
                         rhs,
                         Self::Output::from_elem(m, q, T::zero()),
                         m,
                         self.cols.reify(),
                         rhs.rows.reify(),
                         q)
    }
}


impl<T: Copy, M: Nat, N: Nat, P: Dim, Q: Nat> MatrixMul<DynamicMat<T, P, Q>> for StaticMat<T, M, N>
    where N: DimCompat<P>,
          M: NatMul<N> + NatMul<Q>,
          <M as NatMul<N>>::Result: Link<T>,
          <M as NatMul<Q>>::Result: Link<T>,
          T: Mul<Output = T> + AddAssign + Zero
{
    type Output = StaticMat<T, M, Q>;

    fn mul(self, rhs: DynamicMat<T, P, Q>) -> Self::Output {
        mmul_loop_naive!(self,
                         rhs,
                         Self::Output::from(T::zero()),
                         M::as_usize(),
                         N::as_usize(),
                         rhs.rows.reify(),
                         Q::as_usize())
    }
}


impl<T: Copy, M: Nat, N: Nat, P: Dim> MatrixMul<DynamicMat<T, P, Dyn>> for StaticMat<T, M, N>
    where N: DimCompat<P>,
          M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>,
          T: Mul<Output = T> + AddAssign + Zero
{
    type Output = DynamicMat<T, M, Dyn>;

    fn mul(self, rhs: DynamicMat<T, P, Dyn>) -> Self::Output {
        let q = rhs.cols.reify();

        mmul_loop_naive!(self,
                         rhs,
                         Self::Output::from_elem(q, T::zero()),
                         M::as_usize(),
                         N::as_usize(),
                         rhs.rows.reify(),
                         q)
    }
}


impl<T: Copy, M: Nat, N: Dim, P: Nat, Q: Nat> MatrixMul<StaticMat<T, P, Q>> for DynamicMat<T, M, N>
    where N: DimCompat<P>,
          M: NatMul<Q>,
          P: NatMul<Q>,
          <M as NatMul<Q>>::Result: Link<T>,
          <P as NatMul<Q>>::Result: Link<T>,
          T: Mul<Output = T> + AddAssign + Zero
{
    type Output = StaticMat<T, M, Q>;

    fn mul(self, rhs: StaticMat<T, P, Q>) -> Self::Output {
        mmul_loop_naive!(self,
                         rhs,
                         Self::Output::from(T::zero()),
                         M::as_usize(),
                         self.cols.reify(),
                         P::as_usize(),
                         Q::as_usize())
    }
}


impl<T: Copy, N: Dim, P: Nat, Q: Nat> MatrixMul<StaticMat<T, P, Q>> for DynamicMat<T, Dyn, N>
    where N: DimCompat<P>,
          P: NatMul<Q>,
          <P as NatMul<Q>>::Result: Link<T>,
          T: Mul<Output = T> + AddAssign + Zero
{
    type Output = DynamicMat<T, Dyn, Q>;

    fn mul(self, rhs: StaticMat<T, P, Q>) -> Self::Output {
        let m = self.rows.reify();

        mmul_loop_naive!(self,
                         rhs,
                         Self::Output::from_elem(m, T::zero()),
                         m,
                         self.cols.reify(),
                         P::as_usize(),
                         Q::as_usize())
    }
}


impl<T: Copy, M: Nat, N: Nat, P: Dim, Q: Dim> MatrixAdd<DynamicMat<T, P, Q>> for StaticMat<T, M, N>
    where M: DimCompat<P> + NatMul<N>,
          N: DimCompat<Q>,
          <M as NatMul<N>>::Result: Link<T>,
          T: AddAssign
{
    type Output = StaticMat<T, M, N>;

    fn add(self, rhs: DynamicMat<T, P, Q>) -> Self::Output {
        madd_inplace!(+=, self, rhs, M::as_usize(), N::as_usize(), rhs.rows.reify(), rhs.cols.reify())
    }
}
