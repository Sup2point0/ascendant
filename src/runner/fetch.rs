use std::collections::HashMap;

use anyhow as ah;
use itertools::*;
use tokio as tk;

use crate::*;


/// Fetch puzzles of size `N` and `difficulty`, load existing puzzles, merge the data and save to JSON.
#[tk::main]
pub async fn fetch_load_save<const N: usize>(difficulty: Difficulty) -> ah::Result<()>
    where [(); N+2]:
{
    let urls = Fetcher::get_puzzle_urls::<N>(difficulty);
    let mut grids_fetched = Fetcher::fetch::<N>(urls).await?;

    let mut grids = Loader::load_grids()?;
    grids.append(&mut grids_fetched);

    let grids_grouped = grids.into_iter().into_group_map_by(Grid::size);

    let grids_data = grids_grouped.into_iter().
        map(|(size, grids)|
            (size, grids.into_iter().map(GridExchange::from).collect_vec())
        )
        .collect::<HashMap<usize, Vec<GridExchange>>>();

    Saver::save(grids_data)?;

    Ok(())
}
