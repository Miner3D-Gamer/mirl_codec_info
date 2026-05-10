#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Problems that may occur when parsing an array
pub enum ParserArrayMishap {
    /// - Expected: `["Item", "Any", "Other"]`
    /// - Received: `["Item", , "Other"]`
    EmptyElement,
    /// - Expected: `["Some", "Item"]`
    /// - Received: `["Some" "Item"]`
    MissingElementSeparator,

    #[default]
    /// An ambiguous error which should not be used.
    /// If you with to expand on this lib and encounter an array related error that isn't listed here, open a github issue
    Other,
}
impl std::fmt::Display for ParserArrayMishap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::EmptyElement => "Empty elements are not allowed",
            Self::MissingElementSeparator => {
                "Received data after element finished parsing - Did you forget a comma?"
            }
            Self::Other => "Unknown",
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for ParserArrayMishap {}
