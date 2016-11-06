use std::marker::PhantomData;
use std::mem;
use std::ops::{AddAssign, SubAssign, Mul, MulAssign, Deref, DerefMut, Index, IndexMut};
use std::ptr;

use num::traits::*;
use typehack::data::*;
use typehack::dim::*;
use typehack::binary::*;
use typehack::void::*;

pub mod traits;

use self::traits::*;


// Notation:
// - `N`, `M`, `P`, or `Q` implies a type implementing `Nat`;
// - `N?`, `M?`, `P?`, or `Q?` implies a type implementing `Dim`, and could thus either be a `Nat` or `Dyn`;
// - `Dyn` implies a dynamic dimensionality.
//
// See linalg.todo.


macro_rules! mmul_loop_naive {
    ($lhs:expr, $rhs:expr, $out:expr, $m:expr, $n:expr, $p:expr, $q:expr) => (
        {
            let mut out = $out;

            let n = $n;
            let p = $p;

            debug_assert_eq!(n, p);

            let n = ::std::cmp::min(n, p);

            for i in 0..$m {
                for j in 0..$q {
                    for k in 0..n {
                        out[[i, j]] += $lhs[[i, k]] * $rhs[[k, j]];
                    }
                }
            }

            out
        }
    );
}


macro_rules! madd_inplace {
    (@$layout:ident, $op:tt, $lhs:expr, $rhs:expr) => (
        {
            let m = $lhs.rows.reify();
            let n = $lhs.cols.reify();
            let p = $rhs.rows.reify();
            let q = $rhs.cols.reify();

            debug_assert_eq!(m, p);
            debug_assert_eq!(n, q);

            madd_inplace!(@$layout, $op, $lhs, $rhs, ::std::cmp::min(m, p), ::std::cmp::min(n, q))
        }
    );
    (@col, $op:tt, $lhs:expr, $rhs:expr, $m:expr, $n:expr) => (
        {
            let m = $m;
            let n = $n;

            let mut lhs = $lhs;

            for i in 0..m {
                for j in 0..n {
                    lhs[[i, j]] $op $rhs[[i, j]];
                }
            }

            lhs
        }
    );
    (@row, $op:tt, $lhs:expr, $rhs:expr, $m:expr, $n:expr) => (
        {
            let m = $m;
            let n = $n;

            let mut lhs = $lhs;

            for j in 0..n {
                for i in 0..m {
                    lhs[[i, j]] $op $rhs[[i, j]];
                }
            }

            lhs
        }
    );
}


macro_rules! madd_allocating {
    ($op:tt, $lhs:expr, $rhs:expr, $out:expr, $m:expr, $n:expr, $p:expr, $q:expr) => {
        {
            let mut out = $out;

            let m = $m;
            let n = $n;
            let p = $p;
            let q = $q;

            debug_assert_eq!(m, p);
            debug_assert_eq!(n, q);

            let m = ::std::cmp::min(m, p);
            let n = ::std::cmp::min(n, q);

            for i in 0..m {
                for j in 0..n {
                    out[[i, j]] = $lhs[[i, j]] $op $rhs[[i, j]];
                }
            }

            out
        }
    }
}


#[derive(PartialEq, Eq, Debug)]
#[repr(C)]
pub struct DenseMat<T: Copy, M: Dim, N: Dim, L: Layout>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Store<T>
{
    rows: M,
    cols: N,
    elems: Data<T, <M as DimMul<N>>::Result>,
    phantom: PhantomData<L>,
}


impl<T: Copy, M: Nat, N: Nat, L: Layout> Clone for DenseMat<T, M, N, L>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Store<T>,
          Data<T, <M as NatMul<N>>::Result>: Clone
{
    fn clone(&self) -> Self {
        DenseMat {
            rows: self.rows,
            cols: self.cols,
            elems: self.elems.clone(),
            phantom: PhantomData,
        }
    }
}


impl<T: Copy, M: Dim, N: Dim, L: Layout> Copy for DenseMat<T, M, N, L>
    where DenseMat<T, M, N, L>: Clone,
          M: DimMul<N>,
          <M as DimMul<N>>::Result: Store<T>,
          Data<T, <M as DimMul<N>>::Result>: Copy
{
}


