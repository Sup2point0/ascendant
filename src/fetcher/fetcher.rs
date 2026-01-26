use std::*;
use std::sync::Arc;

use anyhow as ah;
use chromiumoxide as cr2o3;
use chromiumoxide::browser::HeadlessMode;
use futures::StreamExt;
use itertools::*;
use tokio as tk;
use workerpool as wk;

use crate::*;


type Url = String;


pub struct Fetcher;

impl Fetcher
{
    pub fn get_puzzle_url<const N: usize>(
        diff: Difficulty,
        (month, day): (usize, u8),
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
        DATE_RANGES.into_iter()
            .flat_map(|(month, days)|
                (1..=days)
                    .map(|d| Self::get_puzzle_url::<N>(diff, (month, d)))
                    .collect_vec()
            )
            .collect()
    }

    pub async fn fetch<const N: usize>(urls: Vec<Url>) -> ah::Result<Vec<Grid<N>>>
        where [(); N+2]:
    {
        println!(".. launching browser...");
        let (browser, mut handler) = cr2o3::Browser::launch(
            cr2o3::BrowserConfig::builder().headless_mode(HeadlessMode::True).build().map_err(|e| ah::anyhow!(e))?
        ).await?;

        let mut browser = Arc::new(browser);

        let handle = tk::spawn(async move {
            while let Some(_) = handler.next().await {}
        });

        let max_workers = thread::available_parallelism().unwrap().get();
        let jobs = urls.len();

        let pool = wk::Pool::<FetchProcess<N>>::new(max_workers / 2);
        let (tx, rx) = sync::mpsc::channel();

        for url in urls {
            pool.execute_to(tx.clone(), (browser.clone(), url));
        }

        let out = rx.iter()
            .take(jobs)
            .filter_map(|r| r.ok())
            .collect_vec();

        Arc::get_mut(&mut browser).unwrap().close().await?;
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

struct FetchProcess<const N: usize>;

impl<const N: usize> Default for FetchProcess<N>
{
    fn default() -> Self {
        Self
    }
}

impl<const N: usize> wk::Worker for FetchProcess<N>
    where [(); N+2]:
{
    type Input = (Arc<cr2o3::Browser>, String);
    type Output = ah::Result<Grid<N>>;

    #[tk::main]
    async fn execute(&mut self, (browser, url): Self::Input) -> Self::Output
    {
        println!(".. fetching {url}...");

        let page = browser.new_page(url.clone()).await?;

        let grid = page.find_element("table").await?;
        let rows = grid.find_elements("tr").await?;

        let mut digits: Vec<Vec<Digit>> = (0..N+2).map(|_| vec![]).collect();

        for (y, row) in rows.into_iter().enumerate() {
            let cells = row.find_elements("td").await?;

            for cell in cells {
                digits[y].push(Fetcher::extract_digit(cell).await?);
            }
        }

        let out = Grid::<N>::try_construct(digits, Some(url));

        Ok(out)
    }
}
