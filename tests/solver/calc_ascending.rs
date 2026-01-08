use ascendant::*;
use crate::*;


#[test] fn calc_ascending_46()
{
    // 4 | _ _ _ 6 _ _
    let sequence_peak = 5;
    let cells_visible = 3;
    let first_peak_idx = 3;

    assert_eq!( Solver::<6>::calc_ascending(0, sequence_peak, cells_visible, first_peak_idx), set![1,2,3] );
    assert_eq!( Solver::<6>::calc_ascending(1, sequence_peak, cells_visible, first_peak_idx), set![  2,3,4] );
    assert_eq!( Solver::<6>::calc_ascending(2, sequence_peak, cells_visible, first_peak_idx), set![    3,4,5] );

    // 4 | _ _ 5 _ _ _
    let sequence_peak = 4;
    let cells_visible = 2;
    let first_peak_idx = 2;

    assert_eq!( Solver::<6>::calc_ascending(0, sequence_peak, cells_visible, first_peak_idx), set![1,2,3] );
    assert_eq!( Solver::<6>::calc_ascending(1, sequence_peak, cells_visible, first_peak_idx), set![  2,3,4] );

    // 4 | _ 4 _ _ _ _
    let sequence_peak = 3;
    let cells_visible = 1;
    let first_peak_idx = 1;

    assert_eq!( Solver::<6>::calc_ascending(0, sequence_peak, cells_visible, first_peak_idx), set![1,2,3] );
}

#[test] fn calc_ascending_56()
{
    // 5 | _ _ _ 4 6 5
    let sequence_peak = 3;
    let cells_visible = 3;
    let first_peak_idx = 3;

    assert_eq!( Solver::<6>::calc_ascending(0, sequence_peak, cells_visible, first_peak_idx), set![1] );
    assert_eq!( Solver::<6>::calc_ascending(1, sequence_peak, cells_visible, first_peak_idx), set![2] );
    assert_eq!( Solver::<6>::calc_ascending(2, sequence_peak, cells_visible, first_peak_idx), set![3] );
}
