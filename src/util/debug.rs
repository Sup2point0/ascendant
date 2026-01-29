use crate::*;
use crate::cli::detail::OutputDetail;


pub fn reaches_detail_level(required_detail_level: OutputDetail) -> bool
{
    // SAFETY: `cli::OUTPUT_DETAIL` is not modified after CLI starts. Plus this is only for printing debug output.
    unsafe {
        cli::OUTPUT_DETAIL >= required_detail_level
    }
}
