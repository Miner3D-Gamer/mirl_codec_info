use crate::traits::StaticParser;

/// The `GenEx` parser
#[derive(Debug, Clone, Copy)]
pub struct GenEx {}

impl StaticParser for GenEx {
    fn is_number(data: &[char], pos: usize) -> bool {
        super::json::is_number(data, pos)
    }

    fn is_string(data: &[char], pos: usize) -> bool {
        super::json::is_string(data, pos)
    }

    fn is_list(data: &[char], pos: usize) -> bool {
        super::json::is_list(data, pos)
    }

    fn is_map(_data: &[char], _pos: usize) -> bool {
        false
    }

    fn is_none(_data: &[char], _pos: usize) -> bool {
        false
    }

    fn is_bool(_data: &[char], _pos: usize) -> bool {
        false
    }

    // fn is_comment(data: &[char], pos: usize) -> bool {
    //     todo!()
    // }

    fn parse_number(
        data: &[char],
        pos: &mut usize,
        _value_count: &mut usize,
    ) -> Result<crate::values::Value, crate::error::ParsingError> {
        super::json::parse_number(data, pos)
    }

    fn parse_string(
        data: &[char],
        pos: &mut usize,
        _value_count: &mut usize,
    ) -> Result<crate::values::Value, crate::error::ParsingError> {
        super::json::parse_string(data, pos)
    }

    fn parse_list(
        data: &[char],
        pos: &mut usize,
        _value_count: &mut usize,
    ) -> Result<crate::values::Value, crate::error::ParsingError> {
        super::json::parse_list::<Self>(data, pos)
    }

    fn parse_map(
        _data: &[char],
        _pos: &mut usize,
        _value_count: &mut usize,
    ) -> Result<crate::values::Value, crate::error::ParsingError> {
        todo!()
    }

    fn parse_none(
        _data: &[char],
        _pos: &mut usize,
        _value_count: &mut usize,
    ) -> Result<crate::values::Value, crate::error::ParsingError> {
        unimplemented!()
    }

    fn parse_bool(
        data: &[char],
        pos: &mut usize,
        _value_count: &mut usize,
    ) -> Result<crate::values::Value, crate::error::ParsingError> {
        super::json::parse_bool(data, pos)
    }

    fn skip_whitespace(
        data: &[char],
        pos: &mut usize,
        _value_count: &mut usize,
    ) {
        super::helper::skip_whitespace(data, pos);
    }
}
