use crate::{
    error::CodecError,
    parsers::{
        helper::skip_whitespace,
        json::{get_char, is_bool, is_number, is_string, parse_bool, parse_list, parse_number},
    },
    traits::{StaticInfo, StaticParserDetect, StaticParserParse},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// The default toml codec
pub struct DefaultToml;

impl StaticInfo for DefaultToml {
    const NAME: &'static str = "TOML";
    const SUPPORTED_EXTENSIONS: &'static [&'static str] = &["toml"];
}
unsafe impl StaticParserDetect for DefaultToml {
    fn is_number(data: &[char], pos: usize) -> Result<bool, CodecError> {
        is_number(data, pos)
    }
    fn is_bool(data: &[char], pos: usize) -> Result<bool, CodecError> {
        is_bool(data, pos)
    }
    fn is_map(data: &[char], pos: usize) -> Result<bool, CodecError> {
        let char = get_char(data, pos, 0)?;
        if '['.ne(char) {
            return Ok(false);
        }
        let new_line = data[pos..]
            .iter()
            .position(|x| *x == '\n')
            .unwrap_or(data.len() - pos)
            + pos;
        Ok(data[pos..new_line].contains(&']'))
    }
    fn is_string(data: &[char], pos: usize) -> Result<bool, CodecError> {
        is_string(data, pos)
    }
}
unsafe impl StaticParserParse for DefaultToml {
    fn skip_whitespace(data: &[char], pos: &mut usize, value_count: &mut usize) {
        skip_whitespace(data, pos, value_count);
    }
    fn parse_bool(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<crate::values::PositionedValue, crate::error::CodecError> {
        parse_bool(data, pos, value_count)
    }
    fn parse_number(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<crate::values::PositionedValue, crate::error::CodecError> {
        parse_number(data, pos, value_count)
    }
    fn parse_list(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<crate::values::PositionedValue, crate::error::CodecError> {
        parse_list::<Self>(data, pos, value_count)
    }
    fn parse_map(
        _data: &[char],
        _pos: &mut usize,
        _value_count: &mut usize,
    ) -> Result<crate::values::PositionedValue, crate::error::CodecError> {
        todo!()
    }
}
