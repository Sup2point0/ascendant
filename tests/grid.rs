use ascendant::puzzle::*;


#[test] pub fn construct()
{
    let grid = Grid::construct(
        [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ]
    );

    assert_eq!(
        grid.cells,
        vec![1..=9]
    );
}
