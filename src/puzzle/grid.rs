use std::*;

use crate::*;


#[derive(PartialEq, Eq)]
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
            cells: util::arr(cells),
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

        *clue_row = util::arr(row);
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
                else             { Some(Cell::Pencil( Some((1..=N).collect()) )) }
            );

        util::arr(row)
    }
}

impl<const N: usize> Grid<N>
{
    pub fn at(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }
    
    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y][x]
    }

    pub fn look_right(&mut self, row: usize) -> (Option<Digit>, [&mut Cell; N]) {
        ( self.clues.left[row], util::arr(self.cells[row].iter_mut()) )
    }
    pub fn look_left(&mut self, row: usize) -> (Option<Digit>, [&mut Cell; N]) {
        ( self.clues.right[row], util::arr(self.cells[row].iter_mut().rev()) )
    }

    pub fn look_down(&mut self, col: usize) -> (Option<Digit>, [&mut Cell; N]) {
        ( self.clues.upper[col], util::arr(self.cells.iter_mut().map(|row| &mut row[col])) )
    }
    pub fn look_up(&mut self, col: usize) -> (Option<Digit>, [&mut Cell; N]) {
        ( self.clues.lower[col], util::arr(self.cells.iter_mut().rev().map(|row| &mut row[col])) )
    }
}

impl<const N: usize> fmt::Debug for Grid<N>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let col_width = N + 5;

        // I don't even know anymore...
        write!(f, "{}",
            iter::once(
                format!("{}| {} |",
                    util::rep(' ', 3),
                    self.clues.upper.clone()
                        .map(|clue|
                            clue.map(|c| format!(" {}{} ", util::rep(' ', N-1), c.to_string()))
                                .unwrap_or(util::rep(' ', N+2).to_string())
                        ).join(" | ")
                )
            )
            .chain(
                iter::once( util::rep('-', N * col_width + 7) )
            ).chain(
                self.cells.iter().enumerate()
                    .map(|(i, row)|
                        iter::once(
                            self.clues.left.clone()[i].map(|c| c.to_string()).unwrap_or(" ".to_string())
                        )
                        .chain(
                            row.iter()
                                .map(|cell| Cell::render::<N>(cell))
                        )
                        .chain(
                            iter::once( 
                                self.clues.right.clone()[i].map(|c| c.to_string()).unwrap_or(" ".to_string())
                            )
                        )
                        .collect::<Vec<_>>()
                        .join(" | ")
                    )
                    .map(|row| format!(" {row} "))
                    .chain(iter::once( util::rep('-', N * col_width + 7) ))
                    .chain(iter::once( format!(
                        "{}| {} |",
                        util::rep(' ', 3),
                        self.clues.lower.clone()
                            .map(|clue|
                                clue.map(|c| format!(" {}{} ", util::rep(' ', N-1), c.to_string()))
                                    .unwrap_or(util::rep(' ', N+2).to_string())
                            ).join(" | ")
                    ) ))
            )
            .collect::<Vec<_>>()
            .join("\n")
        )
    }
}
