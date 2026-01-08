use ascendant::*;


#[test] pub fn solve()
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
}
