use ascendant::*;
use ascendant::Cell::Solved as Sv;


#[test] pub fn pick_last_in_lane()
{
    const N: usize = 6;

    // 5 | [13] 2 4 5 6 _ -> [1] 2 4 5 6 _
    let clue = 5;
    let mut lane = [ p![1,3], Sv(2), Sv(4), Sv(5), Sv(6), p![1] ];

    Solver::<N>::pick_last_in_lane((Some(clue), util::arr_mut(&mut lane)));
    assert_eq!(lane, [ p![1], Sv(2), Sv(4), Sv(5), Sv(6), p![1] ]);

    // 3 | [14] 3 _ 5 6 _ -> [4] 3 _ 5 6 _
    let clue = 3;
    let mut lane = [ p![1,4], Sv(3), p![1,2], Sv(5), Sv(6), p![1] ];

    Solver::<N>::pick_last_in_lane((Some(clue), util::arr_mut(&mut lane)));
    assert_eq!(lane, [ p![4], Sv(3), p![1,2], Sv(5), Sv(6), p![1] ]);

    // 4 | 1 3 [24] 5 6 _ -> 1 3 [2] 5 6 _
    let clue = 4;
    let mut lane = [ Sv(1), Sv(3), p![2,4], Sv(5), Sv(6), p![1] ];

    Solver::<N>::pick_last_in_lane((Some(clue), util::arr_mut(&mut lane)));
    assert_eq!(lane, [ Sv(1), Sv(3), p![2], Sv(5), Sv(6), p![1] ]);

    // 5 | 1 3 [24] 5 6 _ -> 1 3 [4] 5 6 _
    let clue = 5;
    let mut lane = [ Sv(1), Sv(3), p![2,4], Sv(5), Sv(6), p![1] ];

    Solver::<N>::pick_last_in_lane((Some(clue), util::arr_mut(&mut lane)));
    assert_eq!(lane, [ Sv(1), Sv(3), p![4], Sv(5), Sv(6), p![1] ]);
}
