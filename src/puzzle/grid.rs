use std::*;
use std::collections::{
    HashMap,
    HashSet,
};

use crate::*;


#[derive(Clone, PartialEq, Eq)]
pub struct Grid<const N: usize>
{
    pub url:   Option<String>,
    pub cells: [[Cell<N>; N]; N],
    pub clues: Clues<N>,
}

// == CONSTRUCTORS == //
impl<const N: usize> Grid<N>
{
    /// Construct a grid from an array representation.
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

    /// Try constructing a grid from a general iterator representation. Panics if the received sizes are incorrect.
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
    /// Is the provided x or y co-ordinate on the edge of a puzzle grid (including clues)?
    fn is_prep_edge(xy: usize) -> bool {
        xy == 0 || xy == N+1
    }

    /// Produce a clue from a digit, ignoring `0` clues.
    fn clue_from(n: Digit) -> Option<Digit> {
        (n > 0).then_some(n)
    }

    /// (in-place) Extract clues from `row` and assign them to `clue_row`.
    fn prep_clue_row(
        row: [Digit; N + 2],
        clue_row: &mut [Option<Digit>; N]
    ) -> ()
    {
        let row = row.into_iter().enumerate()
            .filter_map(|(x, n)|
                (!Self::is_prep_edge(x))
                .then(|| Self::clue_from(n))
            );

        *clue_row = util::arr(row);
    }

    /// (impure) Extract clues and cells from `row`, assigning clues to `clues` and returning the row of cells.
    fn prep_row(
        y: usize,
        row: [Digit; N + 2],
        clues: &mut Clues<N>,
    ) -> [Cell<N>; N]
    {
        let row = row.into_iter().enumerate()
            .filter_map(|(x, n)|
                if      x == 0   { if n > 0 { clues.left[y] = Some(n); } None }
                else if x == N+1 { if n > 0 { clues.right[y] = Some(n); } None }
                else if n > 0    { Some( Cell::Solved(n) ) }
                else             { Some( Cell::Pencil((1..=N).collect()) ) }
            );

        util::arr(row)
    }
}

// == QUERY == //
impl<const N: usize> Grid<N>
{
    /// What are the dimensions of the puzzle?
    pub fn size(&self) -> usize
    {
        N
    }

    /// Have all cells been solved, and if so, is the solution valid?
    pub fn is_solved(&self) -> bool
    {
        (0..N).all(|y| Self::validate_lane(self.look_across_row(y)))
        && (0..N).all(|x| Self::validate_lane(self.look_across_col(x)))
    }

