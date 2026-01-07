use std::*;

use crate::*;


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
