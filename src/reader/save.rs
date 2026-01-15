use std::*;

use anyhow as ah;
use regex::Regex;
use serde_json as json;

use crate::*;


pub struct Saver;

impl Saver
{
    pub fn save<const N: usize>(data: impl serde::Serialize) -> ah::Result<()>
    {
        println!(".. saving data...");
        
        let text = json::to_string_pretty(&data)?;
        let re = Regex::new("\n {8,10}").unwrap();
        let text = re.replace_all(&text, "");

        let route = format!("{DATA_ROUTE}/{N}x{N}-puzzles.json");
        fs::write(route, text.into_owned())?;

        Ok(())
    }
}