impl<'a, T: Copy, M: Nat, N: Nat, L: Layout> From<&'a [T]> for DenseMat<T, M, N, L>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Store<T>
{
    fn from(slice: &[T]) -> Self {
        DenseMat {
            rows: M::as_data(),
            cols: N::as_data(),
            elems: Data::from_slice(<M as NatMul<N>>::Result::as_data(), slice),
            phantom: PhantomData,
        }
    }
}


impl<T: Copy, M: Nat, N: Nat, L: Layout> From<T> for DenseMat<T, M, N, L>
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Store<T>
{
    fn from(elem: T) -> Self {
        DenseMat {
            rows: M::as_data(),
            cols: N::as_data(),
            elems: Data::from_elem(<M as NatMul<N>>::Result::as_data(), &elem),
            phantom: PhantomData,
        }
    }
}


impl<T: Copy, M: Dim, N: Dim, L: Layout> DenseMat<T, M, N, L>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Store<T>
{
    fn from_elem(rows: M, cols: N, elem: T) -> DenseMat<T, M, N, L> {
        DenseMat {
            rows: rows,
            cols: cols,
            elems: Data::from_elem(DimMul::mul(rows, cols), &elem),
            phantom: PhantomData,
        }
    }
}


impl<T: Copy, M: Dim, N: Dim> Matrix for DenseMat<T, M, N, Column>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Store<T>
{
    type Rows = M;
    type Cols = N;

    type Scalar = T;


    fn get(&self, row: usize, col: usize) -> &Self::Scalar {
        assert!(row < self.rows.reify() && col < self.cols.reify());

        &self.elems[row * self.rows.reify() + col]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Scalar {
        assert!(row < self.rows.reify() && col < self.cols.reify());

        &mut self.elems[row * self.rows.reify() + col]
    }
}


impl<T: Copy, M: Dim, N: Dim> Matrix for DenseMat<T, M, N, Row>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Store<T>
{
    type Rows = M;
    type Cols = N;

    type Scalar = T;


    fn get(&self, row: usize, col: usize) -> &Self::Scalar {
        assert!(row < self.rows.reify() && col < self.cols.reify());

        &self.elems[row + col * self.cols.reify()]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Scalar {
        assert!(row < self.rows.reify() && col < self.cols.reify());

        &mut self.elems[row + col * self.cols.reify()]
    }
}


impl<T: Copy, N: Nat, L: Layout> Square for DenseMat<T, N, N, L>
    where DenseMat<T, N, N, L>: Matrix<Rows = N, Cols = N>,
          N: NatMul<N>,
          <N as NatMul<N>>::Result: Store<T>
{
}


impl<T: Copy, M: Dim, N: Dim> MatrixTranspose<DenseMat<T, N, M, Row>> for DenseMat<T, M, N, Column>
    where M: DimMul<N>,
          N: DimMul<M, Result = <M as DimMul<N>>::Result>,
          <M as DimMul<N>>::Result: Store<T>
{
    fn transpose(self) -> DenseMat<T, N, M, Row> {
        DenseMat {
            rows: self.cols,
            cols: self.rows,
            elems: self.elems,
            phantom: PhantomData,
        }
    }
}


impl<T: Copy, M: Dim, N: Dim> MatrixTranspose<DenseMat<T, N, M, Column>> for DenseMat<T, M, N, Row>
    where M: DimMul<N>,
          N: DimMul<M, Result = <M as DimMul<N>>::Result>,
          <M as DimMul<N>>::Result: Store<T>
{
    fn transpose(self) -> DenseMat<T, N, M, Column> {
        DenseMat {
            rows: self.cols,
            cols: self.rows,
            elems: self.elems,
            phantom: PhantomData,
        }
    }
}


macro_rules! mtra_impl {
    ($mat:ident) => {
        unsafe {
            let mut out = DenseMat {
                rows: $mat.cols,
                cols: $mat.rows,
                elems: Data::uninitialized($mat.elems.size),
                phantom: PhantomData,
            };

            for i in 0..$mat.rows.reify() {
                for j in 0..$mat.cols.reify() {
                    ptr::write(&mut out[[j, i]], $mat[[i, j]]);
                }
            }

            out
        }
    }
}


impl<T: Copy, M: Dim, N: Dim> MatrixTranspose<DenseMat<T, N, M, Row>> for DenseMat<T, M, N, Row>
    where M: DimMul<N>,
          N: DimMul<M, Result = <M as DimMul<N>>::Result>,
          <M as DimMul<N>>::Result: Store<T>
{
    fn transpose(self) -> DenseMat<T, N, M, Row> {
        mtra_impl!(self)
    }
}


impl<T: Copy, M: Dim, N: Dim> MatrixTranspose<DenseMat<T, N, M, Column>>
    for DenseMat<T, M, N, Column>
    where M: DimMul<N>,
          N: DimMul<M, Result = <M as DimMul<N>>::Result>,
          <M as DimMul<N>>::Result: Store<T>
{
    fn transpose(self) -> DenseMat<T, N, M, Column> {
        mtra_impl!(self)
    }
}


impl<T: Copy, M: Dim, N: Dim> MatrixAdd for DenseMat<T, M, N, Row>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Store<T>,
          T: AddAssign
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        madd_inplace!(@row, +=, self, rhs)
    }
}


