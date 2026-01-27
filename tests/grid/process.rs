use ascendant::*;
use ascendant::{
    Cell::Solved as Sv,
};
use crate::*;


#[test] fn occurrences()
{
    assert_eq!(
        Grid::occurrences(
            &[&mut Sv(1), &mut Sv(2), &mut Sv(3), &mut Sv(4), &mut Sv(5)]
        ),
        map![vec![0], vec![1], vec![2], vec![3], vec![4]]
    );

    assert_eq!(
        Grid::occurrences(
            &[
                &mut p![1,2,3    ],
                &mut p![1,2      ],
                &mut p![  2,3    ],
                &mut p![    3,4,5],
                &mut p![1,      5],
            ]
        ),
        map![
            vec![0,1,    4],
            vec![0,1,2    ],
            vec![0,  2,3, ],
            vec![      3, ],
            vec![      3,4],
        ]
    );

    assert_eq!(
        Grid::occurrences(
            &[
                &mut p![1,2,3  ],
                &mut p![1,2    ],
                &mut Sv(5),
                &mut p![    3,4],
                &mut p![1,     ],
            ]
        ),
        map![
            vec![0,1,    4],
            vec![0,1,     ],
            vec![0  ,  3, ],
            vec![      3, ],
            vec![2        ],
        ]
    );
}

#[test] fn count_visible_in_solved_lane()
{
    const N: usize = 6;

    // 1 2 3 4 5 6
    let lane = [ Sv(1), Sv(2), Sv(3), Sv(4), Sv(5), Sv(6) ];
    assert_eq!( Grid::<N>::count_visible_in_solved_lane(lane), 6 );

    // 6 1 5 2 4 3
    let lane = [ Sv(6), Sv(1), Sv(5), Sv(2), Sv(4), Sv(3) ];
    assert_eq!( Grid::<N>::count_visible_in_solved_lane(lane), 1 );

    // 1 3 2 4 6 5
    let lane = [ Sv(1), Sv(3), Sv(2), Sv(4), Sv(6), Sv(5) ];
    assert_eq!( Grid::<N>::count_visible_in_solved_lane(lane), 4 );
}

#[test] fn count_possible_visible_in_lane()
{
    const N: usize = 6;

    // 1 2 3 4 5 6
    let lane = [ Sv(1), Sv(2), Sv(3), Sv(4), Sv(5), Sv(6) ];
    assert_eq!( Grid::<N>::count_possible_visible_in_lane(&lane), (6, 6) );

    // 6 1 5 2 4 3
    let lane = [ Sv(6), Sv(1), Sv(5), Sv(2), Sv(4), Sv(3) ];
    assert_eq!( Grid::<N>::count_possible_visible_in_lane(&lane), (1, 1) );

    // 4 [15] _ _ 6 [15]
    let lane = [ Sv(4), p![1,5], p![2,3], p![2,3], Sv(6), p![1,5] ];
    assert_eq!( Grid::<N>::count_possible_visible_in_lane(&lane), (2, 3) );

    // 2 [13] 4 [35] 6 [135]
    let lane = [ Sv(2), p![1,3], Sv(4), p![3,5], Sv(6), p![1,3,5] ];
    assert_eq!( Grid::<N>::count_possible_visible_in_lane(&lane), (3, 5) );
}
