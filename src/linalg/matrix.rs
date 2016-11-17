use std::fmt::{self, Debug};
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
                        out[[i, j]] += $lhs[[i, k]].clone() * $rhs[[k, j]].clone();
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

            unsafe {
                for i in 0..m {
                    for j in 0..n {
                        lhs[[i, j]] $op ptr::read(&$rhs[[i, j]]);
                    }
                }

                $rhs.elems.forget();
            }

            lhs
        }
    );
    (@row, $op:tt, $lhs:expr, $rhs:expr, $m:expr, $n:expr) => (
        {
            let m = $m;
            let n = $n;

            let mut lhs = $lhs;

            unsafe {
                for j in 0..n {
                    for i in 0..m {
                        lhs[[i, j]] $op ptr::read(&$rhs[[i, j]]);
                    }
                }

                $rhs.elems.forget();
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

            unsafe {
                for i in 0..m {
                    for j in 0..n {
                        out[[i, j]] = ptr::read(&$lhs[[i, j]]) $op ptr::read(&$rhs[[i, j]]);
                    }
                }

                $lhs.elems.forget()
                $rhs.elems.forget()
            }

            out
        }
    }
}


impl<T: Scalar, M: Dim, N: Dim, L: Layout> Debug for DenseMat<T, M, N, L> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "DenseMat {{ rows: {}, cols: {}, elems: {:?} }}",
               self.rows.reify(),
               self.cols.reify(),
               self.elems)
    }
}


#[derive(Clone, PartialEq, Eq)]
#[repr(C)]
pub struct DenseMat<T: Scalar, M: Dim, N: Dim, L: Layout> {
    rows: M,
    cols: N,
    elems: Data<T, <M as DimMul<N>>::Result>,
    phantom: PhantomData<L>,
}


impl<T: Copy + Scalar, M: Dim, N: Dim, L: Layout> Copy for DenseMat<T, M, N, L>
    where DenseMat<T, M, N, L>: Clone,
          M: DimMul<N>,
          Data<T, <M as DimMul<N>>::Result>: Copy
{
}


impl<'a, T: Copy + Scalar, M: Nat, N: Nat, L: Layout> From<&'a [T]> for DenseMat<T, M, N, L> {
    fn from(slice: &[T]) -> Self {
        DenseMat {
            rows: M::as_data(),
            cols: N::as_data(),
            elems: Data::from_slice(<M as NatMul<N>>::Result::as_data(), slice),
            phantom: PhantomData,
        }
    }
}


impl<T: Copy + Scalar, M: Nat, N: Nat, L: Layout> From<T> for DenseMat<T, M, N, L> {
    fn from(elem: T) -> Self {
        DenseMat {
            rows: M::as_data(),
            cols: N::as_data(),
            elems: Data::from_elem(<M as NatMul<N>>::Result::as_data(), &elem),
            phantom: PhantomData,
        }
    }
}


impl<T: Scalar, M: Dim, N: Dim, L: Layout> DenseMat<T, M, N, L> {
    pub fn from_elem(rows: M, cols: N, elem: T) -> DenseMat<T, M, N, L>
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


    pub fn augment_cols<X: Layout, Y: Layout, P: Dim>
        (self,
         rhs: DenseMat<T, M, P, X>)
         -> DenseMat<T, M, <P as DimAdd<N>>::Result, Y> {
        let mut out: DenseMat<T, M, <P as DimAdd<N>>::Result, Y>;

        unsafe {
            out = DenseMat::uninitialized(self.rows(), rhs.cols().add(self.cols()));

            for i in 0..self.rows().reify() {
                for j in 0..self.cols().reify() {
                    ptr::write(&mut out[[i, j]], ptr::read(&self[[i, j]]));
                }

                for j in 0..rhs.cols().reify() {
                    ptr::write(&mut out[[i, j + self.cols().reify()]],
                               ptr::read(&rhs[[i, j]]));
                }
            }

            self.elems.forget();
            rhs.elems.forget();
        }

        out
    }


    pub fn augment_rows<X: Layout, Y: Layout, P: DimAdd<M>>
        (self,
         rhs: DenseMat<T, P, N, X>)
         -> DenseMat<T, <P as DimAdd<M>>::Result, N, Y> {
        let mut out: DenseMat<T, <P as DimAdd<M>>::Result, N, Y>;

        unsafe {
            out = DenseMat::uninitialized(rhs.rows().add(self.rows()), self.cols());

            for j in 0..self.cols().reify() {
                for i in 0..self.rows().reify() {
                    ptr::write(&mut out[[i, j]], ptr::read(&self[[i, j]]));
                }

                for i in 0..rhs.rows().reify() {
                    ptr::write(&mut out[[i + self.rows().reify(), j]],
                               ptr::read(&rhs[[i, j]]));
                }
            }

            self.elems.forget();
            rhs.elems.forget();
        }

        out
    }
}


