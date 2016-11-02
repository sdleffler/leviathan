use typehack::dim::*;


pub trait Vector {
    type Dims: Dim;

    type Scalar: Copy;
}


impl<'a, V: Vector> Vector for &'a V {
    type Dims = V::Dims;

    type Scalar = V::Scalar;
}


pub trait VectorNeg: Vector {
    type Output: Vector<Dims = Self::Dims>;

    fn neg(self) -> Self::Output;
}


pub trait VectorAdd<RHS: Vector = Self>: Vector<Dims = RHS::Dims> {
    type Output: Vector<Dims = RHS::Dims>;

    fn add(self, RHS) -> Self::Output;
}


pub trait VectorSub<RHS: Vector = Self>: Vector<Dims = RHS::Dims> {
    type Output: Vector<Dims = RHS::Dims>;

    fn sub(self, RHS) -> Self::Output;
}


pub trait VectorHadamard<RHS: Vector = Self>: Vector<Dims = RHS::Dims> {
    type Output: Vector<Dims = RHS::Dims>;

    fn mul(self, RHS) -> Self::Output;
}


pub trait VectorDot<RHS: Vector = Self>: Vector<Dims = RHS::Dims> {
    type Output;

    fn dot(self, RHS) -> Self::Output;
}


pub trait Matrix {
    type Rows: Dim;
    type Cols: Dim;

    type Scalar: Copy;
}


pub trait Square: Matrix<Rows = <Self as Matrix>::Cols> {}


pub trait MatrixTranspose: Matrix {
    type Output: Matrix<Rows = Self::Cols, Cols = Self::Rows>;

    fn transpose(self) -> Self::Output;
}


pub trait MatrixNeg: Matrix {
    type Output: Matrix<Rows = Self::Rows, Cols = Self::Cols>;

    fn neg(Self) -> Self::Output;
}


pub trait MatrixAdd<RHS: Matrix = Self>: Matrix
    where Self::Rows: DimCompat<RHS::Rows>,
          Self::Cols: DimCompat<RHS::Cols>,
          <Self::Output as Matrix>::Rows: DimCompat<Self::Rows> + DimCompat<RHS::Rows>,
          <Self::Output as Matrix>::Cols: DimCompat<Self::Cols> + DimCompat<RHS::Cols>
{
    type Output: Matrix;

    fn add(self, RHS) -> Self::Output;
}


pub trait MatrixSub<RHS: Matrix<Rows = Self::Rows, Cols = Self::Cols> = Self>
    : Matrix {
    type Output: Matrix<Rows = Self::Rows, Cols = RHS::Cols>;

    fn sub(self, RHS) -> Self::Output;
}


pub trait MatrixMul<RHS: Matrix>: Matrix
    where <Self as Matrix>::Cols: DimCompat<RHS::Rows>
{
    type Output: Matrix<Rows = Self::Rows, Cols = RHS::Cols>;

    fn mul(self, RHS) -> Self::Output;
}


pub trait MatrixIdentity: Square {
    fn eye() -> Self;
}
