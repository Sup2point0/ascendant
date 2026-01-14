use anyhow as ah;

use crate::*;


pub fn try_solve_stored() -> ah::Result<()>
{
    seq_macro::seq!(N in 4..=9 {
        if let Ok(grids) = Loader::load_grids::<N>() {
            let total = grids.len();
            let solved = try_solve_all::<N>(grids)?;

            println!("{}x{} -- solved {solved}/{total}", N, N);
        }
    });

    Ok(())
}


pub fn try_solve_all<const N: usize>(puzzles: Vec<Grid<N>>) -> ah::Result<u32>
{
    let mut solved = 0;

    for puzzle in puzzles {
        let grid = Solver::solve(puzzle);

        if grid.is_solved() {
            solved += 1;
        }
    }

    Ok(solved)
}
