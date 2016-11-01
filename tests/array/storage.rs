use lev::array::storage::*;


#[allow(unused_variables)]
#[test]
fn storage_init() {
    let z: Storage<usize, _> = storage![];
    let sz = storage![0usize];
    let ssz = storage![0usize, 1usize];
}


#[test]
fn storage_index() {
    let storage = storage![0, 1, 2, 3, 4, 5, 6, 7];

    for i in 0..8 {
        assert_eq!(storage[i], i);
    }
}


#[test]
#[should_panic]
fn storage_index_oob() {
    let storage = storage![0, 1, 2, 3, 4, 5, 6, 7];
    storage[9];
}
