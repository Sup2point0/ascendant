use std::*;
use std::collections::HashSet;

use crate::*;


pub struct Solver<const N: usize>;

impl<const N: usize> Solver<N>
{
    pub fn solve(mut grid: Grid<N>) -> Grid<N>
    {
        let mut did_deduce;

        loop {
            // println!("starting with = \n{grid:?}\n");

            (grid, did_deduce) = Self::deduce_one_pass(grid);
            if !did_deduce { break; }
        }

        grid
    }

    /// Perform one pass of deductions through the grid. Returns the updated grid and a `bool` indicating if any deductions were successfully made.
    fn deduce_one_pass(mut grid: Grid<N>) -> (Grid<N>, bool)
    {
        let mut did_deduce = false;

        for x in 0..N { did_deduce |= Self::deduce_one_lane(grid.look_down(x)); }
        for x in 0..N { did_deduce |= Self::deduce_one_lane(grid.look_up(x)); }
        for y in 0..N { did_deduce |= Self::deduce_one_lane(grid.look_right(y)); }
        for y in 0..N { did_deduce |= Self::deduce_one_lane(grid.look_left(y)); }

        let mut deduced;

        for x in 0..N {
            for y in 0..N {
                (grid, deduced) = Self::deduce_one_cell_sudoku_style(grid, x, y);
                did_deduce |= deduced;
            }
        }

        println!("pre-pinpoint = \n{grid:?}\n");

        for x in 0..N { did_deduce |= Self::pinpoint_one_lane(grid.look_down(x).1) }
        for x in 0..N { did_deduce |= Self::pinpoint_one_lane(grid.look_up(x).1) }
        for y in 0..N { did_deduce |= Self::pinpoint_one_lane(grid.look_right(y).1) }
        for y in 0..N { did_deduce |= Self::pinpoint_one_lane(grid.look_left(y).1) }

        println!("post-pinpoint = \n{grid:?}\n");
        
        (grid, did_deduce)
    }

    fn deduce_one_lane((clue, lane): (Option<Digit>, [&mut Cell; N])) -> bool
    {
        let mut did_deduce = false;

        for (i, cell) in lane.into_iter().enumerate() {
            let max = Self::calc_max_from_clue(clue, i);
            let cands: HashSet<Digit> = (1..=max).collect();

            if let Some(1) = clue && i == 0 {
                *cell = Cell::Solved(N as Digit);
                continue;
            }
            else if let Some(c) = clue && c == N as Digit {
                *cell = Cell::Solved((i+1) as Digit);
            }

            match cell {
                Cell::Solved{..} => continue,
                Cell::Pencil(digits) => {
                    if let Some(ds) = digits.take() {
                        // TODO might be unnecessary
                        let deduced: HashSet<Digit> = ds.intersection(&cands).copied().collect();

                        if deduced.len() == 1 {
                            let digit = deduced.into_iter().next().unwrap();
                            *cell = Cell::Solved(digit);
                            did_deduce = true;
                        }
                        else {
                            if deduced != ds {
                                did_deduce = true;
                            }
                            *digits = Some(deduced);
                        }
                    }
                },
            }
        }

        did_deduce
    }

    fn calc_max_from_clue(clue: Option<Digit>, i: usize) -> Digit
    {
        let clue_offset = clue.map(|c| c-1).unwrap_or(0);
        let out = (N + i) as Digit - clue_offset;

        out
    }

    fn deduce_one_cell_sudoku_style(mut grid: Grid<N>, x: usize, y: usize) -> (Grid<N>, bool)
    {
        let mut did_deduce = false;

        /* Would make this a little more structured, but then we end up in borrowing conflicts =( */
        if let Cell::Solved{..} = grid.at(x, y) {
            return (grid, did_deduce);
        }

        let mut seen = HashSet::new();

        for cell in grid.look_right(y).1 {
            if let Cell::Solved(digit) = cell { seen.insert(*digit); }
        }
        for cell in grid.look_down(x).1 {
            if let Cell::Solved(digit) = cell { seen.insert(*digit); }
        }

        if let Cell::Pencil(Some(cands)) = grid.at_mut(x, y) {
            for digit in seen {
                did_deduce |= cands.remove(&digit);
            }
        }

        (grid, did_deduce)
    }

    fn pinpoint_one_lane(mut lane: [&mut Cell; N]) -> bool
    {
        let mut did_deduce = false;

        /* NOTE: Index corresponds to digit (-1) */
        let mut seen_indices = [(); N].map(|_| vec![]);

        for (i, cell) in lane.iter().enumerate() {
            match cell {
                Cell::Solved(digit) => {
                    seen_indices[(*digit-1) as usize].push(i);
                }
                Cell::Pencil(Some(digits)) => {
                    for digit in digits {
                        seen_indices[(*digit-1) as usize].push(i);
                    }
                },
                _ => (),
            }
        }

        println!("lane = {:?}", lane);
        println!("seen_indices = {:?}", seen_indices);

        for (i, indices) in seen_indices.iter().enumerate() {
            let digit = (i+1) as Digit;

            if indices.len() == 1 {
                let idx = indices.into_iter().next().unwrap();

                if let cell@Cell::Pencil{..} = &mut lane[*idx] {
                    println!("idx = {:?}", idx);
                    **cell = Cell::Solved(digit);
                    did_deduce = true;
                }
            }
        }

        did_deduce
    }
}