impl<T: Scalar, P: Dim, Q: Dim, L: Layout> DenseMat<T, P, Q, L> {
    pub fn from_rows<R: Iterator<Item = DenseVec<T, Q>> + ExactSizeIterator>(iter: R) -> Self {
        assert!(P::compatible(iter.len()));

        let mut out;

        let mut iter = iter.peekable();
        let q = iter.peek().unwrap().size();

        unsafe {
            out = Self::uninitialized(P::from_usize(iter.len()), q);

            for (i, row) in iter.enumerate() {
                assert_eq!(row.size(), q);

                for (j, elem) in row.into_iter().enumerate() {
                    out[[i, j]] = elem;
                }
            }
        }

        out
    }
}


impl<T: Scalar, P: Dim, Q: Dim, L: Layout> DenseMat<T, P, Q, L> {
    pub fn from_cols<R: IntoIterator<Item = DenseVec<T, P>>>(iter: R) -> Self
        where R::IntoIter: ExactSizeIterator
    {
        let iter = iter.into_iter();

        assert!(Q::compatible(iter.len()));

        let mut out;

        let mut iter = iter.peekable();
        let p = iter.peek().unwrap().size();

        unsafe {
            out = Self::uninitialized(p, Q::from_usize(iter.len()));

            for (i, col) in iter.enumerate() {
                assert_eq!(col.size(), p);

                for (j, elem) in col.into_iter().enumerate() {
                    out[[j, i]] = elem;
                }
            }
        }

        out
    }
}


impl<T: Scalar, P: Dim, Q: Dim, L: Layout> From<DenseMat<T, P, Q, L>> for DenseVec<T, <P as DimMul<Q>>::Result> {
    fn from(mat: DenseMat<T, P, Q, L>) -> DenseVec<T, <P as DimMul<Q>>::Result> {
        DenseVec::from_data(mat.elems)
    }
}


impl<T: Scalar, M: Dim, N: Dim, L: Layout> Matrix for DenseMat<T, M, N, L> {
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


impl<T: Scalar, M: Dim, N: Dim> Matrix for DenseMat<T, M, N, Row> {
    fn get(&self, row: usize, col: usize) -> &Self::Scalar {
        assert!(row < self.rows.reify() && col < self.cols.reify(),
                "Matrix index out of bounds: [[{}, {}]] is out of bounds of [[{}, {}]]!",
                row,
                col,
                self.rows.reify(),
                self.cols.reify());

        &self.elems[row * self.cols.reify() + col]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Scalar {
        assert!(row < self.rows.reify() && col < self.cols.reify(),
                "Matrix index out of bounds: [[{}, {}]] is out of bounds of [[{}, {}]]!",
                row,
                col,
                self.rows.reify(),
                self.cols.reify());

        &mut self.elems[row * self.cols.reify() + col]
    }

    fn swap(&mut self, a: [usize; 2], b: [usize; 2]) {
        self.elems.swap(a[0] * self.cols.reify() + a[1],
                        b[0] * self.cols.reify() + b[1]);
    }
}


impl<T: Scalar, M: Dim, N: Dim> Matrix for DenseMat<T, M, N, Column> {
    fn get(&self, row: usize, col: usize) -> &Self::Scalar {
        assert!(row < self.rows.reify() && col < self.cols.reify(),
                "Matrix index out of bounds: [[{}, {}]] is out of bounds of [[{}, {}]]!",
                row,
                col,
                self.rows.reify(),
                self.cols.reify());

        &self.elems[row + col * self.rows.reify()]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Scalar {
        assert!(row < self.rows.reify() && col < self.cols.reify(),
                "Matrix index out of bounds: [[{}, {}]] is out of bounds of [[{}, {}]]!",
                row,
                col,
                self.rows.reify(),
                self.cols.reify());

        &mut self.elems[row + col * self.rows.reify()]
    }

    fn swap(&mut self, a: [usize; 2], b: [usize; 2]) {
        self.elems.swap(a[0] + a[1] * self.rows.reify(),
                        b[0] + b[1] * self.rows.reify());
    }
}


impl<T: Scalar, N: Dim, L: Layout> Square for DenseMat<T, N, N, L> {
    type Side = N;

    fn side(&self) -> N {
        assert_eq!(self.rows(), self.cols());
        self.rows()
    }
}


impl<T: Scalar, M: Dim, N: Dim> MatrixTranspose<DenseMat<T, N, M, Row>>
    for DenseMat<T, M, N, Column>
    where N: DimMul<M, Result = <M as DimMul<N>>::Result>
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


impl<T: Scalar, M: Dim, N: Dim> MatrixTranspose<DenseMat<T, N, M, Column>>
    for DenseMat<T, M, N, Row>
    where N: DimMul<M, Result = <M as DimMul<N>>::Result>
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
                    ptr::write(&mut out[[j, i]], ptr::read(&$mat[[i, j]]));
                }
            }

