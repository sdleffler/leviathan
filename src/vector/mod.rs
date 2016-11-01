pub mod traits;

mod dense;
use self::dense::*;


use typehack::peano::*;


pub type Vec1<T> = DenseVec<T, S<Z>>;
pub type Vec2<T> = DenseVec<T, S<S<Z>>>;
pub type Vec3<T> = DenseVec<T, S<S<S<Z>>>>;
pub type Vec4<T> = DenseVec<T, S<S<S<S<Z>>>>>;
pub type Vec5<T> = DenseVec<T, S<S<S<S<S<Z>>>>>>;
pub type Vec6<T> = DenseVec<T, S<S<S<S<S<S<Z>>>>>>>;


#[macro_export]
macro_rules! Vec1 {
    (x: $x:expr) => (Vec!($x));
    ($x:expr) => (Vec1::from(&[$x][..]));
}


#[macro_export]
macro_rules! Vec2 {
    (x: $x:expr, y: $y:expr) => (Vec2!($x, $y));
    ($x:expr, $y:expr) => (Vec2::from(&[$x, $y][..]));
}


#[macro_export]
macro_rules! Vec3 {
    (x: $x:expr, y: $y:expr, z: $z:expr) => (Vec2!($x, $y, $z));
    ($x:expr, $y:expr, $z:expr) => (Vec2::from(&[$x, $y, $z][..]));
}


#[macro_export]
macro_rules! Vec4 {
    (x: $x:expr, y: $y:expr, z: $z:expr, w: $w:expr) => (Vec2!($x, $y, $z, $w));
    ($x:expr, $y:expr, $z:expr, $w:expr) => (Vec2::from(&[$x, $y, $z, $w][..]));
}


#[macro_export]
macro_rules! Vec5 {
    ($x:expr, $y:expr, $z:expr, $u:expr, $v:expr) => (Vec5::from(&[$x, $y, $z, $u, $v][..]));
}


#[macro_export]
macro_rules! Vec6 {
    ($x:expr, $y:expr, $z:expr, $u:expr, $v:expr, $w:expr) => (Vec6::from(&[$x, $y, $z, $u, $v, $w][..]))
}
