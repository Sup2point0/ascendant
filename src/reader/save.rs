use std::*;

use anyhow as ah;
use serde_json as json;

use crate::*;


pub struct Saver;

impl Saver
{
    pub fn save(data: impl serde::Serialize) -> ah::Result<()>
    {
        let text = json::to_string_pretty(&data)?;
        fs::write(DATA_ROUTE, text)?;

        Ok(())
    }
}
