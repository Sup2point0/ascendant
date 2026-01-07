use std::*;

use super::*;


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
