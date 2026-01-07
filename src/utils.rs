use std::*;

use arrayvec::ArrayVec;


pub fn as_array<I, T, const N: usize>(iter: I) -> [T; N]
    where
        I: IntoIterator<Item = T>,
        T: fmt::Debug
{
    iter.into_iter()
        .collect::<ArrayVec<T, N>>()
        .into_inner()
        .unwrap()
}
