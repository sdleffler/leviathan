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


pub trait Layout {
    type Alternate: Layout;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Column;

impl Layout for Row {
    type Alternate = Column;
}

impl Layout for Column {
    type Alternate = Row;
}


pub trait Matrix {
    type Rows: Dim;
    type Cols: Dim;

    type Scalar: Copy;


    fn get(&self, row: usize, col: usize) -> &Self::Scalar;
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Scalar;
}


pub trait Square: Matrix<Rows = <Self as Matrix>::Cols> {}


pub trait MatrixTranspose<T: Matrix>: Matrix<Rows = T::Cols, Cols = T::Rows> {
    fn transpose(self) -> T;
}


pub trait MatrixNeg: Matrix {
    type Output: Matrix<Rows = Self::Rows, Cols = Self::Cols>;

    fn neg(Self) -> Self::Output;
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


pub trait MatrixMul<RHS: Matrix<Rows = <Self as Matrix>::Cols>,
                    Output: Matrix<Rows = <Self as Matrix>::Rows, Cols = <RHS as Matrix>::Cols>>
    : Matrix {
    fn mul(self, RHS) -> Output;
}


pub trait MatrixIdentity: Square {
    fn eye(Self::Rows) -> Self;
}
