#[derive(Clone, Debug)]
pub enum State {
    Unknown,
    Guess(char),
    Known(char),
    New(char),
}

impl Default for State {
    fn default() -> Self {
        Self::Unknown
    }
}