impl<T: Copy, M: Dim, N: Dim> MatrixAdd for DenseMat<T, M, N, Column>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Store<T>,
          T: AddAssign
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        madd_inplace!(@col, +=, self, rhs)
    }
}


impl<T: Copy, M: Dim, N: Dim> MatrixSub for DenseMat<T, M, N, Row>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Store<T>,
          T: SubAssign
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        madd_inplace!(@row, -=, self, rhs)
    }
}


impl<T: Copy, M: Dim, N: Dim> MatrixSub for DenseMat<T, M, N, Column>
    where M: DimMul<N>,
          <M as DimMul<N>>::Result: Store<T>,
          T: SubAssign
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        madd_inplace!(@col, -=, self, rhs)
    }
}


impl<T: Copy, M: Dim, N: Dim, P: Dim, L1: Layout, L2: Layout, L3: Layout> MatrixMul<DenseMat<T, N, P, L2>, DenseMat<T, M, P, L3>> for DenseMat<T, M, N, L1>
    where M: DimMul<N> + DimMul<P>,
          N: DimMul<P>,
          <M as DimMul<N>>::Result: Store<T>,
          <M as DimMul<P>>::Result: Store<T>,
          <N as DimMul<P>>::Result: Store<T>,
          DenseMat<T, M, N, L1>: Matrix<Rows = M, Cols = N, Scalar = T>,
          DenseMat<T, N, P, L2>: Matrix<Rows = N, Cols = P, Scalar = T>,
          DenseMat<T, M, P, L3>: Matrix<Rows = M, Cols = P, Scalar = T>,
          T: Zero + Mul<Output = T> + AddAssign
{
    fn mul(self, rhs: DenseMat<T, N, P, L2>) -> DenseMat<T, M, P, L3> {
        mmul_loop_naive!(self, rhs, DenseMat::from_elem(self.rows, rhs.cols, T::zero()), self.rows.reify(), self.cols.reify(), rhs.rows.reify(), rhs.cols.reify())
    }
}


impl<T: Copy, N: Nat, L: Layout> MatrixIdentity for DenseMat<T, N, N, L>
    where DenseMat<T, N, N, L>: Matrix<Rows = N, Cols = N, Scalar = T>,
          N: NatMul<N>,
          <N as NatMul<N>>::Result: Store<T>,
          T: Zero + One
{
    fn eye(n: N) -> Self {
        let mut res = DenseMat {
            rows: n,
            cols: n,
            elems: Data::from_elem(<N as NatMul<N>>::Result::as_data(), &T::zero()),
            phantom: PhantomData,
        };

        for i in 0..N::as_usize() {
            res[[i, i]] = T::one();
        }

        res
    }
}


impl<T: Copy, M: Dim, N: Dim, L: Layout> Index<[usize; 2]> for DenseMat<T, M, N, L>
    where DenseMat<T, M, N, L>: Matrix<Scalar = T>,
          M: DimMul<N>,
          <M as DimMul<N>>::Result: Store<T>
{
    type Output = T;

    #[inline]
    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        self.get(idx[0], idx[1])
    }
}


impl<T: Copy, M: Dim, N: Dim, L: Layout> IndexMut<[usize; 2]> for DenseMat<T, M, N, L>
    where DenseMat<T, M, N, L>: Matrix<Scalar = T>,
          M: DimMul<N>,
          <M as DimMul<N>>::Result: Store<T>
{
    #[inline]
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Self::Output {
        self.get_mut(idx[0], idx[1])
    }
}


