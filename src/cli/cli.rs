use anyhow as ah;
use clap;
use rand::Rng;

use crate::*;
use crate::cli::detail::*;


pub static mut OUTPUT_DETAIL: OutputDetail = OutputDetail::DEFAULT;


#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli
{
    #[command(subcommand)]
    pub mode: Mode,

    #[clap(
        long, global = true,
        help = "Show steps when solving puzzles?"  // passes
    )]
    pub show_steps: bool,

    #[clap(
        short = 'd', long = "debug", global = true,
        help = "Show all sub-steps when solving puzzles?"  // steps
    )]
    pub debug: bool,
}

#[derive(Clone, Debug, clap::Subcommand)]
pub enum Mode
{
    #[command(about = "Solve a single puzzle")]
    SolveOne
    {
        #[arg(
            help = "Size of the puzzle to solve"
        )]
        size: usize,

        #[arg(long,
            value_parser = Cli::try_read_diff,
            help = "Difficulty of the puzzle to solve (1/2/3)"
        )]
        diff: Option<Difficulty>,

        #[arg(long,
            help = "Date of the puzzle in `mmdd` format (e.g. `0420` for April 20)"
        )]
        date: Option<String>,

        #[arg(short = 'r', long, action,
            help = "Pick a random puzzle of the given size and difficulty"
        )]
        random_date: bool,
    },

    #[command(about = "Solve all stored puzzles")]
    SolveAll
    {
        #[arg(long,
            num_args = 1..,
            help = "Sizes of puzzles to solve"
        )]
        sizes: Option<Vec<usize>>,

        #[arg(long,
            num_args = 1..=3,
            value_parser = Cli::try_read_diff,
            help = "Difficulty of the puzzles to solve (1/2/3)"
        )]
        diffs: Option<Vec<Difficulty>>,

        #[clap(long,
            global = true,
            help = "Show failed puzzle solution attempts?"
        )]
        show_fail: bool,
    },

    #[command(about = "Fetch puzzles from brainbashers.com")]
    Fetch
    {
        #[arg(long,
            num_args = 1..,
            help = "Sizes of puzzles to fetch"
        )]
        sizes: Option<Vec<usize>>,

        #[arg(long,
            num_args = 1..=3,
            value_parser = Cli::try_read_diff,
            help = "Difficulties of puzzles to fetch (1, 2 or 3)"
        )]
        diffs: Option<Vec<Difficulty>>,
    }
}

impl Cli
{
    pub fn exec(self)
    {
        println!(">> Running ascendant...");

        // SAFETY: This is not multithreaded, and is only for logging anyway.
        unsafe {
            if      self.debug      { OUTPUT_DETAIL = OutputDetail::DEBUG_STEPS; }
            else if self.show_steps { OUTPUT_DETAIL = OutputDetail::SHOW_PASSES; }
        }

        let start = std::time::Instant::now();

        let res = match self.mode {
            Mode::SolveOne{..} => self.solve_one(),
            Mode::SolveAll{..} => self.solve_all(),
            Mode::Fetch{..}    => self.fetch(),
        };

        match res {
            Ok(..) => {
                let elapsed = (start.elapsed().as_millis() as f64 / 100.0).round() / 10.0;
                println!(">> finished in {elapsed} secs");
            },
            Err(e) => {
                println!("!! {e:?}");
            },
        }
    }

    fn solve_one(self) -> ah::Result<()>
    {
        let Mode::SolveOne { size, diff, date, random_date } = self.mode else { unreachable!() };

        let Some(diff) = diff
            else { Err(ah::anyhow!(
                "No puzzle difficulty specified - please pass in the difficulty of the puzzle via `--diff`"
            ))? };

        let date = if random_date {
            let mut rng = rand::rng();

            let (month, upper) = DATE_RANGES[rng.random_range(0..12)];
            let day = rng.random_range(0..=upper);
            format!("{:0>2}{:0>2}", month, day)
        }
        else {
            date.ok_or(
                ah::anyhow!(
                    "No puzzle date specified - please pass in the date of the puzzle via `--date`"
                )
            )?
        };

        seq_macro::seq!(N in 4..=9 {
            if size == N {
                // SAFETY: This is not multithreaded, and is only for logging anyway.
                unsafe { OUTPUT_DETAIL = OUTPUT_DETAIL.max(OutputDetail::SHOW_PASSES) }

                runner::try_solve_stored_single::<N>(diff, &date)?;
            }
        });

        Ok(())
    }

    fn solve_all(self) -> ah::Result<()>
    {
        let Mode::SolveAll { sizes, diffs, show_fail } = self.mode else { unreachable!() };

        let diffs = diffs.unwrap_or(Difficulty::all());

        if let Some(sizes) = sizes {
            // SAFETY: This is not multithreaded, and is only for logging anyway.
            unsafe {
                if show_fail {
                    OUTPUT_DETAIL = OUTPUT_DETAIL.max(OutputDetail::SHOW_FAIL);
                }
            }

            seq_macro::seq!(N in 4..=9 {
                if sizes.contains(&N) {
                    runner::try_solve_stored::<N>(diffs.clone())?;
                }
            });
        } else {
            runner::try_solve_stored_all()?;
        }

        Ok(())
    }

    fn fetch(self) -> ah::Result<()>
    {
        let Mode::Fetch { sizes, diffs } = self.mode else { unreachable!() };

        let diffs = diffs.unwrap_or_else(|| {
            println!("!! Warning: No difficulties specified, defaulting to fetching Sparse puzzles...");
            vec![Difficulty::Sparse]
        });

        if let Some(sizes) = sizes {
            seq_macro::seq!(N in 4..=9 {
                if sizes.contains(&N) {
                    for diff in &diffs {
                        runner::fetch_load_save::<N>(*diff)?;
                    }
                }
            });

            Ok(())
        }
        else {
            Err(ah::anyhow!(
                "No puzzle sizes specified - please pass 1 or more arguments indicating the size of the puzzles to fetch"
            ))
        }
    }
}

impl Cli
{
    fn try_read_diff(diff: &str) -> ah::Result<Difficulty>
    {
        diff.to_owned().try_into()
    }
}
