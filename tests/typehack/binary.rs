use lev::typehack::binary::*;


#[test]
fn binary_as_usize() {
    assert_eq!(O::<()>::as_usize(), 0);
    assert_eq!(I::<()>::as_usize(), 1);
    assert_eq!(O::<I>::as_usize(), 2);
}


#[test]
fn binary_add() {
    assert_eq!(<O as NatAdd<O>>::Result::as_usize(), 0);
    assert_eq!(<O as NatAdd<I>>::Result::as_usize(), 1);
    assert_eq!(<I as NatAdd<O>>::Result::as_usize(), 1);
    assert_eq!(<I as NatAdd<I>>::Result::as_usize(), 2);
    assert_eq!(<I<I> as NatAdd<O<I>>>::Result::as_usize(), 5);
}


// #[test]
// fn binary_sub() {
//     assert_eq!(<Z as NatSub<Z>>::Result::as_usize(), 0);
//     assert_eq!(<S<Z> as NatSub<Z>>::Result::as_usize(), 1);
//     assert_eq!(<S<Z> as NatSub<S<Z>>>::Result::as_usize(), 0);
//     assert_eq!(<S<S<Z>> as NatSub<S<Z>>>::Result::as_usize(), 1);
//     assert_eq!(<S<S<S<S<Z>>>> as NatSub<S<S<Z>>>>::Result::as_usize(), 2);
// }
//
//
// #[test]
// fn binary_mul() {
//     assert_eq!(<Z as NatMul<Z>>::Result::as_usize(), 0);
//     assert_eq!(<Z as NatMul<S<Z>>>::Result::as_usize(), 0);
//     assert_eq!(<S<Z> as NatMul<Z>>::Result::as_usize(), 0);
//     assert_eq!(<S<Z> as NatMul<S<Z>>>::Result::as_usize(), 1);
//     assert_eq!(<S<Z> as NatMul<S<S<Z>>>>::Result::as_usize(), 2);
//     assert_eq!(<S<S<Z>> as NatMul<S<Z>>>::Result::as_usize(), 2);
//     assert_eq!(<S<S<Z>> as NatMul<S<S<Z>>>>::Result::as_usize(), 4);
//     assert_eq!(<S<S<S<S<Z>>>> as NatMul<S<S<S<Z>>>>>::Result::as_usize(), 12);
// }
