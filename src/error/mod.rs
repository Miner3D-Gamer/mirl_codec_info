use mirl_core::text::position::line_and_column_from_offset;
use mirl_values::values::ValueType;

impl std::error::Error for ParsingError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Any errors that may occur while parsing the given data
pub enum ParsingError {
    /// When a character was expected but another was provided
    UnexpectedCharacter {
        /// The position of the invalid character
        offset: usize,
        /// What character is there
        given: char,
        /// What characters would have been allowed
        expected: Vec<char>,
        /// The input text
        text: String,
        /// Any additional info
        error: ParserMishaps,
    },
    /// Unexpected EOF, there should have been less file!
    ExpectedEOF {
        /// The position of the error
        offset: usize,
        /// What was currently parsed before erroring
        origin: Option<ValueType>,
        /// The input text
        text: String,
    },
    /// Unexpected EOF, there should have been more file!
    UnexpectedEOF {
        /// The position of the error
        offset: usize,
        /// What was currently parsed before erroring
        origin: Option<ValueType>,
        /// The input text
        text: String,
    },
    /// When a element was provided but another expected
    UnexpectedElement {
        /// The value that was unexpected
        value: PositionedValue,
        /// Any additional info
        error: ParserMishaps,
        /// What values would have been allowed in this context
        expected: Vec<ValueType>,
        /// The input text
        text: String,
    },
    /// The character was not recognized as being a valid "thing"
    UnrecognizedType {
        /// The position of the error
        offset: usize,
        /// The first character of the unrecognized type
        starting_char: char,
        /// The input text
        text: String,
    },
    /// A [`ValueType`] is unsupported
    UnsupportedType {
        /// The position of the error
        pos: usize,
        /// The type that is unsupported by the language/current settings
        value_type: ValueType,
        /// The input text
        text: String,
    },
    /// This will error will never be returned yet is internally used
    EmptyFile,
    /// When a type couldn't be determined or something outside of parsing happened
    Unknown,
}
impl std::fmt::Display for ParsingError {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text: &str = match self {
            Self::Unknown => "¯\\_(ツ)_/¯",
            Self::UnexpectedEOF {
                offset: pos,
                origin,
                text,
            } => &{
                let pos = line_and_column_from_offset(*pos, text);
                format!(
                    "{}:{}: Unexpected end of file -> less data than wanted{}",
                    pos.0,
                    pos.1,
                    origin.as_ref().map_or_else(
                        || " (Outside of parsing)".to_string(),
                        |val| format!(" while parsing {val:?}")
                    ),
                )
            },
            Self::ExpectedEOF {
                offset,
                origin,
                text,
            } => &{
                let (line, column) = line_and_column_from_offset(*offset, text);
                let left_over = text.chars().skip(*offset).collect::<String>();
                format!(
                    "{}:{}: Expected end of file -> more data than wanted {}. Remaining data: '{}' ({})",
                    line,
                    column,
                    origin.map_or_else(
                        || " (Outside of parsing)".to_string(),
                        |val| format!(" while parsing {val:?}")
                    ),
                    left_over.escape_debug(),
                    left_over.len()
                )
            },
            Self::UnexpectedCharacter {
                offset,
                given,
                expected,
                text,
                error: description,
            } => &{
                let (line, column) = line_and_column_from_offset(*offset, text);
                format!(
                    "{}:{}: '{}' was found where any of the following would have been expected: {:?} ({}) (Parsing {:?})\n>>{}",
                    line,
                    column,
                    given.escape_debug(),
                    expected,
                    expected.len(),
                    description.get_value_type(),
                    description
                )
            },
            Self::UnrecognizedType {
                offset,
                starting_char,
                text,
            } => &{
                let (line, column) = line_and_column_from_offset(*offset, text);
                format!(
                    "{}:{}: Unable to recognize object; '{}'",
                    line,
                    column,
                    starting_char.escape_debug()
                )
            },
            Self::UnexpectedElement {
                value,
                expected,
                text,
                error,
            } => &{
                let position = value.get_position();
                let (line, column) =
                    line_and_column_from_offset(position.offset, text);
                let (line2, column2) = line_and_column_from_offset(
                    position.offset + position.width,
                    text,
                );
                format!(
                    "{}:{} to {}:{}: Expected: {:?} ({}) but got Element {:?} instead. (Parsing {:?})",
                    line,
                    column,
                    line2,
                    column2,
                    expected,
                    expected.len(),
                    value.get_value_type(),
                    error.get_value_type()
                )
            },
            Self::UnsupportedType {
                pos,
                value_type,
                text,
            } => {
                let (line, column) = line_and_column_from_offset(*pos, text);

                &format!("{line}:{column}, got unsupported type: {value_type}")
            }
            Self::EmptyFile => "File is empty - No value could be extracted",
        };

        std::fmt::Display::fmt(text, f)
    }
}

mod mishaps;
pub use mishaps::*;

use crate::values::PositionedValue;
