use std::cell::LazyCell;

use crate::*;


/// Deductions made by applying miscellaneous rules of Skyscrapers.
impl<const N: usize> Solver<N>
{
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
}
