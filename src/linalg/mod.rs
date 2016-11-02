pub mod matrix;
pub mod vector;
pub mod traits;


use typehack::dim::*;
use typehack::peano::*;


pub use self::matrix::StaticMat as Mat;
pub use self::matrix::DynamicMat as DMat;
pub use self::vector::StaticVec as Vector;


// THEY SAID I WAS CRAZY
pub type Mat1x1<T> = Mat<T, S<Z>, S<Z>>;
pub type Mat1x2<T> = Mat<T, S<Z>, S<S<Z>>>;
pub type Mat1x3<T> = Mat<T, S<Z>, S<S<S<Z>>>>;
pub type Mat1x4<T> = Mat<T, S<Z>, S<S<S<S<Z>>>>>;
pub type Mat1x5<T> = Mat<T, S<Z>, S<S<S<S<S<Z>>>>>>;
pub type Mat1x6<T> = Mat<T, S<Z>, S<S<S<S<S<S<Z>>>>>>>;

pub type Mat1xD<T> = DMat<T, S<Z>, Dyn>;
pub type MatDx1<T> = DMat<T, Dyn, S<Z>>;

// THEY SAID I WAS MAD
pub type Mat2x1<T> = Mat<T, S<S<Z>>, S<Z>>;
pub type Mat2x2<T> = Mat<T, S<S<Z>>, S<S<Z>>>;
pub type Mat2x3<T> = Mat<T, S<S<Z>>, S<S<S<Z>>>>;
pub type Mat2x4<T> = Mat<T, S<S<Z>>, S<S<S<S<Z>>>>>;
pub type Mat2x5<T> = Mat<T, S<S<Z>>, S<S<S<S<S<Z>>>>>>;
pub type Mat2x6<T> = Mat<T, S<S<Z>>, S<S<S<S<S<S<Z>>>>>>>;

pub type Mat2xD<T> = DMat<T, S<S<Z>>, Dyn>;
pub type MatDx2<T> = DMat<T, Dyn, S<S<Z>>>;

// BUT I'LL SHOW THEM
pub type Mat3x1<T> = Mat<T, S<S<S<Z>>>, S<Z>>;
pub type Mat3x2<T> = Mat<T, S<S<S<Z>>>, S<S<Z>>>;
pub type Mat3x3<T> = Mat<T, S<S<S<Z>>>, S<S<S<Z>>>>;
pub type Mat3x4<T> = Mat<T, S<S<S<Z>>>, S<S<S<S<Z>>>>>;
pub type Mat3x5<T> = Mat<T, S<S<S<Z>>>, S<S<S<S<S<Z>>>>>>;
pub type Mat3x6<T> = Mat<T, S<S<S<Z>>>, S<S<S<S<S<S<Z>>>>>>>;

pub type Mat3xD<T> = DMat<T, S<S<S<Z>>>, Dyn>;
pub type MatDx3<T> = DMat<T, Dyn, S<S<S<Z>>>>;

// I'LL SHOW THEM ALL
pub type Mat4x1<T> = Mat<T, S<S<S<S<Z>>>>, S<Z>>;
pub type Mat4x2<T> = Mat<T, S<S<S<S<Z>>>>, S<S<Z>>>;
pub type Mat4x3<T> = Mat<T, S<S<S<S<Z>>>>, S<S<S<Z>>>>;
pub type Mat4x4<T> = Mat<T, S<S<S<S<Z>>>>, S<S<S<S<Z>>>>>;
pub type Mat4x5<T> = Mat<T, S<S<S<S<Z>>>>, S<S<S<S<S<Z>>>>>>;
pub type Mat4x6<T> = Mat<T, S<S<S<S<Z>>>>, S<S<S<S<S<S<Z>>>>>>>;

pub type Mat4xD<T> = DMat<T, S<S<S<S<Z>>>>, Dyn>;
pub type MatDx4<T> = DMat<T, Dyn, S<S<S<S<Z>>>>>;

