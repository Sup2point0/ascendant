mod iter; pub use iter::*;

use std::*;

use arrayvec::ArrayVec;


pub fn arr<I, T, const N: usize>(iter: I) -> [T; N]
    where
        I: IntoIterator<Item = T>,
        T: fmt::Debug
{
    iter.into_iter()
        .collect::<ArrayVec<T, N>>()
        .into_inner()
        .unwrap()
}


pub fn rep(c: char, n: usize) -> String
{
    std::iter::repeat_n(c, n).collect::<String>()
}
