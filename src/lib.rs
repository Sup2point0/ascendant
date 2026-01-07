#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod puzzle;
pub use puzzle::*;

mod solver;
pub use solver::*;

pub mod util;


pub type Digit = u8;
