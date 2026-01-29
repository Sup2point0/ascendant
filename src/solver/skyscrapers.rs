use std::*;
use std::cell::LazyCell;

use itertools::*;

use crate::*;


#[derive(Eq, PartialEq, Debug)]
pub enum Island<'l, const N: usize> {
    /// A single island containing the value of a `Cell::Solved`.
    Peak(usize),
    
    /// An island of `Cell::Pencil`s.
    Uncertain(Vec<&'l mut Cell<N>>),
}

impl<'l, const N: usize> Island<'l, N> {
    pub fn get(self) -> Vec<Cell<N>> {
        match self {
            Island::Peak(d)          => vec![Cell::Solved(d)],
            Island::Uncertain(cells) => cells.into_iter().map(|cell| *cell).collect(),
        }
    }
}


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

    pub fn pick_close_in_grid(grid: &mut Grid<N>) -> bool
    {
        let mut did_deduce = false;

        for x in 0..N { did_deduce |= Self::pick_visible_in_close_lane(grid.look_down_mut(x)); }
        for x in 0..N { did_deduce |= Self::pick_visible_in_close_lane(grid.look_up_mut(x)); }
        for y in 0..N { did_deduce |= Self::pick_visible_in_close_lane(grid.look_right_mut(y)); }
        for y in 0..N { did_deduce |= Self::pick_visible_in_close_lane(grid.look_left_mut(y)); }
        debug!("post-pick:\n{grid:?}");

        did_deduce
    }

    /// Restrict candidates in a lane where toggling the visibility of a single skyscraper can solve the lane.
    /// 
    /// In this lane, we need 1 more visible skyscraper, hence we can deduce the last remaining `Cell::Pencil`:
    /// 
    /// ```text
    ///  4 | 1 2 4 [35] 6 _
    /// -> | 1 2 4   3  6 _
    /// 
    /// !4 | 1 2 4   5  6 _  <-- gives 5 visible, not 4!
    /// ```
    pub fn pick_visible_in_close_lane((clue, lane): (Option<Digit>, [&mut Cell<N>; N])) -> bool
    {
        let mut did_deduce = false;

        let Some(clue) = clue else { return false };
        let already_met_clue = (Grid::count_visible_solved_in_lane(&lane) == clue);

        let (lower, upper) = Grid::count_possible_visible_in_lane(&lane);
        let can_pick       = (upper - lower == 1);
        let will_meet_clue = (lower == clue);
        let need_one_more  = (upper == clue);

        let is_pickable = can_pick && (will_meet_clue || need_one_more);
        if !is_pickable { return false }

        let mut grouped = Self::group_uncertain_in_lane(lane);
        let islands = grouped.len();

        /* NOTE: We don't iterate over first and last cells, as these have no previous and next, respectively. */
        'islands: for i in 0..(islands - 1) {
            if i == 0 {
                if let Island::Uncertain(cells) = &grouped[0]
                && let Island::Peak(next_peak) = &grouped[1]
                && cells.iter().map(|cell| cell.max()).max().unwrap() > *next_peak
                {
                    break 'islands;
                } else {
                    continue;
                }
            }

            /* We also stop once we reach the lane peak. */
            if let Island::Peak(n) = grouped[i] && n == N { continue }

            /* SAFETY:
            Read the raw values (which are `Copy`) of previous and next islands, and get a mutable reference to the current island.
            
            No mutations are made, and this mutable reference is the only reference that escapes this block.
            */
            let (prev_peak, pencil_cells, next_peak) = unsafe
            {
                let ptr = grouped.as_mut_ptr();

                let current = &mut *ptr.add(i);
                let Island::Uncertain(cells) = current else { continue };

                let Island::Peak(prev) = &*ptr.add(i - 1) else { unreachable!() };
                let Island::Peak(next) = &*ptr.add(i + 1) else { unreachable!() };

                (*prev, cells, *next)
            };

            if already_met_clue {
                /* Don't allow any more skyscrapers to be visible if they won't obscure a current peak. Remove all candidates that would create a new peak. */
                for cell in pencil_cells {
                    let Cell::Pencil(cands) = cell else { unreachable!() };

                    if cands.maximum().expect("Cell should not have no candidates") > next_peak {
                        break 'islands;
                    }

                    let before = *cands;
                    // MIGRATE use `retain_nonempty`
                    cands.retain(|d| prev_peak > d);

                    if cands.is_empty() {
                        panic!("Deleted all candidates while trying to hide candidates in lane: {grouped:?}");
                    }

                    did_deduce |= (*cands != before);
                }
            }
            else if need_one_more {
                let mut could_be_visible = pencil_cells.into_iter()
                    .filter(|cell| {
                        let max_cand = cell.max_cand().unwrap_or(0);
                        prev_peak < max_cand && max_cand < next_peak
                    })
                    .collect_vec();

                /* Only force a cell to be visible if only 1 cell is able to be; otherwise, we can't determine which of the cells should be made visible. */
                if let [Cell::Pencil(cands)] = &mut could_be_visible[..] {
                    let before = *cands;
                    // MIGRATE use `retain_nonempty`
                    cands.retain(|d| prev_peak < d);

                    if cands.is_empty() {
                        panic!("Deleted all candidates while trying to show candidates in lane: {grouped:?}");
                    }
                    
                    did_deduce |= (*cands != before);
                }
            }
        }

        did_deduce
    }

    /// Group a lane of `Cell::Solved` and/or `Cell::Pencil` into islands alternating between `Solved` and `Pencil`.
    /// 
    /// - An `Island::Peak(Digit)` contains the value of a solved cell.
    /// - An `Island::Uncertain(Vec)` contains mutable references to unsolved cells.
    pub fn group_uncertain_in_lane(lane: [&mut Cell<N>; N]) -> Vec<Island<'_, N>>
    {
        let mut out = vec![];
        let mut peak = 0;

        for cell in lane {
            match cell {
                Cell::Solved(d) => {
                    if *d > peak {
                        out.push(Island::Peak(*d));
                        peak = *d;
                    }
                },
                Cell::Pencil{..} => {
                    if let Some(Island::Uncertain(cells)) = out.last_mut() {
                        cells.push(cell);
                    } else {
                        let island = Island::Uncertain(vec![cell]);
                        out.push(island);
                    }
                }
            }
        }

        out
    }
}
