use ascendant::*;
use ascendant::Cell::Solved as Sv;


#[test] fn deduce_sequence_in_lane_25()
{
    const N: usize = 5;
    let clue = 2;

    // 2 | _ _ _ _ 5 _
    let mut lane = [ p![1,2,3,4], p![1,2,3,4], p![1,2,3,4], p![1,2,3,4], Sv(5) ];

    Solver::<N>::deduce_sequence_in_lane((Some(clue), util::arr(lane.iter_mut())));
    assert_eq!(lane, [ p![1,2,3,4], p![1,2,3], p![1,2,3], p![1,2,3], Sv(5) ]);

    // 2 | 3 _ _ 5 _
    let mut lane = [Sv(3), p![1,2,3,4], p![1,2,3,4], Sv(5), p![1,2,3,4] ];

    Solver::<N>::deduce_sequence_in_lane((Some(clue), util::arr(lane.iter_mut())));
    assert_eq!(lane, [Sv(3), p![1,2], p![1,2], Sv(5), p![1,2,3,4] ]);
}

#[test] fn deduce_sequence_in_lane_35()
{
    const N: usize = 5;
    let clue = 3;

    // 3 | _ 2 5 _ _
    let mut lane = [ p![1,3], Sv(2), Sv(5), p![3,4], p![3,4] ];

    Solver::<N>::deduce_sequence_in_lane((Some(clue), util::arr(lane.iter_mut())));
    assert_eq!(lane, [ Sv(1), Sv(2), Sv(5), p![3,4], p![3,4] ]);
}

#[test] fn deduce_sequence_in_lane_45()
{
    const N: usize = 5;
    let clue = 4;

    // 4 | _ _ _ 4 5
    let mut lane = [ p![1,2,3], p![1,2,3], p![1,2,3], Sv(4), Sv(5) ];

    Solver::<N>::deduce_sequence_in_lane((Some(clue), util::arr(lane.iter_mut())));
    assert_eq!(lane, [ p![1,2], p![1,2,3], p![1,2,3], Sv(4), Sv(5) ]);
}


#[test] fn deduce_sequence_in_lane_36()
{
    const N: usize = 6;
    let clue = 3;

    // 3 | _ 2 5 1 _ 6
    let mut lane = [ p![3,4], Sv(2), Sv(5), Sv(1), p![3,4], Sv(6) ];

    Solver::<N>::deduce_sequence_in_lane((Some(clue), util::arr(lane.iter_mut())));
    assert_eq!(lane, [ p![3,4], Sv(2), Sv(5), Sv(1), p![3,4], Sv(6) ]);
}

#[test] fn deduce_sequence_in_lane_46()
{
    const N: usize = 6;
    let clue = 4;

    // 4 | _ _ 4 _ 6 _
    let mut lane = [ p![1,2,3], p![1,2,3], Sv(4), p![1,2,3], Sv(6), Sv(5) ];

    Solver::<N>::deduce_sequence_in_lane((Some(clue), util::arr(lane.iter_mut())));
    assert_eq!(lane, [ p![1,2], p![2,3], Sv(4), p![1,2,3], Sv(6), Sv(5) ]);

    // 4 | _ _ _ 4 6 _
    let mut lane = [ p![1,2,3], p![1,2,3], p![1,2,3], Sv(4), Sv(6), Sv(5) ];

    Solver::<N>::deduce_sequence_in_lane((Some(clue), util::arr(lane.iter_mut())));
    assert_eq!(lane, [ p![1,2], p![1,2,3], p![1,2,3], Sv(4), Sv(6), Sv(5) ]);
}

#[test] fn deduce_sequence_in_lane_56()
{
    const N: usize = 6;
    let clue = 5;

    // 5 | _ _ _ 4 6 5
    let mut lane = [ p![1,2,3], p![1,2,3], p![1,2,3], Sv(4), Sv(6), Sv(5) ];

    Solver::<N>::deduce_sequence_in_lane((Some(clue), util::arr(lane.iter_mut())));
    assert_eq!(lane, [ Sv(1), Sv(2), Sv(3), Sv(4), Sv(6), Sv(5) ]);
}


#[test] fn deduce_sequence_in_lane_37()
{
    const N: usize = 7;
    let clue = 3;

    // 3 | _ _ _ _ _ _ 7
    let mut lane = [ p![1;6], p![1;6], p![1;6], p![1;6], p![1;6], p![1;6], Sv(7) ];

    Solver::<N>::deduce_sequence_in_lane((Some(clue), util::arr(lane.iter_mut())));
    assert_eq!(lane, [ p![1;5], p![1;6], p![1;6], p![1;6], p![1;6], p![1;6], Sv(7) ]);
}

#[test] fn deduce_sequence_in_lane_47()
{
    const N: usize = 7;
    let clue = 4;

    // 4 | _ _ 3 5 7 6 4
    let mut lane = [ p![1,2], p![1,2], Sv(3), Sv(5), Sv(7), Sv(6), Sv(4) ];

    Solver::<N>::deduce_sequence_in_lane((Some(clue), util::arr(lane.iter_mut())));
    assert_eq!(lane, [ p![1,2], Sv(1), Sv(3), Sv(5), Sv(7), Sv(6), Sv(4) ]);
}
