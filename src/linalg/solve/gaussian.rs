use std::mem;

use linalg::{Layout, Mat, Matrix, Scalar, Square, Vect, Vector};
use num::traits::Float;
use typehack::data::*;
use typehack::dim::*;


pub trait GaussianNullspaceExt<X>: Matrix
    where X: Vector<Scalar = Self::Scalar, Dims = Self::Cols>
{
    fn ge_null_elem(self) -> X;
}


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<T: Clone + Scalar, M: Size<T> + DimMul<N, Result = P>, N: Size<T>, P: Size<T>, L: Layout> GaussianNullspaceExt<Vect<T, N>> for Mat<T, M, N, L> {
    fn ge_null_elem(mut self) -> Vect<T, N> {
        debug!("Calculating arbitrary nullspace element of matrix {:?}...", self);

        let rows = self.rows().reify();
        let cols = self.cols().reify();

        debug!("Rows, columns: {}, {}", rows, cols);

        for i in 0..rows {
            let mut max = (i, self[[i, i]].abs());
            for j in i+1..rows {
                let abs = self[[j, i]].abs();
                if abs > max.1 {
                    max = (j, abs);
                }
            }
            self.row_switch_mut(i, max.0);

            debug!("Partial pivoting row {} and {}. Matrix: {:?}", i, max.0, self);

            let mut col = 0;
            while col < cols && self[[i, col]].eq_zero() {
                col += 1;
            }

            if col >= cols {
                break;
            }

            {
                let k = mem::replace(&mut self[[i, col]], T::one());

                for j in col+1..cols {
                    self[[i, j]] /= k.clone();
                }
            }

            for j in 0..i {
                debug!("Canceling row {}'s column {}", j, col);

                let k = -self[[j, col]].clone();
                self.row_add_mut(j, i, &k);

                debug!("After row cancellation: {:?}", self);
            }

            for j in i+1..rows {
                debug!("Canceling row {}'s column {}", j, i);

                let k = mem::replace(&mut self[[j, col]], T::zero());
                for c in col+1..cols {
                    self[[j, c]] -= k.clone() * self[[i, c]].clone();
                }

                debug!("After row cancellation: {:?}", self);
            }
        }

        // Our matrix should now be in reduced row-echelon form. We now proceed to find the free
        // variables.
        // We can proceed by attempting to find the first non-pivot column. If we find none, the
        // nullspace is the zero vector.

        for i in 0..cols {
            // This direct zero-comparison makes me really nervous. Numerical instability could
            // seriously kill this. TODO: Find a reasonable epsilon to compare with instead.
            if (i < rows && self[[i, i]].eq_zero()) || i >= rows {
                let mut x = Vect::from_elem(self.cols(), &T::zero());

                for j in 0..rows {
                    if self[[j, i]].eq_zero() {
                        x[j] = T::one();
                        return x;
                    }

                    x[j] = -self[[j, i]].clone();
                }

                x[rows] = T::one();
                return x;
            }
        }

        unreachable!();
    }
}


pub trait GaussianEliminationExt<V, W>: Square
    where V: Vector<Scalar = Self::Scalar, Dims = Self::Side>,
          W: Vector<Scalar = Self::Scalar, Dims = Self::Side>
{
    fn ge_solve(self, V) -> W;
}


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<T: Clone + Scalar, N: Size<T>, M: Size<T>, L: Layout> GaussianEliminationExt<Vect<T, N>, Vect<T, N>>
    for Mat<T, N, N, L>
    where N: DimMul<N, Result = M>
{
    fn ge_solve(mut self, mut b: Vect<T, N>) -> Vect<T, N> {
        let n = self.side().reify();

        for i in 0..n {
            // For every row n, we first find the row i >= n with the maximum absolute value in
            // column n; then, swap row i with row n. This is called partial pivoting, and it
            // can vastly increase the numerical stability of the result.

            let mut max = (i, self[[i, i]].abs());
            for j in i+1..n {
                let abs = self[[j, i]].abs();
                if abs > max.1 {
                    max = (j, abs);
                }
            }
            self.row_switch_mut(i, max.0);
            b.as_mut_slice().swap(i, max.0);

            // For each row n in the matrix, we perform a row multiply-then-add operation to zero
            // out the values in column n of every row below row n.

            // TODO: Optimize: no need to perform the operation when the element in question should
            // be going to zero (also more numerically accurate.)

            for j in i+1..n {
                let k = -(self[[j, i]].clone() / self[[i, i]].clone());
                self.row_add_mut(j, i, &k);
                b[j] += b[i].clone() * k;
            }
        }

        // And now, we use back-substitution to finish the solving process.
        // TODO: Consume these rows rather than clone their elements; this is the last time we use
        // the elements of the matrix, and we do not touch any element any more than once.

        for i in (0..n).rev() {
            for j in i+1..n {
                b[i] -= self[[i, j]].clone() * b[j].clone();
            }
            b[i] /= self[[i, i]].clone();
        }

        b
    }
}


#[cfg(test)]
mod tests {
    extern crate env_logger;

    use super::*;
    use linalg::{Column, Vect, VectorNorm};

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn ge_nullspace_3x4() {
        let _ = env_logger::init();

        let a = Mat![[ 3., 7./2., 9.,  6.],
                     [ 6.,    2., 2.,  5.],
                     [12.,    4., 4., 10.]];

        let c = a.clone().ge_null_elem();

        debug!("c: {:?}", c);
        debug!("a * c: {:?}", Vect::from(a.clone() * c.clone().as_column::<Column>()));

        assert!(Vect::from(a * c.as_column::<Column>()).norm() < 0.000001);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn ge_nullspace_4x5() {
        let _ = env_logger::init();

        let a = Mat![[    9., 1., 8., 7./3., 5./2.],
                     [    3., 5., 1.,    8.,    9.],
                     [ 5./2., 9., 7.,    2.,    2.],
                     [ 3./2., 1., 8.,    2.,    2.]];

        let c = a.clone().ge_null_elem();

        debug!("c: {:?}", c);
        debug!("a * c: {:?}", Vect::from(a.clone() * c.clone().as_column::<Column>()));

        assert!(Vect::from(a * c.as_column::<Column>()).norm() < 0.000001);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn ge_solve_3x3_row_major() {
        let a = Mat![#row [ 2.,  1., -1.],
                          [-3., -1.,  2.],
                          [-2.,  1.,  2.]];
        let b = Vect![8., -11., -3.];

        let c = a.ge_solve(b);

        assert!((c - Vect![2., 3., -1.]).norm() < 0.000001);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn ge_solve_3x3_column_major() {
        let a = Mat![#column [ 2.,  1., -1.],
                             [-3., -1.,  2.],
                             [-2.,  1.,  2.]];
        let b = Vect![8., -11., -3.];

        let c = a.ge_solve(b);


        assert!((c - Vect![2., 3., -1.]).norm() < 0.000001);
    }
}
