use typehack::peano::*;
use typehack::dim::*;


pub trait Matrix {
    type Rows: Dim;
    type Cols: Dim;

    type Scalar: Copy;
}


pub trait Square: Matrix<Rows = <Self as Matrix>::Cols> {}


pub trait MatrixMulCompat<RHS: Matrix, C: Dim, R: Dim>: Matrix {}

impl<LHS, RHS, S: Dim> MatrixMulCompat<RHS, S, S> for LHS
    where LHS: Matrix<Cols = S>,
          RHS: Matrix<Rows = S>
{
}
impl<LHS, RHS, S: Nat> MatrixMulCompat<RHS, S, Dyn> for LHS
    where LHS: Matrix<Cols = S>,
          RHS: Matrix<Rows = Dyn>
{
}
impl<LHS, RHS, S: Nat> MatrixMulCompat<RHS, Dyn, S> for LHS
    where LHS: Matrix<Cols = Dyn>,
          RHS: Matrix<Rows = S>
{
}


pub trait MatrixTranspose: Matrix {
    type Output: Matrix<Rows = Self::Cols, Cols = Self::Rows>;

    fn transpose(self) -> Self::Output;
}


pub trait MatrixAdd<RHS: Matrix<Rows = Self::Rows, Cols = Self::Cols> = Self>
    : Matrix {
    type Output: Matrix<Rows = Self::Rows, Cols = RHS::Cols>;

    fn add(self, RHS) -> Self::Output;
}


pub trait MatrixSub<RHS: Matrix<Rows = Self::Rows, Cols = Self::Cols> = Self>
    : Matrix {
    type Output: Matrix<Rows = Self::Rows, Cols = RHS::Cols>;

    fn sub(self, RHS) -> Self::Output;
}


pub trait MatrixMul<RHS>
    where Self: Matrix + MatrixMulCompat<RHS, <Self as Matrix>::Cols, RHS::Rows>,
          RHS: Matrix
{
    type Output: Matrix<Rows = Self::Rows, Cols = RHS::Cols>;

    fn mul(self, RHS) -> Self::Output;
}


pub trait MatrixIdentity: Square {
    fn eye() -> Self;
}
