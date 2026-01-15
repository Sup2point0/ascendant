use std::*;
use std::collections::HashSet;

use lazy_static::*;


lazy_static! {
    static ref ARGS: HashSet<String> = env::args().collect();
}


pub fn args(arg: impl AsRef<str>) -> bool
{
    ARGS.contains(arg.as_ref())
}
