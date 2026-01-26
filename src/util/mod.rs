pub mod debug;

mod iter; pub use iter::*;


#[macro_export]
macro_rules! debug
{
    ($threshold:expr => $body:block) =>
    {
        if util::debug::reaches_detail_level($threshold) {
            $body
        }
    };

    ($threshold:expr => $text:expr) =>
    {
        if util::debug::reaches_detail_level($threshold) {
            println!($text);
        }
    };

    ($body:block) =>
    {
        if util::debug::reaches_detail_level(cli::detail::OutputDetail::DEBUG_STEPS) {
            $body
        }
    };

    ($text:expr) =>
    {
        if util::debug::reaches_detail_level(cli::detail::OutputDetail::DEBUG_STEPS) {
            println!($text);
        }
    };
}
