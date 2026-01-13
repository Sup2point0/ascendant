use std::*;

use anyhow as ah;
use itertools::*;
use serde_json as json;

use crate::*;


pub struct Saver;

impl Saver
{
    pub fn save(grids: impl IntoIterator<Item = GridExchange>) -> ah::Result<()>
    {
        let data = grids.into_iter().collect_vec();
        let text = json::to_string_pretty(&data)?;
        fs::write("data/grids.json", text)?;

        Ok(())
    }
}
