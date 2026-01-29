use anyhow as ah;

use crate::*;
use crate::cli::detail::OutputDetail;


pub fn try_solve_stored_single<const N: usize>(difficulty: Difficulty, date: &str) -> ah::Result<()>
{
    let mut difficulties = Loader::load_grids::<N>()?;
    let diff = difficulty.to_string();

    let Some(puzzles) = difficulties.remove(&diff)
        else { return Err(ah::anyhow!("Failed to find puzzles of difficulty {diff}")) };

    let Some(puzzle) = (
        puzzles.into_iter()
            .find_map_maybe(|puzzle| puzzle.url.clone(), |url| url.contains(date))
    ) else {
        return Err(ah::anyhow!("Failed to find puzzles of difficulty {diff}"))
    };
    
    Solver::solve(puzzle);

    Ok(())
}


pub fn try_solve_stored_all() -> ah::Result<()>
{
    seq_macro::seq!(N in 4..=9 {
        try_solve_stored::<N>()?;
    });

    Ok(())
}


pub fn try_solve_stored<const N: usize>() -> ah::Result<()>
{
    let difficulties = Loader::load_grids::<N>()?;

    for (diff, grids) in difficulties {
        let total = grids.len();

        match try_solve_all::<N>(grids) {
            Ok(solved) => println!(
                ".. {n}x{n} -- difficulty {diff} -- solved {solved}/{total}",
                n = N,
            ),
            Err(e) => println!("{e:?}"),
        }
    }

    Ok(())
}


pub fn try_solve_all<const N: usize>(puzzles: Vec<Grid<N>>) -> ah::Result<u32>
{
    let mut solved = 0;
    let t = puzzles.len();
    let j = t / 4;

    for (i, puzzle) in puzzles.into_iter().enumerate() {
        let orig = puzzle.clone();
        let grid = Solver::solve(puzzle);

        if grid.is_solved() {
            solved += 1;
        }
        else {
            debug! {
                OutputDetail::SHOW_FAIL => {
                    if let Some(ref url) = orig.url {
                        println!("\nsolving puzzle from {url}");
                    }
                    println!("{orig:?}");
                    println!("{grid:?}");
                }
            };
        }

        if i % j == 0 && i > 0 {
            println!(".. attempted {i} of {t}");
        }
    }

    Ok(solved)
}
