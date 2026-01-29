use std::*;

use crate::*;
use crate::cli::detail::OutputDetail;


/// Algorithm for solving puzzles. Call `::solve()` and pass in a puzzle to attempt solving it as far as possible.
pub struct Solver<const N: usize>;

impl<const N: usize> Solver<N>
{
    /// Perform deductions on a puzzle until no further deductions can be made.
    pub fn solve(mut grid: Grid<N>) -> Grid<N>
    {
        if let Some(ref url) = grid.url {
            debug! { OutputDetail::SHOW_PASSES => "\nsolving grid from {url}" };
        }
        
        let mut did_deduce;
        let mut use_special = false;

        for i in 0.. {
            debug! { OutputDetail::SHOW_PASSES => "pass #{i}:\n{grid:?}" };

            (grid, did_deduce) = Self::deduce_one_pass(grid, use_special);

            /* If we've failed to make further deductions, try applying more specialised 'endgame' deductions to see if we can progress any further. No need to spend extra time applying these at the start as they are unlikely to contribute much. */
            if !did_deduce {
                if !use_special {
                    use_special = true;
                    debug!(".. enabled special deductions.");
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

        for x in 0..N { did_deduce |= Self::deduce_cells_in_lane(grid.look_down_mut(x)); }
        for x in 0..N { did_deduce |= Self::deduce_cells_in_lane(grid.look_up_mut(x)); }
        for y in 0..N { did_deduce |= Self::deduce_cells_in_lane(grid.look_right_mut(y)); }
        for y in 0..N { did_deduce |= Self::deduce_cells_in_lane(grid.look_left_mut(y)); }
        debug!("post-deduce:\n{grid:?}");

        did_deduce |= Self::deduce_all_sudoku_style(&mut grid);

        for x in 0..N { did_deduce |= Self::deduce_sequence_in_lane(grid.look_down_mut(x)) }
        for x in 0..N { did_deduce |= Self::deduce_sequence_in_lane(grid.look_up_mut(x)) }
        debug!("post-seq-up-down:\n{grid:?}");

        did_deduce |= Self::pinpoint_all_in_grid(&mut grid);

        for y in 0..N { did_deduce |= Self::deduce_sequence_in_lane(grid.look_right_mut(y)) }
        for y in 0..N { did_deduce |= Self::deduce_sequence_in_lane(grid.look_left_mut(y)) }
        debug!("post-seq-left-right:\n{grid:?}");

        did_deduce |= Self::pinpoint_all_in_grid(&mut grid);

        if use_special_deductions {
            did_deduce |= Self::isolate_all_in_grid(&mut grid);
            did_deduce |= Self::pinpoint_all_in_grid(&mut grid);

            did_deduce |= Self::pick_close_in_grid(&mut grid);
            did_deduce |= Self::pinpoint_all_in_grid(&mut grid);

            did_deduce |= Self::pick_last_in_grid(&mut grid);
            did_deduce |= Self::pinpoint_all_in_grid(&mut grid);
        }
        
        (grid, did_deduce)
    }
}
