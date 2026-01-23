use natbitset::Bitset;

use crate::*;


/// Deductions made by applying the rules of Sudoku.
impl<const N: usize> Solver<N>
{
    /// Apply the rules of Sudoku to eliminate candidates from a `Cell::Pencil` at (`x`, `y`) of `grid`.
    pub fn deduce_one_cell_sudoku_style(mut grid: Grid<N>, x: usize, y: usize) -> (Grid<N>, bool)
    {
        let mut did_deduce = false;

        /* Would make this a little more structured, but then we end up in borrowing conflicts =( */
        if let Cell::Solved{..} = grid.at(x, y) {
            return (grid, did_deduce);
        }

        let mut seen = Bitset::<N>::none();

        for cell in grid.look_right_mut(y).1 {
            // TODO FIXME fix `+=` implementation for Bitset
            if let Cell::Solved(digit) = cell { seen |= Bitset::from([*digit]); }
        }
        for cell in grid.look_down_mut(x).1 {
            if let Cell::Solved(digit) = cell { seen |= Bitset::from([*digit]); }
        }

        if let Cell::Pencil(digits) = grid.at_mut(x, y) {
            for d in seen {
                // TODO FIXME add `.has()` method for Bitset
                did_deduce |= digits.members().contains(&d);
                // TODO FIXME fix `-=` implementation for Bitset
                *digits /= Bitset::from([d]);

                if digits.len() == 0 {
                    panic!("Deleted all candidates while performing Sudoku deductions!");
                }
            }
        }

        (grid, did_deduce)
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
            && let Some(d) = digits.single()
            {
                *cell = Cell::Solved(d);
                did_deduce = true;
            }
        }

        did_deduce
    }
}