// THE DEPTH OF MY MADNESS
pub type Mat5x1<T> = Mat<T, S<S<S<S<S<Z>>>>>, S<Z>>;
pub type Mat5x2<T> = Mat<T, S<S<S<S<S<Z>>>>>, S<S<Z>>>;
pub type Mat5x3<T> = Mat<T, S<S<S<S<S<Z>>>>>, S<S<S<Z>>>>;
pub type Mat5x4<T> = Mat<T, S<S<S<S<S<Z>>>>>, S<S<S<S<Z>>>>>;
pub type Mat5x5<T> = Mat<T, S<S<S<S<S<Z>>>>>, S<S<S<S<S<Z>>>>>>;
pub type Mat5x6<T> = Mat<T, S<S<S<S<S<Z>>>>>, S<S<S<S<S<S<Z>>>>>>>;

pub type Mat5xD<T> = DMat<T, S<S<S<S<S<Z>>>>>, Dyn>;
pub type MatDx5<T> = DMat<T, Dyn, S<S<S<S<S<Z>>>>>>;

// and the sum total of my wit
pub type Mat6x1<T> = Mat<T, S<S<S<S<S<S<Z>>>>>>, S<Z>>;
pub type Mat6x2<T> = Mat<T, S<S<S<S<S<S<Z>>>>>>, S<S<Z>>>;
pub type Mat6x3<T> = Mat<T, S<S<S<S<S<S<Z>>>>>>, S<S<S<Z>>>>;
pub type Mat6x4<T> = Mat<T, S<S<S<S<S<S<Z>>>>>>, S<S<S<S<Z>>>>>;
pub type Mat6x5<T> = Mat<T, S<S<S<S<S<S<Z>>>>>>, S<S<S<S<S<Z>>>>>>;
pub type Mat6x6<T> = Mat<T, S<S<S<S<S<S<Z>>>>>>, S<S<S<S<S<S<Z>>>>>>>;

pub type Mat6xD<T> = DMat<T, S<S<S<S<S<S<Z>>>>>>, Dyn>;
pub type MatDx6<T> = DMat<T, Dyn, S<S<S<S<S<S<Z>>>>>>>;

pub type MatDxD<T> = DMat<T, Dyn, Dyn>;


pub type Vec1<T> = Vector<T, S<Z>>;
pub type Vec2<T> = Vector<T, S<S<Z>>>;
pub type Vec3<T> = Vector<T, S<S<S<Z>>>>;
pub type Vec4<T> = Vector<T, S<S<S<S<Z>>>>>;
pub type Vec5<T> = Vector<T, S<S<S<S<S<Z>>>>>>;
pub type Vec6<T> = Vector<T, S<S<S<S<S<S<Z>>>>>>>;


#[macro_export]
macro_rules! Vec1 {
    (x: $x:expr) => (Vec1!($x));
    ($x:expr) => ($crate::linalg::vector::Vec1::from(&[$x][..]));
}


#[macro_export]
macro_rules! Vec2 {
    (x: $x:expr, y: $y:expr) => (Vec2!($x, $y));
    ($x:expr, $y:expr) => ($crate::linalg::vector::Vec2::from(&[$x, $y][..]));
}


#[macro_export]
macro_rules! Vec3 {
    (x: $x:expr, y: $y:expr, z: $z:expr) => (Vec3!($x, $y, $z));
    ($x:expr, $y:expr, $z:expr) => ($crate::linalg::vector::Vec3::from(&[$x, $y, $z][..]));
}


#[macro_export]
macro_rules! Vec4 {
    (x: $x:expr, y: $y:expr, z: $z:expr, w: $w:expr) => (Vec4!($x, $y, $z, $w));
    ($x:expr, $y:expr, $z:expr, $w:expr) => ($crate::linalg::vector::Vec4::from(&[$x, $y, $z, $w][..]));
}


#[macro_export]
macro_rules! Vec5 {
    ($x:expr, $y:expr, $z:expr, $u:expr, $v:expr) => ($crate::linalg::vector::Vec5::from(&[$x, $y, $z, $u, $v][..]));
}


#[macro_export]
macro_rules! Vec6 {
    ($x:expr, $y:expr, $z:expr, $u:expr, $v:expr, $w:expr) => ($crate::linalg::vector::Vec6::from(&[$x, $y, $z, $u, $v, $w][..]))
}
