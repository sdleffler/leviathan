use lev::vector::*;
use lev::vector::traits::*;


#[test]
fn vec2_add() {
    let x = Vec2! { x: 1, y: 0 };
    let y = Vec2! { x: 0, y: 1 };
    assert_eq!(x.add(y), Vec2! { x: 1, y: 1 });
}
