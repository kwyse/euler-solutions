//! Abstractions over Project Euler problems

#![feature(step_by)]

pub use code::binomial as bi;
pub use code::collatz as collatz;
pub use code::fibonacci as fib;
pub use code::matrix as mat;
pub use code::prime as prime;
pub use code::triangle as tri;
pub use code::util as ut;

mod p001_010;
mod p011_020;
mod p013;
mod p014;
mod code;
mod problems;
