#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Errors that may occur when parsing a map
pub enum ParserMapMishap {
    /// - Expected: `{"Key": "Value"}`
    /// - Received: `{"Key" ?: "Value"}`
    DataAfterKeyElement,
    /// - Expected: `{"Key": "Value"}`
    /// - Received: `{"Key": "Value" ?}`
    DataAfterValueElement,

    /// - Expected: `{"10": "Value"}`
    /// - Received: `{1000: "Value"}`
    InvalidKeyType,

    /// - Expected: `{"Key": "Value"}`
    /// - Received: `{ ??? : "Value"}`
    MissingKey,

    /// - Expected: `{"Key": "Value"}`
    /// - Received: `{"Key":  ????? }`
    MissingValue,

    #[default]
    /// An ambiguous error which should not be used.
    /// If you with to expand on this lib and encounter an map related error that isn't listed here, open a github issue
    Other,
}

impl std::fmt::Display for ParserMapMishap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::DataAfterKeyElement => "Value after Key element - Did you forget a colon?",
            Self::DataAfterValueElement => "Value after Key element - Did you forget a comma?",
            Self::InvalidKeyType => "Expected Key element that is not supported",
            Self::MissingKey => "No key found",
            Self::MissingValue => "No value found for key",
            Self::Other => "Unknown",
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for ParserMapMishap {}
