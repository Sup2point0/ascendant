use std::collections::HashMap;

use anyhow as ah;
use itertools::*;
use tokio as tk;

use crate::*;


#[tk::main]
pub async fn fetch_and_save<const N: usize>() -> ah::Result<()>
    where [(); N+2]:
{
    let urls = Fetcher::get_puzzle_urls::<N>(Difficulty::Sparse);
    let grids = Fetcher::fetch::<N>(urls).await?;

    let grids_grouped = grids.into_iter().into_group_map_by(Grid::size);

    let grids_data = grids_grouped.into_iter().
        map(|(size, grids)|
            (size, grids.into_iter().map(GridExchange::from).collect_vec())
        )
        .collect::<HashMap<usize, Vec<GridExchange>>>();

    Saver::save(grids_data)?;

    Ok(())
}
