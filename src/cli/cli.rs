use anyhow as ah;
use clap;

use crate::*;


#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli
{
    #[command(subcommand)]
    pub mode: Mode,

    #[clap(
        short = 'd', long = "debug", global = true,
        help = "Enable debug output?"
    )]
    pub debug: bool,

    #[clap(
        long, global = true,
        help = "Show failed puzzle solution attempts?"
    )]
    pub show_fail: bool,
}

#[derive(Clone, Debug, clap::Subcommand)]
pub enum Mode
{
    #[command(about = "Solve a single puzzle, showing all steps")]
    SOLVE
    {
        #[arg(
            help = "Size of the puzzle to solve"
        )]
        size: usize,

        #[arg(long,
            value_parser = Cli::try_read_diff,
            help = "Difficulty of the puzzle to solve (1, 2 or 3)"
        )]
        diff: Option<Difficulty>,

        #[arg(long,
            help = "Date of the puzzle in the format `mmdd` (e.g. `0420` for April 20)"
        )]
        date: Option<String>,
    },

    #[command(about = "Solve all stored puzzles of a given size(s), showing overall performance")]
    SOLVE_ALL
    {
        #[arg(
            help = "Sizes of puzzles to solve"
        )]
        sizes: Option<Vec<usize>>,
    },

    #[command(about = "Fetch puzzles of a given size from brainbashers.com")]
    FETCH
    {
        #[arg(
            help = "Sizes of puzzles to fetch"
        )]
        sizes: Option<Vec<usize>>,

        #[arg(long,
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
        let start = std::time::Instant::now();

        let res = match self.mode {
            Mode::SOLVE{..}     => self.solve(),
            Mode::SOLVE_ALL{..} => self.solve_all(),
            Mode::FETCH{..}     => self.fetch(),
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

    fn solve(self) -> ah::Result<()>
    {
        let Mode::SOLVE { size, diff, date } = self.mode else { unreachable!() };

        let Some(diff) = diff
            else { Err(ah::anyhow!(
                "No puzzle difficulty specified - please pass in the difficulty of the puzzle via `--diff`"
            ))? };

        let Some(date) = date
            else { Err(ah::anyhow!(
                "No puzzle date specified - please pass in the date of the puzzle via `--date`"
            ))? };

        seq_macro::seq!(N in 4..=9 {
            if size == N {
                runner::try_solve_stored_single::<N>(diff, &date)?;
            }
        });

        Ok(())
    }

    fn solve_all(self) -> ah::Result<()>
    {
        let Mode::SOLVE_ALL { sizes } = &self.mode else { unreachable!() };

        if let Some(sizes) = sizes {
            seq_macro::seq!(N in 4..=9 {
                if sizes.contains(&N) {
                    runner::try_solve_stored::<N>()?;
                }
            });
        } else {
            runner::try_solve_stored_all()?;
        }

        Ok(())
    }

    fn fetch(self) -> ah::Result<()>
    {
        let Mode::FETCH { sizes, diffs } = self.mode else { unreachable!() };

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
