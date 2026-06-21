use crate::{error::CodecError, values::PositionedValue};

#[allow(unused_variables)]
/// Detection half — only looks at the data, never mutates position
///
/// # Safety
/// The parsing functions don't check if the output of these functions are orrect
pub unsafe trait StaticParserDetect {
    /// If the next value is a number
    ///
    /// # Errors
    /// The given input is this type, however it is malformed in some way
    fn is_number(data: &[char], pos: usize) -> Result<bool, CodecError> {
        Ok(false)
    }
    /// If the next value is a string
    ///
    /// # Errors
    /// The given input is this type, however it is malformed in some way
    fn is_string(data: &[char], pos: usize) -> Result<bool, CodecError> {
        Ok(false)
    }
    /// If the next value is a list
    ///
    /// # Errors
    /// The given input is this type, however it is malformed in some way
    fn is_list(data: &[char], pos: usize) -> Result<bool, CodecError> {
        Ok(false)
    }
    /// If the next value is a map
    ///
    /// # Errors
    /// The given input is this type, however it is malformed in some way
    fn is_map(data: &[char], pos: usize) -> Result<bool, CodecError> {
        Ok(false)
    }
    /// If the next value is none
    ///
    /// # Errors
    /// The given input is this type, however it is malformed in some way
    fn is_none(data: &[char], pos: usize) -> Result<bool, CodecError> {
        Ok(false)
    }
    /// If the next value is a bool
    ///
    /// # Errors
    /// The given input is this type, however it is malformed in some way
    fn is_bool(data: &[char], pos: usize) -> Result<bool, CodecError> {
        Ok(false)
    }
    /// If the next value is a time
    ///
    /// # Errors
    /// The given input is this type, however it is malformed in some way
    fn is_time(data: &[char], pos: usize) -> Result<bool, CodecError> {
        Ok(false)
    }
    /// If the next value is a datetime
    ///
    /// # Errors
    /// The given input is this type, however it is malformed in some way
    fn is_datetime(data: &[char], pos: usize) -> Result<bool, CodecError> {
        Ok(false)
    }
    /// If the next value is an angle
    ///
    /// # Errors
    /// The given input is this type, however it is malformed in some way
    fn is_angle(data: &[char], pos: usize) -> Result<bool, CodecError> {
        Ok(false)
    }
    /// If the next value is a literal
    ///
    /// # Errors
    /// The given input is this type, however it is malformed in some way
    fn is_literal(data: &[char], pos: usize) -> Result<bool, CodecError> {
        Ok(false)
    }
    /// If the next value is a length
    ///
    /// # Errors
    /// The given input is this type, however it is malformed in some way
    fn is_length(data: &[char], pos: usize) -> Result<bool, CodecError> {
        Ok(false)
    }
    /// If the next value is a color
    ///
    /// # Errors
    /// The given input is this type, however it is malformed in some way
    fn is_color(data: &[char], pos: usize) -> Result<bool, CodecError> {
        Ok(false)
    }
    /// If the next value is bytes
    ///
    /// # Errors
    /// The given input is this type, however it is malformed in some way
    fn is_bytes(data: &[char], pos: usize) -> Result<bool, CodecError> {
        Ok(false)
    }
}

#[allow(unused_variables)]
#[allow(clippy::missing_errors_doc)]
/// Parsing half — mutates position and value count
///
/// # Safety
/// These functions assume that [`StaticParserDetect`] has correctly identified a type without checking again
pub unsafe trait StaticParserParse {
    /// This is called once before anything else is parsed
    fn parse_entry(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<Option<PositionedValue>, CodecError> {
        Ok(None)
    }

    /// Parses with the expectancy that the next value is guaranteed to be a number
    fn parse_number(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, CodecError> {
        Err(CodecError::UnsupportedType {
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
    ) -> Result<PositionedValue, CodecError> {
        Err(CodecError::UnsupportedType {
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
    ) -> Result<PositionedValue, CodecError> {
        Err(CodecError::UnsupportedType {
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
    ) -> Result<PositionedValue, CodecError> {
        Err(CodecError::UnsupportedType {
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
    ) -> Result<PositionedValue, CodecError> {
        Err(CodecError::UnsupportedType {
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
    ) -> Result<PositionedValue, CodecError> {
        Err(CodecError::UnsupportedType {
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
    ) -> Result<PositionedValue, CodecError> {
        Err(CodecError::UnsupportedType {
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
    ) -> Result<PositionedValue, CodecError> {
        Err(CodecError::UnsupportedType {
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
    ) -> Result<PositionedValue, CodecError> {
        Err(CodecError::UnsupportedType {
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
    ) -> Result<PositionedValue, CodecError> {
        Err(CodecError::UnsupportedType {
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
    ) -> Result<PositionedValue, CodecError> {
        Err(CodecError::UnsupportedType {
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
    ) -> Result<PositionedValue, CodecError> {
        Err(CodecError::UnsupportedType {
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
    ) -> Result<PositionedValue, CodecError> {
        Err(CodecError::UnsupportedType {
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
