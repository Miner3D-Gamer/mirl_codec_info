use crate::{error::ParsingError, values::PositionedValue};

#[allow(unused_variables)]
#[rustfmt::skip]
/// Detection half — only looks at the data, never mutates position
pub trait StaticParserDetect {
    #[must_use]
    /// If the next value is a number
    fn is_number(data: &[char], pos: usize) -> bool {false}
    #[must_use]
    /// If the next value is a string
    fn is_string(data: &[char], pos: usize) -> bool {false}
    #[must_use]
    /// If the next value is a list
    fn is_list(data: &[char], pos: usize) -> bool {false}
    #[must_use]
    /// If the next value is a map
    fn is_map(data: &[char], pos: usize) -> bool {false}
    #[must_use]
    /// If the next value is none
    fn is_none(data: &[char], pos: usize) -> bool {false}
    #[must_use]
    /// If the next value is a bool
    fn is_bool(data: &[char], pos: usize) -> bool {false}
    #[must_use]
    /// If the next value is a time
    fn is_time(data: &[char], pos: usize) -> bool {false}
    #[must_use]
    /// If the next value is a datetime
    fn is_datetime(data: &[char], pos: usize) -> bool {false}
    #[must_use]
    /// If the next value is an angle
    fn is_angle(data: &[char], pos: usize) -> bool {false}
    #[must_use]
    /// If the next value is a literal
    fn is_literal(data: &[char], pos: usize) -> bool {false}
    #[must_use]
    /// If the next value is a length
    fn is_length(data: &[char], pos: usize) -> bool {false}
    #[must_use]
    /// If the next value is a color
    fn is_color(data: &[char], pos: usize) -> bool {false}
    #[must_use]
    /// If the next value is bytes
    fn is_bytes(data: &[char], pos: usize) -> bool {false}
}

#[allow(unused_variables)]
#[allow(clippy::missing_errors_doc)]
/// Parsing half — mutates position and value count
pub trait StaticParserParse {
    /// Parses with the expectancy that the next value is guaranteed to be a number
    fn parse_number(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Err(ParsingError::UnsupportedType {
            pos: *pos,
            value_type: mirl_values::prelude::ValueType::Number,
            text: data.iter().collect(),
        })
    }
    /// Parses with the expectancy that the next value is guaranteed to be a string
    fn parse_string(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Err(ParsingError::UnsupportedType {
            pos: *pos,
            value_type: mirl_values::prelude::ValueType::String,
            text: data.iter().collect(),
        })
    }
    /// Parses with the expectancy that the next value is guaranteed to be a list
    fn parse_list(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Err(ParsingError::UnsupportedType {
            pos: *pos,
            value_type: mirl_values::prelude::ValueType::Vec,
            text: data.iter().collect(),
        })
    }
    /// Parses with the expectancy that the next value is guaranteed to be a map
    fn parse_map(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Err(ParsingError::UnsupportedType {
            pos: *pos,
            value_type: mirl_values::prelude::ValueType::Map,
            text: data.iter().collect(),
        })
    }
    /// Parses with the expectancy that the next value is guaranteed to be None
    fn parse_none(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Err(ParsingError::UnsupportedType {
            pos: *pos,
            value_type: mirl_values::prelude::ValueType::None,
            text: data.iter().collect(),
        })
    }
    /// Parses with the expectancy that the next value is guaranteed to be a bool
    fn parse_bool(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Err(ParsingError::UnsupportedType {
            pos: *pos,
            value_type: mirl_values::prelude::ValueType::Bool,
            text: data.iter().collect(),
        })
    }
    /// Parses with the expectancy that the next value is guaranteed to be a time
    fn parse_time(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Err(ParsingError::UnsupportedType {
            pos: *pos,
            value_type: mirl_values::prelude::ValueType::Time,
            text: data.iter().collect(),
        })
    }
    /// Parses with the expectancy that the next value is guaranteed to be a datetime
    fn parse_datetime(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Err(ParsingError::UnsupportedType {
            pos: *pos,
            value_type: mirl_values::prelude::ValueType::DateTime,
            text: data.iter().collect(),
        })
    }
    /// Parses with the expectancy that the next value is guaranteed to be an angle
    fn parse_angle(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Err(ParsingError::UnsupportedType {
            pos: *pos,
            value_type: mirl_values::prelude::ValueType::Angle,
            text: data.iter().collect(),
        })
    }
    /// Parses with the expectancy that the next value is guaranteed to be a literal
    fn parse_literal(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Err(ParsingError::UnsupportedType {
            pos: *pos,
            value_type: mirl_values::prelude::ValueType::Literal,
            text: data.iter().collect(),
        })
    }
    /// Parses with the expectancy that the next value is guaranteed to be a length
    fn parse_length(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Err(ParsingError::UnsupportedType {
            pos: *pos,
            value_type: mirl_values::prelude::ValueType::Length,
            text: data.iter().collect(),
        })
    }
    /// Parses with the expectancy that the next value is guaranteed to be a color
    fn parse_color(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Err(ParsingError::UnsupportedType {
            pos: *pos,
            value_type: mirl_values::prelude::ValueType::Color,
            text: data.iter().collect(),
        })
    }
    /// Parses with the expectancy that the next value is guaranteed to be bytes
    fn parse_bytes(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Err(ParsingError::UnsupportedType {
            pos: *pos,
            value_type: mirl_values::prelude::ValueType::Bytes,
            text: data.iter().collect(),
        })
    }

    /// Skip unnecessary whitespace so item parsers can properly do their work
    fn skip_whitespace(data: &[char], pos: &mut usize, value_count: &mut usize);
}
/// An object capable of identifying and parsing data types
///
/// This is automatically implemented for objects that implement [`StaticParserDetect`] and [`StaticParserParse`]
pub trait StaticParser: StaticParserDetect + StaticParserParse {}

impl<T: StaticParserDetect + StaticParserParse> StaticParser for T {}
