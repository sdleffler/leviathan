extern crate leviathan as lev;


use lev::typehack::peano::*;


fn main() {
    let _ = <Z as NatSub<S<Z>>>::Result::as_usize(); //~ERROR the trait bound `lev::typehack::peano::Z: lev::typehack::peano::NatSub<lev::typehack::peano::S<lev::typehack::peano::Z>>` is not satisfied [E0277]
}
