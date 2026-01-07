use ascendant::*;


fn main()
{
    let grid: Grid<5> = Grid::construct(
        [
            [ 0,  1, 2, 3, 4, 5,  0 ].into(),

            [ 1,  0, 0, 0, 0, 0,  1 ].into(),
            [ 2,  0, 0, 0, 0, 0,  2 ].into(),
            [ 3,  0, 0, 0, 0, 0,  3 ].into(),
            [ 4,  0, 0, 0, 0, 0,  4 ].into(),
            [ 5,  0, 0, 0, 0, 0,  5 ].into(),

            [ 0,  1, 2, 3, 4, 5,  0 ].into(),
        ].into()
    );

    println!("{grid:?}");

    let res = Solver::solve(grid);

    println!("{res:?}");
}
