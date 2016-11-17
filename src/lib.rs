#![feature(specialization)]

#[macro_use]
extern crate log;

extern crate exact_size_iterator_traits as iter_exact;
extern crate unreachable;
extern crate void;

#[macro_use]
pub mod typehack;

#[macro_use]
pub mod array;

#[macro_use]
pub mod linalg;

#[macro_use]
pub mod geometry;

pub mod num;
