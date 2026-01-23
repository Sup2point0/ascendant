#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use ascendant::*;

mod runner;


fn main()
{
    let start = std::time::Instant::now();

    // let res = runner::fetch_load_save::<6>(Difficulty::Full);

    // let res = runner::try_solve_stored_all();
    let res = runner::try_solve_stored::<5>();
    // let res = runner::try_solve_stored_single::<6>(Difficulty::Full, "0316");

    match res {
        Ok(..) => println!(">> finished in {} secs",
            (start.elapsed().as_millis() as f64 / 100.0).round() / 10.0
        ),
        Err(e) => println!("!! {:?}", e),
    }
}
