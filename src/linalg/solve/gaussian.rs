use linalg::{Mat, Matrix, Scalar, Square, Vect, Vector};
use typehack::data::*;
use typehack::dim::*;


pub trait GaussianEliminationExt<V, W>: Square
    where V: Vector<Scalar = Self::Scalar, Dims = Self::Side>,
          W: Vector<Scalar = Self::Scalar, Dims = Self::Side>
{
    fn ge_solve(self, V) -> W;
}


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<T: ::std::fmt::Debug + Clone + Scalar, N: Size<T>, M: Size<T>> GaussianEliminationExt<Vect<T, N>, Vect<T, N>>
    for Mat<T, N, N>
    where N: DimMul<N, Result = M>
{
    fn ge_solve(mut self, mut b: Vect<T, N>) -> Vect<T, N> {
        // Begin partial pivoting on the augmented matrix.
        // For every row n, find the row i >= n with the maximum absolute value in column n; then,
        // swap row i with row n.

        let n = self.side().reify();

        for i in 0..n {
            let mut max = (i, self[[i, i]].abs());
            for j in i+1..n {
                let abs = self[[j, i]].abs();
                if abs > max.1 {
                    max = (j, abs);
                }
            }
            self.row_switch_mut(i, max.0);
            b.as_mut_slice().swap(i, max.0);
        }

        // With partial pivoting out of the way, we can begin the process of Gaussian elimination.
        // We now proceed to put the matrix into row-echelon form.

        for i in 0..n {
            // For each row n in the matrix, we perform a row multiply-then-add operation to zero
            // out the values in column n of every row below row n.
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
    use super::*;
    use linalg::{VectorNorm, Mat3x3, Vect3};

    #[test]
    fn ge_solve_3x3() {
        let a = Mat3x3::from(&[2., -3., -2., 1., -1., 1., -1., 2., 2.][..]);
        let b = Vect3::from(&[8., -11., -3.][..]);

        let c = a.ge_solve(b);


        assert!((c - Vect3::from(&[2., 3., -1.][..])).norm() < 0.000001);
    }
}
