use std::*;


/// The level of detail of print output.
#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Debug)]
pub enum OutputDetail
{
    /// Default user-facing output.
    DEFAULT,

    /// Print the start and end states of puzzles that the algorithm fails to solve.
    SHOW_FAIL,

    /// Print the state of a puzzle after each pass.
    SHOW_PASSES,

    /// Print the state of a puzzle after every type of deduction (for debugging algorithm).
    DEBUG_STEPS,
}

impl Default for OutputDetail
{
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Into<usize> for OutputDetail
{
    fn into(self) -> usize
    {
        match self {
            Self::DEFAULT     => 0,
            Self::SHOW_FAIL   => 1,
            Self::SHOW_PASSES => 2,
            Self::DEBUG_STEPS => 3,
        }
    }
}

impl Ord for OutputDetail
{
    fn cmp(&self, other: &Self) -> cmp::Ordering
    {
        let left: usize = (*self).into();
        let right: usize = (*other).into();
        
        left.cmp(&right)
    }
}
