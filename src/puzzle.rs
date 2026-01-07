use std::*;

use itertools::*;

use crate::utils;


type Digit = u8;


#[derive(Clone, PartialEq, Eq)]
pub enum Cell {
    Solved(Digit),
    Pencil(collections::HashSet<Digit>),
}

impl fmt::Debug for Cell
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Solved(d)  => format!(" {d} "),
            Self::Pencil(ds) => format!("{}", ds.iter().map(|n| n.to_string()).collect::<Vec<String>>().join("")),
        })
    }
}


pub struct Clues<const N: usize>
{
    pub upper: [Option<Digit>; N],
    pub lower: [Option<Digit>; N],
    pub left:  [Option<Digit>; N],
    pub right: [Option<Digit>; N],
}

impl<const N: usize> Clues<N>
{
    pub fn new() -> Self
    {
        Self {
            upper: [None; N],
            left:  [None; N], right: [None; N],
            lower: [None; N],
        }
    }
}


pub struct Grid<const N: usize>
{
    cells: [[Cell; N]; N],
    clues: Clues<N>,
}

impl<const N: usize> Grid<N>
{
    pub fn construct(data: [[Digit; N+2]; N+2]) -> Self
    {
        let mut clues = Clues::new();

        let cells =
            data.into_iter()
                .enumerate()
                .filter_map(|(y, row)| match y {
                    0              => { Self::prep_clue_row(row, &mut clues.upper); None }
                    _ if y == N - 1 => { Self::prep_clue_row(row, &mut clues.lower); None }
                    _              => { Some( Self::prep_row(y, row, &mut clues) ) }
                });

        Self {
            cells: utils::as_array(cells),
            clues,
        }
    }

    fn prep_clue_row(
        row: [Digit; N + 2],
        clue_row: &mut [Option<Digit>; N]
    ) -> ()
    {
        let row = row.into_iter().enumerate()
            .filter_map(|(x, n)|
                if x == 0 || x == N - 1 {
                    None
                } else {
                    Some( if n > 0 { Some(n) } else { None } )
                });

        *clue_row = utils::as_array(row);
    }

    fn prep_row(
        x: usize,
        row: [Digit; N + 2],
        clues: &mut Clues<N>,
    ) -> [Cell; N]
    {
        let row = row.into_iter().enumerate()
            .filter_map(|(y, n)|
                if      y == 0    { clues.left[x - 1] = Some(n); None }
                else if y == N - 1 { clues.right[x - 1] = Some(n); None }
                else if n > 0     { Some(Cell::Solved(n)) }
                else              { Some(Cell::Pencil((1..=N as Digit).collect())) }
            );

        utils::as_array(row)
    }
}

impl<const N: usize> Grid<N>
{
    pub fn cells(&self) -> &[[Cell; N]; N] {
        &self.cells
    }

    pub fn clues(&self) -> &Clues<N> {
        &self.clues
    }
}

impl<const N: usize> Grid<N>
{
    pub fn at(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }

    // pub fn look_right(&self, row: usize) -> [&Cell; N] {
    //     self.cells[row]
    // }
}
