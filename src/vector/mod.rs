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