            $mat.elems.forget();

            out
        }
    }
}


impl<T: Scalar, M: Dim, N: Dim> MatrixTranspose<DenseMat<T, N, M, Row>> for DenseMat<T, M, N, Row>
    where N: DimMul<M, Result = <M as DimMul<N>>::Result>
{
    fn transpose(self) -> DenseMat<T, N, M, Row> {
        mtra_impl!(self)
    }
}


impl<T: Scalar, M: Dim, N: Dim> MatrixTranspose<DenseMat<T, N, M, Column>>
    for DenseMat<T, M, N, Column>
    where N: DimMul<M, Result = <M as DimMul<N>>::Result>
{
    fn transpose(self) -> DenseMat<T, N, M, Column> {
        mtra_impl!(self)
    }
}


impl<T: Scalar, M: Dim, N: Dim> MatrixAdd for DenseMat<T, M, N, Row> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        madd_inplace!(@row, +=, self, rhs)
    }
}


impl<T: Scalar, M: Dim, N: Dim> MatrixAdd for DenseMat<T, M, N, Column> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        madd_inplace!(@col, +=, self, rhs)
    }
}


impl<T: Scalar, M: Dim, N: Dim> MatrixSub for DenseMat<T, M, N, Row> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        madd_inplace!(@row, -=, self, rhs)
    }
}


impl<T: Copy + Scalar, M: Dim, N: Dim> MatrixSub for DenseMat<T, M, N, Column> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        madd_inplace!(@col, -=, self, rhs)
    }
}


impl<T: Clone + Scalar, M: Dim, N: Dim, P: Dim, L1: Layout, L2: Layout, L3: Layout> MatrixMul<DenseMat<T, N, P, L2>, DenseMat<T, M, P, L3>> for DenseMat<T, M, N, L1>
{
    fn mul(self, rhs: DenseMat<T, N, P, L2>) -> DenseMat<T, M, P, L3> {
        mmul_loop_naive!(self, rhs, DenseMat::from_elem(self.rows, rhs.cols, T::zero()), self.rows.reify(), self.cols.reify(), rhs.rows.reify(), rhs.cols.reify())
    }
}


impl<T: Clone + Scalar, M: Dim, N: Dim, P: Dim, L1: Layout, L2: Layout> Mul<DenseMat<T, N, P, L2>> for DenseMat<T, M, N, L1>
{
    type Output = DenseMat<T, M, P, L1>;

    fn mul(self, rhs: DenseMat<T, N, P, L2>) -> DenseMat<T, M, P, L1> {
        MatrixMul::mul(self, rhs)
    }
}


impl<T: Clone + Scalar, N: Dim, L: Layout> MatrixIdentity for DenseMat<T, N, N, L> {
    fn eye(n: N) -> Self {
        let mut res = DenseMat {
            rows: n,
            cols: n,
            elems: Data::from_elem(n.mul(n), &T::zero()),
            phantom: PhantomData,
        };

        for i in 0..n.reify() {
            res[[i, i]] = T::one();
        }

        res
    }
}


impl<T: Scalar, M: Dim, N: Dim, L: Layout> Index<[usize; 2]> for DenseMat<T, M, N, L> {
    type Output = T;

    #[inline]
    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        self.get(idx[0], idx[1])
    }
}


impl<T: Scalar, M: Dim, N: Dim, L: Layout> IndexMut<[usize; 2]> for DenseMat<T, M, N, L> {
    #[inline]
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Self::Output {
        self.get_mut(idx[0], idx[1])
    }
}


#[cfg(test)]
mod tests {
    use linalg::*;
    use typehack::binary::*;

    #[test]
    fn mat_test_index_column_major() {
        let a = Mat![#column [0, 1], [2, 3], [4, 5]];

        assert_eq!(a[[0, 0]], 0);
        assert_eq!(a[[0, 1]], 1);
        assert_eq!(a[[1, 0]], 2);
        assert_eq!(a[[1, 1]], 3);
        assert_eq!(a[[2, 0]], 4);
        assert_eq!(a[[2, 1]], 5);
    }


    #[test]
    fn mat_test_index_row_major() {
        let a = Mat![#row [0, 1], [2, 3], [4, 5]];

        assert_eq!(a[[0, 0]], 0);
        assert_eq!(a[[0, 1]], 1);
        assert_eq!(a[[1, 0]], 2);
        assert_eq!(a[[1, 1]], 3);
        assert_eq!(a[[2, 0]], 4);
        assert_eq!(a[[2, 1]], 5);
    }
}
