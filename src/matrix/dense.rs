use std::mem;
use std::ops::{AddAssign, SubAssign, Mul, Index, IndexMut};
use std::ptr;

use array::storage::*;
use matrix::traits::*;
use num::traits::*;
use typehack::peano::*;


#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct DenseMat<T: Copy, M: Nat, N: Nat>(Storage<T, <M as NatMul<N>>::Result>)
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>;


impl<T: Copy, M: Nat, N: Nat> Copy for DenseMat<T, M, N>
    where DenseMat<T, M, N>: Clone,
          M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>,
          Storage<T, <M as NatMul<N>>::Result>: Copy
{
}


impl<'a, T: Copy, M: Nat, N: Nat> From<&'a [T]> for DenseMat<T, M, N>
    where T: Copy,
          M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    fn from(slice: &[T]) -> Self {
        DenseMat(Storage::from_slice(slice))
    }
}


impl<T: Copy, M: Nat, N: Nat> Matrix for DenseMat<T, M, N>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    type Rows = M;
    type Cols = N;

    type Scalar = T;
}


impl<T: Copy, N: Nat> Square for DenseMat<T, N, N>
    where N: NatMul<N>,
          <N as NatMul<N>>::Result: Link<T>
{
}


impl<T: Copy, M: Nat, N: Nat> MatrixTranspose for DenseMat<T, M, N>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>,
          N: NatMul<M, Result = <M as NatMul<N>>::Result>
{
    type Output = DenseMat<T, N, M>;

    fn transpose(self) -> DenseMat<T, N, M> {
        let mut res: Self::Output; // Transposing in-place is more work than it's worth.

        unsafe {
            res = mem::uninitialized(); // Give ourselves some scratch space.

            for i in 0..M::as_usize() {
                for j in 0..N::as_usize() {
                    ptr::write(&mut res.0[j + i * M::as_usize()],
                               self.0[i + j * N::as_usize()]);
                }
            }

            // `res` should now be fully initialized.
        }

        res
    }
}


impl<T: Copy, M: Nat, N: Nat> MatrixAdd for DenseMat<T, M, N>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>,
          T: AddAssign
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        for i in 0..N::as_usize() * M::as_usize() {
            self.0[i] += rhs.0[i];
        }

        self
    }
}


impl<T: Copy, M: Nat, N: Nat> MatrixSub for DenseMat<T, M, N>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>,
          T: SubAssign
{
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self {
        for i in 0..N::as_usize() * M::as_usize() {
            self.0[i] -= rhs.0[i];
        }

        self
    }
}


impl<T: Copy, M: Nat, N: Nat, P: Nat> MatrixMul<DenseMat<T, N, P>> for DenseMat<T, M, N>
    where M: NatMul<N> + NatMul<P>,
          N: NatMul<P>,
          <M as NatMul<N>>::Result: Link<T>,
          <N as NatMul<P>>::Result: Link<T>,
          <M as NatMul<P>>::Result: Link<T>,
          T: AddAssign + Mul<Output = T> + Zero
{
    type Output = DenseMat<T, M, P>;

    fn mul(self, rhs: DenseMat<T, N, P>) -> DenseMat<T, M, P> {
        let mut output: DenseMat<T, M, P>;

        unsafe {
            output = mem::uninitialized();

            for i in 0..M::as_usize() {
                for j in 0..P::as_usize() {
                    let mut sum = T::zero();

                    for k in 0..N::as_usize() {
                        sum += self[[k, i]] * rhs[[j, k]];
                    }

                    ptr::write(&mut output[[i, j]], sum); // Avoid dropping uninitialized memory.
                }
            }

            // `output` should now be fully initialized.
        }

        output
    }
}


impl<T: Copy, N: Nat> MatrixIdentity for DenseMat<T, N, N>
    where N: NatMul<N>,
          <N as NatMul<N>>::Result: Link<T>,
          T: Zero + One
{
    fn eye() -> Self {
        let mut res = DenseMat(Storage::from_elem(&T::zero()));

        for i in 0..N::as_usize() {
            res[[i, i]] = T::one();
        }

        res
    }
}


impl<T: Copy, M: Nat, N: Nat> Index<[usize; 2]> for DenseMat<T, M, N>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    type Output = T;

    #[inline]
    fn index(&self, idx: [usize; 2]) -> &T {
        assert!(idx[0] < N::as_usize());

        &self.0[idx[0] + idx[1] * N::as_usize()]
    }
}


impl<T: Copy, M: Nat, N: Nat> IndexMut<[usize; 2]> for DenseMat<T, M, N>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>
{
    #[inline]
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut T {
        assert!(idx[0] < N::as_usize());

        &mut self.0[idx[0] + idx[1] * N::as_usize()]
    }
}
