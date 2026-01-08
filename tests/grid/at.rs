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
