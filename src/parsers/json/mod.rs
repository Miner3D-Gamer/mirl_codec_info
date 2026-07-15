#![allow(clippy::missing_errors_doc)]
use std::str::FromStr;

use mirl_extensions::*;
use mirl_graphics::misc::hex_to_number;
use mirl_values::{
    prelude::*,
    values::{ContainerValue, Number, Value},
};

use crate::{
    CodecError, PositionRange, StaticParser,
    error::{ParserMishaps, ParserNumberMishap, ParserStringMishap},
    parsers::helper::{
        access_data, deal_with_data, does_data_start_with_keyword, figure_out_next_type,
        skip_whitespace,
    },
    settings::*,
    traits::{
        MarshalError, StaticCompactMarshalBase, StaticInfo, StaticParserDetect, StaticParserParse,
    },
    values::{PositionedValue, PositionedValueInner},
};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// The Json parser
pub struct DefaultJson;

impl StaticInfo for DefaultJson {
    const NAME: &'static str = "JSON";
    const SUPPORTED_EXTENSIONS: &'static [&'static str] = &["json"];
}
unsafe impl StaticParserDetect for DefaultJson {
    fn is_number(data: &[char], pos: usize) -> Result<bool, CodecError> {
        is_number(data, pos)
    }

    fn is_string(data: &[char], pos: usize) -> Result<bool, CodecError> {
        is_string(data, pos)
    }

    fn is_list(data: &[char], pos: usize) -> Result<bool, CodecError> {
        is_list(data, pos)
    }

    fn is_map(data: &[char], pos: usize) -> Result<bool, CodecError> {
        is_map(data, pos)
    }

    fn is_none(data: &[char], pos: usize) -> Result<bool, CodecError> {
        is_none(data, pos)
    }

    fn is_bool(data: &[char], pos: usize) -> Result<bool, CodecError> {
        is_bool(data, pos)
    }
}

