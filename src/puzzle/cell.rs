use std::*;

use itertools::*;
use natbitset::Bitset;

use crate::*;


/// A cell in the puzzle grid, which may contain either a single `Solved(Digit)` or many possible `Pencil`-marks.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Cell<const N: usize> {
    Solved(Digit),

    /// A cell that hasn't been solved yet, containing pencil marks of possible digits.
    Pencil(Bitset<N>),
}

/// Construct a `Cell::Pencil` with the provided digits.
#[macro_export]
macro_rules! p {
    ( $($digit:expr),* $(,)? ) =>
    {
        Cell::Pencil(
            natbitset::Bitset::from(
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

impl<const N: usize> Cell<N>
{
    /// Create a [`Cell::Pencil`] with all the possible digits of an `N`x`N` grid.
    pub fn new() -> Self
    {
        Self::Pencil(
            (1..=N).collect()
        )
    }

    /// Create a set of the candidate digits `lower..=upper`, but if the range is invalid, instead return a full set of 1 to N.
    pub fn cands(lower: impl Into<Digit>, upper: impl Into<Digit>) -> Result<Bitset<{N}>, String>
    {
        let lower: Digit = lower.into();
        let upper: Digit = upper.into();

        Ok((
            if upper >= lower {
                lower..= upper
            }
            else {
                debug! {
                    {
                        return Err(format!(
                            "Deduced no candidates for cell, calculated lower: `{lower}`, upper: `{upper}`"
                        ));
                    }
                };
                
                1..=N
            }
        ).collect())
    }
}

impl<const N: usize> Cell<N>
{
    /// If the cell is solved, extract the digit, otherwise return `0`.
    pub fn digit(&self) -> Digit
    {
        self.solved_digit().unwrap_or(0)
    }

    /// Extract the solved digit of a `Cell::Solved`.
    pub fn solved_digit(&self) -> Option<Digit>
    {
        match self {
            Self::Solved(digit) => Some(*digit),
            Self::Pencil{..}    => None,
        }
    }

    /// Return the maximum digit a cell could be, whether it is solved or pencil marks.
    pub fn max(&self) -> Digit
    {
        match self {
            Self::Solved(digit)  => *digit,
            Self::Pencil(cands) => cands.maximum().expect("Cell cannot have 0 candidates"),
        }
    }

    /// Extract the greatest candidate of a `Cell::Pencil`.
    pub fn max_cand(&self) -> Option<Digit>
    {
        match self {
            Self::Pencil(cands) => Some(cands.maximum().expect("Cell cannot have 0 candidates")),
            Self::Solved{..}    => None
        }
    }

    /// For a `Cell::Pencil`, combine its current candidates with `candidates`. Returns `true` if a deduction was made as a result.
    /// 
    /// Panics if the set of candidates is empty (logic error).
    pub fn intersect(&mut self, candidates: Bitset<N>, lane: [Cell<N>; N]) -> bool
    {
        let Self::Pencil(digits) = self else { return false; };
        let before = *digits;
        
        if let Err(e) = digits.intersect_nonempty(candidates) {
            panic!("Conflicting deductions in lane: `{lane:?}`, caused by: {e}");
        }

        let did_deduce = (*digits != before);

        if let Some(digit) = digits.only() {
            *self = Cell::Solved(digit);
        }

        did_deduce
    }
}

impl<const N: usize> Cell<N>
{
    pub fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            Self::Solved(digit) => write!(f, " {: ^1$} ", digit, N),

            Self::Pencil(digits) => {
                let str = digits
                    .members().into_iter()
                    .sorted()
                    .map(|n| n.to_string())
                    .join("");

                write!(f, "[{: >1$}]", str, N)
            },
        }
    }
}

impl<const N: usize> fmt::Debug for Cell<N>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            Self::Solved(d) => write!(f, "'{d}'"),

            Self::Pencil(digits) => write!(f, "[{}]",
                digits
                    .members().into_iter()
                    .sorted()
                    .map(|n| n.to_string())
                    .join("")
            ),
        }
    }
}

impl<const N: usize> AsRef<Cell<N>> for Cell<N>
{
    fn as_ref(&self) -> &Cell<N> {
        &self
    }
}
