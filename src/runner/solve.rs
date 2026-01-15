use anyhow as ah;

use crate::*;


pub fn try_solve_stored() -> ah::Result<()>
{
    seq_macro::seq!(N in 4..=9 {
        if let Ok(difficulties) = Loader::load_grids::<N>() {
            for (diff, grids) in difficulties {
                let total = grids.len();
                
                match try_solve_all::<N>(grids) {
                    Ok(solved)
                        => println!(".. {}x{} -- {} solved {solved}/{total}", N, N, diff.to_string()),
                    Err(e)
                        => println!("{e:?}"),
                }
            }
        }
    });

    Ok(())
}


pub fn try_solve_all<const N: usize>(puzzles: Vec<Grid<N>>) -> ah::Result<u32>
{
    let mut solved = 0;
    let t = puzzles.len();

    for (i, puzzle) in puzzles.into_iter().enumerate() {
        let grid = Solver::solve(puzzle);

        if grid.is_solved() {
            solved += 1;
        }
        if i % 10 == 0 {
            println!(".. attempted {i} of {t}");
        }
    }

    Ok(solved)
}