unsafe impl StaticParserParse for DefaultJson {
    fn parse_number(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, CodecError> {
        parse_number(data, pos, value_count)
    }

    fn parse_string(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, CodecError> {
        parse_string(data, pos, value_count)
    }

    fn parse_list(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, CodecError> {
        parse_list::<Self>(data, pos, value_count)
    }

    fn parse_map(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, CodecError> {
        parse_map(data, pos, value_count)
    }

    fn parse_none(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, CodecError> {
        parse_none(data, pos, value_count)
    }

    fn parse_bool(
        data: &[char],
        pos: &mut usize,
        value_count: &mut usize,
    ) -> Result<PositionedValue, CodecError> {
        parse_bool(data, pos, value_count)
    }
    fn skip_whitespace(data: &[char], pos: &mut usize, value_count: &mut usize) {
        skip_whitespace(data, pos, value_count);
    }

    // fn is_comment(data: &[char], pos: usize) -> Result <bool, CodecError> {
    //     super::helper::is_comment(data, pos)
    // }

    // fn purge_comments(
    //     data: &mut [char],
    //     pos: &mut usize,
    // ) -> Result< ParsingError> {
    //     super::helper::purge_comments(data, pos)
    //     // Err(ParsingError::Unknown)
    // }
}
use crate::traits::StaticCompactMarshal;

impl StaticCompactMarshalBase<PositionedValueInner> for DefaultJson {
    fn marshal_compact_string(input: &str, _depth: usize) -> Result<String, MarshalError> {
        Ok(format!("\"{input}\""))
    }

    fn marshal_compact_number(input: &Number, _depth: usize) -> Result<String, MarshalError> {
        Ok(input.to_string())
    }

    fn marshal_compact_array(
        input: &[PositionedValue],
        depth: usize,
    ) -> Result<String, MarshalError> {
        let mut output = String::new();
        for i in input {
            output += &format!("{},", Self::to_compact_string(i, depth)?);
        }
        output.pop();

        Ok(format!("[{output}]"))
    }

    fn marshal_compact_bool(input: bool, _depth: usize) -> Result<String, MarshalError> {
        Ok(if input { TRUE_KEYWORD } else { FALSE_KEYWORD }.to_string())
    }

    fn marshal_compact_none(_depth: usize) -> Result<String, MarshalError> {
        Ok(NONE_KEYWORD.to_string())
    }

    fn marshal_compact_map(
        input: &MapType<PositionedValue, PositionedValue>,
        depth: usize,
    ) -> Result<String, MarshalError> {
        let mut output = String::new();

        for (key, val) in input.iter() {
            output += &format!(
                "{}:{},",
                Self::to_compact_string(key, depth)?,
                Self::to_compact_string(val, depth)?
            );
        }
        output.pop();

        Ok(format!("{{{output}}}"))
    }
}

/// Checks if the char is a number, -, or .
pub fn is_number(data: &[char], pos: usize) -> Result<bool, CodecError> {
    let first = *get_char(data, pos, 0)?;

    Ok(first.is_numeric()
        || (first == '-' && (get_char(data, pos, 1)?).is_numeric())
        || (MISSING_INTEGER_AUTOMATICALLY_PLACED && first == '.'))
}
/// Get char from data at offset, error otherwise
pub fn get_char(data: &[char], pos: usize, offset: usize) -> Result<&char, CodecError> {
    let Some(first) = data.get(pos + offset) else {
        return Err(CodecError::UnexpectedEOF {
            offset: pos,
            origin: Some(ValueType::Number),
            text: "Expected value, got end of file".to_string(),
        });
    };
    Ok(first)
}

/// Checks if the first char is a quote
pub fn is_string(data: &[char], pos: usize) -> Result<bool, CodecError> {
    for i in STRING_INDICATOR {
        if first_char_is_value(data, *i, pos)? {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Checks if the first char is a list start symbol
pub fn is_list(data: &[char], pos: usize) -> Result<bool, CodecError> {
    first_char_is_value(data, LIST_START, pos)
}

/// Checks if the first char is a map start symbol
pub fn is_map(data: &[char], pos: usize) -> Result<bool, CodecError> {
    first_char_is_value(data, MAP_START, pos)
}

/// Checks if the first char of the none keyword matches
pub fn is_none(data: &[char], pos: usize) -> Result<bool, CodecError> {
    first_char_is_value(
        data,
        unsafe { NONE_KEYWORD.chars().next().unwrap_unchecked() },
        pos,
    )
}

/// Checks if the first char of any boolean keyword matches
pub fn is_bool(data: &[char], pos: usize) -> Result<bool, CodecError> {
    Ok(first_char_is_value(
        data,
        unsafe { TRUE_KEYWORD.chars().next().unwrap_unchecked() },
        pos,
    )? || first_char_is_value(
        data,
        unsafe { FALSE_KEYWORD.chars().next().unwrap_unchecked() },
        pos,
    )?)
}
/// Checks if if the first char matches the given char
pub fn first_char_is_value(data: &[char], value: char, pos: usize) -> Result<bool, CodecError> {
    let char = *get_char(data, pos, 0)?;
    Ok(if ALLOW_NON_LOWERCASE_KEYWORDS {
        char.to_ascii_lowercase()
    } else {
        char
    } == value)
}
#[allow(clippy::too_many_lines)]
/// Parse a number - int or float
pub fn parse_number(
    data: &[char],
    pos: &mut usize,
    value_count: &mut usize,
) -> Result<PositionedValue, CodecError> {
    // let c = data.iter().collect::<String>();
    // if c.len() < 500 {
    //     println!("Parsing num at {} with {} ({})", pos, value_count, c);
    // } else {
    //     println!("Input too big");
    // }
    let mut output = String::new();
    let start = *pos;
    // Minus is allowed at start
    if let Some(char) = data.get(*pos)
        && *char == '-'
    {
        output.push(*char);
        *pos += 1;
    }
    let value_is_zero = !ALLOW_UNNECESSARY_ZEROS
        && if let Some(char) = data.get(*pos)
            && *char == '0'
        {
            output.push(*char);
            *pos += 1;
            true
        } else {
            false
        };
    let mut value_is_float = false;

    let mut scientific_notation = 0;

    while let Some(char) = data.get(*pos) {
        // If number or dot, add to number
        if char.is_ascii_digit() || *char == '.'
        // && (scientific_notation != 1 || scientific_notation == 2)
        {
            // But not if it's a second dot
            if *char == '.' {
                if output.contains('.') {
                    let message = if unsafe { output.pop().unwrap_unchecked() } == '.' {
                        ParserNumberMishap::RangeNotAllowed
                    } else {
                        ParserNumberMishap::MultipleDots
                    };
                    return Err(CodecError::UnexpectedCharacter {
                        offset: *pos,
                        given: *char,
                        expected: NUMBER_CHARS.to_vec(),

                        text: data.iter().collect(),
                        error: ParserMishaps::Number(message),
                    });
                }
                if ['-'].contains(&data[*pos - 1]) {
                    if MISSING_INTEGER_AUTOMATICALLY_PLACED {
                        output.push('0');
                    } else {
                        return Err(CodecError::UnexpectedCharacter {
                            offset: *pos,
                            given: *char,
                            expected: NUMBER_CHARS.to_vec(),
                            text: data.iter().collect(),
                            error: ParserMishaps::Number(
                                crate::error::ParserNumberMishap::FloatWithoutInt,
                            ),
                        });
                    }
                }
                value_is_float = true;
            }
            output.push(*char);
            *pos += 1;
        } else if NUMBER_ALLOW_UNDERSCORE && *char == '_' {
            *pos += 1;
            continue;
        } else if scientific_notation == 0 && char.eq_ignore_ascii_case(&'e')
            || (scientific_notation == 1 && (*char == '-' || *char == '+'))
        {
            if char.eq_ignore_ascii_case(&'e') && ['.', '-'].contains(&data[*pos - 1]) {
                return Err(CodecError::UnexpectedCharacter {
                    offset: *pos,
                    given: *char,
                    expected: NUMBER_CHARS.to_vec(),

                    text: data.iter().collect(),
                    error: ParserMishaps::Number(
                        crate::error::ParserNumberMishap::ExponentWithoutBase,
                    ),
                });
            }
            output.push(*char);
            *pos += 2; // THIS SHOULD BE A 2 INSTEAD OF A 1, FINALLY
            scientific_notation += 1;
        } else if WHITESPACE_CHARACTERS
            .to_vec()
            .combined_with([LIST_END, MAP_END, ELEMENT_SEPARATOR].to_vec())
            .contains(char)
        {
            // Number ends naturally
            break;
        } else {
            return Err(CodecError::UnexpectedCharacter {
                offset: *pos,
                given: *char,
                expected: NUMBER_CHARS
                    .to_vec()
                    .combined_with(ALLOWED_ELEMENT_INTERRUPTIONS.to_vec()),

                text: data.iter().collect(),
                error: ParserMishaps::Number(
                    crate::error::ParserNumberMishap::NumberContainsNonNumber,
                ),
            });
        }
        if value_is_zero && !value_is_float && scientific_notation == 0 {
            return Err(CodecError::UnexpectedCharacter {
                offset: *pos,
                given: *char,
                expected: ALLOWED_ELEMENT_INTERRUPTIONS.to_vec(),

                text: data.iter().collect(),
                error: ParserMishaps::Number(crate::error::ParserNumberMishap::LeadingZeros),
            });
        }
    }
    if output == "-" || output == "." {
        return Err(CodecError::UnexpectedCharacter {
            offset: *pos,
            given: '-',
            expected: if MISSING_INTEGER_AUTOMATICALLY_PLACED {
                NUMBER_CHARS.to_vec().combined('.')
            } else {
                NUMBER_CHARS.to_vec()
            },

            text: data.iter().collect(),
            error: ParserMishaps::Number(crate::error::ParserNumberMishap::NumberWithoutNumber),
        });
    }
    if output.ends_with('.') {
        if MISSING_INTEGER_AUTOMATICALLY_PLACED {
            output.push('0');
        } else {
            return Err(CodecError::UnexpectedCharacter {
                offset: *pos,
                given: '.',
                expected: NUMBER_CHARS.to_vec(),

                text: data.iter().collect(),
                error: ParserMishaps::Number(crate::error::ParserNumberMishap::IncompleteFloat),
            });
        }
    }
    // if scientific_notation == 1 {
    //     return Err(ParsingError::UnexpectedCharacter {
    //         offset: *pos,
    //         given: 'e',
    //         expected: vec!['+', '-'],
    //
    //         text: data.iter().collect(),
    //         description: "Expected + or - after incomplete scientific notation"
    //             .to_string(),
    //     });
    // }
    if ['+', '-', 'e', 'E'].contains(&data[*pos - 1]) {
        return Err(CodecError::UnexpectedCharacter {
            offset: *pos,
            given: data[*pos - 1],
            expected: NUMBER_CHARS.to_vec(),

            text: data.iter().collect(),
            error: ParserMishaps::Number(
                crate::error::ParserNumberMishap::IncompleteScientificNotation,
            ),
        });
    }
    let v = PositionedValue {
        value: SimpleValue::Number(Number::from_str(&output).map_or_else(
            |()| {
                Err(CodecError::UnexpectedCharacter {
                    offset: *pos,
                    given: '?',
                    expected: Vec::new(),
                    text: data.iter().collect(),
                    error: ParserMishaps::Number(ParserNumberMishap::Other),
                })
            },
            Ok,
        )?)
        .into(),
        position: PositionRange::new(start, *pos),
        item_id: *value_count,
        container: None,
    };
    *value_count += 1;
    // println!("Output {}: {:?}", pos, v);
    Ok(v)
}
#[allow(clippy::too_many_lines)]
/// Parse a string value
pub fn parse_string(
    data: &[char],
    pos: &mut usize,
    value_count: &mut usize,
) -> Result<PositionedValue, CodecError> {
    let mut output = String::new();
    let start = *pos;
    let mut escaping = false;
    let mut done = false;

    let string_type = *data.get(*pos).map_or_else(
        || {
            Err(CodecError::UnexpectedEOF {
                offset: *pos,
                origin: Some(ValueType::String),
                text: data.iter().collect(),
            })
        },
        Ok,
    )?; // Man, I love rust
    if string_type != '"' {
        // TODO: ERROR
        return Err(CodecError::UnexpectedCharacter {
            offset: *pos,
            given: string_type,
            expected: vec!['"'],

            text: data.iter().collect(),
            error: ParserMishaps::String(ParserStringMishap::WrongQuotation),
        });
    }

    *pos += 1;

    while let Some(char) = data.get(*pos) {
        if *char == ESCAPE_CHARACTER {
            if !escaping {
                escaping = true;
                *pos += 1;
                continue;
            }
        } else if *char == string_type && !escaping {
            done = true;
            *pos += 1;
            break;
        } else if escaping && *char == 'u' {
            let potential_error_pos = *pos - 1;
            output.push(*char);
            *pos += 1;
            // Special unicode support \uXXXX
            let mut all_good = true;
            let mut temp_unicode = Vec::new();
            for _ in 0..4 {
                if let Some(unicode_char) = data.get(*pos) {
                    temp_unicode.push(*unicode_char);
                    *pos += 1;
                } else {
                    all_good = false;
                    break;
                }
            }
            if !all_good {
                return Err(CodecError::UnexpectedEOF {
                    offset: *pos,
                    origin: Some(ValueType::String),
                    text: data.iter().collect(),
                });
            }
            let string = temp_unicode.iter().collect::<String>();
            let Some(_number) = hex_to_number(&string) else {
                return Err(CodecError::UnexpectedElement {
                    value: PositionedValue {
                        value: SimpleValue::String(string).into(),
                        position: PositionRange::new(potential_error_pos, *pos),
                        item_id: *value_count,
                        container: None,
                    },

                    error: ParserMishaps::String(ParserStringMishap::InvalidUnicodeSequence),
                    expected: vec![ValueType::String],
                    text: data.iter().collect(), // Invalid unicode sequence
                });
            };
            // if number > u32::from(u16::MAX) {
            //     return Err(ParsingError::UnexpectedElement {
            //         value: Value::String(
            //             string,
            //             PositionRange::new(potential_error_pos, *pos),
            //         ),
            //         origin: Some(ValueType::String),
            //         expected: vec![ValueType::String],
            //         text: "Invalid unicode sequence (Value too big)"
            //             .to_string(),
            //     });
            // }

            output.extend(temp_unicode);
            escaping = false;
            continue;
        }
        if !ALLOW_NULL_CHARACTER && *char as usize == 0 {
            return Err(CodecError::UnexpectedCharacter {
                offset: *pos,
                given: *char,
                expected: Vec::new(),

                text: data.iter().collect(),
                error: ParserMishaps::String(ParserStringMishap::NullCharacter),
            });
        }
        let disallowed_to_escape = ['\t'];

        if escaping {
            if (CONTROL_CHARACTER_MUST_BE_ESCAPED
                && (!char.is_control() || disallowed_to_escape.contains(char)))
                && !ALLOWED_ESCAPED.contains(char)
            {
                return Err(CodecError::UnexpectedCharacter {
                    offset: *pos,
                    given: *char,
                    expected: ALLOWED_ESCAPED.to_vec(),

                    text: data.iter().collect(),
                    error: ParserMishaps::String(ParserStringMishap::EscapedInvalidCharacter),
                });
            }
            // println!(
            //     "here: '{}' {} {}",
            //     char,
            //     char.escape_debug(),
            //     char.is_control()
            // );
        } else if CONTROL_CHARACTER_MUST_BE_ESCAPED
            && char.is_control()
            && !CONTROL_CHARACTERS_ALLOWED_TO_BE_UNESCAPED.contains(char)
        {
            return Err(CodecError::UnexpectedCharacter {
                offset: *pos,
                given: *char,
                expected: Vec::new(),

                text: data.iter().collect(),
                error: ParserMishaps::String(ParserStringMishap::UnescapedControlCharacter),
            });
        }
        escaping = false;

        output.push(*char);
        *pos += 1;
    }
    if done {
        let end = *pos;
        Ok(PositionedValue {
            value: SimpleValue::String(output).into(),
            position: PositionRange::new(start, end),
            item_id: *value_count,
            container: None,
        })
    } else {
        Err(CodecError::UnexpectedEOF {
            offset: *pos,
            origin: Some(ValueType::String),
            text: data.iter().collect::<String>(),
        })
    }
}
/// Parse a list like structure
#[allow(clippy::too_many_lines)]
pub fn parse_list<P: StaticParser>(
    data: &[char],
    pos: &mut usize,
    value_count: &mut usize,
) -> Result<PositionedValue, CodecError> {
    let start = *pos;
    *pos += 1;
    if access_data(data, pos, Some(ValueType::Vec))? == LIST_END {
        *pos += 1;
        let v = PositionedValue {
            value: Value::new_vec(),
            position: PositionRange::new(start, *pos),
            item_id: *value_count,
            container: None,
        };
        *value_count += 1;
        return Ok(v);
    }
    if access_data(data, pos, Some(ValueType::Vec))? == ELEMENT_SEPARATOR {
        return Err(CodecError::UnexpectedCharacter {
            offset: *pos,
            given: ELEMENT_SEPARATOR,
            expected: vec![ELEMENT_SEPARATOR, LIST_END],

            text: data.iter().collect::<String>(),
            error: ParserMishaps::Array(crate::error::ParserArrayMishap::EmptyElement),
        });
    }
    let list_id = *value_count;
    *value_count += 1;
    let val = deal_with_data::<P>(data, pos, value_count)?;

    let mut values: Vec<PositionedValue> = vec![val];

    skip_whitespace(data, pos, value_count);
    let mut force_parse_next = false;

    while let Some(char) = data.get(*pos) {
        if *char == ELEMENT_SEPARATOR || force_parse_next {
            *pos += 1;
            force_parse_next = false;
            if !force_parse_next {
                P::skip_whitespace(data, pos, value_count);
                if let Some(c) = data.get(*pos) {
                    if *c == ELEMENT_SEPARATOR {
                        return Err(CodecError::UnexpectedCharacter {
                            offset: *pos,
                            given: *c,
                            expected: vec![ELEMENT_SEPARATOR, LIST_END],

                            text: data.iter().collect::<String>(),
                            error: ParserMishaps::Array(
                                crate::error::ParserArrayMishap::EmptyElement,
                            ),
                        });
                    }
                } else {
                    Err(CodecError::UnexpectedEOF {
                        offset: *pos,
                        origin: Some(ValueType::Vec),
                        text: data.iter().collect::<String>(),
                    })?;
                }
            }

            let value = super::helper::parse_next::<P>(
                data,
                pos,
                super::helper::figure_out_next_type::<P>(data, *pos)?,
                value_count,
            )?;
            values.push(value);

            skip_whitespace(data, pos, value_count);
        } else if *char == LIST_END {
            *pos += 1;
            return Ok(PositionedValue {
                value: ContainerValue::Vec(values).into(),
                position: PositionRange::new(start, *pos),
                item_id: list_id,
                container: None,
            });
        } else {
            *pos -= 0; // Why -2? Idk but it works
            let test = figure_out_next_type::<P>(data, *pos)?;
            // println!(">> '{}'", char);
            // println!("!> '{:?}'", test);
            // if test == ValueType::Comment {
            //     let (_, extra_comments) =
            //         parse_comment::<DefaultJson, false>(data, pos)?;
            //     comments.extend(extra_comments);
            //     continue;
            // }
            if AUTOMATIC_SEPARATOR_INSERTION {
                // println!(">> '{}'", char);
                // println!("!> '{:?}'", test);
                if test != ValueType::Invalid {
                    force_parse_next = true;
                    continue;
                }
            }
            return Err(CodecError::UnexpectedCharacter {
                offset: *pos,
                given: *char,
                expected: vec![ELEMENT_SEPARATOR, LIST_END],

                text: data.iter().collect::<String>(),
                error: ParserMishaps::Array(
                    crate::error::ParserArrayMishap::MissingElementSeparator,
                ),
            });
        }
    }

    Err(CodecError::UnexpectedEOF {
        offset: *pos,
        origin: Some(ValueType::Vec),
        text: data.iter().collect::<String>(),
    })
}
/// Parse {key}:{value}
pub fn parse_key_and_item(
    data: &[char],
    pos: &mut usize,
    value_count: &mut usize,
) -> Result<(PositionedValue, PositionedValue), CodecError> {
    let key = deal_with_data::<DefaultJson>(data, pos, value_count)?;

    if !ALLOW_NON_STRING_KEYS && key.get_value_type() != ValueType::String {
        return Err(CodecError::UnexpectedElement {
            value: key,
            error: ParserMishaps::Map(crate::error::ParserMapMishap::InvalidKeyType),
            expected: vec![ValueType::String],
            text: data.iter().collect(),
        });
    }

    skip_whitespace(data, pos, value_count);
    if let Some(first) = data.get(*pos) {
        if *first == MAP_POINTER {
            *pos += 1;

            let value = deal_with_data::<DefaultJson>(data, pos, value_count)?;

            return Ok((key, value));
        }
        if AUTOMATIC_SEPARATOR_INSERTION {
            *pos -= 2;

            let test = figure_out_next_type::<DefaultJson>(data, *pos)?;

            if test == ValueType::Invalid {
                return Err(CodecError::UnexpectedCharacter {
                    offset: *pos,
                    given: *first,
                    expected: [MAP_POINTER, MAP_START, LIST_START]
                        .to_vec()
                        .combined_with(NUMBER_CHARS.to_vec()),
                    text: data.iter().collect::<String>(),
                    error: ParserMishaps::Map(crate::error::ParserMapMishap::DataAfterKeyElement),
                });
            }

            *pos += 1;

            let value = deal_with_data::<DefaultJson>(data, pos, value_count)?;

            return Ok((key, value));
        }
        Err(CodecError::UnexpectedCharacter {
            offset: *pos,
            given: *first,
            expected: vec![MAP_POINTER],

            text: data.iter().collect::<String>(),
            error: ParserMishaps::Map(crate::error::ParserMapMishap::DataAfterKeyElement),
        })
    } else {
        Err(CodecError::UnexpectedEOF {
            offset: *pos,
            origin: Some(ValueType::Map),
            text: data.iter().collect::<String>(),
        })
    }
}
/// Parse a map, object, dict, or whatever else you call it {}
pub fn parse_map(
    data: &[char],
    pos: &mut usize,
    value_count: &mut usize,
) -> Result<PositionedValue, CodecError> {
    let start = *pos;
    *pos += 1;
    if access_data(data, pos, Some(ValueType::Map))? == MAP_END {
        *pos += 1;
        return Ok(PositionedValue {
            value: Value::new_map(),
            position: PositionRange::new(start, *pos),
            item_id: *value_count,
            container: None,
        });
    }

    let mut map: MapType<PositionedValue, PositionedValue> = MapType::new();
    let first = parse_key_and_item(data, pos, value_count)?;
    map.insert(first.0, first.1);
    skip_whitespace(data, pos, value_count);

    let mut force_parse_next = false;

    while let Some(first_char) = data.get(*pos) {
        if *first_char == ELEMENT_SEPARATOR || force_parse_next {
            *pos += 1;
            force_parse_next = false;
            let values = parse_key_and_item(data, pos, value_count)?;

            map.insert(values.0, values.1);
            skip_whitespace(data, pos, value_count);
        } else if *first_char == MAP_END {
            *pos += 1;
            return Ok(PositionedValue {
                value: ContainerValue::Map(map).into(),
                position: PositionRange::new(start, *pos),
                item_id: *value_count,
                container: None,
            });
        } else {
            if AUTOMATIC_SEPARATOR_INSERTION {
                //*pos -= 4; // Why -2? Idk but it works
                // println!(">> '{}'", first_char);
                let test = figure_out_next_type::<DefaultJson>(data, *pos)?;
                //println!("!> '{:?}'", test);
                *pos -= 2;
                if test != ValueType::Invalid {
                    force_parse_next = true;
                    continue;
                }
            }
            return Err(CodecError::UnexpectedCharacter {
                offset: *pos,
                given: *first_char,
                expected: vec![ELEMENT_SEPARATOR, MAP_END],
                text: data.iter().collect::<String>(),
                error: ParserMishaps::Map(crate::error::ParserMapMishap::DataAfterValueElement),
            });
        }
    }

    Err(CodecError::UnexpectedEOF {
        offset: *pos,
        origin: Some(ValueType::Map),
        text: data.iter().collect::<String>(),
    })
}
/// Parse the none, `null` keyword
pub fn parse_none(
    data: &[char],
    pos: &mut usize,
    value_count: &mut usize,
) -> Result<PositionedValue, CodecError> {
    let val = does_data_start_with_keyword(data, pos, NONE_KEYWORD.to_string(), ValueType::None)?;
    Ok(PositionedValue {
        value: Value::new_none(),
        position: val,
        item_id: *value_count,
        container: None,
    })
}
/// Parse true false
pub fn parse_bool(
    data: &[char],
    pos: &mut usize,
    value_count: &mut usize,
) -> Result<PositionedValue, CodecError> {
    if access_data(data, pos, Some(ValueType::Bool))?
        == unsafe { TRUE_KEYWORD.chars().next().unwrap_unchecked() }
    {
        Ok(PositionedValue {
            value: Value::from_bool(true),
            position: does_data_start_with_keyword(
                data,
                pos,
                TRUE_KEYWORD.to_string(),
                ValueType::Bool,
            )?,
            item_id: *value_count,
            container: None,
        })
    } else {
        Ok(PositionedValue {
            value: Value::from_bool(false),
            position: does_data_start_with_keyword(
                data,
                pos,
                FALSE_KEYWORD.to_string(),
                ValueType::Bool,
            )?,
            item_id: *value_count,
            container: None,
        })
    }
}
