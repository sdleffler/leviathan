use lev::typehack::binary::*;


#[test]
fn binary_as_usize() {
    assert_eq!(B0::as_usize(), 0);
    assert_eq!(B1::as_usize(), 1);
    assert_eq!(B2::as_usize(), 2);
}


#[test]
fn binary_add() {
    assert_eq!(<B0 as NatAdd<B0>>::Result::as_usize(), 0);
    assert_eq!(<B0 as NatAdd<B1>>::Result::as_usize(), 1);
    assert_eq!(<B1 as NatAdd<B0>>::Result::as_usize(), 1);
    assert_eq!(<B1 as NatAdd<B1>>::Result::as_usize(), 2);
    assert_eq!(<B3 as NatAdd<B2>>::Result::as_usize(), 5);
}


#[test]
fn binary_sub() {
    assert_eq!(<B0 as NatSub<B0>>::Result::as_usize(), 0);
    assert_eq!(<B1 as NatSub<B0>>::Result::as_usize(), 1);
    assert_eq!(<B1 as NatSub<B1>>::Result::as_usize(), 0);
    assert_eq!(<B2 as NatSub<B1>>::Result::as_usize(), 1);
    assert_eq!(<B4 as NatSub<B1>>::Result::as_usize(), 3);
    assert_eq!(<B4 as NatSub<B2>>::Result::as_usize(), 2);
}


#[test]
fn binary_mul() {
    assert_eq!(<B0 as NatMul<B0>>::Result::as_usize(), 0);
    assert_eq!(<B0 as NatMul<B1>>::Result::as_usize(), 0);
    assert_eq!(<B1 as NatMul<B0>>::Result::as_usize(), 0);
    assert_eq!(<B1 as NatMul<B1>>::Result::as_usize(), 1);
    assert_eq!(<B1 as NatMul<B2>>::Result::as_usize(), 2);
    assert_eq!(<B2 as NatMul<B1>>::Result::as_usize(), 2);
    assert_eq!(<B2 as NatMul<B2>>::Result::as_usize(), 4);
    assert_eq!(<B4 as NatMul<B3>>::Result::as_usize(), 12);
}


#[test]
fn binary_shl() {
    assert_eq!(<B0 as NatShl<B2>>::Result::as_usize(), 0);
    assert_eq!(<B1 as NatShl<B0>>::Result::as_usize(), 1);
    assert_eq!(<B1 as NatShl<B1>>::Result::as_usize(), 2);
    assert_eq!(<B1 as NatShl<B2>>::Result::as_usize(), 4);
    assert_eq!(<B1 as NatShl<B3>>::Result::as_usize(), 8);
    assert_eq!(<B2 as NatShl<B0>>::Result::as_usize(), 2);
    assert_eq!(<B2 as NatShl<B1>>::Result::as_usize(), 4);
    assert_eq!(<B2 as NatShl<B2>>::Result::as_usize(), 8);
    assert_eq!(<B2 as NatShl<B3>>::Result::as_usize(), 16);
    assert_eq!(<B3 as NatShl<B0>>::Result::as_usize(), 3);
    assert_eq!(<B3 as NatShl<B1>>::Result::as_usize(), 6);
    assert_eq!(<B3 as NatShl<B2>>::Result::as_usize(), 12);
    assert_eq!(<B3 as NatShl<B3>>::Result::as_usize(), 24);
}
