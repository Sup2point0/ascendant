#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#![allow(dead_code)]
#![allow(unused_imports)]

use anyhow as ah;
use itertools::Itertools;
use tokio as tk;

use ascendant::*;

mod test; use test::*;


fn main()
{
    // let res = solve();
    let res = fetch::<5>();
    // let res = try_solve_stored();

    match res {
        Ok(..) => println!(">> done!"),
        Err(e) => println!("{:?}", e),
    }
}

fn solve()
{
    // let grids = [
    //     // examples::grid_4x4_full_1(),  //
    //     // examples::grid_4x4_sparse_1(),  //
    //     // examples::grid_4x4_sparse_2(),  //

    //     // examples::grid_5x5_full_easy_1(),  //
    //     // examples::grid_5x5_full_hard_1(),  //
    //     // examples::grid_5x5_sparse_1(),  //

    //     // examples::grid_6x6_full_easy_1(),  //
    //     // examples::grid_6x6_full_hard_1(),  //
    //     // examples::grid_6x6_sparse_1(),

    //     // examples::grid_7x7_full_easy_1(),  //
    //     examples::grid_7x7_full_hard_1(),  //

    //     // examples::grid_8x8_full_easy_1(),

    //     // examples::grid_9x9_full_1(),
    // ];

    let grids = Loader::load_grids::<5>().unwrap();

    for (i, grid) in grids.into_iter().enumerate() {
        println!("solving grid #{} from {}", i+1, grid.url.clone().unwrap());
        Solver::solve(grid);
    }
}

#[tk::main]
async fn fetch<const N: usize>() -> ah::Result<()>
    where [(); N+2]:
{
    let urls = Fetcher::get_puzzle_urls::<N>(Difficulty::Sparse);
    let grids = Fetcher::fetch::<N>(urls).await?;

    // for (url, grid) in grids {
    //     println!("url = {:?}", url);
    //     println!("{:?}", grid);
    // }

    let grids_data = grids.into_iter().map(GridExchange::from);
    let grids_data = grids_data.into_group_map_by(|grid| grid.clues.upper.len());
    Saver::save(grids_data)?;

    Ok(())
}
