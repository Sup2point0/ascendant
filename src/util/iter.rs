use std::*;
use std::collections::HashMap;


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
