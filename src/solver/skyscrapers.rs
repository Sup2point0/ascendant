use std::*;
use std::cell::LazyCell;

use crate::*;


#[derive(Eq, PartialEq, Debug)]
pub enum Island<'l, const N: usize> {
    Peak(usize),
    Uncertain(Vec<&'l mut Cell<N>>),
}

impl<'l, const N: usize> Island<'l, N> {
    pub fn get(self) -> Vec<Cell<N>> {
        match self {
            Island::Peak(d)          => vec![Cell::Solved(d)],
            Island::Uncertain(cells) => cells.into_iter().map(|cell| cell.clone()).collect(),
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

    /// In a lane where the currently possibly visible number of skyscrapers is 
    pub fn pick_visible_in_close_lane((clue, lane): (Option<Digit>, [&mut Cell<N>; N])) -> bool
    {
        let mut did_deduce = false;

        let Some(clue) = clue else { return false };

        let (lower, upper) = Grid::count_possible_visible_in_lane(&lane);
        println!("lower = {:?}", lower);
        println!("upper = {:?}", upper);
        let is_one_away = (upper - lower == 1) && (lower == clue) || (upper == clue);
        if !is_one_away { return false }

        let mut grouped = Self::group_uncertain_in_lane(lane);

        let islands = grouped.len();
        let ptr = grouped.as_mut_ptr();

        /* NOTE: We don't iterate over first and last cells, as these have no previous and next, respectively. */
        for i in 0..(islands - 1) {
            if i == 0 { continue }

            /* SAFETY:
            Read the raw values (which are `Copy`) of previous and next islands, and get a mutable reference to the current island.
            
            No mutations are made, and this mutable reference is the only reference that escapes this block.
            */
            let (prev, cells, next) = unsafe
            {
                let current = slice::from_raw_parts_mut(ptr.add(i), 1);
                let [Island::Uncertain(cells)] = current else { continue };

                let prev = slice::from_raw_parts(ptr.add(i - 1), 1);
                let next = slice::from_raw_parts(ptr.add(i + 1), 1);

                let [Island::Peak(prev)] = prev else { unreachable!() };
                let [Island::Peak(next)] = next else { unreachable!() };

                (*prev, cells, *next)
            };

            println!("cells = {:?}", cells);

            for cell in cells.drain(..) {
                println!("cell = {:?}", cell);
                let possible_peak = cell.max();

                if prev < possible_peak && possible_peak < next {
                    if lower == clue {
                        let Cell::Pencil(cands) = cell else { unreachable!() };

                        let before = *cands;
                        // MIGRATE Wait for `retain()`
                        *cands = cands.into_iter().filter(|d| prev > *d).collect();

                        did_deduce |= (*cands != before);
                    }
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

        for cell in lane {
            match cell {
                Cell::Solved(d) => out.push(Island::Peak(*d)),
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
