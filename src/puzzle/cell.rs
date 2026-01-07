use std::*;
use std::collections::HashSet;

use crate::*;


#[derive(Clone, PartialEq, Eq)]
pub enum Cell {
    Solved(Digit),
    Pencil(Option<HashSet<Digit>>),
}

#[macro_export]
macro_rules! pen {
    ( $($digit: expr),* $(,)? ) =>
    {
        Cell::Pencil(
            Some(
                std::collections::HashSet::from(
                    [$( $digit, )*]
                )
            )
        )
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
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join("");

                let c = str.clone().chars().count();
                let fill = iter::repeat_n(' ', N - c).collect::<String>();

                format!("[{fill}{str}]")
            },

            Self::Pencil(None) => iter::repeat_n(' ', N+2).collect::<String>(),
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
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            ),

            Self::Pencil(None) => format!("[ ? ]"),
        })
    }
}
