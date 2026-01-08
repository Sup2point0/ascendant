use ascendant::*;
use crate::*;


#[test] fn calc_cands_from_peak_24()
{
    let clue = 2;

    // 2 | _ _ _ 4
    let peak_idx = 3;
    assert_eq!( Solver::<4>::calc_cands_from_peak(clue, 0, peak_idx), set![3] );
    assert_eq!( Solver::<4>::calc_cands_from_peak(clue, 1, peak_idx), set![1,2] );
    assert_eq!( Solver::<4>::calc_cands_from_peak(clue, 2, peak_idx), set![1,2] );
    
    // 2 | _ _ 4 _
    let peak_idx = 2;
    assert_eq!( Solver::<4>::calc_cands_from_peak(clue, 0, peak_idx), set![1,2,3] );
    assert_eq!( Solver::<4>::calc_cands_from_peak(clue, 1, peak_idx), set![1,2] );
    
    // 2 | _ 4 _ _
    let peak_idx = 1;
    assert_eq!( Solver::<4>::calc_cands_from_peak(clue, 0, peak_idx), set![1,2,3] );
}

#[test] fn calc_cands_from_peak_36()
{
    let clue = 3;

    // 3 | _ _ _ _ 6 _
    let peak_idx = 4;
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 0, peak_idx), set![1,2,3,4] );
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 1, peak_idx), set![1,2,3,4,5] );
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 2, peak_idx), set![1,2,3,4,5] );
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 3, peak_idx), set![1,2,3,4,5] );

    // 3 | _ _ _ 6 _ _
    let peak_idx = 3;
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 0, peak_idx), set![1,2,3,4] );
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 1, peak_idx), set![1,2,3,4,5] );
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 2, peak_idx), set![1,2,3,4,5] );

    // 3 | _ _ 6 _ _ _
    let peak_idx = 2;
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 0, peak_idx), set![1,2,3,4] );
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 1, peak_idx), set![  2,3,4,5] );
}

#[test] fn calc_cands_from_peak_46()
{
    let clue = 4;

    // 4 | _ _ _ _ _ 6
    let peak_idx = 5;
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 0, peak_idx), set![1,2,3] );
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 1, peak_idx), set![1,2,3,4] );
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 2, peak_idx), set![1,2,3,4,5] );
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 3, peak_idx), set![1,2,3,4,5] );
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 4, peak_idx), set![1,2,3,4,5] );

    // 4 | _ _ _ _ 6 _
    let peak_idx = 4;
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 0, peak_idx), set![1,2,3] );
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 1, peak_idx), set![1,2,3,4] );
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 2, peak_idx), set![1,2,3,4,5] );
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 3, peak_idx), set![1,2,3,4,5] );

    // 4 | _ _ _ 6 _ _
    let peak_idx = 3;
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 0, peak_idx), set![1,2,3] );
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 1, peak_idx), set![  2,3,4] );
    assert_eq!( Solver::<6>::calc_cands_from_peak(clue, 2, peak_idx), set![    3,4,5] );
}
