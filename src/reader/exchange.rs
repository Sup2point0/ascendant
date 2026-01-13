use itertools::*;

use crate::*;


#[derive(serde::Serialize, serde::Deserialize)]
pub struct CluesExchange
{
    pub upper: Vec<Digit>,
    pub lower: Vec<Digit>,
    pub left:  Vec<Digit>,
    pub right: Vec<Digit>,
}

impl CluesExchange
{
    fn lane_from(lane: impl IntoIterator<Item = Option<Digit>>) -> Vec<Digit>
    {
        lane.into_iter()
            .map(|clue| clue.unwrap_or(0))
            .collect_vec()
    }
}

impl<const N: usize> From<Clues<N>> for CluesExchange
{
    fn from(clues: Clues<N>) -> Self
    {
        Self {
            upper: Self::lane_from(clues.upper),
            lower: Self::lane_from(clues.lower),
            left:  Self::lane_from(clues.left),
            right: Self::lane_from(clues.right),
        }
    }
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct GridExchange
{
    pub url: String,
    pub cells: Vec<Vec<Digit>>,
    pub clues: CluesExchange,
}

impl GridExchange
{
    fn digit_from_cell(cell: Cell) -> Digit
    {
        match cell {
            Cell::Solved(digit) => digit,
            Cell::Pencil(..)    => 0,
        }
    }
}

impl<const N: usize> From<(String, Grid<N>)> for GridExchange
{
    fn from((url, grid): (String, Grid<N>)) -> Self
    {
        Self {
            url,
            
            cells:
                grid.cells.into_iter()
                    .map(|row|
                        row.into_iter()
                            .map(|cell|
                                Self::digit_from_cell(cell)
                            )
                        .collect_vec()
                    )
                    .collect_vec(),

            clues: CluesExchange::from(grid.clues),
        }
    }
}
