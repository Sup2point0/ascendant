#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Difficulty {
    Full,
    Sparse,
}

impl ToString for Difficulty
{
    fn to_string(&self) -> String {
        match self {
            Self::Full   => "2",
            Self::Sparse => "3",
        }.to_string()
    }
}
