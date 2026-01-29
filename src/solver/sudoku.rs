use itertools::Itertools;
use natbitset::Bitset;

use crate::*;


/// Deductions made by applying the rules of Sudoku.
impl<const N: usize> Solver<N>
{
    // TEMP until we implement `Bitset::is_subset`
    fn is_subset(subset: Bitset<N>, superset: Bitset<N>) -> bool
    {
        *(subset / superset) == 0
    }

    /// Given a collection of sets, find the combinations of sets which between them are guaranteed to consume all the digits in their union.
    pub fn find_isolated_groups<I>(sets: I) -> Vec<Bitset<N>>
        where I: IntoIterator<Item = Bitset<N>> + Clone
    {
        let combinations = (2..=N).flat_map(|n|
            sets.clone().into_iter()
                .combinations(n)
        );

        let groups = combinations
            .filter_map(|sets| {
                let union = sets.iter().fold(
                    Bitset::none(),
                    |p, q| (p | *q)
                );
                let is_isolated = sets.len() == union.len();
                is_isolated.then_some(union)
            });
        
        groups.collect()
    }

    /// Applies `isolate_groups_in_lane` to all lanes in the grid.
    /// 
    /// (See that method for how this works.)
    pub fn isolate_all_in_grid(grid: &mut Grid<N>) -> bool
    {
        let mut did_deduce = false;

        for x in 0..N { did_deduce |= Self::isolate_groups_in_lane(grid.look_across_row_mut(x).1) }
        for y in 0..N { did_deduce |= Self::isolate_groups_in_lane(grid.look_across_col_mut(y).1) }
        debug!("post-isolate:\n{grid:?}");

        did_deduce
    }

    /// Find isolated groups in `lane` and eliminate their candidates from other `Cell::Pencil`s.
    /// 
    /// An “isolated” group is a set of `Cell::Pencil`s which, between them, are *guaranteed* to consume the candidates they contain. For instance, if we have two `Cell::Pencil(Bitset(1, 2))`, we know one must contain the `1`, and the other must contain the `2`.
    /// 
    /// We don't know which way round (yet), but we can still use this information - it means that `1` and `2` cannot go anywhere else in the lane. Hence we can eliminate `1` and `2` as candidates from all other `Cell::Pencil`s in the lane.
    pub fn isolate_groups_in_lane(mut lane: [&mut Cell<N>; N]) -> bool
    {
        let mut did_deduce = false;

        let marks = lane.iter()
            .filter_map(|cell| {
                if let Cell::Pencil(digits) = cell {
                    Some(*digits)
                } else {
                    None
                }
            })
            .collect_vec();

        let groups = Self::find_isolated_groups(marks);

        for group in groups {
            for cell in &mut lane {
                let Cell::Pencil(marks) = cell else { continue; };

                if !Self::is_subset(*marks, group) {
                    let before = *marks;
                    *marks /= group;
                    
                    did_deduce |= (*marks != before);
                }
            }
        }

        did_deduce
    }

    /// Apply `deduce_one_cell_sudoku_style` to all cells in the grid.
    /// 
    /// (See that method for how this works.)
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
                did_deduce |= digits.has(d);
                *digits -= d;

                if digits.is_empty() {
                    panic!("Deleted all candidates while performing Sudoku deductions!");
                }
            }
        }

        did_deduce
    }

    /// Apply `pinpoint_cells_in_lane` to all lanes in the grid.
    /// 
    /// (See that method for how this works.)
    /// 
    /// We call this very often, because it cleans up the output of other deductions and significantly speeds up solving.
    pub fn pinpoint_all_in_grid(grid: &mut Grid<N>) -> bool
    {
        let mut did_deduce = false;

        for x in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_across_row_mut(x).1) }
        for y in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_across_col_mut(y).1) }
        debug!("post-pinpoint:\n{grid:?}");

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
