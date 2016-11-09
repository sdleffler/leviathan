use std::marker::PhantomData;
use std::ops::{AddAssign, SubAssign, Mul, Index, IndexMut};
use std::ptr;

use linalg::traits::*;
use linalg::vector::*;
use num::traits::*;
use typehack::data::*;
use typehack::dim::*;
use typehack::binary::*;


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
    (@$layout:ident, $op:tt, $lhs:expr, $rhs:expr) => (
        {
            let m = $lhs.rows.reify();
            let n = $lhs.cols.reify();
            let p = $rhs.rows.reify();
            let q = $rhs.cols.reify();

            debug_assert_eq!(m, p);
            debug_assert_eq!(n, q);

            madd_inplace!(@$layout, $op, $lhs, $rhs, ::std::cmp::min(m, p), ::std::cmp::min(n, q))
        }
    );
    (@col, $op:tt, $lhs:expr, $rhs:expr, $m:expr, $n:expr) => (
        {
            let m = $m;
            let n = $n;

            let mut lhs = $lhs;

            for i in 0..m {
                for j in 0..n {
                    lhs[[i, j]] $op $rhs[[i, j]];
                }
            }

            lhs
        }
    );
    (@row, $op:tt, $lhs:expr, $rhs:expr, $m:expr, $n:expr) => (
        {
            let m = $m;
            let n = $n;

            let mut lhs = $lhs;

            for j in 0..n {
                for i in 0..m {
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


#[derive(PartialEq, Eq, Debug)]
#[repr(C)]
pub struct DenseMat<T: Scalar, M: Dim, N: Dim, L: Layout>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Size<T>
{
    rows: M,
    cols: N,
    elems: Data<T, <M as DimMul<N>>::Result>,
    phantom: PhantomData<L>,
}


impl<T: Clone + Scalar, M: Nat, N: Nat, L: Layout> Clone for DenseMat<T, M, N, L>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Size<T>,
          Data<T, <M as NatMul<N>>::Result>: Clone
{
    fn clone(&self) -> Self {
        DenseMat {
            rows: self.rows,
            cols: self.cols,
            elems: self.elems.clone(),
            phantom: PhantomData,
        }
    }
}


impl<T: Copy + Scalar, M: Dim, N: Dim, L: Layout> Copy for DenseMat<T, M, N, L>
    where DenseMat<T, M, N, L>: Clone,
          M: DimMul<N>,
          <M as DimMul<N>>::Result: Size<T>,
          Data<T, <M as DimMul<N>>::Result>: Copy
{
}


impl<'a, T: Copy + Scalar, M: Nat, N: Nat, L: Layout> From<&'a [T]> for DenseMat<T, M, N, L>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Size<T>
{
    fn from(slice: &[T]) -> Self {
        DenseMat {
            rows: M::as_data(),
            cols: N::as_data(),
            elems: Data::from_slice(<M as NatMul<N>>::Result::as_data(), slice),
            phantom: PhantomData,
        }
    }
}


impl<T: Copy + Scalar, M: Nat, N: Nat, L: Layout> From<T> for DenseMat<T, M, N, L>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Size<T>
{
    fn from(elem: T) -> Self {
        DenseMat {
            rows: M::as_data(),
            cols: N::as_data(),
            elems: Data::from_elem(<M as NatMul<N>>::Result::as_data(), &elem),
            phantom: PhantomData,
        }
    }
}


impl<T: Scalar, M: Dim, N: Dim, L: Layout> DenseMat<T, M, N, L>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Size<T>
{
    fn from_elem(rows: M, cols: N, elem: T) -> DenseMat<T, M, N, L>
        where T: Clone
    {
        DenseMat {
            rows: rows,
            cols: cols,
            elems: Data::from_elem(DimMul::mul(rows, cols), &elem),
            phantom: PhantomData,
        }
    }


    pub unsafe fn uninitialized(rows: M, cols: N) -> Self {
        DenseMat {
            rows: rows,
            cols: cols,
            elems: Data::uninitialized(rows.mul(cols)),
            phantom: PhantomData,
        }
    }


    pub fn from_data(rows: M, cols: N, data: Data<T, <M as DimMul<N>>::Result>) -> Self {
        DenseMat {
            rows: rows,
            cols: cols,
            elems: data,
            phantom: PhantomData,
        }
    }
}


impl<T: Scalar, N: Size<T>, P: DimMul<Q, Result = N>, Q: Dim, L: Layout> From<DenseMat<T, P, Q, L>> for DenseVec<T, N> {
    fn from(mat: DenseMat<T, P, Q, L>) -> DenseVec<T, N> {
        DenseVec::from_data(mat.elems)
    }
}


impl<T: Scalar, M: Dim, N: Dim, L: Layout> Matrix for DenseMat<T, M, N, L>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Size<T>
{
    type Rows = M;
    type Cols = N;

    type Scalar = T;


    fn rows(&self) -> M {
        self.rows
    }

    fn cols(&self) -> N {
        self.cols
    }


    default fn get(&self, _: usize, _: usize) -> &Self::Scalar {
        unimplemented!();
    }

    default fn get_mut(&mut self, _: usize, _: usize) -> &mut Self::Scalar {
        unimplemented!();
    }

    default fn swap(&mut self, _: [usize; 2], _: [usize; 2]) {
        unimplemented!();
    }


    fn row_switch_mut(&mut self, i: usize, j: usize) {
        for k in 0..self.cols.reify() {
            self.swap([i, k], [j, k]);
        }
    }

    fn row_mul_mut(&mut self, i: usize, c: &T)
        where T: Clone
    {
        for k in 0..self.cols.reify() {
            self[[i, k]] *= c.clone();
        }
    }

    fn row_add_mut(&mut self, i: usize, j: usize, c: &T)
        where T: Clone
    {
        for k in 0..self.cols.reify() {
            self[[i, k]] += self[[j, k]].clone() * c.clone();
        }
    }


    fn col_switch_mut(&mut self, i: usize, j: usize) {
        for k in 0..self.rows.reify() {
            self.swap([k, i], [k, j]);
        }
    }

    fn col_mul_mut(&mut self, i: usize, c: &T)
        where T: Clone
    {
        for k in 0..self.rows.reify() {
            self[[k, i]] *= c.clone();
        }
    }

    fn col_add_mut(&mut self, i: usize, j: usize, c: &T)
        where T: Clone
    {
        for k in 0..self.rows.reify() {
            self[[k, i]] += self[[k, j]].clone() * c.clone();
        }
    }
}


impl<T: Scalar, M: Dim, N: Dim> Matrix for DenseMat<T, M, N, Row>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Size<T>
{
    fn get(&self, row: usize, col: usize) -> &Self::Scalar {
        assert!(row < self.rows.reify() && col < self.cols.reify());

        &self.elems[row * self.rows.reify() + col]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Scalar {
        assert!(row < self.rows.reify() && col < self.cols.reify());

        &mut self.elems[row * self.rows.reify() + col]
    }

    fn swap(&mut self, a: [usize; 2], b: [usize; 2]) {
        self.elems.swap(a[0] * self.rows.reify() + a[1],
                        b[0] * self.rows.reify() + b[1]);
    }
}


impl<T: Scalar, M: Dim, N: Dim> Matrix for DenseMat<T, M, N, Column>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Size<T>
{
    fn get(&self, row: usize, col: usize) -> &Self::Scalar {
        assert!(row < self.rows.reify() && col < self.cols.reify());

        &self.elems[row + col * self.cols.reify()]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Scalar {
        assert!(row < self.rows.reify() && col < self.cols.reify());

        &mut self.elems[row + col * self.cols.reify()]
    }

    fn swap(&mut self, a: [usize; 2], b: [usize; 2]) {
        self.elems.swap(a[0] + a[1] * self.cols.reify(),
                        b[0] + b[1] * self.cols.reify());
    }
}


impl<T: Scalar, N: Dim, L: Layout> Square for DenseMat<T, N, N, L>
    where DenseMat<T, N, N, L>: Matrix<Rows = N, Cols = N>,
          N: DimMul<N>,
          <N as DimMul<N>>::Result: Size<T>
{
    type Side = N;

    fn side(&self) -> N {
        assert_eq!(self.rows(), self.cols());
        self.rows()
    }
}


impl<T: Copy + Scalar, M: Dim, N: Dim> MatrixTranspose<DenseMat<T, N, M, Row>>
    for DenseMat<T, M, N, Column>
    where M: DimMul<N>,
          N: DimMul<M, Result = <M as DimMul<N>>::Result>,
          <M as DimMul<N>>::Result: Size<T>
{
    fn transpose(self) -> DenseMat<T, N, M, Row> {
        DenseMat {
            rows: self.cols,
            cols: self.rows,
            elems: self.elems,
            phantom: PhantomData,
        }
    }
}


impl<T: Copy + Scalar, M: Dim, N: Dim> MatrixTranspose<DenseMat<T, N, M, Column>>
    for DenseMat<T, M, N, Row>
    where M: DimMul<N>,
          N: DimMul<M, Result = <M as DimMul<N>>::Result>,
          <M as DimMul<N>>::Result: Size<T>
{
    fn transpose(self) -> DenseMat<T, N, M, Column> {
        DenseMat {
            rows: self.cols,
            cols: self.rows,
            elems: self.elems,
            phantom: PhantomData,
        }
    }
}


macro_rules! mtra_impl {
    ($mat:ident) => {
        unsafe {
            let mut out = DenseMat {
                rows: $mat.cols,
                cols: $mat.rows,
                elems: Data::uninitialized($mat.elems.size),
                phantom: PhantomData,
            };

            for i in 0..$mat.rows.reify() {
                for j in 0..$mat.cols.reify() {
                    ptr::write(&mut out[[j, i]], $mat[[i, j]]);
                }
            }

            out
        }
    }
}


impl<T: Copy + Scalar, M: Dim, N: Dim> MatrixTranspose<DenseMat<T, N, M, Row>>
    for DenseMat<T, M, N, Row>
    where M: DimMul<N>,
          N: DimMul<M, Result = <M as DimMul<N>>::Result>,
          <M as DimMul<N>>::Result: Size<T>
{
    fn transpose(self) -> DenseMat<T, N, M, Row> {
        mtra_impl!(self)
    }
}


impl<T: Copy + Scalar, M: Dim, N: Dim> MatrixTranspose<DenseMat<T, N, M, Column>>
    for DenseMat<T, M, N, Column>
    where M: DimMul<N>,
          N: DimMul<M, Result = <M as DimMul<N>>::Result>,
          <M as DimMul<N>>::Result: Size<T>
{
    fn transpose(self) -> DenseMat<T, N, M, Column> {
        mtra_impl!(self)
    }
}


impl<T: Copy + Scalar, M: Dim, N: Dim> MatrixAdd for DenseMat<T, M, N, Row>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Size<T>,
          T: AddAssign
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        madd_inplace!(@row, +=, self, rhs)
    }
}


impl<T: Copy + Scalar, M: Dim, N: Dim> MatrixAdd for DenseMat<T, M, N, Column>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Size<T>,
          T: AddAssign
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        madd_inplace!(@col, +=, self, rhs)
    }
}


impl<T: Copy + Scalar, M: Dim, N: Dim> MatrixSub for DenseMat<T, M, N, Row>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Size<T>,
          T: SubAssign
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        madd_inplace!(@row, -=, self, rhs)
    }
}


