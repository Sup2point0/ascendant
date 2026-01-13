use std::*;

use anyhow as ah;
use serde_json as json;
use serde_json::value::Value;

use crate::*;


pub struct Loader;

impl Loader
{
    pub fn load_grids<const N: usize>() -> ah::Result<Vec<Grid<N>>>
    {
        let grids_data = Self::load_grids_data::<N>()?;

        let out = grids_data.into_iter()
            .map(|exchange| exchange.into())
            .collect();

        Ok(out)
    }

    pub fn load_grids_data<const N: usize>() -> ah::Result<Vec<GridExchange>>
    {
        let file = fs::File::open(DATA_ROUTE)?;
        let reader = io::BufReader::new(file);
        let data = json::from_reader(reader)?;

        if let Value::Object(sizes) = data {
            for (size, grids) in sizes {
                if size == N.to_string() {
                    return Ok(json::from_value(grids)?);
                }
            }
            Err(ah::anyhow!("Failed to find grids of size {}", N))
        }
        else {
            Err(ah::anyhow!("Failed to parse JSON, might be malformed"))
        }
    }
}
