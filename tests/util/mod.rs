use ascendant::*;


#[macro_export]
macro_rules! set {
    ( $($digit: expr),* $(,)? ) =>
    {
        std::collections::HashSet::from(
            [ $( $digit, )* ]
        )
    };
}


pub fn test_grid() -> Grid<5> {
    Grid::construct(
        [
            [ 0,   1,  2,  3,  4,  5,   0 ],

            [ 1,  00, 10, 20, 30, 40,   6 ],
            [ 2,  01, 11, 21, 31, 41,   7 ],
            [ 3,  02, 12, 22, 32, 42,   8 ],
            [ 4,  03, 13, 23, 33, 43,   9 ],
            [ 5,  04, 14, 24, 34, 44,  10 ],

            [ 0,   6,  7,  8,  9, 10,   0 ],
        ].into()
    )
}
