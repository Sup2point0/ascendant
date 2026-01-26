use ascendant::*;
use ascendant::Cell::Solved as Sv;


#[test] fn isolate_groups_in_lane()
{
    const N: usize = 6;

    // [12] [12] [123] 4 5 6
    let mut lane = [ p![1,2], p![12], p![1;3], Sv(4), Sv(5), Sv(6) ];

    Solver::<N>::isolate_groups_in_lane(util::arr(lane.iter_mut()));
    assert_eq!(lane, [ p![1,2], p![1,2], p![3], Sv(4), Sv(5), Sv(6) ]);
}
