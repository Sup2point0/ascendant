use ascendant::*;


#[test] pub fn solve_4x4()
{
    let cc = 0;
    
    assert_eq!(
        Solver::solve(examples::grid_4x4_full_1()).cells,
        Grid::<4>::construct(
            [
                [cc, cc, cc, cc, cc, cc].into(),
                [cc,  2,  3,  4,  1, cc].into(),
                [cc,  4,  1,  2,  3, cc].into(),
                [cc,  3,  4,  1,  2, cc].into(),
                [cc,  1,  2,  3,  4, cc].into(),
                [cc, cc, cc, cc, cc, cc].into(),
            ]
        ).cells
    );
    
    assert_eq!(
        Solver::solve(examples::grid_4x4_sparse_1()).cells,
        Grid::<4>::construct(
            [
                [cc, cc, cc, cc, cc, cc].into(),
                [cc,  4,  3,  1,  2, cc].into(),
                [cc,  1,  2,  4,  3, cc].into(),
                [cc,  2,  1,  3,  4, cc].into(),
                [cc,  3,  4,  2,  1, cc].into(),
                [cc, cc, cc, cc, cc, cc].into(),
            ]
        ).cells
    );
}

#[test] pub fn solve_5x5()
{
    let cc = 0;
    
    assert_eq!(
        Solver::solve(examples::grid_5x5_full_easy_1()).cells,
        Grid::<5>::construct(
            [
                [cc, cc, cc, cc, cc, cc, cc].into(),
                [cc,  4,  3,  5,  1,  2, cc].into(),
                [cc,  1,  5,  4,  2,  3, cc].into(),
                [cc,  2,  4,  1,  3,  5, cc].into(),
                [cc,  5,  2,  3,  4,  1, cc].into(),
                [cc,  3,  1,  2,  5,  4, cc].into(),
                [cc, cc, cc, cc, cc, cc, cc].into(),
            ]
        ).cells
    );
}

#[test] pub fn solve_6x6()
{
    let cc = 0;
    
    // assert_eq!(
    //     Solver::solve(examples::grid_6x6_full_easy_1()).cells,
    //     Grid::<6>::construct(
    //         [
    //             [cc, cc, cc, cc, cc, cc, cc, cc].into(),
    //             [cc,  1,  6,  3,  4,  2,  5, cc].into(),
    //             [cc,  6,  5,  4,  2,  3,  1, cc].into(),
    //             [cc,  5,  3,  1,  6,  4,  2, cc].into(),
    //             [cc,  2,  1,  5,  3,  6,  4, cc].into(),
    //             [cc,  4,  2,  6,  5,  1,  3, cc].into(),
    //             [cc,  3,  4,  2,  1,  5,  6, cc].into(),
    //             [cc, cc, cc, cc, cc, cc, cc, cc].into(),
    //         ]
    //     ).cells
    // );
    
    assert_eq!(
        Solver::solve(examples::grid_6x6_full_hard_1()).cells,
        Grid::<6>::construct(
            [
                [cc, cc, cc, cc, cc, cc, cc, cc].into(),
                [cc,  1,  6,  3,  4,  2,  5, cc].into(),
                [cc,  6,  5,  4,  2,  3,  1, cc].into(),
                [cc,  5,  3,  1,  6,  4,  2, cc].into(),
                [cc,  2,  1,  5,  3,  6,  4, cc].into(),
                [cc,  4,  2,  6,  5,  1,  3, cc].into(),
                [cc,  3,  4,  2,  1,  5,  6, cc].into(),
                [cc, cc, cc, cc, cc, cc, cc, cc].into(),
            ]
        ).cells
    );
}
