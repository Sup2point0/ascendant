#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod puzzle;
pub use puzzle::*;

mod solver;
pub use solver::*;

pub mod utils;


pub type Digit = u8;
