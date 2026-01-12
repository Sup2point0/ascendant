use std::*;

use anyhow as ah;
use chromiumoxide as cr2o3;
use futures::StreamExt;
use tokio as tk;

use crate::*;


type Url = String;


#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Difficulty {
    Full,
    Sparse,
}

impl ToString for Difficulty
{
    fn to_string(&self) -> String {
        match self {
            Self::Full => "3",
            Self::Sparse => "2",
        }.to_string()
    }
}


pub struct Fetcher<const N: usize>;

impl<const N: usize> Fetcher<N> where [(); N+2]:
{
    pub fn get_puzzle_url(diff: Difficulty, (month, day): (usize, usize)) -> Url
    {
        format!(
            "https://www.brainbashers.com/showskyscrapers.asp?date={:0>2}{:0>2}&size={}&diff={}",
            month,
            day,
            N,
            diff.to_string()
        )
    }

    pub fn get_puzzle_urls(diff: Difficulty) -> Vec<Url>
    {
        [
            (1, 31),
            // (1, 31), (2, 28), (3, 31), (4, 30), (5, 31), (6, 30),
            // (7, 31), (8, 31), (9, 30), (10, 31), (11, 30), (12, 31),
        ].into_iter().map(
            |(month, day)| Self::get_puzzle_url(diff, (month, day))
        ).collect()
    }

    pub async fn fetch(urls: Vec<Url>) -> ah::Result<Vec<(Url, Grid<N>)>>
    {
        let mut out = vec![];

        let (mut browser, mut handler) = cr2o3::Browser::launch(
            cr2o3::BrowserConfig::builder().with_head().build().map_err(|e| ah::anyhow!(e))?
        ).await?;

        let handle = tk::spawn(async move {
            while let Some(_) = handler.next().await {}
        });

        for url in urls {
            let page = browser.new_page(url.clone()).await?;

            let grid = page.find_element("table").await?;
            let rows = grid.find_elements("tr").await?;

            let mut digits = [[0; N+2]; N+2];

            for (y, row) in rows.into_iter().enumerate() {
                let cells = row.find_elements("td").await?;

                for (x, cell) in cells.into_iter().enumerate() {
                    digits[y][x] = Self::extract_digit(cell).await?;
                }
            }

            out.push((url, Grid::<N>::construct(digits)));
        }

        browser.close().await?;
        handle.await?;
        
        Ok(out)
    }

    async fn extract_digit(elem: cr2o3::Element) -> ah::Result<Digit>
    {
        let out: Digit;

        if let Ok(img) = elem.find_element("img").await {
            let id = img.attribute("id").await?
                .ok_or(ah::anyhow!("Failed to extract id of <img>"))?;
            
            let digit = id.chars().last()
                .ok_or(ah::anyhow!("Failed to extract digit from id `{}`", id))?;

            out = digit.to_digit(10)
                .ok_or(ah::anyhow!("Failed to convert char `{}` to digit", digit))?
            as Digit;
        }
        else if let Ok(input) = elem.find_element("input").await {
            let value = input.attribute("value").await?
                .ok_or(ah::anyhow!("Failed to extract value of <input>"))?;

            out = value.as_str().parse::<Digit>()?;
        }
        else {
            out = 0;
        }

        Ok(out)
    }
}
