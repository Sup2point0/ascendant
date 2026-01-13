#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![allow(dead_code)]

use anyhow as ah;
use tokio as tk;

use ascendant::*;


fn main()
{
    fetch().unwrap();
}

fn solve()
{
    let grids = [
        // examples::grid_4x4_full_1(),  //
        // examples::grid_4x4_sparse_1(),  //
        // examples::grid_4x4_sparse_2(),  //

        // examples::grid_5x5_full_easy_1(),  //
        // examples::grid_5x5_full_hard_1(),  //
        // examples::grid_5x5_sparse_1(),  //

        // examples::grid_6x6_full_easy_1(),  //
        // examples::grid_6x6_full_hard_1(),  //
        // examples::grid_6x6_sparse_1(),

        // examples::grid_7x7_full_easy_1(),  //
        examples::grid_7x7_full_hard_1(),  //

        // examples::grid_8x8_full_easy_1(),

        // examples::grid_9x9_full_1(),
    ];

    for (i, grid) in grids.into_iter().enumerate() {
        println!("solving grid #{}", i+1);
        Solver::solve(grid);
    }
}

#[tk::main]
async fn fetch() -> ah::Result<()>
{
    let urls = Fetcher::get_puzzle_urls::<5>(Difficulty::Sparse);
    let grids = Fetcher::fetch::<5>(urls).await?;

    for (url, grid) in grids {
        println!("url = {:?}", url);
        println!("{:?}", grid);
    }

    Ok(())
}
