use lev::array::container::*;
use lev::typehack::binary::*;


#[test]
fn static_array_from_elems() {
    let mut xs = Array::from_elem(dims![B3, B3, B3], &0);

    xs[[1, 0, 2]] = 1;

    assert_eq!(xs[[1, 0, 2]], 1);
}


#[test]
fn static_array_from_fn() {
    let xs = Array::from_fn(dims![B3, B3, B3], |idx| {
        if idx[0] == 1 && idx[1] == 0 && idx[2] == 2 {
            1
        } else {
            0
        }
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


#[test]
fn dynamic_array_from_fn() {
    let xs = Array::from_fn(dims![(3), (3), (3)], |idx| {
        if idx[0] == 1 && idx[1] == 0 && idx[2] == 2 {
            1
        } else {
            0
        }
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


#[test]
fn hybrid_array_from_fn() {
    let xs = Array::from_fn(dims![(3), B3, (3)], |idx| {
        if idx[0] == 1 && idx[1] == 0 && idx[2] == 2 {
            1
        } else {
            0
        }
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