#[derive(PartialEq, Eq, Debug)]
#[repr(C)]
pub struct DenseVec<T, N: Nat, L: Layout = Column>
    where N: Store<T>
{
    elems: Data<T, N>,
    phantom: PhantomData<L>,
}


impl<T: Clone, N: Nat + Store<T>, L: Layout> Clone for DenseVec<T, N, L>
    where Data<T, N>: Copy
{
    fn clone(&self) -> Self {
        DenseVec {
            elems: self.elems.clone(),
            phantom: PhantomData,
        }
    }
}


impl<T: Copy, N: Nat + Store<T>, L: Layout> Copy for DenseVec<T, N, L>
    where DenseVec<T, N>: Clone,
          Data<T, N>: Copy
{
}


impl<'a, T: Copy, N: Nat, L: Layout> From<&'a [T]> for DenseVec<T, N, L>
    where N: Store<T>
{
    fn from(slice: &[T]) -> Self {
        DenseVec {
            elems: Data::from_slice(N::as_data(), slice),
            phantom: PhantomData,
        }
    }
}


impl<T, L: Layout> Deref for DenseVec<T, B1, L> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        &self.elems[0]
    }
}


impl<T, L: Layout> DerefMut for DenseVec<T, B1, L> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        &mut self.elems[0]
    }
}


#[repr(C)]
pub struct Vec2View<T> {
    pub x: T,
    pub y: T,
    void: Void,
}


impl<T, L: Layout> Deref for DenseVec<T, B2, L> {
    type Target = Vec2View<T>;

    fn deref<'a>(&'a self) -> &'a Vec2View<T> {
        unsafe { mem::transmute::<&'a DenseVec<T, B2, L>, &'a Vec2View<T>>(self) }
    }
}


impl<T, L: Layout> DerefMut for DenseVec<T, B2, L> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec2View<T> {
        unsafe { mem::transmute::<&'a mut DenseVec<T, B2, L>, &'a mut Vec2View<T>>(self) }
    }
}


#[repr(C)]
pub struct Vec3View<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    void: Void,
}


impl<T, L: Layout> Deref for DenseVec<T, B3, L> {
    type Target = Vec3View<T>;

    fn deref<'a>(&'a self) -> &'a Vec3View<T> {
        unsafe { mem::transmute::<&'a DenseVec<T, B3, L>, &'a Vec3View<T>>(self) }
    }
}


impl<T, L: Layout> DerefMut for DenseVec<T, B3, L> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec3View<T> {
        unsafe { mem::transmute::<&'a mut DenseVec<T, B3, L>, &'a mut Vec3View<T>>(self) }
    }
}


#[repr(C)]
pub struct Vec4View<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
    void: Void,
}


impl<T, L: Layout> Deref for DenseVec<T, B4, L> {
    type Target = Vec4View<T>;

    fn deref<'a>(&'a self) -> &'a Vec4View<T> {
        unsafe { mem::transmute::<&'a DenseVec<T, B4, L>, &'a Vec4View<T>>(self) }
    }
}


impl<T, L: Layout> DerefMut for DenseVec<T, B4, L> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vec4View<T> {
        unsafe { mem::transmute::<&'a mut DenseVec<T, B4, L>, &'a mut Vec4View<T>>(self) }
    }
}


impl<T: Copy, N: Nat + Store<T>, L: Layout> Vector for DenseVec<T, N, L> {
    type Dims = N;

    type Scalar = T;
}


impl<T: Copy, N: Nat + Store<T>, L: Layout> VectorAdd for DenseVec<T, N, L>
    where T: AddAssign
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        for i in 0..N::as_usize() {
            self.elems[i] += rhs.elems[i];
        }

        self
    }
}


impl<T: Copy, N: Nat + Store<T>, L: Layout> VectorSub for DenseVec<T, N, L>
    where T: SubAssign
{
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self {
        for i in 0..N::as_usize() {
            self.elems[i] -= rhs.elems[i];
        }

        self
    }
}