impl<T: Copy + Scalar, M: Dim, N: Dim> MatrixSub for DenseMat<T, M, N, Column>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Size<T>,
          T: SubAssign
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        madd_inplace!(@col, -=, self, rhs)
    }
}


impl<T: Copy + Scalar, M: Dim, N: Dim, P: Dim, L1: Layout, L2: Layout, L3: Layout> MatrixMul<DenseMat<T, N, P, L2>, DenseMat<T, M, P, L3>> for DenseMat<T, M, N, L1>
    where M: DimMul<N> + DimMul<P>,
          N: DimMul<P>,
          <M as DimMul<N>>::Result: Size<T>,
          <M as DimMul<P>>::Result: Size<T>,
          <N as DimMul<P>>::Result: Size<T>,
          DenseMat<T, M, N, L1>: Matrix<Rows = M, Cols = N, Scalar = T>,
          DenseMat<T, N, P, L2>: Matrix<Rows = N, Cols = P, Scalar = T>,
          DenseMat<T, M, P, L3>: Matrix<Rows = M, Cols = P, Scalar = T>,
          T: Zero + Mul<Output = T> + AddAssign
{
    fn mul(self, rhs: DenseMat<T, N, P, L2>) -> DenseMat<T, M, P, L3> {
        mmul_loop_naive!(self, rhs, DenseMat::from_elem(self.rows, rhs.cols, T::zero()), self.rows.reify(), self.cols.reify(), rhs.rows.reify(), rhs.cols.reify())
    }
}


