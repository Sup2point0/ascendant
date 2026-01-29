mod examples;
pub use examples::*;

mod util;
pub use util::*;


mod grid
{
    mod construct;

    mod query;

    mod process;
}


mod solver
{
    mod solve;

    mod calc_cands_from_peak;

    mod deduce_sequence_in_lane;
    
    mod calc_ascending;

    mod sudoku
    {
        mod find_isolated_groups;

        mod isolate_groups_in_lane;
    }

    mod skyscrapers
    {
        mod pick_visible_in_close_lane;

        mod pick_last_in_lane;
    }
}
