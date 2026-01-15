use std::*;
use std::collections::HashMap;

use anyhow as ah;
use itertools::*;
use serde_json as json;
use serde_json::value::Value;

use crate::*;


type Difficulties<T> = HashMap<String, Vec<T>>;


pub struct Loader;

impl Loader
{
    /// Load `N`x`N` puzzles from JSON into `Grid` objects, grouped by difficulty.
    pub fn load_grids<const N: usize>() -> ah::Result<Difficulties<Grid<N>>>
    {
        let grids_data = Self::load_grids_data::<N>()?;

        for (diff, grids) in &grids_data {
            println!(
                ".. loaded {len} {N}x{N} puzzles of difficulty {diff}",
                len = grids.len()
            );
        }

        let out = grids_data.into_iter()
            .map(|(diff, grids)|
                (
                    diff,
                    grids.into_iter()
                        .map(|exchange| exchange.into())
                        .collect_vec()
                )
            )
            .collect();

        Ok(out)
    }

    /// Load `N`x`N` puzzles into JSON exchange objects, grouped by difficulty.
    fn load_grids_data<const N: usize>() -> ah::Result<Difficulties<GridExchange>>
    {
        let route = format!("{DATA_ROUTE}/{N}x{N}-puzzles.json");
        let file = fs::File::open(route)?;

        let reader = io::BufReader::new(file);
        let data = json::from_reader(reader)?;

        if let Value::Object(difficulties) = data {
            Ok(difficulties.into_iter()
                .map(|(diff, grids)|
                    (
                        diff,
                        json::from_value(grids)
                            .expect("Failed to load grid data")
                    )
                )
                .collect()
            )
        }
        else {
            Err(ah::anyhow!("Failed to parse JSON, might be malformed"))
        }
    }
}
