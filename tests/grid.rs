use ascendant::*;
use ascendant::{
    Cell::Solved as Sv,
};


fn test_grid() -> Grid<5> {
    Grid::construct(
        [
            [ 0,   1,  2,  3,  4,  5,   0 ].into(),

            [ 1,  00, 10, 20, 30, 40,   1 ].into(),
            [ 2,  01, 11, 21, 31, 41,   2 ].into(),
            [ 3,  02, 12, 22, 32, 42,   3 ].into(),
            [ 4,  03, 13, 23, 33, 43,   4 ].into(),
            [ 5,  04, 14, 24, 34, 44,   5 ].into(),

            [ 0,   1,  2,  3,  4,  5,   0 ].into(),
        ].into()
    )
}

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
        grid.cells,
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
        grid.cells,
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
            [ 0, 10, 10, 10,  0].into(),
            [10,  0,  0,  0, 10].into(),
            [10,  0,  0,  0, 10].into(),
            [10,  0,  0,  0, 10].into(),
            [ 0, 10, 10, 10,  0].into(),
        ].into()
    );
    assert_eq!(
        grid.clues,
        Clues {
            upper: [Some(10), Some(10), Some(10)],
            lower: [Some(10), Some(10), Some(10)],
            left:  [Some(10), Some(10), Some(10)],
            right: [Some(10), Some(10), Some(10)],
        }
    );

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
        grid.clues,
        Clues {
            upper: [None, Some(1), None],
            lower: [None, None,    None],
            left:  [None, Some(2), Some(3)],
            right: [None, None,    Some(4)],
        }
    );
}

#[test] pub fn index()
{
    let grid = test_grid();

    assert_eq!( *grid.at(0, 0), pen!(1,2,3,4,5) );
    assert_eq!( *grid.at(0, 1), Sv(01) );
    assert_eq!( *grid.at(1, 0), Sv(10) );
    assert_eq!( *grid.at(1, 1), Sv(11) );
}

#[test] pub fn look()
{
    let mut grid = test_grid();

    let (clue, row) = grid.look_right(1);
    assert_eq!( clue, Some(2) );
    assert_eq!( row, [&Sv(01), &Sv(11), &Sv(21), &Sv(31), &Sv(41)] );

    let (clue, row) = grid.look_left(3);
    assert_eq!( clue, Some(4) );
    assert_eq!( row, [&Sv(43), &Sv(33), &Sv(23), &Sv(13), &Sv(03)] );

    let (clue, col) = grid.look_down(2);
    assert_eq!( clue, Some(3) );
    assert_eq!( col, [&Sv(20), &Sv(21), &Sv(22), &Sv(23), &Sv(24)] );

    let (clue, col) = grid.look_up(4);
    assert_eq!( clue, Some(5) );
    assert_eq!( col, [&Sv(44), &Sv(43), &Sv(42), &Sv(41), &Sv(40)] );
}
