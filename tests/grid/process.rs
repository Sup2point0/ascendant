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
