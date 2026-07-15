/// Everything that could go wrong when parsing a String.
///
/// The `?` indicates a missing/wrong character
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ParserStringMishap {
    /// - Expected: `"Hello"`
    /// - Received: `'Hello'`
    WrongQuotation,

    /// - Expected: `\u0001`, `\u0002`, ... `\uffff`
    /// - Received: `\u????`
    InvalidUnicodeSequence,

    /// - Expected: `\u0001`, `\u0002`, ... `\uffff`
    /// - Received: `\u0000`
    NullCharacter,

    /// - Expected: `\n`, `\u`
    /// - Received: `\?`
    EscapedInvalidCharacter,

    /// - Expected: `\?`
    /// - Received: `?`
    UnescapedControlCharacter,

    #[default]
    /// An ambiguous error which should not be used.
    /// If you with to expand on this lib and encounter a number related error that isn't listed here, open a github issue
    Other,
}

impl std::fmt::Display for ParserStringMishap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::WrongQuotation => "Another quotation type was expected",
            Self::InvalidUnicodeSequence => "Invalid unicode sequence",
            Self::NullCharacter => "Null character is not allowed",
            Self::EscapedInvalidCharacter => "Escaping this character does not make sense",
            Self::UnescapedControlCharacter => "Control characters must be escaped",
            Self::Other => "Unknown",
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for ParserStringMishap {}