impl<T: Copy + Scalar, N: Nat, L: Layout> MatrixIdentity for DenseMat<T, N, N, L>
    where DenseMat<T, N, N, L>: Matrix<Rows = N, Cols = N, Scalar = T>,
          N: NatMul<N>,
          <N as NatMul<N>>::Result: Size<T>,
          T: Zero + One
{
    fn eye(n: N) -> Self {
        let mut res = DenseMat {
            rows: n,
            cols: n,
            elems: Data::from_elem(<N as NatMul<N>>::Result::as_data(), &T::zero()),
            phantom: PhantomData,
        };

        for i in 0..N::as_usize() {
            res[[i, i]] = T::one();
        }

        res
    }
}


impl<T: Scalar, M: Dim, N: Dim, L: Layout> Index<[usize; 2]> for DenseMat<T, M, N, L>
    where DenseMat<T, M, N, L>: Matrix<Scalar = T>,
          M: DimMul<N>,
          <M as DimMul<N>>::Result: Size<T>
{
    type Output = T;

    #[inline]
    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        self.get(idx[0], idx[1])
    }
}


impl<T: Scalar, M: Dim, N: Dim, L: Layout> IndexMut<[usize; 2]> for DenseMat<T, M, N, L>
    where DenseMat<T, M, N, L>: Matrix<Scalar = T>,
          M: DimMul<N>,
          <M as DimMul<N>>::Result: Size<T>
{
    #[inline]
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Self::Output {
        self.get_mut(idx[0], idx[1])
    }
}