impl<T: Copy, N: Nat + Store<T>, L: Layout> VectorHadamard for DenseVec<T, N, L>
    where T: MulAssign
{
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self {
        for i in 0..N::as_usize() {
            self.elems[i] *= rhs.elems[i];
        }

        self
    }
}


impl<T: Copy, N: Nat + Store<T>, L: Layout> VectorDot for DenseVec<T, N, L>
    where T: Mul,
          <T as Mul>::Output: Zero + AddAssign
{
    type Output = <T as Mul>::Output;

    fn dot(self, rhs: Self) -> Self::Output {
        let mut accum = <T as Mul>::Output::zero();

        let lhs = &self.elems[..N::as_usize()];
        let rhs = &rhs.elems[..N::as_usize()];

        for i in 0..N::as_usize() {
            accum += lhs[i] * rhs[i];
        }

        accum
    }
}


impl<'a, 'b, T: Copy, N: Nat + Store<T>, L: Layout> VectorDot<&'b DenseVec<T, N, L>> for &'a DenseVec<T, N, L>
    where T: Mul,
          <T as Mul>::Output: Zero + AddAssign
{
    type Output = <T as Mul>::Output;

    fn dot(self, rhs: &'b DenseVec<T, N, L>) -> Self::Output {
        let mut accum = <T as Mul>::Output::zero();

        let lhs = &self.elems[..N::as_usize()];
        let rhs = &rhs.elems[..N::as_usize()];

        for i in 0..N::as_usize() {
            accum += lhs[i] * rhs[i];
        }

        accum
    }
}


impl<T: Copy, N: Nat + Store<T>, L: Layout> Index<usize> for DenseVec<T, N, L> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        &self.elems[idx]
    }
}


impl<T: Copy, N: Nat + Store<T>, L: Layout> IndexMut<usize> for DenseVec<T, N, L> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        &mut self.elems[idx]
    }
}


// ************************************************************************************************
// Matrix-vector multiplication
// vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv


// ************************************************************************************************
// Convenience type synonyms for dense matrices/vectors
// vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv


pub type Mat<T, M, N, L> = DenseMat<T, M, N, L>;
pub type Vect<T, N> = DenseVec<T, N>;


// THEY SAID I WAS CRAZY
pub type Mat1x1<T, L = Column> = Mat<T, B1, B1, L>;
pub type Mat1x2<T, L = Column> = Mat<T, B1, B3, L>;
pub type Mat1x3<T, L = Column> = Mat<T, B1, B3, L>;
pub type Mat1x4<T, L = Column> = Mat<T, B1, B4, L>;
pub type Mat1x5<T, L = Column> = Mat<T, B1, B5, L>;
pub type Mat1x6<T, L = Column> = Mat<T, B1, B6, L>;

pub type Mat1xD<T, L = Column> = Mat<T, B1, Dyn, L>;
pub type MatDx1<T, L = Column> = Mat<T, Dyn, B1, L>;

// THEY SAID I WAS MAD
pub type Mat2x1<T, L = Column> = Mat<T, B3, B1, L>;
pub type Mat2x2<T, L = Column> = Mat<T, B3, B3, L>;
pub type Mat2x3<T, L = Column> = Mat<T, B3, B3, L>;
pub type Mat2x4<T, L = Column> = Mat<T, B3, B4, L>;
pub type Mat2x5<T, L = Column> = Mat<T, B3, B5, L>;
pub type Mat2x6<T, L = Column> = Mat<T, B3, B6, L>;

pub type Mat2xD<T, L = Column> = Mat<T, B3, Dyn, L>;
pub type MatDx2<T, L = Column> = Mat<T, Dyn, B3, L>;

// BUT I'LL SHOW THEM
pub type Mat3x1<T, L = Column> = Mat<T, B3, B1, L>;
pub type Mat3x2<T, L = Column> = Mat<T, B3, B3, L>;
pub type Mat3x3<T, L = Column> = Mat<T, B3, B3, L>;
pub type Mat3x4<T, L = Column> = Mat<T, B3, B4, L>;
pub type Mat3x5<T, L = Column> = Mat<T, B3, B5, L>;
pub type Mat3x6<T, L = Column> = Mat<T, B3, B6, L>;

pub type Mat3xD<T, L = Column> = Mat<T, B3, Dyn, L>;
pub type MatDx3<T, L = Column> = Mat<T, Dyn, B3, L>;

