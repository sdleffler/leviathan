use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg, Index,
               IndexMut};

use num::traits::{One, Zero};
use typehack::dim::*;


pub trait Scalar: Clone + Sized + Zero + One +
                  Add<Output = Self> +
                  Sub<Output = Self> +
                  Mul<Output = Self> +
                  Div<Output = Self> +
                  AddAssign + SubAssign +
                  MulAssign + DivAssign +
                  Neg<Output = Self> +
                  PartialOrd + PartialEq + ::std::fmt::Debug +
                  Product + Sum {
    fn abs(&self) -> Self;


    fn from_usize(usize) -> Self;


    fn eq_zero(&self) -> bool {
        self == &Self::zero()
    }

    fn eq_one(&self) -> bool {
        self == &Self::one()
    }


    fn lt_zero(&self) -> bool {
        self < &Self::zero()
    }

    fn gt_zero(&self) -> bool {
        self > &Self::zero()
    }

    fn lte_zero(&self) -> bool {
        self <= &Self::zero()
    }

    fn gte_zero(&self) -> bool {
        self >= &Self::zero()
    }


    fn lt_one(&self) -> bool {
        self < &Self::one()
    }

    fn gt_one(&self) -> bool {
        self > &Self::one()
    }

    fn lte_one(&self) -> bool {
        self <= &Self::one()
    }

    fn gte_one(&self) -> bool {
        self >= &Self::one()
    }
}


macro_rules! impl_scalar {
    ($($t:ident),*) => {
        $(
            impl Scalar for $t {
                fn abs(&self) -> Self {
                    $t::abs(*self)
                }

                fn from_usize(i: usize) -> Self {
                    i as $t
                }
            }
        )*
    }
}


impl_scalar!(f32, f64, i8, i16, i32, i64);


pub trait Vector {
    type Dims: Dim;

    type Scalar: Scalar;

    fn size(&self) -> Self::Dims;
}


pub trait VectorNeg: Vector {
    type Output: Vector<Dims = Self::Dims>;

    fn neg(self) -> Self::Output;
}


pub trait Dot<RHS: Vector = Self>: Vector<Dims = RHS::Dims> {
    fn dot(self, RHS) -> Self::Scalar;
}


pub trait VectorNorm: Vector {
    fn norm(&self) -> Self::Scalar;

    fn squared_norm(&self) -> Self::Scalar {
        self.norm() * self.norm()
    }
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


pub trait Matrix
    : Index<[usize; 2], Output = <Self as Matrix>::Scalar> + IndexMut<[usize; 2]>
    {
    type Rows: Dim;
    type Cols: Dim;

    type Scalar: Scalar;

    fn rows(&self) -> Self::Rows;
    fn cols(&self) -> Self::Cols;

    fn get(&self, row: usize, col: usize) -> &Self::Scalar;
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Scalar;

    fn swap(&mut self, a: [usize; 2], b: [usize; 2]);

    fn row_switch_mut(&mut self, i: usize, j: usize);
    fn row_mul_mut(&mut self, i: usize, c: &Self::Scalar) where Self::Scalar: Clone;
    fn row_add_mut(&mut self, i: usize, j: usize, c: &Self::Scalar) where Self::Scalar: Clone;

    fn col_switch_mut(&mut self, i: usize, j: usize);
    fn col_mul_mut(&mut self, i: usize, c: &Self::Scalar) where Self::Scalar: Clone;
    fn col_add_mut(&mut self, i: usize, j: usize, c: &Self::Scalar) where Self::Scalar: Clone;
}


#[cfg_attr(rustfmt, rustfmt_skip)]
pub trait Square: Matrix<Rows = <Self as Square>::Side, Cols = <Self as Square>::Side> {
    type Side: Dim;

    fn side(&self) -> Self::Side;
}


pub trait MatrixTranspose<T: Matrix>: Matrix<Rows = T::Cols, Cols = T::Rows> {
    fn transpose(self) -> T;
}


pub trait MatrixNeg: Matrix {
    type Output: Matrix<Rows = Self::Rows, Cols = Self::Cols>;

    fn neg(Self) -> <Self as MatrixNeg>::Output;
}


pub trait MatrixAdd<RHS: Matrix<Rows = Self::Rows, Cols = Self::Cols> = Self>
    : Matrix {
    type Output: Matrix<Rows = Self::Rows, Cols = RHS::Cols>;

    fn add(self, RHS) -> <Self as MatrixAdd<RHS>>::Output;
}


pub trait MatrixSub<RHS: Matrix<Rows = Self::Rows, Cols = Self::Cols> = Self>
    : Matrix {
    type Output: Matrix<Rows = Self::Rows, Cols = RHS::Cols>;

    fn sub(self, RHS) -> <Self as MatrixSub<RHS>>::Output;
}


pub trait MatrixMul<RHS: Matrix<Rows = <Self as Matrix>::Cols>,
                    Output: Matrix<Rows = <Self as Matrix>::Rows, Cols = <RHS as Matrix>::Cols>>
    : Matrix {
    fn mul(self, RHS) -> Output;
}


pub trait MatrixIdentity: Square {
    fn eye(Self::Side) -> Self;
}
