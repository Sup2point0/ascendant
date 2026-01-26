use ascendant::*;

use crate::*;


// == 4x4 == //

#[test] pub fn solve_4x4_full()
{
    let cc = 0;
    
    assert_eq!(
        Solver::solve(examples::grid_4x4_full_1()).cells,
        Grid::<4>::construct(
            [
                [cc, cc, cc, cc, cc, cc],
                [cc,  2,  3,  4,  1, cc],
                [cc,  4,  1,  2,  3, cc],
                [cc,  3,  4,  1,  2, cc],
                [cc,  1,  2,  3,  4, cc],
                [cc, cc, cc, cc, cc, cc],
            ]
        ).cells
    );
}

#[test] pub fn solve_4x4_sparse()
{
    let cc = 0;
    
    assert_eq!(
        Solver::solve(examples::grid_4x4_sparse_1()).cells,
        Grid::<4>::construct(
            [
                [cc, cc, cc, cc, cc, cc],
                [cc,  4,  3,  1,  2, cc],
                [cc,  1,  2,  4,  3, cc],
                [cc,  2,  1,  3,  4, cc],
                [cc,  3,  4,  2,  1, cc],
                [cc, cc, cc, cc, cc, cc],
            ]
        ).cells
    );
    
    assert_eq!(
        Solver::solve(examples::grid_4x4_sparse_2()).cells,
        Grid::<4>::construct(
            [
                [cc, cc, cc, cc, cc, cc],
                [cc,  3,  2,  4,  1, cc],
                [cc,  2,  3,  1,  4, cc],
                [cc,  1,  4,  2,  3, cc],
                [cc,  4,  1,  3,  2, cc],
                [cc, cc, cc, cc, cc, cc],
            ]
        ).cells
    );
}


// == 5x5 == //

#[test] pub fn solve_5x5_full()
{
    let cc = 0;
    
    assert_eq!(
        Solver::solve(examples::grid_5x5_full_easy_1()).cells,
        Grid::<5>::construct(
            [
                [cc, cc, cc, cc, cc, cc, cc],
                [cc,  4,  3,  5,  1,  2, cc],
                [cc,  1,  5,  4,  2,  3, cc],
                [cc,  2,  4,  1,  3,  5, cc],
                [cc,  5,  2,  3,  4,  1, cc],
                [cc,  3,  1,  2,  5,  4, cc],
                [cc, cc, cc, cc, cc, cc, cc],
            ]
        ).cells
    );
    
    assert_eq!(
        Solver::solve(examples::grid_5x5_full_hard_1()).cells,
        Grid::<5>::construct(
            [
                [cc, cc, cc, cc, cc, cc, cc],
                [cc,  2,  3,  5,  4,  1, cc],
                [cc,  3,  1,  4,  5,  2, cc],
                [cc,  4,  2,  3,  1,  5, cc],
                [cc,  5,  4,  1,  2,  3, cc],
                [cc,  1,  5,  2,  3,  4, cc],
                [cc, cc, cc, cc, cc, cc, cc],
            ]
        ).cells
    );
}

#[test] pub fn solve_5x5_sparse()
{
    let cc = 0;
    
    assert_eq!(
        Solver::solve(examples::grid_5x5_sparse_1()).cells,
        Grid::<5>::construct(
            [
                [cc, cc, cc, cc, cc, cc, cc],
                [cc,  3,  2,  5,  4,  1, cc],
                [cc,  1,  5,  4,  3,  2, cc],
                [cc,  4,  1,  3,  2,  5, cc],
                [cc,  2,  4,  1,  5,  3, cc],
                [cc,  5,  3,  2,  1,  4, cc],
                [cc, cc, cc, cc, cc, cc, cc],
            ]
        ).cells
    );
}


// == 6x6 == //

#[test] pub fn solve_6x6_full()
{
    let cc = 0;
    
    assert_eq!(
        Solver::solve(examples::grid_6x6_full_easy_1()).cells,
        Grid::<6>::construct(
            [
                [cc, cc, cc, cc, cc, cc, cc, cc],
                [cc,  1,  6,  3,  4,  2,  5, cc],
                [cc,  6,  5,  4,  2,  3,  1, cc],
                [cc,  5,  3,  1,  6,  4,  2, cc],
                [cc,  2,  1,  5,  3,  6,  4, cc],
                [cc,  4,  2,  6,  5,  1,  3, cc],
                [cc,  3,  4,  2,  1,  5,  6, cc],
                [cc, cc, cc, cc, cc, cc, cc, cc],
            ]
        ).cells
    );
    
    assert_eq!(
        Solver::solve(examples::grid_6x6_full_hard_1()).cells,
        Grid::<6>::construct(
            [
                [cc, cc, cc, cc, cc, cc, cc, cc],
                [cc,  2,  6,  4,  5,  3,  1, cc],
                [cc,  3,  4,  5,  1,  2,  6, cc],
                [cc,  5,  1,  3,  2,  6,  4, cc],
                [cc,  4,  5,  2,  6,  1,  3, cc],
                [cc,  1,  2,  6,  3,  4,  5, cc],
                [cc,  6,  3,  1,  4,  5,  2, cc],
                [cc, cc, cc, cc, cc, cc, cc, cc],
            ]
        ).cells
    );
}


// == 7x7 == //

#[test] pub fn solve_7x7_full()
{
    let cc = 0;
    
    assert_eq!(
        Solver::solve(examples::grid_7x7_full_easy_1()).cells,
        Grid::<7>::construct(
            [
                [ cc, cc, cc, cc, cc, cc, cc, cc, cc ],
                [ cc,  5,  3,  7,  1,  4,  6,  2, cc ],
                [ cc,  6,  2,  5,  4,  3,  7,  1, cc ],
                [ cc,  1,  4,  6,  5,  7,  2,  3, cc ],
                [ cc,  7,  6,  4,  3,  2,  1,  5, cc ],
                [ cc,  3,  5,  1,  2,  6,  4,  7, cc ],
                [ cc,  4,  1,  2,  7,  5,  3,  6, cc ],
                [ cc,  2,  7,  3,  6,  1,  5,  4, cc ],
                [ cc, cc, cc, cc, cc, cc, cc, cc, cc ],
            ]
        ).cells
    );
    
    assert_eq!(
        Solver::solve(examples::grid_7x7_full_hard_1()).cells,
        Grid::<7>::construct(
            [
                [ cc, cc, cc, cc, cc, cc, cc, cc, cc ],
                [ cc,  6,  5,  1,  2,  4,  7,  3, cc ],
                [ cc,  3,  1,  6,  7,  2,  4,  5, cc ],
                [ cc,  4,  2,  7,  5,  6,  3,  1, cc ],
                [ cc,  2,  6,  5,  4,  3,  1,  7, cc ],
                [ cc,  7,  4,  3,  6,  1,  5,  2, cc ],
                [ cc,  5,  3,  2,  1,  7,  6,  4, cc ],
                [ cc,  1,  7,  4,  3,  5,  2,  6, cc ],
                [ cc, cc, cc, cc, cc, cc, cc, cc, cc ],
            ]
        ).cells
    );
}


// == 8x8 == //

#[test] pub fn solve_8x8_full()
{    
    // assert_eq!(
    //     Solver::solve(examples::grid_8x8_full_easy_1()).cells,
    //    examples::sol_8x8_full_easy_1().cells
    // );
}


// == 9x9 == //

#[test] pub fn solve_9x9_full()
{    
    // assert_eq!(
    //     Solver::solve(examples::grid_9x9_full_1()).cells,
    //     examples::sol_9x9_full_1().cells
    // );
}
