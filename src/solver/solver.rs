use std::*;

use crate::*;


/// Algorithm for solving puzzles. Call `::solve()` and pass in a puzzle to attempt solving it as far as possible.
pub struct Solver<const N: usize>;

impl<const N: usize> Solver<N>
{
    /// Perform deductions on a puzzle until no further deductions can be made.
    pub fn solve(mut grid: Grid<N>) -> Grid<N>
    {
        let mut did_deduce;
        let debug = util::args("debug") || util::args("DEBUG");

        if debug && let Some(ref url) = grid.url {
            println!("\nsolving grid from {url}");
        }

        loop {
            if debug {
                println!("\n{grid:?}\n");
            }

            (grid, did_deduce) = Self::deduce_one_pass(grid);
            if !did_deduce { break; }
        }

        grid
    }

    /// Perform one pass of deductions through the grid. Returns the updated grid and `true` if any deductions were successfully made.
    pub fn deduce_one_pass(mut grid: Grid<N>) -> (Grid<N>, bool)
    {
        let mut did_deduce = false;
        let debug = util::args("DEBUG");

        for x in 0..N { did_deduce |= Self::deduce_cells_in_lane(grid.look_down_mut(x)); }
        for x in 0..N { did_deduce |= Self::deduce_cells_in_lane(grid.look_up_mut(x)); }
        for y in 0..N { did_deduce |= Self::deduce_cells_in_lane(grid.look_right_mut(y)); }
        for y in 0..N { did_deduce |= Self::deduce_cells_in_lane(grid.look_left_mut(y)); }
        if debug { println!("post-deduce:\n{grid:?}"); }

        let mut deduced;
        for x in 0..N {
            for y in 0..N {
                (grid, deduced) = Self::deduce_one_cell_sudoku_style(grid, x, y);
                did_deduce |= deduced;
            }
        }

        for x in 0..N { did_deduce |= Self::deduce_sequence_in_lane(grid.look_down_mut(x)) }
        for x in 0..N { did_deduce |= Self::deduce_sequence_in_lane(grid.look_up_mut(x)) }
        if debug { println!("post-seq-up-down:\n{grid:?}"); }

        for x in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_down_mut(x).1) }
        for x in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_up_mut(x).1) }
        for y in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_right_mut(y).1) }
        for y in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_left_mut(y).1) }
        if debug { println!("post-pinpoint:\n{grid:?}"); }

        for y in 0..N { did_deduce |= Self::deduce_sequence_in_lane(grid.look_right_mut(y)) }
        for y in 0..N { did_deduce |= Self::deduce_sequence_in_lane(grid.look_left_mut(y)) }
        if debug { println!("post-seq-left-right:\n{grid:?}"); }

        for x in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_down_mut(x).1) }
        for x in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_up_mut(x).1) }
        for y in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_right_mut(y).1) }
        for y in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_left_mut(y).1) }
        if debug { println!("post-pinpoint:\n{grid:?}"); }
        
        (grid, did_deduce)
    }
}
