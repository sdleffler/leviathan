use std::ops::{Index, IndexMut};

use array::storage::Link;
use matrix::dense::DenseMat;
use matrix::traits::*;
use typehack::peano::*;
use typehack::dim::*;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DynamicMat<T: Copy, M: Dim, N: Dim> {
    rows: M,
    cols: N,
    elems: Vec<T>,
}


impl<T: Copy, M: Dim, N: Dim> Matrix for DynamicMat<T, M, N> {
    type Rows = M;
    type Cols = N;

    type Scalar = T;
}


impl<T: Copy, M: Dim + Nat, N: Dim, P: Dim + Nat> MatrixMul<DynamicMat<T, N, P>>
    for DynamicMat<T, M, N>
    where M: NatMul<P>,
          <M as NatMul<P>>::Result: Link<T>
{
    type Output = DenseMat<T, M, P>;

    fn mul(self, _rhs: DynamicMat<T, N, P>) -> Self::Output {
        unimplemented!();
    }
}


impl<T: Copy, N: Dim, P: Dim + Nat> MatrixMul<DynamicMat<T, N, P>> for DynamicMat<T, Dyn, N> {
    type Output = DynamicMat<T, Dyn, P>;

    fn mul(self, _rhs: DynamicMat<T, N, P>) -> Self::Output {
        unimplemented!();
    }
}


impl<T: Copy, M: Dim + Nat, N: Dim> MatrixMul<DynamicMat<T, N, Dyn>> for DynamicMat<T, M, N> {
    type Output = DynamicMat<T, M, Dyn>;

    fn mul(self, _rhs: DynamicMat<T, N, Dyn>) -> Self::Output {
        unimplemented!();
    }
}


impl<T: Copy, N: Dim> MatrixMul<DynamicMat<T, N, Dyn>> for DynamicMat<T, Dyn, N> {
    type Output = DynamicMat<T, Dyn, Dyn>;

    fn mul(self, _rhs: DynamicMat<T, N, Dyn>) -> Self::Output {
        unimplemented!();
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
