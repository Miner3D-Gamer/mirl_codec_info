#![allow(clippy::missing_errors_doc)]
use mirl_values::prelude::*;

use crate::{
    PositionRange, error::CodecError, parsers::{helper::{deal_with_data, skip_whitespace}, json::get_char}, settings::MapType, traits::{StaticInfo, StaticParserDetect, StaticParserParse}, values::PositionedValue
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// The css parser
pub struct DefaultCSS;

impl StaticInfo for DefaultCSS {
    const NAME: &'static str = "CSS";

    const SUPPORTED_EXTENSIONS: &'static [&'static str] = &["css", "css3"];
}
/// What chars the keys are allowed to have
pub const ALLOWED_KEY_CHARS: &str = "qwertyuiopasdfghjklzxcvbnm-";
/// What chars the map names are allowed to have + chars between
pub const ALLOWED_MAP_CHARS: &str = "qwertyuiopasdfghjklzxcvbnm:,. \n-1234567890";
/// What chars the map names are allowed to have
pub const ALLOWED_MAP_NAME_CHARS: &str = "qwertyuiopasdfghjklzxcvbnm";

unsafe impl StaticParserDetect for DefaultCSS {
    fn is_map(data: &[char], pos: usize) -> Result<bool, CodecError> {
        let mut pos = pos;
        let mut has_name = false;
        loop {
            let char = get_char(data, pos, 0)?;
            if '{'.eq(char) {
                return Ok(has_name);
            }

            if ALLOWED_MAP_NAME_CHARS
                .contains(unsafe { char.to_lowercase().nth(0).unwrap_unchecked() })
            {
                has_name = true;
            }
            if !ALLOWED_MAP_CHARS.contains(unsafe { char.to_lowercase().nth(0).unwrap_unchecked() })
            {
                return Ok(false);
            }

            pos += 1;
        }
    }
}
unsafe impl StaticParserParse for DefaultCSS {
    fn skip_whitespace(data: &[char], pos: &mut usize, value_count: &mut usize) {
        skip_whitespace(data, pos, value_count);
    }
}
/// Parse a css file
pub fn parse_top_level(
    data: &[char],
    pos: &mut usize,
    value_count: &mut usize,
) -> Result<crate::values::PositionedValue, crate::error::CodecError> {
    let mut css_file = MapType::new();
    let start = *pos;

    loop {
        skip_whitespace(data, pos, value_count);
        if *pos == data.len() - 1 {
            break;
        }
        let names = parse_names(data, pos, value_count)?;

        let items = parse_map(data, pos, value_count)?;

        *value_count += 1;
        css_file.insert(names, items);
    }
    let v = PositionedValue {
        value: mirl_values::values::Value::Container(mirl_values::values::ContainerValue::Map(
            css_file,
        )),
        position: PositionRange::new(start, *pos),
        item_id: *value_count,
        container: None,
    };
    Ok(v)
}
/// Parse a CSS map
pub fn parse_map(
    data: &[char],
    pos: &mut usize,
    value_count: &mut usize,
) -> Result<crate::values::PositionedValue, crate::error::CodecError> {
    let mut map = MapType::new();
    let start = *pos;
    *pos += 1;
    loop {
        let key = parse_key_value(data, pos, value_count)?;
        let Some(char) = data.get(*pos) else {
            return Err(crate::error::CodecError::ExpectedEOF {
                offset: *pos,
                origin: Some(ValueType::Map),
                text: data.iter().collect(),
            });
        };
        skip_whitespace(data, pos, value_count);

        if ':'.ne(char) {
            return Err(crate::error::CodecError::UnexpectedCharacter {
                offset: *pos,
                given: *char,
                expected: vec![':'],
                text: data.iter().collect(),
                error: crate::error::ParserMishaps::Map(
                    crate::error::ParserMapMishap::DataAfterKeyElement,
                ),
            });
        }
        *pos += 1;
        let value = parse_value(data, pos, value_count)?;
        skip_whitespace(data, pos, value_count);
        let Some(char) = data.get(*pos) else {
            return Err(crate::error::CodecError::ExpectedEOF {
                offset: *pos,
                origin: Some(ValueType::Map),
                text: data.iter().collect(),
            });
        };
        map.insert(key, value);
        if '}'.eq(char) {
            return Ok(PositionedValue {
                value: mirl_values::values::Value::Container(
                    mirl_values::values::ContainerValue::Map(map),
                ),
                position: PositionRange::new(start, *pos),
                item_id: *value_count,
                container: None,
            });
        }
        if ','.eq(char) {
            *pos += 1;
            continue;
        }
        return Err(crate::error::CodecError::UnexpectedCharacter {
            offset: *pos,
            given: *char,
            expected: vec!['}'],
            text: data.iter().collect(),
            error: crate::error::ParserMishaps::Map(
                crate::error::ParserMapMishap::DataAfterValueElement,
            ),
        });
    }
}
/// Parse the nex css value
pub fn parse_value(
    data: &[char],
    pos: &mut usize,
    value_count: &mut usize,
) -> Result<crate::values::PositionedValue, crate::error::CodecError> {
    let val = deal_with_data::<DefaultCSS>(data, pos, value_count)?;
    Ok(val)
}
/// Parse the next css value
pub fn parse_key_value(
    data: &[char],
    pos: &mut usize,
    value_count: &mut usize,
) -> Result<crate::values::PositionedValue, crate::error::CodecError> {
    let start = *pos;
    let mut key = String::new();
    while let Some(char) = data.get(*pos) {
        if !ALLOWED_KEY_CHARS.contains(*char) {
            skip_whitespace(data, pos, value_count);
            let Some(char) = data.get(*pos) else {
                return Err(crate::error::CodecError::UnexpectedEOF {
                    offset: *pos,
                    origin: Some(ValueType::Map),
                    text: data.iter().collect(),
                });
            };
            if ALLOWED_KEY_CHARS.contains(*char) {
                key.push(*char);

                *pos += 1;
            }
            break;
        }
        *pos += 1;
    }
    if key.is_empty() {
        return Err(crate::error::CodecError::UnexpectedCharacter {
            offset: *pos,
            given: ':',
            expected: ALLOWED_KEY_CHARS.chars().collect(),
            text: data.iter().collect(),
            error: crate::error::ParserMishaps::Map(crate::error::ParserMapMishap::MissingKey),
        });
    }

    skip_whitespace(data, pos, value_count);
    Ok(PositionedValue {
        value: mirl_values::values::Value::Simple(SimpleValue::String(key)),
        position: PositionRange::new(start, *pos),
        item_id: *value_count,
        container: None,
    })
}
/// Parse the CSS names
pub fn parse_names(
    data: &[char],
    pos: &mut usize,
    value_count: &mut usize,
) -> Result<crate::values::PositionedValue, crate::error::CodecError> {
    let start = *pos;
    let mut state_collect_more = false;
    let mut names: Vec<crate::values::PositionedValue> = Vec::new();
    loop {
        let name_start = *pos;
        let mut name = String::new();
        // Get name
        loop {
            let Some(char) = data.get(*pos) else {
                return Err(crate::error::CodecError::ExpectedEOF {
                    offset: *pos,
                    origin: Some(ValueType::Map),
                    text: data.iter().collect(),
                });
            };
            if !ALLOWED_MAP_NAME_CHARS.contains(*char) {
                skip_whitespace(data, pos, value_count);

                let Some(char) = data.get(*pos) else {
                    return Err(crate::error::CodecError::ExpectedEOF {
                        offset: *pos,
                        origin: Some(ValueType::Map),
                        text: data.iter().collect(),
                    });
                };
                if '{'.eq(char) {
                    state_collect_more = false;
                    break;
                }
                if ','.eq(char) {
                    *pos += 1;
                    skip_whitespace(data, pos, value_count);
                    state_collect_more = true;
                    break;
                }
                return Err(crate::error::CodecError::UnexpectedCharacter {
                    offset: *pos,
                    given: *char,
                    expected: vec![',', '{'],
                    text: data.iter().collect(),
                    error: crate::error::ParserMishaps::Array(
                        crate::error::ParserArrayMishap::Other,
                    ),
                });
            }
            name.push(*char);
            if !state_collect_more {
                break;
            }
        }
        names.push(PositionedValue {
            value: mirl_values::values::Value::Simple(SimpleValue::Literal(name)),
            position: PositionRange::new(name_start, *pos),
            item_id: *value_count,
            container: None,
        });
        *value_count += 1;
        if !state_collect_more {
            break;
        }
    }
    let v = PositionedValue {
        value: mirl_values::values::Value::Container(mirl_values::values::ContainerValue::Vec(
            names,
        )),
        position: PositionRange::new(start, *pos),
        item_id: *value_count,
        container: None,
    };
    *value_count += 1;
    Ok(v)
}
