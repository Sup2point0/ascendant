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
            println!("\n{grid:?}\n");

            (grid, did_deduce) = Self::deduce_one_pass(grid);
            if !did_deduce { break; }
        }

        grid
    }

    /// Perform one pass of deductions through the grid. Returns the updated grid and a `bool` indicating if any deductions were successfully made.
    pub fn deduce_one_pass(mut grid: Grid<N>) -> (Grid<N>, bool)
    {
        let mut did_deduce = false;

        for x in 0..N { did_deduce |= Self::deduce_cells_in_lane(grid.look_down(x)); }
        for x in 0..N { did_deduce |= Self::deduce_cells_in_lane(grid.look_up(x)); }
        for y in 0..N { did_deduce |= Self::deduce_cells_in_lane(grid.look_right(y)); }
        for y in 0..N { did_deduce |= Self::deduce_cells_in_lane(grid.look_left(y)); }

        let mut deduced;
        for x in 0..N {
            for y in 0..N {
                (grid, deduced) = Self::deduce_one_cell_sudoku_style(grid, x, y);
                did_deduce |= deduced;
            }
        }

        for x in 0..N { did_deduce |= Self::deduce_sequence_in_lane(grid.look_down(x)) }
        for x in 0..N { did_deduce |= Self::deduce_sequence_in_lane(grid.look_up(x)) }
        for y in 0..N { did_deduce |= Self::deduce_sequence_in_lane(grid.look_right(y)) }
        for y in 0..N { did_deduce |= Self::deduce_sequence_in_lane(grid.look_left(y)) }

        for x in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_down(x).1) }
        for x in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_up(x).1) }
        for y in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_right(y).1) }
        for y in 0..N { did_deduce |= Self::pinpoint_cells_in_lane(grid.look_left(y).1) }
        
        (grid, did_deduce)
    }

    pub fn deduce_cells_in_lane((clue, lane): (Option<Digit>, [&mut Cell; N])) -> bool
    {
        let mut did_deduce = false;
        let lane_snap: [Cell; N] = util::arr(lane.iter().map(|cell| (*cell).clone()));

        for (i, cell) in lane.into_iter().enumerate()
        {
            if let Cell::Solved{..} = cell { continue; }

            if let Some(1) = clue && i == 0 {
                *cell = Cell::Solved(N);
                continue;
            }
            else if let Some(c) = clue && c == N {
                *cell = Cell::Solved(i+1);
            }

            if let Cell::Pencil(digits) = cell
            && let Some(ds) = digits.take()
            {
                let peak_idx = lane_snap.iter().position(|c| *c == Cell::Solved(N));

                let cands = {
                    if let Some(c) = clue
                    && let Some(idx) = peak_idx
                    && i < idx
                    {
                        Self::calc_cands_from_peak(c, i, idx)
                    }
                    else {
                        Self::calc_cands_from_clue(clue, i)
                    }
                };

                let deduced: HashSet<Digit> = ds.intersection(&cands).copied().collect();

                if deduced != ds {
                    did_deduce = true;
                }

                *digits = Some(deduced);
            }
        }

        did_deduce
    }

    pub fn calc_cands_from_clue(clue: Option<Digit>, i: usize) -> HashSet<Digit>
    {
        let clue_offset = clue.map(|c| c-1).unwrap_or(0);
        let out = N + i - clue_offset;

        (1..=out).collect()
    }

    pub fn calc_cands_from_peak(clue: Digit, i: usize, peak_idx: usize) -> HashSet<Digit>
    {
        let lower = 1 + if peak_idx < clue {i} else {0};

        let upper = {
            if clue == 2 {
                if i == 0 {
                    if peak_idx == N-1 {
                        return HashSet::from([N-1]);
                    }
                    N - 1
                }
                else {
                    N - 2
                }
            }
            else {
                (1 + N - clue + i).min(N-1)
            }
        };
        
        (lower..=upper).collect()
    }

    pub fn deduce_one_cell_sudoku_style(mut grid: Grid<N>, x: usize, y: usize) -> (Grid<N>, bool)
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

    pub fn deduce_sequence_in_lane((clue, mut lane): (Option<Digit>, [&mut Cell; N])) -> bool
    {
        let mut did_deduce = false;

        'exit: {
            let clue = match clue {
                None    => break 'exit,
                Some(1) => break 'exit,
                Some(2) => break 'exit,
                Some(c) => c,
            };

            let mut target = N;
            let mut peaks = 0;
            let mut last_index = N;

            /* 1st pass from end: Find peaks */
            for (i, cell) in lane.iter().enumerate().rev() {
                if let Cell::Solved(digit) = cell {
                    if *digit == target {
                        target -= 1;
                        peaks += 1;
                        last_index = i;
                    }
                    else if *digit == N-1 || peaks > 0 {
                        break 'exit;
                    }
                }
            }

            if peaks == 0 {
                break 'exit;
            }

            /* 2nd pass from start: Enforce ascending sequence */
            if last_index != clue - peaks
            || last_index == 0 {
                break 'exit;
            }

            /* NOTE: If the algorithm's working correctly, this cell should always be `Pencil` with at least 1 possible digit. */
            if let Cell::Pencil(Some(digits)) = lane[last_index - 1]
                && let Some(d) = digits.iter().max()
            {
                target = target.min(*d);
            }
            else { break 'exit; }

            for (i, cell) in lane[0..last_index].iter_mut().enumerate() {
                if let Cell::Pencil(digits) = cell
                && let Some(ds) = digits.take()
                {
                    let cands = Self::calc_ascending(target, i, last_index);
                    let deduced: HashSet<Digit> = ds.intersection(&cands).copied().collect();

                    if deduced != ds {
                        did_deduce = true;
                    }

                    *digits = Some(deduced);
                }
            }
        }

        did_deduce
    }

    pub fn calc_ascending(target: Digit, i: usize, last_index: usize) -> HashSet<Digit>
    {
        let j = (last_index - 1) - i;

        let lower = 1 + i;
        let upper = target - j;

        (lower..=upper).collect()
    }

    pub fn pinpoint_cells_in_lane(mut lane: [&mut Cell; N]) -> bool
    {
        let mut did_deduce = false;

        /* NOTE: Index corresponds to digit (-1) */
        let mut seen_indices = [(); N].map(|_| vec![]);

        for (i, cell) in lane.iter().enumerate() {
            match cell {
                Cell::Solved(digit) => {
                    seen_indices[*digit - 1].push(i);
                }
                Cell::Pencil(Some(digits)) => {
                    for digit in digits {
                        seen_indices[*digit - 1].push(i);
                    }
                },
                _ => (),
            }
        }

        for (i, indices) in seen_indices.iter().enumerate() {
            let digit = i + 1;

            if indices.len() == 1 {
                let idx = indices.into_iter().next().unwrap();

                if let cell@Cell::Pencil{..} = &mut lane[*idx] {
                    **cell = Cell::Solved(digit);
                    did_deduce = true;
                }
            }
        }

        for cell in lane {
            if let Cell::Pencil(Some(digits)) = cell
                && digits.len() == 1
            {
                let digit = digits.iter().next().unwrap();
                *cell = Cell::Solved(*digit);
                did_deduce = true;
            }
        }

        did_deduce
    }
}
