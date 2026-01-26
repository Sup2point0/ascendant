use std::*;

use crate::*;


/// Algorithm for solving puzzles. Call `::solve()` and pass in a puzzle to attempt solving it as far as possible.
pub struct Solver<const N: usize>;

impl<const N: usize> Solver<N>
{
    /// Perform deductions on a puzzle until no further deductions can be made.
    pub fn solve(mut grid: Grid<N>) -> Grid<N>
    {
        let debug = util::args("debug") || util::args("DEBUG");

        if debug && let Some(ref url) = grid.url {
            println!("\nsolving grid from {url}");
        }
        
        let mut did_deduce;
        let mut use_special = false;

        loop {
            if debug { println!("pass:\n{grid:?}"); }

            (grid, did_deduce) = Self::deduce_one_pass(grid, use_special);

            /* If we've failed to make further deductions, try applying more specialised 'endgame' deductions to see if we can progress any further. No need to spend extra time applying these at the start as they are unlikely to contribute much. */
            if !did_deduce {
                if !use_special {
                    use_special = true;
                } else {
                    break;
                }
            }
        }

        grid
    }

    /// Perform one pass of deductions through the grid. Returns the updated grid and `true` if any deductions were successfully made.
    pub fn deduce_one_pass(mut grid: Grid<N>, use_special_deductions: bool) -> (Grid<N>, bool)
    {
        let mut did_deduce = false;
        let debug = util::args("DEBUG");

        for x in 0..N { did_deduce |= Self::deduce_cells_in_lane(grid.look_down_mut(x)); }
        for x in 0..N { did_deduce |= Self::deduce_cells_in_lane(grid.look_up_mut(x)); }
        for y in 0..N { did_deduce |= Self::deduce_cells_in_lane(grid.look_right_mut(y)); }
        for y in 0..N { did_deduce |= Self::deduce_cells_in_lane(grid.look_left_mut(y)); }
        if debug { println!("post-deduce:\n{grid:?}"); }

        did_deduce |= Self::deduce_all_sudoku_style(&mut grid);

        for x in 0..N { did_deduce |= Self::deduce_sequence_in_lane(grid.look_down_mut(x)) }
        for x in 0..N { did_deduce |= Self::deduce_sequence_in_lane(grid.look_up_mut(x)) }
        if debug { println!("post-seq-up-down:\n{grid:?}"); }

        did_deduce |= Self::pinpoint_all_in_grid(&mut grid);

        for y in 0..N { did_deduce |= Self::deduce_sequence_in_lane(grid.look_right_mut(y)) }
        for y in 0..N { did_deduce |= Self::deduce_sequence_in_lane(grid.look_left_mut(y)) }
        if debug { println!("post-seq-left-right:\n{grid:?}"); }

        did_deduce |= Self::pinpoint_all_in_grid(&mut grid);

        if use_special_deductions {
            did_deduce |= Self::isolate_all_in_grid(&mut grid);
            did_deduce |= Self::pinpoint_all_in_grid(&mut grid);
        }
        
        (grid, did_deduce)
    }
}
