use ascendant::*;


#[test] fn deduce_sequence_in_lane_25()
{
    const N: usize = 5;
    let clue = 2;

    // 4 | _ 2 5 _ _
    let mut lane = [
        pen![1,2,3],
        Cell::Solved(2),
        Cell::Solved(5),
        pen![3,4],
        pen![3,4],
    ];

    Solver::<N>::deduce_sequence_in_lane((Some(clue), util::arr(lane.iter_mut())));

    assert_eq!(lane, [
        pen![1,2,3],
        Cell::Solved(2),
        Cell::Solved(5),
        pen![3,4],
        pen![3,4],
    ]);
}

#[test] fn deduce_sequence_in_lane_45()
{
    const N: usize = 5;
    let clue = 4;

    // 4 | _ _ _ 4 5
    let mut lane = [
        pen![1,2,3],
        pen![1,2,3],
        pen![1,2,3],
        Cell::Solved(4),
        Cell::Solved(5),
    ];

    Solver::<N>::deduce_sequence_in_lane((Some(clue), util::arr(lane.iter_mut())));

    assert_eq!(lane, [
        pen![1,2],
        pen![1,2,3],
        pen![1,2,3],
        Cell::Solved(4),
        Cell::Solved(5),
    ]);
}

#[test] fn deduce_sequence_in_lane_46()
{
    const N: usize = 6;
    let clue = 4;

    // 4 | _ _ 4 _ 6 _
    let mut lane = [
        pen![1,2,3],
        pen![1,2,3],
        Cell::Solved(4),
        pen![1,2,3],
        Cell::Solved(6),
        Cell::Solved(5),
    ];

    Solver::<N>::deduce_sequence_in_lane((Some(clue), util::arr(lane.iter_mut())));

    assert_eq!(lane, [
        pen![1,2],
        pen![  2,3],
        Cell::Solved(4),
        pen![1,2,3],
        Cell::Solved(6),
        Cell::Solved(5),
    ]);

    // 4 | _ _ _ 4 6 _
    let mut lane = [
        pen![1,2,3],
        pen![1,2,3],
        pen![1,2,3],
        Cell::Solved(4),
        Cell::Solved(6),
        Cell::Solved(5),
    ];

    Solver::<N>::deduce_sequence_in_lane((Some(clue), util::arr(lane.iter_mut())));

    assert_eq!(lane, [
        pen![1,2],
        pen![1,2,3],
        pen![1,2,3],
        Cell::Solved(4),
        Cell::Solved(6),
        Cell::Solved(5),
    ]);
}

#[test] fn deduce_sequence_in_lane_56()
{
    const N: usize = 6;
    let clue = 5;

    // 5 | _ _ _ 4 6 5
    let mut lane = [
        pen![1,2,3],
        pen![1,2,3],
        pen![1,2,3],
        Cell::Solved(4),
        Cell::Solved(6),
        Cell::Solved(5),
    ];

    Solver::<N>::deduce_sequence_in_lane((Some(clue), util::arr(lane.iter_mut())));

    assert_eq!(lane, [
        pen![1],
        pen![2],
        pen![3],
        Cell::Solved(4),
        Cell::Solved(6),
        Cell::Solved(5),
    ]);
}
