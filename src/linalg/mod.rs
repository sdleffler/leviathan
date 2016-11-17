use typehack::dim::*;
use typehack::binary::*;


#[macro_export]
macro_rules! Mat {
    (@count $x:expr $(, $xs:expr)*) => (<Mat!(@count $($xs:tt),*) as $crate::typehack::binary::Nat>::Succ);
    (@count) => ($crate::typehack::binary::O);
    (@unpack row: {$($xs:expr),*} [$($ys:expr),*] $(, $rows:tt)*) => (Mat!(@unpack row: {$($xs,)* $($ys),*} $($rows),*));
    (@unpack row: {$($xs:expr),*}) => (data![$($xs),*]);
    (@unpack column: {$($xs:expr),*} $([$y:expr $(, $ys:expr)*]),+) => (Mat!(@unpack column: {$($xs,)* $($y),*} $([$($ys),*]),+));
    (@unpack column: {$($xs:expr),*} $([]),+) => (data![$($xs),*]);
    (@layout row) => ($crate::linalg::Row);
    (@layout column) => ($crate::linalg::Column);
    (#$layout:ident [$($cols:expr),*] $(, $rows:tt)*) => (
        $crate::linalg::Mat::<_, _, _, Mat!(@layout $layout)>::from_data(
            <Mat!(@count [] $(, $rows)*) as $crate::typehack::binary::Nat>::as_data(),
            <Mat!(@count $($cols),*) as $crate::typehack::binary::Nat>::as_data(),
            Mat!(@unpack $layout: {} [$($cols),*] $(, $rows)*))
    );
    ([$($cols:expr),*] $(, $rows:tt)*) => (Mat![#column [$($cols),*] $(, $rows)*]);
}


#[macro_export]
macro_rules! Vect {
    ($($xs:expr),*) => ($crate::linalg::Vect::from_data(data![$($xs),*]));
}


#[macro_export]
macro_rules! Vect1 {
    (x: $x:expr) => (Vect1!($x));
    ($x:expr) => ($crate::linalg::Vect1::from_data(data![$x]));
}


#[macro_export]
macro_rules! Vect2 {
    (x: $x:expr, y: $y:expr) => (Vect2!($x, $y));
    ($x:expr, $y:expr) => ($crate::linalg::Vect2::from_data(data![$x, $y]));
}


#[macro_export]
macro_rules! Vect3 {
    (x: $x:expr, y: $y:expr, z: $z:expr) => (Vect3!($x, $y, $z));
    ($x:expr, $y:expr, $z:expr) => ($crate::linalg::Vect3::from_data(data![$x, $y, $z]));
}


#[macro_export]
macro_rules! Vect4 {
    (x: $x:expr, y: $y:expr, z: $z:expr, w: $w:expr) => (Vect4!($x, $y, $z, $w));
    ($x:expr, $y:expr, $z:expr, $w:expr) => ($crate::linalg::Vect4::from_data(data![$x, $y, $z, $w]));
}


#[macro_export]
macro_rules! Vect5 {
    ($x:expr, $y:expr, $z:expr, $u:expr, $v:expr) => ($crate::linalg::Vect5::from_data(data![$x, $y, $z, $u, $v]));
}


#[macro_export]
macro_rules! Vect6 {
    ($x:expr, $y:expr, $z:expr, $u:expr, $v:expr, $w:expr) => ($crate::linalg::Vect6::from_data(data![$x, $y, $z, $u, $v, $w]))
}


pub mod matrix;
pub use self::matrix::*;

pub mod solve;
pub mod lp;

pub mod traits;
pub use self::traits::*;

pub mod vector;
pub use self::vector::*;


pub type Mat<T, M, N, L = Column> = DenseMat<T, M, N, L>;
pub type Vect<T, N> = DenseVec<T, N>;


// THEY SAID I WAS CRAZY
pub type Mat1x1<T, L = Column> = Mat<T, B1, B1, L>;
pub type Mat1x2<T, L = Column> = Mat<T, B1, B2, L>;
pub type Mat1x3<T, L = Column> = Mat<T, B1, B3, L>;
pub type Mat1x4<T, L = Column> = Mat<T, B1, B4, L>;
pub type Mat1x5<T, L = Column> = Mat<T, B1, B5, L>;
pub type Mat1x6<T, L = Column> = Mat<T, B1, B6, L>;

pub type Mat1xD<T, L = Column> = Mat<T, B1, Dyn, L>;
pub type MatDx1<T, L = Column> = Mat<T, Dyn, B1, L>;

// THEY SAID I WAS MAD
pub type Mat2x1<T, L = Column> = Mat<T, B2, B1, L>;
pub type Mat2x2<T, L = Column> = Mat<T, B2, B2, L>;
pub type Mat2x3<T, L = Column> = Mat<T, B2, B3, L>;
pub type Mat2x4<T, L = Column> = Mat<T, B2, B4, L>;
pub type Mat2x5<T, L = Column> = Mat<T, B2, B5, L>;
pub type Mat2x6<T, L = Column> = Mat<T, B2, B6, L>;

pub type Mat2xD<T, L = Column> = Mat<T, B2, Dyn, L>;
pub type MatDx2<T, L = Column> = Mat<T, Dyn, B2, L>;

// BUT I'LL SHOW THEM
pub type Mat3x1<T, L = Column> = Mat<T, B3, B1, L>;
pub type Mat3x2<T, L = Column> = Mat<T, B3, B2, L>;
pub type Mat3x3<T, L = Column> = Mat<T, B3, B3, L>;
pub type Mat3x4<T, L = Column> = Mat<T, B3, B4, L>;
pub type Mat3x5<T, L = Column> = Mat<T, B3, B5, L>;
pub type Mat3x6<T, L = Column> = Mat<T, B3, B6, L>;

pub type Mat3xD<T, L = Column> = Mat<T, B3, Dyn, L>;
pub type MatDx3<T, L = Column> = Mat<T, Dyn, B3, L>;

// I'LL SHOW THEM ALL
pub type Mat4x1<T, L = Column> = Mat<T, B4, B1, L>;
pub type Mat4x2<T, L = Column> = Mat<T, B4, B2, L>;
pub type Mat4x3<T, L = Column> = Mat<T, B4, B3, L>;
pub type Mat4x4<T, L = Column> = Mat<T, B4, B4, L>;
pub type Mat4x5<T, L = Column> = Mat<T, B4, B5, L>;
pub type Mat4x6<T, L = Column> = Mat<T, B4, B6, L>;

pub type Mat4xD<T, L = Column> = Mat<T, B4, Dyn, L>;
pub type MatDx4<T, L = Column> = Mat<T, Dyn, B4, L>;

// THE DEPTH OF MY MADNESS
pub type Mat5x1<T, L = Column> = Mat<T, B5, B1, L>;
pub type Mat5x2<T, L = Column> = Mat<T, B5, B2, L>;
pub type Mat5x3<T, L = Column> = Mat<T, B5, B3, L>;
pub type Mat5x4<T, L = Column> = Mat<T, B5, B4, L>;
pub type Mat5x5<T, L = Column> = Mat<T, B5, B5, L>;
pub type Mat5x6<T, L = Column> = Mat<T, B5, B6, L>;

pub type Mat5xD<T, L = Column> = Mat<T, B5, Dyn, L>;
pub type MatDx5<T, L = Column> = Mat<T, Dyn, B5, L>;

// and the sum total of my wit
pub type Mat6x1<T, L = Column> = Mat<T, B6, B1, L>;
pub type Mat6x2<T, L = Column> = Mat<T, B6, B2, L>;
pub type Mat6x3<T, L = Column> = Mat<T, B6, B3, L>;
pub type Mat6x4<T, L = Column> = Mat<T, B6, B4, L>;
pub type Mat6x5<T, L = Column> = Mat<T, B6, B5, L>;
pub type Mat6x6<T, L = Column> = Mat<T, B6, B6, L>;

pub type Mat6xD<T, L = Column> = Mat<T, B6, Dyn, L>;
pub type MatDx6<T, L = Column> = Mat<T, Dyn, B6, L>;

pub type MatDxD<T, L = Column> = Mat<T, Dyn, Dyn, L>;


pub type Vect1<T> = Vect<T, B1>;
pub type Vect2<T> = Vect<T, B2>;
pub type Vect3<T> = Vect<T, B3>;
pub type Vect4<T> = Vect<T, B4>;
pub type Vect5<T> = Vect<T, B5>;
pub type Vect6<T> = Vect<T, B6>;

pub type VectD<T> = Vect<T, Dyn>;
