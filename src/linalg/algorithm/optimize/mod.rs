use linalg::{Matrix, Scalar, Vector};


pub mod simplex;


pub struct LinearProgram<T: Scalar,
                         C: Vector<Scalar = T>,
                         A: Matrix<Scalar = T, Rows = B::Dims, Cols = C::Dims>,
                         B: Vector<Scalar = T>>
{
    pub objective: Objective<T, C>,
    pub constraint: Constraint<T, A, B>,
}


pub struct Objective<T: Scalar, C: Vector<Scalar = T>> {
    pub c: C,
}


pub struct Constraint<T: Scalar, A: Matrix<Scalar = T>, B: Vector<Scalar = T, Dims = A::Rows>> {
    pub a: A,
    pub b: B,
}
