use array::storage::*;

use typehack::peano::*;


#[repr(C)]
pub struct Mat<T, M: Nat, N: Nat>(Storage<T, <M as NatMul<N>>::Result>)
    where M: NatMul<N>,
          <M as NatMul<N>>::Result: Link<T>;


// THEY SAID I WAS CRAZY
pub type Mat1x1<T> = Mat<T, S<Z>, S<Z>>;
pub type Mat1x2<T> = Mat<T, S<Z>, S<S<Z>>>;
pub type Mat1x3<T> = Mat<T, S<Z>, S<S<S<Z>>>>;
pub type Mat1x4<T> = Mat<T, S<Z>, S<S<S<S<Z>>>>>;
pub type Mat1x5<T> = Mat<T, S<Z>, S<S<S<S<S<Z>>>>>>;
pub type Mat1x6<T> = Mat<T, S<Z>, S<S<S<S<S<S<Z>>>>>>>;

// THEY SAID I WAS MAD
pub type Mat2x1<T> = Mat<T, S<S<Z>>, S<Z>>;
pub type Mat2x2<T> = Mat<T, S<S<Z>>, S<S<Z>>>;
pub type Mat2x3<T> = Mat<T, S<S<Z>>, S<S<S<Z>>>>;
pub type Mat2x4<T> = Mat<T, S<S<Z>>, S<S<S<S<Z>>>>>;
pub type Mat2x5<T> = Mat<T, S<S<Z>>, S<S<S<S<S<Z>>>>>>;
pub type Mat2x6<T> = Mat<T, S<S<Z>>, S<S<S<S<S<S<Z>>>>>>>;

// BUT I'LL SHOW THEM
pub type Mat3x1<T> = Mat<T, S<S<S<Z>>>, S<Z>>;
pub type Mat3x2<T> = Mat<T, S<S<S<Z>>>, S<S<Z>>>;
pub type Mat3x3<T> = Mat<T, S<S<S<Z>>>, S<S<S<Z>>>>;
pub type Mat3x4<T> = Mat<T, S<S<S<Z>>>, S<S<S<S<Z>>>>>;
pub type Mat3x5<T> = Mat<T, S<S<S<Z>>>, S<S<S<S<S<Z>>>>>>;
pub type Mat3x6<T> = Mat<T, S<S<S<Z>>>, S<S<S<S<S<S<Z>>>>>>>;

// I'LL SHOW THEM ALL
pub type Mat4x1<T> = Mat<T, S<S<S<S<Z>>>>, S<Z>>;
pub type Mat4x2<T> = Mat<T, S<S<S<S<Z>>>>, S<S<Z>>>;
pub type Mat4x3<T> = Mat<T, S<S<S<S<Z>>>>, S<S<S<Z>>>>;
pub type Mat4x4<T> = Mat<T, S<S<S<S<Z>>>>, S<S<S<S<Z>>>>>;
pub type Mat4x5<T> = Mat<T, S<S<S<S<Z>>>>, S<S<S<S<S<Z>>>>>>;
pub type Mat4x6<T> = Mat<T, S<S<S<S<Z>>>>, S<S<S<S<S<S<Z>>>>>>>;

// THE DEPTH OF MY MADNESS
pub type Mat5x1<T> = Mat<T, S<S<S<S<S<Z>>>>>, S<Z>>;
pub type Mat5x2<T> = Mat<T, S<S<S<S<S<Z>>>>>, S<S<Z>>>;
pub type Mat5x3<T> = Mat<T, S<S<S<S<S<Z>>>>>, S<S<S<Z>>>>;
pub type Mat5x4<T> = Mat<T, S<S<S<S<S<Z>>>>>, S<S<S<S<Z>>>>>;
pub type Mat5x5<T> = Mat<T, S<S<S<S<S<Z>>>>>, S<S<S<S<S<Z>>>>>>;
pub type Mat5x6<T> = Mat<T, S<S<S<S<S<Z>>>>>, S<S<S<S<S<S<Z>>>>>>>;

// and the sum total of my wit
pub type Mat6x1<T> = Mat<T, S<S<S<S<S<S<Z>>>>>>, S<Z>>;
pub type Mat6x2<T> = Mat<T, S<S<S<S<S<S<Z>>>>>>, S<S<Z>>>;
pub type Mat6x3<T> = Mat<T, S<S<S<S<S<S<Z>>>>>>, S<S<S<Z>>>>;
pub type Mat6x4<T> = Mat<T, S<S<S<S<S<S<Z>>>>>>, S<S<S<S<Z>>>>>;
pub type Mat6x5<T> = Mat<T, S<S<S<S<S<S<Z>>>>>>, S<S<S<S<S<Z>>>>>>;
pub type Mat6x6<T> = Mat<T, S<S<S<S<S<S<Z>>>>>>, S<S<S<S<S<S<Z>>>>>>>;
