use std::*;
use std::collections::HashSet;

use itertools::*;

use crate::*;


/// A cell in the puzzle grid, which may contain either a single `Solved(Digit)` or many possible `Pencil`-marks.
#[derive(Clone, PartialEq, Eq)]
pub enum Cell {
    Solved(Digit),

    /// A cell that hasn't been solved yet, containing pencil marks of possible digits.
    Pencil(HashSet<Digit>),
}

/// Construct a `Cell::Pencil` with the provided digits.
#[macro_export]
macro_rules! p {
    ( $($digit:expr),* $(,)? ) =>
    {
        Cell::Pencil(
            std::collections::HashSet::from(
                [ $($digit,)* ]
            )
        )
    };
    ( $lower:expr; $upper:expr ) =>
    {
        Cell::Pencil(
            ($lower ..= $upper).collect()
        )
    }
}

impl Cell
{
    /// Create a `Cell::Pencil` with all the possible digits of an `n`x`n` grid.
    pub fn new(n: usize) -> Self
    {
        Self::Pencil(
            (1..=n).collect()
        )
    }

    /// Create a set of the candidate digits between `lower` and `upper` (inclusive), but if the range is invalid, instead return a full set of 1 to N.
    pub fn cands<const N: usize>(lower: impl Into<Digit>, upper: impl Into<Digit>) -> HashSet<Digit>
    {
        let lower: Digit = lower.into();
        let upper: Digit = upper.into();

        if upper >= lower { lower..= upper } else { 1..=N }
            .collect()
    }
}

impl Cell
{
    /// For a `Cell::Solved`, extract the solved digit, otherwise return `0`.
    pub fn digit(&self) -> Digit
    {
        match self {
            Cell::Solved(digit) => *digit,
            Cell::Pencil(..)    => 0,
        }
    }

    /// Return the maximum digit a cell could be, whether it is solved or pencil marks.
    pub fn max(&self) -> Digit
    {
        match self {
            Self::Solved(digit)  => *digit,
            Self::Pencil(digits) => *digits.iter().max().unwrap(),
        }
    }

    /// For a `Cell::Pencil`, combine its current candidates with `candidates`. Returns `true` if a deduction was made as a result.
    /// 
    /// Panics if the set of candidates is not present (contract violation), or is empty (logic error).
    pub fn intersect(&mut self, candidates: &HashSet<Digit>) -> bool
    {
        let mut did_deduce = false;

        if let Self::Pencil(digits) = self
        {
            let deduced: HashSet<Digit> =
                digits.intersection(&candidates).copied().collect();
            
            if deduced != *digits {
                did_deduce = true;
            }

            match deduced.len() {
                0 => panic!("Conflicting deductions! Old: {digits:?}; New: {candidates:?}"),
                1 => *self = Cell::Solved(*deduced.iter().next().unwrap()),
                _ => *digits = deduced,
            }
        }

        did_deduce
    }
}

impl Cell
{
    pub fn render<const N: usize>(&self) -> String
    {
        match self {
            Self::Solved(d) => {
                let fill = iter::repeat_n(' ', N+1).collect::<String>();

                format!("{fill}{d}")
            },

            Self::Pencil(digits) => {
                let str = digits.iter()
                    .sorted()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join("");

                let c = str.clone().chars().count();
                let fill = iter::repeat_n(' ', N - c).collect::<String>();

                format!("[{fill}{str}]")
            },
        }
    }
}

impl fmt::Debug for Cell
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", match self {
            Self::Solved(d)  => format!("'{d}'"),

            Self::Pencil(digits) => format!("[{}]",
                digits.iter()
                    .sorted()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            ),
        })
    }
}

impl AsRef<Cell> for Cell
{
    fn as_ref(&self) -> &Cell {
        &self
    }
}
