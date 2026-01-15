#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#![allow(dead_code)]
#![allow(unused_imports)]

use ascendant::*;

mod runner;


fn main()
{
    let res = runner::fetch_load_save::<5>(Difficulty::Full);
    // let res = runner::try_solve_stored();

    match res {
        Ok(..) => println!(">> done!"),
        Err(e) => println!("!! {:?}", e),
    }
}
