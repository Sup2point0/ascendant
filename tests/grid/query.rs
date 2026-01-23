use ascendant::*;
use ascendant::{
    Cell::Solved as Sv,
};
use crate::*;


#[test] fn is_solved()
{
    assert!( !test_grid().is_solved() );
    assert!( examples::sol_9x9_full_1().is_solved() );

    let invalid_grid = Grid::<9>::construct([
        [ 00,   1,  2,  3,  4,  5,  6,  7,  8,  9,  00 ],
        [  1,   9,  6,  8,  3,  1,  4,  2,  7,  5,   1 ],
        [  2,   7,  1,  5,  6,  8,  2,  9,  4,  3,   2 ],
        [  3,   4,  5,  3,  7,  6,  1,  8,  9,  2,   3 ],
        [  4,   3,  9,  4,  2,  7,  6,  5,  1,  8,   4 ],
        [  5,   1,  7,  6,  8,  5,  3,  4,  2,  9,   5 ],
        [  6,   2,  8,  7,  5,  3,  9,  1,  6,  4,   6 ],
        [  7,   5,  3,  2,  4,  9,  7,  6,  8,  1,   7 ],
        [  8,   6,  2,  9,  1,  4,  8,  3,  5,  7,   8 ],
        [  9,   8,  4,  1,  9,  2,  5,  7,  3,  6,   9 ],
        [ 00,   1,  2,  3,  4,  5,  6,  7,  8,  9,  00 ],
    ]);

    assert!( !invalid_grid.is_solved() );
}

#[test] pub fn at()
{
    let grid = test_grid();

    assert_eq!( grid.at(0, 0), p!(1,2,3,4,5) );
    assert_eq!( grid.at(0, 1), Sv(01) );
    assert_eq!( grid.at(1, 0), Sv(10) );
    assert_eq!( grid.at(1, 1), Sv(11) );
}

#[test] pub fn look()
{
    let mut grid = test_grid();

    let (clue, row) = grid.look_right_mut(1);
    assert_eq!( clue, Some(2) );
    assert_eq!( row, [&Sv(01), &Sv(11), &Sv(21), &Sv(31), &Sv(41)] );

    let (clue, row) = grid.look_left_mut(3);
    assert_eq!( clue, Some(9) );
    assert_eq!( row, [&Sv(43), &Sv(33), &Sv(23), &Sv(13), &Sv(03)] );

    let (clue, col) = grid.look_down_mut(2);
    assert_eq!( clue, Some(3) );
    assert_eq!( col, [&Sv(20), &Sv(21), &Sv(22), &Sv(23), &Sv(24)] );

    let (clue, col) = grid.look_up_mut(4);
    assert_eq!( clue, Some(10) );
    assert_eq!( col, [&Sv(44), &Sv(43), &Sv(42), &Sv(41), &Sv(40)] );
}

#[test] pub fn look_across()
{
    let grid = test_grid();

    let (clue_start, row, clue_end) = grid.look_across_row(1);
    assert_eq!( clue_start, Some(2) );
    assert_eq!( clue_end, Some(7) );
    assert_eq!( row, [Sv(01), Sv(11), Sv(21), Sv(31), Sv(41)] );

    let (clue_start, col, clue_end) = grid.look_across_col(3);
    assert_eq!( clue_start, Some(4) );
    assert_eq!( clue_end, Some(9) );
    assert_eq!( col, [Sv(30), Sv(31), Sv(32), Sv(33), Sv(34)] );
}
