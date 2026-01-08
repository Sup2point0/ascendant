use ascendant::*;
use ascendant::{
    Cell::Solved as Sv,
};
use crate::*;


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
    assert_eq!( clue, Some(9) );
    assert_eq!( row, [&Sv(43), &Sv(33), &Sv(23), &Sv(13), &Sv(03)] );

    let (clue, col) = grid.look_down(2);
    assert_eq!( clue, Some(3) );
    assert_eq!( col, [&Sv(20), &Sv(21), &Sv(22), &Sv(23), &Sv(24)] );

    let (clue, col) = grid.look_up(4);
    assert_eq!( clue, Some(10) );
    assert_eq!( col, [&Sv(44), &Sv(43), &Sv(42), &Sv(41), &Sv(40)] );
}
