#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Everything that could go wrong when parsing a number.
///
/// The `?` indicates a missing/wrong character
pub enum ParserNumberMishap {
    /// - Expected: `1.5`
    /// - Received: `1.00.5`
    MultipleDots,
    /// - Expected: `1.5`
    /// - Received: `1..5`
    RangeNotAllowed,
    /// - Expected: `1.0`
    /// - Received: `?.0`
    FloatWithoutInt,
    /// - Expected: `1e100`
    /// - Received: `?e100`
    ExponentWithoutBase,
    /// - Expected: (Any) `0` `1` `2` `3` `4` `5` `6` `7` `8` `9`
    /// - Received: `?`
    NumberContainsNonNumber,
    /// - Expected: `1`
    /// - Received: `01`
    LeadingZeros,
    /// - Expected: `-10`
    /// - Received: `-?`
    NumberWithoutNumber,
    /// - Expected: `1.0`
    /// - Received: `1.?`
    IncompleteFloat,
    /// - Expected: `10e100`
    /// - Received: `10e?`
    IncompleteScientificNotation,
    #[default]
    /// An ambiguous error which should not be used.
    /// If you with to expand on this lib and encounter a number related error that isn't listed here, open a github issue
    Other,
}

impl std::fmt::Display for ParserNumberMishap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::ExponentWithoutBase => "Exponent requires a base number",
            Self::FloatWithoutInt => "Float is missing a integer",
            Self::IncompleteFloat => "Expected decimals after dot",
            Self::IncompleteScientificNotation => "Expected number after scientific notation",
            Self::LeadingZeros => "A value of '01' is invalid because it simplifies to '1'",
            Self::MultipleDots => "A float cannot have 2 dots",
            Self::RangeNotAllowed => "You cannot define a range as a number",
            Self::NumberContainsNonNumber => "Number mustn't contain non numbers",
            Self::NumberWithoutNumber => "Number must contain a number",
            Self::Other => "Unknown",
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for ParserNumberMishap {}
