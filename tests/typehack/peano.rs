use lev::typehack::peano::*;


#[test]
fn peano_as_usize() {
    assert_eq!(Z::as_usize(), 0);
    assert_eq!(S::<Z>::as_usize(), 1);
    assert_eq!(S::<S<Z>>::as_usize(), 2);
}


#[test]
fn peano_add() {
    assert_eq!(<Z as NatAdd<Z>>::Result::as_usize(), 0);
    assert_eq!(<Z as NatAdd<S<Z>>>::Result::as_usize(), 1);
    assert_eq!(<S<Z> as NatAdd<Z>>::Result::as_usize(), 1);
    assert_eq!(<S<Z> as NatAdd<S<Z>>>::Result::as_usize(), 2);
    assert_eq!(<S<S<S<Z>>> as NatAdd<S<S<Z>>>>::Result::as_usize(), 5);
}


#[test]
fn peano_sub() {
    assert_eq!(<Z as NatSub<Z>>::Result::as_usize(), 0);
    assert_eq!(<S<Z> as NatSub<Z>>::Result::as_usize(), 1);
    assert_eq!(<S<Z> as NatSub<S<Z>>>::Result::as_usize(), 0);
    assert_eq!(<S<S<Z>> as NatSub<S<Z>>>::Result::as_usize(), 1);
    assert_eq!(<S<S<S<S<Z>>>> as NatSub<S<S<Z>>>>::Result::as_usize(), 2);
}


#[test]
fn peano_mul() {
    assert_eq!(<Z as NatMul<Z>>::Result::as_usize(), 0);
    assert_eq!(<Z as NatMul<S<Z>>>::Result::as_usize(), 0);
    assert_eq!(<S<Z> as NatMul<Z>>::Result::as_usize(), 0);
    assert_eq!(<S<Z> as NatMul<S<Z>>>::Result::as_usize(), 1);
    assert_eq!(<S<Z> as NatMul<S<S<Z>>>>::Result::as_usize(), 2);
    assert_eq!(<S<S<Z>> as NatMul<S<Z>>>::Result::as_usize(), 2);
    assert_eq!(<S<S<Z>> as NatMul<S<S<Z>>>>::Result::as_usize(), 4);
    assert_eq!(<S<S<S<S<Z>>>> as NatMul<S<S<S<Z>>>>>::Result::as_usize(), 12);
}
