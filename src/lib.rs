//! Abstractions over Project Euler problems

#![allow(dead_code)]
#![feature(iterator_step_by)]

extern crate kb;
extern crate num;

pub use code::binomial as bi;
pub use code::collatz as collatz;
pub use code::fibonacci as fib;
pub use code::matrix as mat;
pub use code::prime as prime;
pub use code::triangle as tri;
pub use code::util as ut;

mod code;
mod problems;
