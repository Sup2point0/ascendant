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

    fn lane_into<const N: usize>(lane: impl IntoIterator<Item = Digit>) -> [Option<Digit>; N]
    {
        util::arr(
            lane.into_iter()
                .map(|clue| if clue == 0 {None} else {Some(clue)})
        )
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

impl<const N: usize> Into<Clues<N>> for CluesExchange
{
    fn into(self) -> Clues<N> {
        Clues {
            upper: Self::lane_into(self.upper),
            lower: Self::lane_into(self.lower),
            left:  Self::lane_into(self.left),
            right: Self::lane_into(self.right),
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

    fn digit_into_cell<const N: usize>(digit: Digit) -> Cell
    {
        if digit == 0 { Cell::new(N) }
        else          { Cell::Solved(digit) }
    }
}

impl<const N: usize> From<Grid<N>> for GridExchange
{
    fn from(grid: Grid<N>) -> Self
    {
        Self {
            url: grid.url.expect("Saved grid should have a URL"),
            
            cells:
                grid.cells.into_iter()
                    .map(|row|
                        row.into_iter()
                            .map(Self::digit_from_cell)
                            .collect_vec()
                    )
                    .collect_vec(),

            clues: CluesExchange::from(grid.clues),
        }
    }
}

impl<const N: usize> Into<Grid<N>> for GridExchange
{
    fn into(self) -> Grid<N> {
        Grid {
            url: Some(self.url),
            cells: util::arr(
                self.cells.into_iter()
                    .map(|row| util::arr(
                        row.into_iter()
                            .map(Self::digit_into_cell::<N>)
                    ))
            ),
            clues: self.clues.into(),
        }
    }
}