// I'LL SHOW THEM ALL
pub type Mat4x1<T, L = Column> = Mat<T, B4, B1, L>;
pub type Mat4x2<T, L = Column> = Mat<T, B4, B3, L>;
pub type Mat4x3<T, L = Column> = Mat<T, B4, B3, L>;
pub type Mat4x4<T, L = Column> = Mat<T, B4, B4, L>;
pub type Mat4x5<T, L = Column> = Mat<T, B4, B5, L>;
pub type Mat4x6<T, L = Column> = Mat<T, B4, B6, L>;

pub type Mat4xD<T, L = Column> = Mat<T, B4, Dyn, L>;
pub type MatDx4<T, L = Column> = Mat<T, Dyn, B4, L>;

// THE DEPTH OF MY MADNESS
pub type Mat5x1<T, L = Column> = Mat<T, B5, B1, L>;
pub type Mat5x2<T, L = Column> = Mat<T, B5, B3, L>;
pub type Mat5x3<T, L = Column> = Mat<T, B5, B3, L>;
pub type Mat5x4<T, L = Column> = Mat<T, B5, B4, L>;
pub type Mat5x5<T, L = Column> = Mat<T, B5, B5, L>;
pub type Mat5x6<T, L = Column> = Mat<T, B5, B6, L>;

pub type Mat5xD<T, L = Column> = Mat<T, B5, Dyn, L>;
pub type MatDx5<T, L = Column> = Mat<T, Dyn, B5, L>;

// and the sum total of my wit
pub type Mat6x1<T, L = Column> = Mat<T, B6, B1, L>;
pub type Mat6x2<T, L = Column> = Mat<T, B6, B3, L>;
pub type Mat6x3<T, L = Column> = Mat<T, B6, B3, L>;
pub type Mat6x4<T, L = Column> = Mat<T, B6, B4, L>;
pub type Mat6x5<T, L = Column> = Mat<T, B6, B5, L>;
pub type Mat6x6<T, L = Column> = Mat<T, B6, B6, L>;

pub type Mat6xD<T, L = Column> = Mat<T, B6, Dyn, L>;
pub type MatDx6<T, L = Column> = Mat<T, Dyn, B6, L>;

pub type MatDxD<T, L = Column> = Mat<T, Dyn, Dyn, L>;


pub type Vec1<T> = Vect<T, B1>;
pub type Vec2<T> = Vect<T, B3>;
pub type Vec3<T> = Vect<T, B3>;
pub type Vec4<T> = Vect<T, B4>;
pub type Vec5<T> = Vect<T, B5>;
pub type Vec6<T> = Vect<T, B6>;

pub type VecD<T> = Vect<T, Dyn>;


#[macro_export]
macro_rules! Vec1 {
    (x: $x:expr) => (Vec1!($x));
    ($x:expr) => ($crate::linalg::Vec1::from(&[$x][..]));
}


#[macro_export]
macro_rules! Vec2 {
    (x: $x:expr, y: $y:expr) => (Vec2!($x, $y));
    ($x:expr, $y:expr) => ($crate::linalg::Vec2::from(&[$x, $y][..]));
}


#[macro_export]
macro_rules! Vec3 {
    (x: $x:expr, y: $y:expr, z: $z:expr) => (Vec3!($x, $y, $z));
    ($x:expr, $y:expr, $z:expr) => ($crate::linalg::Vec3::from(&[$x, $y, $z][..]));
}


#[macro_export]
macro_rules! Vec4 {
    (x: $x:expr, y: $y:expr, z: $z:expr, w: $w:expr) => (Vec4!($x, $y, $z, $w));
    ($x:expr, $y:expr, $z:expr, $w:expr) => ($crate::linalg::Vec4::from(&[$x, $y, $z, $w][..]));
}


#[macro_export]
macro_rules! Vec5 {
    ($x:expr, $y:expr, $z:expr, $u:expr, $v:expr) => ($crate::linalg::Vec5::from(&[$x, $y, $z, $u, $v][..]));
}


#[macro_export]
macro_rules! Vec6 {
    ($x:expr, $y:expr, $z:expr, $u:expr, $v:expr, $w:expr) => ($crate::linalg::Vec6::from(&[$x, $y, $z, $u, $v, $w][..]))
}
