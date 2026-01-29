use std::*;
use std::collections::HashMap;

use arrayvec::ArrayVec;

use crate::*;


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


pub fn arr_mut<T, const N: usize>(iter: &mut [T]) -> [&mut T; N]
    where
        T: fmt::Debug
{
    iter.iter_mut()
        .collect::<ArrayVec<&mut T, N>>()
        .into_inner()
        .unwrap()
}


pub fn rep(c: char, n: usize) -> String
{
    std::iter::repeat_n(c, n).collect::<String>()
}


pub fn snap_lane<const N: usize>(lane: &[&mut Cell<N>; N]) -> [Cell<N>; N]
{
    arr(
        lane.iter().map(|cell| **cell)
    )
}


pub trait FindMapMaybe<T> where Self: Iterator<Item = T>
{
    /// Find the first element in `self` for which `predicate(map(element))` is true, ignoring elements for which `map` returns `None`.
    fn find_map_maybe<F, Mid, P>(&mut self, mut map: F, mut predicate: P) -> Option<T>
        where
            Self: Sized,
            F: FnMut(&Self::Item) -> Option<Mid>,
            P: FnMut(Mid) -> bool
    {
        self.find(|each| map(each).map(|some| predicate(some)) == Some(true))
    }
}

impl<I,T> FindMapMaybe<T> for I
    where I: Iterator<Item = T>
{}


pub trait MapValues<V,W>
{
    type Output;

    fn map_values(self, f: impl FnMut(V) -> W) -> Self::Output;
}

impl<K,V,W> MapValues<V,W> for HashMap<K,V>
    where K: Eq + hash::Hash
{
    type Output = HashMap<K,W>;

    fn map_values(self, mut f: impl FnMut(V) -> W) -> Self::Output
    {
        self.into_iter()
            .map(|(key, val)|
                (key, f(val))
            )
            .collect()
    }
}
