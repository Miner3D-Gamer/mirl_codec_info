use crate::{
    PositionedValue,
    error::ParsingError,
    traits::{DynInfo, DynParserHelper, StaticParser},
};

#[rustfmt::skip]
#[allow(unused_variables)]
/// Detection half of [`DynParser`] — only looks at the data, never mutates position
pub trait DynParserDetect {
    /// If the next value is a number
    fn is_number(&mut self, data: &[char], pos: usize) -> bool {false}
    /// If the next value is a string
    fn is_string(&mut self, data: &[char], pos: usize) -> bool {false}
    /// If the next value is a list
    fn is_list(&mut self, data: &[char], pos: usize) -> bool {false}
    /// If the next value is a map
    fn is_map(&mut self, data: &[char], pos: usize) -> bool {false}
    /// If the next value is none
    fn is_none(&mut self, data: &[char], pos: usize) -> bool {false}
    /// If the next value is a bool
    fn is_bool(&mut self, data: &[char], pos: usize) -> bool {false}
    /// If the next value is a time
    fn is_time(&mut self, data: &[char], pos: usize) -> bool {false}
    /// If the next value is a datetime
    fn is_datetime(&mut self, data: &[char], pos: usize) -> bool {false}
    /// If the next value is an angle
    fn is_angle(&mut self, data: &[char], pos: usize) -> bool {false}
    /// If the next value is a literal
    fn is_literal(&mut self, data: &[char], pos: usize) -> bool {false}
    /// If the next value is a length
    fn is_length(&mut self, data: &[char], pos: usize) -> bool {false}
    /// If the next value is a color
    fn is_color(&mut self, data: &[char], pos: usize) -> bool {false}
    /// If the next value is bytes
    fn is_bytes(&mut self, data: &[char], pos: usize) -> bool {false}
}
#[allow(unused_variables)]
#[allow(clippy::missing_errors_doc)]
/// Parsing half of [`DynParser`] — mutates position and value count
pub trait DynParserParse {
    /// Parses with the expectancy that the next value is guaranteed to be a number
    fn parse_number(
        &mut self,
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
        &mut self,
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
        &mut self,
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
        &mut self,
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
        &mut self,
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
        &mut self,
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
        &mut self,
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
        &mut self,
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
        &mut self,
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
        &mut self,
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
        &mut self,
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
        &mut self,
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
        &mut self,
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
    fn skip_whitespace(
        &self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    );
}

/// A parser that rust can't optimize as well but has dyn compatibility
///
/// This is automatically implemented for objects that implement [`DynParserDetect`] and [`DynParserParse`],
/// and also for any [`StaticParser`]
pub trait DynParser:
    DynParserDetect + DynParserParse + DynParserHelper
{
}

impl<T: DynParserDetect + DynParserParse + DynInfo> DynParser for T {}

impl<T: StaticParser> DynParserDetect for T {
    fn is_number(&mut self, data: &[char], pos: usize) -> bool {
        Self::is_number(data, pos)
    }
    fn is_string(&mut self, data: &[char], pos: usize) -> bool {
        Self::is_string(data, pos)
    }
    fn is_list(&mut self, data: &[char], pos: usize) -> bool {
        Self::is_list(data, pos)
    }
    fn is_map(&mut self, data: &[char], pos: usize) -> bool {
        Self::is_map(data, pos)
    }
    fn is_none(&mut self, data: &[char], pos: usize) -> bool {
        Self::is_none(data, pos)
    }
    fn is_bool(&mut self, data: &[char], pos: usize) -> bool {
        Self::is_bool(data, pos)
    }
    fn is_time(&mut self, data: &[char], pos: usize) -> bool {
        Self::is_time(data, pos)
    }
    fn is_datetime(&mut self, data: &[char], pos: usize) -> bool {
        Self::is_datetime(data, pos)
    }
    fn is_angle(&mut self, data: &[char], pos: usize) -> bool {
        Self::is_angle(data, pos)
    }
    fn is_literal(&mut self, data: &[char], pos: usize) -> bool {
        Self::is_literal(data, pos)
    }
    fn is_length(&mut self, data: &[char], pos: usize) -> bool {
        Self::is_length(data, pos)
    }
    fn is_color(&mut self, data: &[char], pos: usize) -> bool {
        Self::is_color(data, pos)
    }
    fn is_bytes(&mut self, data: &[char], pos: usize) -> bool {
        Self::is_bytes(data, pos)
    }
}

impl<T: StaticParser> DynParserParse for T {
    fn parse_number(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Self::parse_number(data, pos, value_count)
    }
    fn parse_string(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Self::parse_string(data, pos, value_count)
    }
    fn parse_list(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Self::parse_list(data, pos, value_count)
    }
    fn parse_map(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Self::parse_map(data, pos, value_count)
    }
    fn parse_none(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Self::parse_none(data, pos, value_count)
    }
    fn parse_bool(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Self::parse_bool(data, pos, value_count)
    }
    fn parse_time(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Self::parse_time(data, pos, value_count)
    }
    fn parse_datetime(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Self::parse_datetime(data, pos, value_count)
    }
    fn parse_angle(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Self::parse_angle(data, pos, value_count)
    }
    fn parse_literal(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Self::parse_literal(data, pos, value_count)
    }
    fn parse_length(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Self::parse_length(data, pos, value_count)
    }
    fn parse_color(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Self::parse_color(data, pos, value_count)
    }
    fn parse_bytes(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        Self::parse_bytes(data, pos, value_count)
    }

    fn skip_whitespace(
        &self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) {
        Self::skip_whitespace(data, pos, value_count);
    }
}
