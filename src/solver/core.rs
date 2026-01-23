use std::*;
use std::cell::LazyCell;

use natbitset::Bitset;

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

    /// Perform one pass of deductions through the grid. Returns the updated grid and a `bool` indicating if any deductions were successfully made.
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

    pub fn deduce_cells_in_lane((clue, mut lane): (Option<Digit>, [&mut Cell<N>; N])) -> bool
    {
        let mut did_deduce = false;

        for i in 0..lane.len()
        {
            let lane_snap = util::snap_lane(&lane);

            let cell = &mut lane[i];
            if let Cell::Solved(_) = cell { continue; }

            if let Some(1) = clue && i == 0 {
                **cell = Cell::Solved(N);
                continue;
            }
            else if let Some(c) = clue && c == N {
                **cell = Cell::Solved(i+1);
            }

            if let Cell::Pencil(_) = cell
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

                did_deduce = cell.intersect(cands, lane_snap);
            }
        }

        did_deduce
    }

    /// For one cell, calculate candidates based on the lane's clue.
    pub fn calc_cands_from_clue(clue: Option<Digit>, i: usize) -> Bitset<N>
    {
        let clue_offset = clue.map(|c| c-1).unwrap_or(0);
        let out = N + i - clue_offset;

        Cell::cands(1 as usize, out)
            .expect(&format!(
                "Produced no candidates for cell at idx: `{i}`, deducing from clue: `{clue:?}`, caused by"
            ))
    }

    /// For one cell, calculate candidates based on both the lane's clue and the index of its peak.
    pub fn calc_cands_from_peak(clue: Digit, i: usize, peak_idx: usize) -> Bitset<N>
    {
        let lower = 1 + if peak_idx < clue {i} else {0};

        let upper = {
            if clue == 2 {
                if i == 0 {
                    if peak_idx == N-1 {
                        return Bitset::from([N-1]);
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
        
        Cell::cands(lower, upper)
            .expect(&format!(
                "Produced no candidates for cell at idx: `{i}`, deducing from clue: `{clue}` and peak-idx: `{peak_idx}`, caused by"
            ))
    }

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

    /// Use the clue and peaks of a lane to narrow down candidates based on ascending sequences.
    pub fn deduce_sequence_in_lane((clue, mut lane): (Option<Digit>, [&mut Cell<N>; N])) -> bool
    {
        let mut did_deduce = false;

        'exit: {
            /* 1st pass from end: Descend peaks */
            let peak_index = match Grid::find_peak(&lane) {
                None    => break 'exit,
                Some(i) => i,
            };

            let clue = match clue {
                None    => break 'exit,
                Some(1) => break 'exit,
                Some(2) => return Self::deduce_haven_in_lane(lane, N, peak_index),
                Some(c) => c,
            };
            
            let seen_indices = Grid::occurrences(&lane);
            
            let mut first_peak_idx = peak_index;
            let mut first_peak = N;
            let mut peaks = 1;

            // println!("lane = {:?}", lane);

            for d in (2..N).rev()
            {
                let indices = &seen_indices[&d];
                // println!("-- d={d}, indices={indices:?}, first-peak={first_peak}, peaks={peaks}");

                /* If this is a solved skyscraper left of our current first peak, set it as the new first peak. */
                if indices.len() == 1 {
                    // println!("PEAK");
                    if indices[0] < first_peak_idx {
                        first_peak_idx = indices[0];
                        first_peak = d;
                        peaks += 1;

                        if peaks == clue {
                            // println!("DELEGATING");
                            return Self::deduce_haven_in_lane(lane, N, first_peak_idx);
                        }
                    }
                }
                /* If this skyscraper may appear earlier than the current first peak, it may or may not contribute to the sequence. */
                else if !indices.iter().all(|i| *i > first_peak_idx || *i == 0) {
                    /* If  */
                    if (0..first_peak_idx).any(|i| matches!(lane[i], Cell::Solved{..})) {
                        break 'exit;
                    }
                    break;
                }
            }

            /* 2nd pass from start: Enforce ascending sequence */
            if first_peak_idx == 0 { break 'exit; }

            if peaks == clue - 1 {
                return Self::deduce_haven_in_lane(lane, first_peak, first_peak_idx);
            }

            let sequence_peak = first_peak - 1;
            let cells_visible = clue - peaks;

            for i in 0..first_peak_idx
            {
                let lane_snap = util::snap_lane(&lane);
                let cell = &mut lane[i];

                if let Cell::Pencil(_) = cell {
                    let cands = Self::calc_ascending(i, sequence_peak, cells_visible, first_peak_idx);
                    did_deduce = cell.intersect(cands, lane_snap);
                }
            }
        }

        did_deduce
    }

    /// For one cell, calculates its candidates based on its place in an ascending sequence..
    pub fn calc_ascending(
        i: usize,               // What index is the current cell we're considering?
        sequence_peak: Digit,   // What's the tallest a skyscraper in the gap can be?
        cells_visible: usize,   // How many skyscrapers are visible in the gap before the first peak?
        first_peak_idx: usize,  // What index was the shortest peak in the lane?
    ) -> Bitset<N>
    {
        let j = ((cells_visible as i32 - 1) - i as i32).max(0) as Digit;

        let lower = 1 + if first_peak_idx == cells_visible {i} else {0};
        let upper = sequence_peak - j;

        Cell::cands(lower, upper)
            .expect(&format!(
                "Produced no candidates for cell at idx: `{i}`, deducing from ascending sequence with peak: `{sequence_peak}`, cells-visible: `{cells_visible}`, first-peak-idx: `{first_peak_idx}`, caused by"
            ))
    }

    pub fn deduce_haven_in_lane(
        mut lane: [&mut Cell<N>; N],
        peak: Digit,
        peak_idx: usize,
    ) -> bool
    {
        let mut did_deduce = false;
        if peak_idx == 0 { return false; }

        let lane_snap = util::snap_lane(&lane);
        let blockade = lane[0].max().min(peak - 1);

        /* Head must obscure all of tail */
        let lower = peak_idx.max(
            lane[1..peak_idx].iter()
                .filter_map(|cell| cell.solved_digit())
                .max()
                .unwrap_or(1)
        );
        
        if let cell@Cell::Pencil(_) = &mut lane[0]
        {
            let cands = Cell::cands(lower, blockade)
                .expect(&format!(
                    "Produced no candidates using 2-haven for head cell in lane: `{lane_snap:?}`, with peak: `{peak}` at idx: `{peak_idx}`, caused by"
                ));
            
            did_deduce |= cell.intersect(cands, lane_snap);
        }

        /* Tail can be arbitrarily low */
        let cands = LazyCell::new(||
            Cell::cands(1 as usize, blockade - 1)
            .expect(&format!(
                "Produced no candidates using 2-haven for tail cells in lane: `{lane_snap:?}`, with peak: `{peak}` at idx: `{peak_idx}`, caused by"
            ))
        );
    
        let mut lane_snap = util::snap_lane(&lane);

        for i in 1..peak_idx
        {
            if let cell@Cell::Pencil(_) = &mut lane[i] {
                did_deduce |= cell.intersect(*cands, lane_snap);
                lane_snap = util::snap_lane(&lane);
            }
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
            && let Some(d) = digits.single()
            {
                *cell = Cell::Solved(d);
                did_deduce = true;
            }
        }

        did_deduce
    }
}
