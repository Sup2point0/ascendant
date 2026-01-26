use std::collections::HashSet;

use natbitset::*;

use ascendant::*;


#[test] fn find_isolates()
{
    assert!(
        Solver::find_isolates([
            byteset![1,2],
            byteset![1,2],
            byteset![3,4],
        ])
        .into_iter().eq(
            vec![
                HashSet::from([byteset![1,2]])
            ]
        )
    );
}
