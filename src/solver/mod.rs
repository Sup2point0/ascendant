use std::*;
use std::collections::HashSet;

use crate::*;


pub struct Solver<const N: usize>;

impl<const N: usize> Solver<N>
{
    pub fn solve(mut grid: Grid<N>) -> Grid<N>
    {
        grid = Self::deduce_one_pass(grid);

        grid
    }

    fn deduce_one_pass(mut grid: Grid<N>) -> Grid<N>
    {
        for x in 0..N {
            let (clue, col) = grid.look_down(x);

            for (i, cell) in col.into_iter().enumerate() {
                let max = (N + i) as Digit - (clue.unwrap_or(1) - 1);
                let cands: HashSet<Digit> = (1..=max).collect();

                match cell {
                    Cell::Solved{..} => continue,
                    Cell::Pencil(digits) => {
                        if let Some(ds) = digits.take() {
                            *digits = Some(ds.intersection(&cands).copied().collect());
                        }
                    },
                }
            }
        }

        grid
    }
}
