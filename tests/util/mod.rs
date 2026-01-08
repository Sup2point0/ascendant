#[macro_export]
macro_rules! set {
    ( $($digit: expr),* $(,)? ) =>
    {
        std::collections::HashSet::from(
            [ $( $digit, )* ]
        )
    };
}
