use std::*;

use anyhow as ah;
use chromiumoxide as cr2o3;
use chromiumoxide::browser::HeadlessMode;
use futures::StreamExt;
use itertools::*;
use tokio as tk;

use crate::*;


type Url = String;


pub struct Fetcher;

impl Fetcher
{
    pub fn get_puzzle_url<const N: usize>(
        diff: Difficulty,
        (month, day): (usize, usize),
    ) -> Url
    {
        format!(
            "https://www.brainbashers.com/showskyscrapers.asp?date={:0>2}{:0>2}&size={}&diff={}",
            month, day,
            N,
            diff.to_string()
        )
    }

    pub fn get_puzzle_urls<const N: usize>(diff: Difficulty) -> Vec<Url>
    {
        [
            (1, 31), (2, 28), (3, 31), (4, 30), (5, 31), (6, 30),
            (7, 31), (8, 31), (9, 30), (10, 31), (11, 30), (12, 31),
        ].into_iter()
        .flat_map(|(month, days)|
            (1..=days)
            .map(|d|
                Self::get_puzzle_url::<N>(diff, (month, d))
            )
            .collect_vec()
        ).collect()
    }

    pub async fn fetch<const N: usize>(urls: Vec<Url>) -> ah::Result<Vec<Grid<N>>>
        where [(); N+2]:
    {
        let mut out = vec![];

        println!(".. launching browser...");
        let (mut browser, mut handler) = cr2o3::Browser::launch(
            // cr2o3::BrowserConfig::builder().with_head().build().map_err(|e| ah::anyhow!(e))?
            cr2o3::BrowserConfig::builder().headless_mode(HeadlessMode::True).build().map_err(|e| ah::anyhow!(e))?
        ).await?;

        let handle = tk::spawn(async move {
            while let Some(_) = handler.next().await {}
        });

        for url in urls {
            println!(".. fetching {url}...");
            let page = browser.new_page(url.clone()).await?;

            let grid = page.find_element("table").await?;
            let rows = grid.find_elements("tr").await?;

            let mut digits: Vec<Vec<Digit>> = (0..N+2).map(|_| vec![]).collect();

            for (y, row) in rows.into_iter().enumerate() {
                let cells = row.find_elements("td").await?;

                for cell in cells {
                    digits[y].push(Self::extract_digit(cell).await?);
                }
            }

            let res = Grid::<N>::try_construct(digits, Some(url));
            out.push(res);
        }

        browser.close().await?;
        handle.await?;
        
        Ok(out)
    }

    async fn extract_digit(elem: cr2o3::Element) -> ah::Result<Digit>
    {
        let out: Digit;

        if let Ok(img) = elem.find_element("img").await {
            let alt = img.attribute("alt").await?
                .ok_or(ah::anyhow!("Failed to extract `alt` of <img>"))?;
            
            let text = alt.split_whitespace().skip(1).next()
                .ok_or(ah::anyhow!("Failed to extract digit from alt `{}`", alt))?;

            let digit = text.chars().next()
                .ok_or(ah::anyhow!("Digit extracted from `alt` of <img> was empty"))?;

            out = digit.to_digit(10)
                .ok_or(ah::anyhow!("Failed to convert char `{}` to digit", digit))?
            as Digit;
        }
        else if let Ok(input) = elem.find_element("input").await {
            let value = input.attribute("value").await?
                .ok_or(ah::anyhow!("Failed to extract value of <input>"))?;

            out = value.as_str().parse::<Digit>().unwrap_or(0);
        }
        else {
            out = 0;
        }

        Ok(out)
    }
}
