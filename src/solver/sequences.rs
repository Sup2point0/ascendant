use natbitset::Bitset;

use crate::*;


/// Deductions made by enforcing ascending sequences, following the rules of Skyscrapers.
impl<const N: usize> Solver<N>
{
    pub fn deduce_cells_in_lane((clue, mut lane): (Option<Digit>, [&mut Cell<N>; N])) -> bool
    {
        // TODO short circuit earlier on no clue
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
}
