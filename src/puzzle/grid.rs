use std::*;
use std::collections::{
    HashMap,
    HashSet,
};

use crate::*;


#[derive(PartialEq, Eq)]
pub struct Grid<const N: usize>
{
    pub url:   Option<String>,
    pub cells: [[Cell; N]; N],
    pub clues: Clues<N>,
}

// == CONSTRUCTORS == //
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
            url: None,
            cells: util::arr(cells),
            clues,
        }
    }

    pub fn try_construct<I,J>(data: I, url: Option<String>) -> Self
        where
            I: IntoIterator<Item = J>,
            J: IntoIterator<Item = Digit>,
            [(); N+2]:
    {
        let mut clues = Clues::new();

        let cells =
            data.into_iter()
                .enumerate()
                .filter_map(|(y, row)| {
                    let row = util::arr(row);
                    match y {
                        0             => { Self::prep_clue_row(row, &mut clues.upper); None }
                        _ if y == N+1 => { Self::prep_clue_row(row, &mut clues.lower); None }
                        _             => { Some( Self::prep_row(y-1, row, &mut clues) ) }
                    }
                });

        Self {
            url,
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

// == QUERY == //
impl<const N: usize> Grid<N>
{
    pub fn is_solved(&self) -> bool
    {
        (0..N).all(|y| Self::validate_lane(self.look_across_row(y)))
        && (0..N).all(|x| Self::validate_lane(self.look_across_col(x)))
    }

    fn validate_lane((clue_start, lane, clue_end): (Option<Digit>, [&Cell; N], Option<Digit>)) -> bool
    {
        let mut invalid = false;

        invalid |=
            lane.iter()
                .map(|cell| cell.digit())
                .collect::<HashSet<_>>()
            !=
                (1..=N).collect::<HashSet<_>>();

        if let Some(clue) = clue_start {
            invalid |= Self::count_visible(lane) != clue;
        }
        if let Some(clue) = clue_end {
            invalid |= Self::count_visible(lane.into_iter().rev()) != clue;
        }

        !invalid
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

    pub fn look_right(&self, row: usize) -> (Option<Digit>, [&Cell; N]) {
        ( self.clues.left[row], util::arr(self.cells[row].iter()) )
    }
    pub fn look_right_mut(&mut self, row: usize) -> (Option<Digit>, [&mut Cell; N]) {
        ( self.clues.left[row], util::arr(self.cells[row].iter_mut()) )
    }

    pub fn look_left(&self, row: usize) -> (Option<Digit>, [&Cell; N]) {
        ( self.clues.right[row], util::arr(self.cells[row].iter().rev()) )
    }
    pub fn look_left_mut(&mut self, row: usize) -> (Option<Digit>, [&mut Cell; N]) {
        ( self.clues.right[row], util::arr(self.cells[row].iter_mut().rev()) )
    }

    pub fn look_down(&self, col: usize) -> (Option<Digit>, [&Cell; N]) {
        ( self.clues.upper[col], util::arr(self.cells.iter().map(|row| &row[col])) )
    }
    pub fn look_down_mut(&mut self, col: usize) -> (Option<Digit>, [&mut Cell; N]) {
        ( self.clues.upper[col], util::arr(self.cells.iter_mut().map(|row| &mut row[col])) )
    }

    pub fn look_up(&mut self, col: usize) -> (Option<Digit>, [&Cell; N]) {
        ( self.clues.lower[col], util::arr(self.cells.iter().rev().map(|row| &row[col])) )
    }
    pub fn look_up_mut(&mut self, col: usize) -> (Option<Digit>, [&mut Cell; N]) {
        ( self.clues.lower[col], util::arr(self.cells.iter_mut().rev().map(|row| &mut row[col])) )
    }

    pub fn look_across_row(&self, row: usize) -> (Option<Digit>, [&Cell; N], Option<Digit>) {
        (
            self.clues.left[row],
            util::arr(self.cells[row].iter()),
            self.clues.right[row],
        )
    }
    pub fn look_across_row_mut(&mut self, row: usize) -> (Option<Digit>, [&mut Cell; N], Option<Digit>) {
        (
            self.clues.left[row],
            util::arr(self.cells[row].iter_mut()),
            self.clues.right[row],
        )
    }

    pub fn look_across_col(&self, col: usize) -> (Option<Digit>, [&Cell; N], Option<Digit>) {
        (
            self.clues.upper[col],
            util::arr(self.cells.iter().map(|row| &row[col])),
            self.clues.lower[col],
        )
    }
    pub fn look_across_col_mut(&mut self, col: usize) -> (Option<Digit>, [&mut Cell; N], Option<Digit>) {
        (
            self.clues.upper[col],
            util::arr(self.cells.iter_mut().map(|row| &mut row[col])),
            self.clues.lower[col],
        )
    }
}

// == PROCESS == //
impl<const N: usize> Grid<N>
{
    pub fn count_visible<'a>(lane: impl IntoIterator<Item = &'a Cell>) -> usize
    {
        let mut visible = 0;
        let mut peak = 0;

        for cell in lane.into_iter() {
            let digit = cell.digit();

            if digit > peak {
                visible += 1;
                peak = digit;
            }
        }

        visible
    }

    pub fn find_visible_indices(lane: &[&mut Cell; N]) -> Vec<usize>
    {
        let mut visible = vec![];
        let mut peak = 0;

        for (i, cell) in lane.iter().enumerate() {
            let digit = cell.digit();

            if digit > peak {
                visible.push(i);
                peak = digit;
            }
        }

        visible
    }

    pub fn occurrences(lane: &[&mut Cell; N]) -> HashMap<Digit, Vec<usize>>
    {
        let mut seen_indices: HashMap<Digit, Vec<usize>> =
            (1..=N)
            .map(|digit|
                (digit, vec![])
            )
            .collect();

        for (i, cell) in lane.iter().enumerate() {
            match cell {
                Cell::Solved(digit) => {
                    seen_indices.get_mut(digit).unwrap().push(i);
                }
                Cell::Pencil(Some(digits)) => {
                    for digit in digits {
                        seen_indices.get_mut(digit).unwrap().push(i);
                    }
                },
                _ => (),
            }
        }

        seen_indices
    }

    pub fn find_peak(lane: &[impl AsRef<Cell>; N]) -> Option<usize>
    {
        lane.iter().position(|c| *c.as_ref() == Cell::Solved(N))
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
                            clue.map(|c| format!(" {}{} ", util::rep(' ', N-1), c))
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
                                .map(Cell::render::<N>)
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
                                clue.map(|c| format!(" {}{} ", util::rep(' ', N-1), c))
                                    .unwrap_or(util::rep(' ', N+2).to_string())
                            ).join(" | ")
                    ) ))
            )
            .collect::<Vec<_>>()
            .join("\n")
        )
    }
}
