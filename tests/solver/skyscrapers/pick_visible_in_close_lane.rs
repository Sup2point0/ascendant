use ascendant::*;
use ascendant::Cell::Solved as Sv;
use itertools::*;


#[test] fn group_uncertain_in_lane()
{
    fn _group_uncertain_in_lane_<const N: usize>(mut lane: [Cell<N>; N]) -> Vec<Vec<Cell<N>>>
    {
        let lane = util::arr(lane.iter_mut());

        Solver::<N>::group_uncertain_in_lane(lane)
            .into_iter()
            .map(|each| each.get())
            .collect_vec()
    }

    // 1 2 3 4 5 6
    assert_eq!(
        _group_uncertain_in_lane_([ Sv(1), Sv(2), Sv(3), Sv(4), Sv(5), Sv(6) ]),
        vec![ vec![Sv(1)], vec![Sv(2)], vec![Sv(3)], vec![Sv(4)], vec![Sv(5)], vec![Sv(6)] ]
    );

    // 1 2 3 4 [56] [56]
    assert_eq!(
        _group_uncertain_in_lane_([ Sv(1), Sv(2), Sv(3), Sv(4), p![5,6], p![5,6] ]),
        vec![ vec![Sv(1)], vec![Sv(2)], vec![Sv(3)], vec![Sv(4)], vec![p![5,6], p![5,6]] ]
    );

    // 1 [23] [23] 4 [56] [56]
    assert_eq!(
        _group_uncertain_in_lane_([ Sv(1), p![2,3], p![2,3], Sv(4), p![5,6], p![5,6] ]),
        vec![ vec![Sv(1)], vec![p![2,3], p![2,3]], vec![Sv(4)], vec![p![5,6], p![5,6]] ]
    );
}

#[test] fn pick_visible_in_close_lane()
{
    const N: usize = 6;

    // 2 | 4 [15] _ _ 6 _ -> 4 5 _ _ 6 _
    let mut lane = [ Sv(4), p![1,5], p![2,3], p![2,3], Sv(6), p![1,5] ];

    Solver::<N>::pick_visible_in_close_lane((Some(2), util::arr_mut(&mut lane)));
    assert_eq!(lane, [ Sv(4), p![1], p![2,3], p![2,3], Sv(6), p![1,5] ]);

    // 3 | 4 [15] _ _ 6 _ -> 4 5 _ _ 6 _
    let mut lane = [ Sv(4), p![1,5], p![2,3], p![2,3], Sv(6), p![1,5] ];

    Solver::<N>::pick_visible_in_close_lane((Some(3), util::arr_mut(&mut lane)));
    assert_eq!(lane, [ Sv(4), p![5], p![2,3], p![2,3], Sv(6), p![1,5] ]);

    // 5 | 1 2 3 [45] [45] 6 -> no change
    let mut lane = [ Sv(1), Sv(2), Sv(3), p![4,5], p![4,5], Sv(6) ];

    Solver::<N>::pick_visible_in_close_lane((Some(5), util::arr_mut(&mut lane)));
    assert_eq!(lane, [ Sv(1), Sv(2), Sv(3), p![4,5], p![4,5], Sv(6) ]);

    // 5 | 1 2 4 [35] [3] 6 -> 1 2 4 [5] [3] 6
    let mut lane = [ Sv(1), Sv(2), Sv(4), p![3,5], p![3], Sv(6) ];

    Solver::<N>::pick_visible_in_close_lane((Some(5), util::arr_mut(&mut lane)));
    assert_eq!(lane, [ Sv(1), Sv(2), Sv(4), p![5], p![3], Sv(6) ]);
}
