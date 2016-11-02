use lev::matrix::*;
use lev::matrix::traits::*;


#[allow(unused_variables)]
#[test]
fn mat2x2_i32_copyable() {
    let a: Mat2x2<i32> = [1, 0, 0, 1][..].into();
    let b = a;
    let c = a;
}


#[test]
fn from_mat2x2_id_mul_sanity() {
    let a: Mat2x2<i32> = [1, 0, 0, 1][..].into();
    let b: Mat2x2<i32> = [0, 1, 1, 0][..].into();
    let c: Mat2x2<i32> = [0, 1, 1, 0][..].into();

    assert_eq!(a.mul(b), c);
}


#[test]
fn from_mat2x2_add_sanity() {
    let a: Mat2x2<i32> = [1, 0, 0, 1][..].into();
    let b: Mat2x2<i32> = [0, 1, 1, 0][..].into();
    let c: Mat2x2<i32> = [1, 1, 1, 1][..].into();

    assert_eq!(a.add(b), c);
}


#[test]
fn from_mat2x2_sub_sanity() {
    let a: Mat2x2<i32> = [1, 0, 0, 1][..].into();
    let b: Mat2x2<i32> = [0, 1, 1, 0][..].into();
    let c: Mat2x2<i32> = [1, -1, -1, 1][..].into();

    assert_eq!(a.sub(b), c);
}
