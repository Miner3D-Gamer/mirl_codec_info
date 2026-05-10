#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Everything that could go wrong when parsing none.
///
/// The `?` indicates a missing/wrong character
pub enum ParserNoneMishap {
    #[default]
    /// An ambiguous error which should not be used.
    /// If you with to expand on this lib and encounter a none related error that isn't listed here, open a github issue
    Other,
}

impl std::fmt::Display for ParserNoneMishap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Other => "Unknown",
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for ParserNoneMishap {}
