use natbitset::Bitset;

use crate::*;


/// Deductions made by applying the rules of Sudoku.
impl<const N: usize> Solver<N>
{
    /// Apply the rules of Sudoku to eliminate candidates from all `Cell::Pencil` in `grid`.
    pub fn deduce_all_sudoku_style(grid: &mut Grid<N>) -> bool
    {
        let mut did_deduce = false;

        for x in 0..N {
            for y in 0..N {
                did_deduce |= Self::deduce_one_cell_sudoku_style(grid, x, y);
            }
        }

        did_deduce
    }

    /// Apply the rules of Sudoku to eliminate candidates from a `Cell::Pencil` at (`x`, `y`) of `grid`.
    pub fn deduce_one_cell_sudoku_style(grid: &mut Grid<N>, x: usize, y: usize) -> bool
    {
        let mut did_deduce = false;

        /* Would like to make this a little more structured, but then we end up in borrowing conflicts =( */
        if let Cell::Solved(..) = grid.at(x, y) {
            return false;
        }

        let mut seen = Bitset::<N>::none();

        for cell in grid.look_right_mut(y).1 {
            if let Cell::Solved(digit) = cell { seen += *digit; }
        }
        for cell in grid.look_down_mut(x).1 {
            if let Cell::Solved(digit) = cell { seen += *digit; }
        }

        if let Cell::Pencil(digits) = grid.at_mut(x, y) {
            for d in seen {
                did_deduce |= digits.contains(d);
                *digits -= d;

                if digits.len() == 0 {
                    panic!("Deleted all candidates while performing Sudoku deductions!");
                }
            }
        }

        did_deduce
    }

    /// Find cells in `grid` that can be solved and turn them from `Cell::Pencil` to `Cell::Solved`.
    pub fn pinpoint_all_in_grid(grid: &mut Grid<N>) -> bool
    {
        let mut did_deduce = false;

        for x in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_down_mut(x).1) }
        for x in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_up_mut(x).1) }
        for y in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_right_mut(y).1) }
        for y in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_left_mut(y).1) }
        
        if util::args("DEBUG") {
            println!("post-pinpoint:\n{grid:?}");
        }

        did_deduce
    }

    /// Find cells in a lane that can be solved and turn them from `Cell::Pencil` to `Cell::Solved`.
    pub fn pinpoint_cells_in_lane(mut lane: [&mut Cell<N>; N]) -> bool
    {
        let mut did_deduce = false;

        for (digit, indices) in Grid::occurrences(&lane) {
            if indices.len() == 1 {
                let idx = indices.into_iter().next().unwrap();

                if let cell@Cell::Pencil(_) = &mut lane[idx] {
                    **cell = Cell::Solved(digit);
                    did_deduce = true;
                }
            }
        }

        for cell in lane {
            if let Cell::Pencil(digits) = cell
            && let Some(d) = digits.only()
            {
                *cell = Cell::Solved(d);
                did_deduce = true;
            }
        }

        did_deduce
    }
}
