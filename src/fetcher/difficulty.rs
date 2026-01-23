#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Difficulty {
    FullEasy,
    FullHard,
    Sparse,
}

impl ToString for Difficulty
{
    fn to_string(&self) -> String
    {
        match self {
            Self::FullEasy => "1",
            Self::FullHard => "2",
            Self::Sparse   => "3",
        }.to_string()
    }
}
