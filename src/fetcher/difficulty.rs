use anyhow as ah;


#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Difficulty {
    FullEasy,
    FullHard,
    Sparse,
}

impl Difficulty
{
    pub fn all() -> Vec<Self>
    {
        vec![
            Self::FullEasy,
            Self::FullHard,
            Self::Sparse,
        ]
    }
}

impl TryFrom<String> for Difficulty
{
    type Error = ah::Error;

    fn try_from(diff: String) -> Result<Self, Self::Error>
    {
        match diff.as_str() {
            "1" => Ok(Difficulty::FullEasy),
            "2" => Ok(Difficulty::FullHard),
            "3" => Ok(Difficulty::Sparse),
            d   => Err(ah::anyhow!(
                "Failed to convert `{d}` to a `Difficulty` - the only valid values are `1`, `2` or `3`."
            )),
        }
    }
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
