use crate::{
    error::ParsingError, settings::ERROR_ON_EXPECTED_EOF, traits::DynInfo,
    values::PositionedValue,
};
mod dynamic;
mod undeviating; // 'static' is already a keyword
pub use dynamic::*;
use mirl_values::values::ValueType;
pub use undeviating::*;

#[allow(clippy::wrong_self_convention)]
/// Helper functions for parsers
pub trait DynParserHelper {
    /// Parse json text into [Value]
    /// # Errors
    /// Errors upon invalid/corrupt data
    fn from_str(
        &mut self,
        data: &str,
    ) -> Result<Option<PositionedValue>, ParsingError>;
    /// Skips any whitespace and parses the next element
    ///
    /// # Errors
    /// Errors upon invalid/corrupt data
    fn deal_with_data(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError>;
    /// Skips any whitespace and parses the next element
    ///
    /// # Errors
    /// Errors upon invalid/corrupt data
    fn figure_out_next_type(&mut self, data: &[char], pos: usize) -> ValueType;
    #[track_caller]
    /// Parse the given data into the next value
    ///
    /// # Errors
    /// Errors upon invalid/corrupt data
    fn parse_next(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_type: ValueType,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError>;
}

impl<T: DynParser + DynInfo> DynParserHelper for T {
    #[track_caller]
    fn parse_next(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_type: ValueType,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        match value_type {
            ValueType::Number => self.parse_number(data, pos, value_count),
            ValueType::String => self.parse_string(data, pos, value_count),
            ValueType::Vec => self.parse_list(data, pos, value_count),
            ValueType::Map => self.parse_map(data, pos, value_count),
            ValueType::None => self.parse_none(data, pos, value_count),
            ValueType::Bool => self.parse_bool(data, pos, value_count),
            ValueType::Invalid => Err({
                data.get(*pos).map_or_else(
                    || ParsingError::UnexpectedEOF {
                        offset: *pos,
                        origin: Some(ValueType::Invalid),
                        text: data.iter().collect(),
                    },
                    |c| ParsingError::UnrecognizedType {
                        offset: *pos,
                        starting_char: *c,
                        text: data.iter().collect(),
                    },
                )
            }),
            _ => Err(ParsingError::UnsupportedType {
                pos: *pos,
                value_type,
                text: data.iter().collect(),
            }),
        }
    }
    fn figure_out_next_type(&mut self, data: &[char], pos: usize) -> ValueType {
        if self.is_string(data, pos) {
            ValueType::String
        } else if self.is_list(data, pos) {
            ValueType::Vec
        } else if self.is_number(data, pos) {
            ValueType::Number
        } else if self.is_map(data, pos) {
            ValueType::Map
        } else if self.is_bool(data, pos) {
            ValueType::Bool
        } else if self.is_none(data, pos) {
            ValueType::None
        } else {
            ValueType::Invalid
        }
    }
    #[track_caller]
    fn deal_with_data(
        &mut self,
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        self.skip_whitespace(data, pos, value_count);
        let value = self.figure_out_next_type(data, *pos);
        self.parse_next(data, pos, value, value_count)
    }
    fn from_str(
        &mut self,
        data: &str,
    ) -> Result<Option<PositionedValue>, ParsingError> {
        let chars: Vec<char> = data.chars().collect();
        let mut pos = 0;
        let mut value_count = 0;
        // let mut comment_pos = 0;

        // let comments: Vec<Comment> = if self.are_comments_allowed() {
        //     self.purge_comments(&mut chars, &mut comment_pos)?
        // } else {
        //     Vec::new()
        // };
        let val = match self.deal_with_data(&chars, &mut pos, &mut value_count)
        {
            Ok(val) => val,
            Err(err) => {
                if err == ParsingError::EmptyFile {
                    return Ok(None);
                }
                Err(err)?
            }
        };
        if ERROR_ON_EXPECTED_EOF {
            let temp = chars.clone();
            let temp_pos = pos;
            self.skip_whitespace(&chars, &mut pos, &mut value_count);
            if chars.len() != pos {
                return Err(ParsingError::ExpectedEOF {
                    offset: temp_pos,
                    origin: None,
                    text: temp.iter().collect(),
                });
            }
        }
        Ok(Some(val))
    }
}

/// Helper functions for parsers
pub trait StaticParserHelper: StaticParser {
    /// Parse json text into [Value]
    ///
    /// Only returns Ok(None) when the file contains no value (empty file)
    ///
    /// # Errors
    /// Errors upon invalid/corrupt data
    fn from_str(data: &str) -> Result<Option<PositionedValue>, ParsingError>;
    /// Skips any whitespace and parses the next element
    ///
    /// # Errors
    /// Errors upon invalid/corrupt data
    fn deal_with_data(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError>;
    /// Skips any whitespace and parses the next element
    ///
    /// # Errors
    /// Errors upon invalid/corrupt data
    fn figure_out_next_type(data: &[char], pos: usize) -> ValueType;
    #[track_caller]
    /// Parse the given data into the next value
    ///
    /// # Errors
    /// Errors upon invalid/corrupt data
    fn parse_next(
        data: &[char],
        pos: &mut usize,
        value_type: ValueType,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError>;
}

impl<T: StaticParser> StaticParserHelper for T {
    #[track_caller]
    fn parse_next(
        data: &[char],
        pos: &mut usize,
        value_type: ValueType,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        crate::parsers::helper::parse_next::<T>(
            data,
            pos,
            value_type,
            value_count,
        )
    }
    fn figure_out_next_type(data: &[char], pos: usize) -> ValueType {
        crate::parsers::helper::figure_out_next_type::<T>(data, pos)
    }
    #[track_caller]
    fn deal_with_data(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, ParsingError> {
        T::skip_whitespace(data, pos, value_count);
        let value = T::figure_out_next_type(data, *pos);
        T::parse_next(data, pos, value, value_count)
    }
    fn from_str(data: &str) -> Result<Option<PositionedValue>, ParsingError> {
        crate::from_str::<T>(data)
    }
}