    /// Does a lane contain every digit from 1 to N and satisfy any clues applied to it?
    fn validate_lane((clue_start, lane, clue_end): (Option<Digit>, [Cell<N>; N], Option<Digit>)) -> bool
    {
        let mut invalid = false;

        /* If a cell is not solved, `.digit()` gives `0`, which is not a valid digit, thus invalidating the lane. Bit cleaner than pattern matching on `Cell::Pencil`! */
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
    /// Get the cell at (col `x`, row `y`) of the grid.
    pub fn at(&self, x: usize, y: usize) -> Cell<N> {
        self.cells[y][x]
    }
    
    /// Get a mutable reference to the cell at (col `x`, row `y`) of the grid.
    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut Cell<N> {
        &mut self.cells[y][x]
    }

    pub fn look_right(&self, row: usize) -> (Option<Digit>, [Cell<N>; N]) {
        ( self.clues.left[row], util::arr(self.cells[row].into_iter()) )
    }
    pub fn look_right_mut(&mut self, row: usize) -> (Option<Digit>, [&mut Cell<N>; N]) {
        ( self.clues.left[row], util::arr(self.cells[row].iter_mut()) )
    }

    pub fn look_left(&self, row: usize) -> (Option<Digit>, [Cell<N>; N]) {
        ( self.clues.right[row], util::arr(self.cells[row].into_iter().rev()) )
    }
    pub fn look_left_mut(&mut self, row: usize) -> (Option<Digit>, [&mut Cell<N>; N]) {
        ( self.clues.right[row], util::arr(self.cells[row].iter_mut().rev()) )
    }

    pub fn look_down(&self, col: usize) -> (Option<Digit>, [Cell<N>; N]) {
        ( self.clues.upper[col], util::arr(self.cells.into_iter().map(|row| row[col])) )
    }
    pub fn look_down_mut(&mut self, col: usize) -> (Option<Digit>, [&mut Cell<N>; N]) {
        ( self.clues.upper[col], util::arr(self.cells.iter_mut().map(|row| &mut row[col])) )
    }

    pub fn look_up(&mut self, col: usize) -> (Option<Digit>, [Cell<N>; N]) {
        ( self.clues.lower[col], util::arr(self.cells.into_iter().rev().map(|row| row[col])) )
    }
    pub fn look_up_mut(&mut self, col: usize) -> (Option<Digit>, [&mut Cell<N>; N]) {
        ( self.clues.lower[col], util::arr(self.cells.iter_mut().rev().map(|row| &mut row[col])) )
    }

    /// Get the left clue, cells and right clue of a row.
    pub fn look_across_row(&self, row: usize) -> (Option<Digit>, [Cell<N>; N], Option<Digit>) {
        (
            self.clues.left[row],
            util::arr(self.cells[row].into_iter()),
            self.clues.right[row],
        )
    }
    /// Get the left clue, mutable cells and right clue of a row.
    pub fn look_across_row_mut(&mut self, row: usize) -> (Option<Digit>, [&mut Cell<N>; N], Option<Digit>) {
        (
            self.clues.left[row],
            util::arr(self.cells[row].iter_mut()),
            self.clues.right[row],
        )
    }

    /// Get the upper clue, cells and lower clue of a row.
    pub fn look_across_col(&self, col: usize) -> (Option<Digit>, [Cell<N>; N], Option<Digit>) {
        (
            self.clues.upper[col],
            util::arr(self.cells.iter().map(|row| row[col])),
            self.clues.lower[col],
        )
    }
    /// Get the upper clue, mutable cells and lower clue of a row.
    pub fn look_across_col_mut(&mut self, col: usize) -> (Option<Digit>, [&mut Cell<N>; N], Option<Digit>) {
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
    /// Looking across a lane of cells (which should be solved), how many skyscrapers are not obscured?
    pub fn count_visible(lane: impl IntoIterator<Item = Cell<N>>) -> usize
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

    /// Get the indices of the visible cells in a lane.
    pub fn find_visible_indices(lane: &[&mut Cell<N>; N]) -> Vec<usize>
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

    /// For each digit 1 to N, find the indices of the lane in which it could be present. Returns a `HashMap` of each digit to its list of indices.
    pub fn occurrences(lane: &[&mut Cell<N>; N]) -> HashMap<Digit, Vec<usize>>
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
                Cell::Pencil(digits) => {
                    for digit in digits.members() {
                        seen_indices.get_mut(&digit).unwrap().push(i);
                    }
                },
            }
        }

        seen_indices
    }

    /// Try to find a solved cell with an N-skyscraper, if there is one.
    pub fn find_peak(lane: &[impl AsRef<Cell<N>>; N]) -> Option<usize>
    {
        lane.iter().position(|c| *c.as_ref() == Cell::Solved(N))
    }
}

impl<const N: usize> fmt::Debug for Grid<N>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let col_width = N + 5;

        // upper clues
        write!(f, "   |")?;
        for clue in &self.clues.upper {
            let digit = Clues::<N>::render(*clue);
            write!(f, "  {: ^1$}  |", digit, N)?;
        }

        // ---
        write!(f, "\n{}", util::rep('-', N * col_width + 7))?;

        // grid
        for (y, row) in self.cells.iter().enumerate() {
            // left clue
            let digit = Clues::<N>::render(self.clues.left[y]);
            write!(f, "\n {digit} | ")?;

            // row
            for cell in row {
                cell.fmt(f)?;
                write!(f, " | ")?;
            }

            // right clue
            let digit = Clues::<N>::render(self.clues.right[y]);
            write!(f, "{digit}")?;
        }

        // ---
        write!(f, "\n{}", util::rep('-', N * col_width + 7))?;

        // lower clues
        write!(f, "\n   |")?;
        for clue in &self.clues.lower {
            let digit = Clues::<N>::render(*clue);
            write!(f, "  {: ^1$}  |", digit, N)?;
        }

        Ok(())
    }
}
