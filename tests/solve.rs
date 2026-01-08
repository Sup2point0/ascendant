use ascendant::*;


#[test] pub fn solve_4x4()
{
    let cc = 0;
    
    assert_eq!(
        Solver::solve(examples::grid_4x4_easy()).cells,
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
        Solver::solve(examples::grid_4x4_hard_1()).cells,
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
