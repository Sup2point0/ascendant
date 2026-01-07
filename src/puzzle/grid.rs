use std::*;

use crate::*;


pub struct Grid<const N: usize>
{
    pub cells: [[Cell; N]; N],
    pub clues: Clues<N>,
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
                else             { Some(Cell::Pencil( Some((1..=N as Digit).collect()) )) }
            );

        utils::as_array(row)
    }
}

impl<const N: usize> Grid<N>
{
    pub fn at(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }

    pub fn look_right(&mut self, row: usize) -> (Option<Digit>, [&mut Cell; N]) {
        ( self.clues.left[row], utils::as_array(self.cells[row].iter_mut()) )
    }
    pub fn look_left(&mut self, row: usize) -> (Option<Digit>, [&mut Cell; N]) {
        ( self.clues.right[row], utils::as_array(self.cells[row].iter_mut().rev()) )
    }

    pub fn look_down(&mut self, col: usize) -> (Option<Digit>, [&mut Cell; N]) {
        ( self.clues.upper[col], utils::as_array(self.cells.iter_mut().map(|row| &mut row[col])) )
    }
    pub fn look_up(&mut self, col: usize) -> (Option<Digit>, [&mut Cell; N]) {
        ( self.clues.upper[col], utils::as_array(self.cells.iter_mut().rev().map(|row| &mut row[col])) )
    }
}

impl<const N: usize> fmt::Debug for Grid<N>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
            self.cells.iter()
                .map(|row|
                    row.iter()
                        .map(|cell| Cell::render::<N>(cell))
                        .collect::<Vec<_>>()
                        .join(" | ")
                )
                .map(|row| format!("| {row} |"))
                .chain(iter::once(iter::repeat_n('-', N * (N+5) + 1).collect::<String>()))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
