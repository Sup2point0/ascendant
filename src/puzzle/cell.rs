use std::*;
use std::collections::HashSet;

use itertools::*;

use crate::*;


#[derive(Clone, PartialEq, Eq)]
pub enum Cell {
    Solved(Digit),

    /// A cell we haven't deduced yet, containing pencil marks of possible digits.
    /// 
    /// We use an `Option<>` to allow `.take()` on it, which will allow mutably replacing the value. This field should always atomically be `Some()` by contract.
    Pencil(Option<HashSet<Digit>>),
}

#[macro_export]
macro_rules! p {
    ( $($digit:expr),* $(,)? ) =>
    {
        Cell::Pencil(
            Some(
                std::collections::HashSet::from(
                    [ $($digit,)* ]
                )
            )
        )
    };
    ( $lower:expr; $upper:expr ) =>
    {
        Cell::Pencil(
            Some(
                ( $lower ..= $upper).collect()
            )
        )
    }
}

impl Cell
{
    pub fn new(n: usize) -> Self
    {
        Self::Pencil(Some(
            (1..=n).collect()
        ))
    }

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
    /// For a `Cell::Pencil`, combine its current candidates with `candidates`. Returns `true` if a deduction was made as a result.
    /// 
    /// Panics if the set of candidates is not present (contract violation), or is empty (logic error).
    pub fn intersect(&mut self, candidates: &HashSet<Digit>) -> bool
    {
        let mut did_deduce = false;

        if let Self::Pencil(digits) = self
        {
            match digits {
                None => panic!("Encountered cell with pencilmarks stolen!"),
                
                Some(ds) => {
                    let deduced: HashSet<Digit> =
                        ds.intersection(&candidates).copied().collect();
                    
                    if deduced != *ds {
                        did_deduce = true;
                    }

                    match deduced.len() {
                        0 => panic!("Conflicting deductions! Old: {ds:?}; New: {candidates:?}"),
                        1 => *self = Cell::Solved(*deduced.iter().next().unwrap()),
                        _ => *ds = deduced,
                    }
                }
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

            Self::Pencil(Some(digits)) => {
                let str = digits.iter()
                    .sorted()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join("");

                let c = str.clone().chars().count();
                let fill = iter::repeat_n(' ', N - c).collect::<String>();

                format!("[{fill}{str}]")
            },

            Self::Pencil(None) => iter::repeat_n('?', N+2).collect::<String>(),
        }
    }
}

impl fmt::Debug for Cell
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", match self {
            Self::Solved(d)  => format!("'{d}'"),

            Self::Pencil(Some(digits)) => format!("[{}]",
                digits.iter()
                    .sorted()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            ),

            Self::Pencil(None) => "[ ? ]".to_string(),
        })
    }
}

impl AsRef<Cell> for Cell
{
    fn as_ref(&self) -> &Cell {
        &self
    }
}
