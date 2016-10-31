extern crate leviathan as lev;


use lev::peano::*;


fn main() {
    let _ = <Z as NatSub<S<Z>>>::Result::as_usize(); //~ERROR the trait bound `lev::peano::Z: lev::peano::NatSub<lev::peano::S<lev::peano::Z>>` is not satisfied [E0277]
}
