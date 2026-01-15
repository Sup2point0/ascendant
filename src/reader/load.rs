use std::*;
use std::collections::HashMap;

use anyhow as ah;
use serde_json as json;
use serde_json::value::Value;

use crate::*;


pub struct Loader;

impl Loader
{
    pub fn load_grids<const N: usize>() -> ah::Result<Vec<Grid<N>>>
    {
        let mut grids_data = Self::load_grids_data()?;
        let grids = grids_data.remove_entry(&N)
            .ok_or(ah::anyhow!("Failed to find grids of size {N}"))?.1;

        let out = grids.into_iter()
            .map(|exchange| exchange.into())
            .collect();

        Ok(out)
    }

    pub fn load_grids_data() -> ah::Result<HashMap<usize, Vec<GridExchange>>>
    {
        let file = fs::File::open(DATA_ROUTE)?;
        let reader = io::BufReader::new(file);
        let data = json::from_reader(reader)?;

        if let Value::Object(sizes) = data {
            Ok(sizes.into_iter()
                .map(|(size, grids)|
                    (
                        size.parse::<usize>()
                            .expect("Failed to parse grid size"),
                        json::from_value::<Vec<GridExchange>>(grids)
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
