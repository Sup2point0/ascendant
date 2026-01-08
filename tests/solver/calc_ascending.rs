use ascendant::*;
use crate::*;


#[test] fn calc_ascending_46()
{
    // 4 | _ _ _ 6 _ _
    let last_index = 3;
    let target = 5;
    assert_eq!( Solver::<6>::calc_ascending(target, 0, last_index), set![1,2,3] );
    assert_eq!( Solver::<6>::calc_ascending(target, 1, last_index), set![  2,3,4] );
    assert_eq!( Solver::<6>::calc_ascending(target, 2, last_index), set![    3,4,5] );

    // 4 | _ _ 5 _ _ _
    let last_index = 2;
    let target = 4;
    assert_eq!( Solver::<6>::calc_ascending(target, 0, last_index), set![1,2,3] );
    assert_eq!( Solver::<6>::calc_ascending(target, 1, last_index), set![  2,3,4] );

    // 4 | _ 4 _ _ _ _
    let last_index = 1;
    let target = 3;
    assert_eq!( Solver::<6>::calc_ascending(target, 0, last_index), set![1,2,3] );
}
