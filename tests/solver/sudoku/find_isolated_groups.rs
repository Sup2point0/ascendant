use maplit::hashmap;
use natbitset::*;
use natbitset::byteset as b;

use ascendant::*;


#[test] fn find_isolated_groups()
{
    // can merge
    assert_eq!(
        Solver::find_isolated_groups([
            b![1,2],
            b![1,2],
        ]),
        hashmap! {
            b![1,2] => vec![b![1,2], b![1,2]],
        }
    );
    
    // can ignore
    assert_eq!(
        Solver::find_isolated_groups([
            b![1,2],
            b![1,2],
            b![3,4],
        ]),
        hashmap! {
            b![1,2] => vec![b![1,2], b![1,2]],
        }
    );
    
    // can chain merge
    assert_eq!(
        Solver::find_isolated_groups([
            b![1,2],
            b![2,3],
            b![3,1],
        ]),
        hashmap! {
            b![1,2,3] => vec![b![1,2], b![2,3], b![3,1]],
        }
    );
    
    // should cross merge
    assert_eq!(
        Solver::find_isolated_groups([
            b![1,2],
            b![1,2],
            b![3,4],
            b![3,4],
        ]),
        hashmap! {
            b![1,2] => vec![b![1,2], b![1,2]],
            b![3,4] => vec![b![3,4], b![3,4]],
            b![1,2,3,4] => vec![b![1,2], b![1,2], b![3,4], b![3,4]],
        }
    );
}
