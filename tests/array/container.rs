use lev::array::container::*;
use lev::typehack::peano::*;


#[test]
fn static_array_from_elems() {
    let mut xs: StaticArray<_, Dims![P3, P3, P3]> = StaticArray::from_elem(&0);

    xs[[1, 0, 2]] = 1;

    assert_eq!(xs[[1, 0, 2]], 1);
}


#[test]
fn static_array_from_fn() {
    let xs = StaticArray::<_, Dims![P3, P3, P3]>::from_fn(|idx| {
        if idx[0] == 1 && idx[1] == 0 && idx[2] == 2 { 1 } else { 0 }
    });

    assert_eq!(xs[[0, 0, 0]], 0);
    assert_eq!(xs[[0, 0, 1]], 0);
    assert_eq!(xs[[0, 0, 2]], 0);
    assert_eq!(xs[[0, 1, 0]], 0);
    assert_eq!(xs[[0, 1, 1]], 0);
    assert_eq!(xs[[0, 1, 2]], 0);
    assert_eq!(xs[[0, 2, 0]], 0);
    assert_eq!(xs[[0, 2, 1]], 0);
    assert_eq!(xs[[0, 2, 2]], 0);
    assert_eq!(xs[[1, 0, 0]], 0);
    assert_eq!(xs[[1, 0, 1]], 0);
    assert_eq!(xs[[1, 0, 2]], 1);
    assert_eq!(xs[[1, 1, 0]], 0);
    assert_eq!(xs[[1, 1, 1]], 0);
    assert_eq!(xs[[1, 1, 2]], 0);
    assert_eq!(xs[[1, 2, 0]], 0);
    assert_eq!(xs[[1, 2, 1]], 0);
    assert_eq!(xs[[1, 2, 2]], 0);
    assert_eq!(xs[[2, 0, 0]], 0);
    assert_eq!(xs[[2, 0, 1]], 0);
    assert_eq!(xs[[2, 0, 2]], 0);
    assert_eq!(xs[[2, 1, 0]], 0);
    assert_eq!(xs[[2, 1, 1]], 0);
    assert_eq!(xs[[2, 1, 2]], 0);
    assert_eq!(xs[[2, 2, 0]], 0);
    assert_eq!(xs[[2, 2, 1]], 0);
    assert_eq!(xs[[2, 2, 2]], 0);
}
