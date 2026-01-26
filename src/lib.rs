#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(never_type)]

#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

mod puzzle;  pub use puzzle::*;
mod solver;  pub use solver::*;
mod fetcher; pub use fetcher::*;
mod reader;  pub use reader::*;

pub mod cli;
pub mod runner;

pub mod util;
pub use util::{ MapValues };


pub type Digit = usize;
