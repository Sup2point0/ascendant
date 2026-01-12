#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod puzzle;  pub use puzzle::*;
mod solver;  pub use solver::*;
mod fetcher; pub use fetcher::*;

pub mod examples;
pub mod util;


pub type Digit = usize;
