use super::LinearProgram;

use iter_exact::CollectExactExt;

use linalg::{Matrix, MatrixIdentity, Mat, Row, Scalar, Vector, Vect};
use num::traits::Float;
use typehack::data::{Data, Size};
use typehack::dim::{Dim, DimAdd, DimMul};


pub trait SimplexExt<T: Scalar,
                     C: Vector<Scalar = T, Dims = X::Dims>,
                     A: Matrix<Scalar = T, Rows = B::Dims, Cols = X::Dims>,
                     B: Vector<Scalar = T>,
                     X: Vector<Scalar = T>> {
    fn simplex_solve(self) -> X;
}


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<T: Clone + Scalar + Float,
     M: Size<T> + DimMul<M, Result = Q> + DimMul<N, Result = P> + DimAdd<N, Result = W> + DimMul<W, Result = V> + ,
     N: Size<T> + Size<Option<usize>> + DimAdd<M, Result = S>,
     P: Size<T>,
     Q: Size<T>,
     S: DimMul<N, Result = U> + Size<T>,
     U: Size<T>,
     V: Size<T>,
     W: Dim> SimplexExt<T, Vect<T, N>, Mat<T, M, N>, Vect<T, M>, Vect<T, N>>
     for LinearProgram<T, Vect<T, N>, Mat<T, M, N>, Vect<T, M>> {
    fn simplex_solve(self) -> Vect<T, N> {
        let mut basics = Data::from_elem(self.constraint.a.cols(), &None);
        let mut xs: Mat<_, _, _, Row> = {
            let a_rows = self.constraint.a.rows();
            self.constraint.a.augment_cols(Mat::<_, _, _, Row>::eye(a_rows))
        };
        // TODO: Eliminate the fact that `xs`, `ss`, and `p` are not all part of the same matrix.
        let mut p = (-self.objective.c).augment(Vect::from_elem(xs.rows(), &T::zero()));
        let mut rhs = self.constraint.b;
        // We now have our tableau. It looks like:
        // `xs`.... `ss`.... | rhs
        //  `p`........... 1 | ...
        // Where the `...`s indicate that the values from the left-hand (or above) side fill in that area.

        loop {
            // We now pick the pivot column. The pivot column is the index of `p[i]` where `p[i]`
            // has the most negative value in `p`.

            println!("Tableau:\nxs: {:?}\np: {:?}\nrhs: {:?}\nbasics: {:?}", xs, p, rhs, basics);

            let pcol = match p.as_slice().iter()
                              .enumerate()
                              .fold(None, |mi, (j, &ref r)| match mi {
                                  Some(i) if p[i].lt_zero() && r.lt_zero() && r < &p[i] => Some(j),
                                  Some(i) => Some(i),
                                  None if r.lt_zero() => Some(j),
                                  None => None,
                              }) {
                Some(pcol) => pcol,
                None => break,
            };


            let (mut prow, mut prat) = (0, rhs[0].clone() / xs[[0, pcol]].clone());
            for i in 1..xs.rows().reify() {
                // To pick the pivot row, we find the ratio of rhs[i] to xs[[i, pcol]], looking for
                // the smallest nonnegative ratio.

                let rat = rhs[i].clone() / xs[[i, pcol]].clone();
                if rat.gt_zero() && (rat < prat || prat.lte_zero())  {
                    prow = i;
                    prat = rat;
                }
            }

            basics[pcol] = Some(prow);

            if prat.lt_zero() {
                break;
            }

            {
                let pelem = xs[[prow, pcol]].clone();
                let pelem = pelem.recip();
                xs.row_mul_mut(prow, &pelem);
                rhs[prow] *= pelem.clone();
            }

            // We now use row operations to clear the pivot column.
            for i in 0..xs.rows().reify() {
                if i == prow {
                    continue;
                }

                let celem = -xs[[i, pcol]].clone();
                xs.row_add_mut(i, prow, &celem);
                rhs[i] += rhs[prow].clone() * celem;
            }

            {
                let celem = -p[pcol].clone();
                for i in 0..xs.cols().reify() {
                    p[i] += xs[[prow, i]].clone() * celem.clone();
                }
            }
        }

        basics.into_iter().map(|opt_i| opt_i.map_or(T::zero(), |i| rhs[i].clone())).collect_exact()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    use linalg::{Dot, VectorNorm};

    #[test]
    fn simplex_solve_3x2() {
        let a = Mat![[1., 2.], [1., 1.], [3., 2.]];
        let b = Vect![16., 9., 24.];
        let c = Vect![40., 30.];

        let constraint = Constraint { a: a, b: b };

        let objective = Objective { c: c };

        let program = LinearProgram {
            constraint: constraint,
            objective: objective,
        };

        let x = program.simplex_solve();

        println!("x: {:?}", x);

        assert!((x - Vect![6., 3.]).norm() < 0.00001);
    }


    #[test]
    fn simplex_solve_2x3() {
        let a = Mat![[3., 2., 1.], [2., 5., 3.]];
        let b = Vect![10., 15.];
        let c = Vect![2., 3., 4.];

        let constraint = Constraint { a: a, b: b };

        let objective = Objective { c: c.clone() };

        let program = LinearProgram {
            constraint: constraint,
            objective: objective,
        };

        let x = program.simplex_solve();

        println!("x: {:?}", x);
        println!("x `dot` c: {:?}", x.clone().dot(c.clone()));

        assert!((x.clone() - Vect![0., 0., 5.]).norm() < 0.00001);
        assert!(x.dot(c) - (20.) < 0.00001);
    }


    #[test]
    fn simplex_solve_5x2() {
        let a = Mat![[-4., -3.], [2., 3.], [-3., 2.], [0., 2.], [2., 1.]];
        let b = Vect![0., 6., 3., 5., 4.];
        let c = Vect![4., 3.];

        let constraint = Constraint { a: a, b: b };

        let objective = Objective { c: c };

        let program = LinearProgram {
            constraint: constraint,
            objective: objective,
        };

        let x = program.simplex_solve();

        println!("x: {:?}", x);

        assert!((x - Vect![1.5, 1.]).norm() < 0.00001);
    }
}
