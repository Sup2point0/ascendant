use std::*;

use itertools::*;

use crate::utils;


type Digit = u8;


#[derive(Clone, PartialEq, Eq)]
pub enum Cell {
    Solved(Digit),
    Pencil(collections::HashSet<Digit>),
}

#[macro_export]
macro_rules! pen {
    ( $($digit: expr),* $(,)? ) =>
    {
        Cell::Pencil(std::collections::HashSet::from([$( $digit, )*]))
    }
}

impl fmt::Debug for Cell
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Solved(d)  => format!("'{d}'"),
            Self::Pencil(ds) => format!("[{}]", ds.iter().map(|n| n.to_string()).collect::<Vec<String>>().join("")),
        })
    }
}


#[derive(PartialEq, Eq)]
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

impl<const N: usize> fmt::Debug for Clues<N>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        [self.upper, self.left, self.right, self.lower]
            .concat()
            .fmt(f)
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
                    0             => { Self::prep_clue_row(row, &mut clues.upper); None }
                    _ if y == N+1 => { Self::prep_clue_row(row, &mut clues.lower); None }
                    _             => { Some( Self::prep_row(y-1, row, &mut clues) ) }
                });

        Self {
            cells: utils::as_array(cells),
            clues,
        }
    }
}

impl<const N: usize> Grid<N>
{
    fn is_edge(xy: usize) -> bool {
        xy == 0 || xy == N+1
    }

    fn clue_from(n: Digit) -> Option<Digit> {
        if n > 0 { Some(n) } else { None }
    }

    fn prep_clue_row(
        row: [Digit; N + 2],
        clue_row: &mut [Option<Digit>; N]
    ) -> ()
    {
        let row = row.into_iter().enumerate()
            .filter_map(|(x, n)|
                if Self::is_edge(x) { None }
                else                { Some(Self::clue_from(n)) }
            );

        *clue_row = utils::as_array(row);
    }

    fn prep_row(
        y: usize,
        row: [Digit; N + 2],
        clues: &mut Clues<N>,
    ) -> [Cell; N]
    {
        let row = row.into_iter().enumerate()
            .filter_map(|(x, n)|
                if      x == 0   { if n > 0 { clues.left[y] = Some(n); } None }
                else if x == N+1 { if n > 0 { clues.right[y] = Some(n); } None }
                else if n > 0    { Some(Cell::Solved(n)) }
                else             { Some(Cell::Pencil((1..=N as Digit).collect())) }
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

    pub fn look_right(&self, row: usize) -> [&Cell; N] {
        utils::as_array(self.cells[row].iter())
    }
}
