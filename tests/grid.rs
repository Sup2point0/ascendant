use ascendant::puzzle::*;
use ascendant::puzzle::Cell::{Solved as Sv};


#[test] pub fn construct()
{
    let grid: Grid<3> = Grid::construct(
        [
            [0, 0,  0,  0,  0].into(),
            [0, 10, 20, 30, 0].into(),
            [0, 40, 50, 60, 0].into(),
            [0, 70, 80, 90, 0].into(),
            [0, 0,  0,  0,  0].into(),
        ].into()
    );

    assert_eq!(
        *grid.cells(),
        [
            [Sv(10), Sv(20), Sv(30)],
            [Sv(40), Sv(50), Sv(60)],
            [Sv(70), Sv(80), Sv(90)],
        ]
    );
}
