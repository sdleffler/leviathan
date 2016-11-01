use std::ops::{Index, IndexMut};

use typehack::peano::*;
use typehack::dim::*;


pub trait Vector {
    type Dims: Dim;

    type Scalar: Copy;
}


impl<'a, V: Vector> Vector for &'a V {
    type Dims = V::Dims;

    type Scalar = V::Scalar;
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


pub trait FieldX: Vector {
    fn x(&self) -> &Self::Scalar;
    fn x_mut(&mut self) -> &mut Self::Scalar;
}


impl<V: Vector> FieldX for V
    where V::Dims: NatSub<S<Z>>,
          V: Index<usize, Output = <V as Vector>::Scalar> + IndexMut<usize>
{
    fn x(&self) -> &Self::Scalar {
        &self[0]
    }

    fn x_mut(&mut self) -> &mut Self::Scalar {
        &mut self[0]
    }
}


pub trait FieldY: Vector {
    fn y(&self) -> &Self::Scalar;
    fn y_mut(&mut self) -> &mut Self::Scalar;
}


impl<V: Vector> FieldY for V
    where V::Dims: NatSub<S<S<Z>>>,
          V: Index<usize, Output = <V as Vector>::Scalar> + IndexMut<usize>
{
    fn y(&self) -> &Self::Scalar {
        &self[1]
    }

    fn y_mut(&mut self) -> &mut Self::Scalar {
        &mut self[1]
    }
}


pub trait FieldZ: Vector {
    fn z(&self) -> &Self::Scalar;
    fn z_mut(&mut self) -> &mut Self::Scalar;
}


impl<V: Vector> FieldZ for V
    where V::Dims: NatSub<S<S<S<Z>>>>,
          V: Index<usize, Output = <V as Vector>::Scalar> + IndexMut<usize>
{
    fn z(&self) -> &Self::Scalar {
        &self[2]
    }

    fn z_mut(&mut self) -> &mut Self::Scalar {
        &mut self[2]
    }
}


pub trait FieldW: Vector {
    fn w(&self) -> &Self::Scalar;
    fn w_mut(&mut self) -> &mut Self::Scalar;
}


impl<V: Vector> FieldW for V
    where V::Dims: NatSub<S<S<S<S<Z>>>>>,
          V: Index<usize, Output = <V as Vector>::Scalar> + IndexMut<usize>
{
    fn w(&self) -> &Self::Scalar {
        &self[3]
    }

    fn w_mut(&mut self) -> &mut Self::Scalar {
        &mut self[3]
    }
}
