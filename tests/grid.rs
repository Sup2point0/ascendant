use ascendant::*;
use ascendant::{
    Cell::Solved as Sv,
};


#[test] pub fn construct_grid()
{
    let grid: Grid<3> = Grid::construct(
        [
            [0,  0,  0,  0, 0].into(),
            [0, 10, 20, 30, 0].into(),
            [0, 40, 50, 60, 0].into(),
            [0, 70, 80, 90, 0].into(),
            [0,  0,  0,  0, 0].into(),
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
    
    let grid: Grid<3> = Grid::construct(
        [
            [0,  0,  0,  0, 0].into(),
            [0,  0, 20, 30, 0].into(),
            [0, 40,  0, 60, 0].into(),
            [0, 70,  0,  0, 0].into(),
            [0,  0,  0,  0, 0].into(),
        ].into()
    );
    assert_eq!(
        *grid.cells(),
        [
            [pen!(1,2,3), Sv(20),      Sv(30)     ],
            [Sv(40),      pen!(1,2,3), Sv(60)     ],
            [Sv(70),      pen!(1,2,3), pen!(1,2,3)],
        ]
    );
}

#[test] pub fn construct_clues()
{
    let grid: Grid<3> = Grid::construct(
        [
            [0, 0, 1, 0, 0].into(),
            [0, 0, 0, 0, 0].into(),
            [2, 0, 0, 0, 0].into(),
            [3, 0, 0, 0, 4].into(),
            [0, 0, 0, 0, 0].into(),
        ].into()
    );
    assert_eq!(
        *grid.clues(),
        Clues {
            upper: [None, Some(1), None],
            lower: [None, None,    None],
            left:  [None, Some(2), Some(3)],
            right: [None, None,    Some(4)],
        }
    );
}
