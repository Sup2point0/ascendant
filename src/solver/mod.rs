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
                let cands = {
                    if let Some(c) = clue
                    && let Some(idx) = Grid::find_peak(&lane_snap)
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

            /* 1st pass from end: Descend peaks */
            let peak_index = match Grid::find_peak(&lane) {
                None    => break 'exit,
                Some(i) => i,
            };
            
            let seen_indices = Grid::occurrences(&lane);
            
            let mut first_peak_idx = peak_index;
            let mut target = N;
            let mut peaks = 1;

            for digit in (2..N).rev() {
                let indices = &seen_indices[&digit];

                /* This is a solved skyscraper since it only appears in one cell in the lane. */
                if indices.len() == 1 {
                    if indices[0] < first_peak_idx {
                        first_peak_idx = indices[0];
                        target = digit;
                        peaks += 1;
                    }
                }
                /* If we encounter an uncertain peak that may or may not contribute to the sequence, then we can't determine the bounds of the sequence with certainty. */
                else if !indices.iter().all(|i| *i > first_peak_idx || *i == 0)
                    && (0..first_peak_idx).any(|i| matches!(lane[i], Cell::Solved{..}))
                {
                    break 'exit;
                }
            }

            target -= 1;

            /* 2nd pass from start: Enforce ascending sequence */
            let cells_visible = clue - peaks;

            if first_peak_idx == 0 {
                break 'exit;
            }

            for (i, cell) in lane[0..first_peak_idx].iter_mut().enumerate() {
                if let Cell::Pencil(digits) = cell
                && let Some(ds) = digits.take()
                {
                    let cands = Self::calc_ascending(i, target, cells_visible, first_peak_idx);
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

    pub fn calc_ascending(
        i: usize,               // What index is the current cell we're considering?
        sequence_peak: Digit,   // What's the tallest a skyscraper in the gap can be?
        cells_visible: usize,   // How many skyscrapers are visible in the gap before the first peak?
        first_peak_idx: usize,  // What index was the shortest peak in the lane?
    ) -> HashSet<Digit>
    {
        let j = ((cells_visible as i32 - 1) - i as i32).max(0) as Digit;

        let lower = 1 + if first_peak_idx == cells_visible {i} else {0};
        let upper = sequence_peak - j;

        (lower..=upper).collect()
    }

    pub fn pinpoint_cells_in_lane(mut lane: [&mut Cell; N]) -> bool
    {
        let mut did_deduce = false;

        for (digit, indices) in Grid::occurrences(&lane) {
            if indices.len() == 1 {
                let idx = indices.into_iter().next().unwrap();

                if let cell@Cell::Pencil{..} = &mut lane[idx] {
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
