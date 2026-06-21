use mirl_values::prelude::*;

use crate::{
    PositionRange, error::CodecError, settings::*, traits::StaticParser, values::PositionedValue,
};
/// A generic whitespace skipper
pub fn skip_whitespace(data: &[char], pos: &mut usize, _value_count: &mut usize) {
    while let Some(char) = data.get(*pos) {
        if WHITESPACE_CHARACTERS.contains(char) {
            *pos += 1;
        } else {
            break;
        }
    }
}
/// Safely access the current position
///
/// # Errors
/// When the position if out of bounce
pub fn access_data(
    data: &[char],
    pos: &mut usize,
    origin: Option<ValueType>,
) -> Result<char, CodecError> {
    data.get(*pos).map_or_else(
        || {
            Err(CodecError::UnexpectedEOF {
                offset: *pos,
                origin,
                text: data.iter().collect(),
            })
        },
        |val| Ok(*val),
    )
}
#[track_caller]
/// Parse the given data into the next value
///
/// # Errors
/// Errors upon invalid/corrupt data
pub fn parse_next<T: StaticParser>(
    data: &[char],
    pos: &mut usize,
    value_type: ValueType,
    value_count: &mut usize,
) -> Result<PositionedValue, CodecError> {
    match value_type {
        ValueType::Number => T::parse_number(data, pos, value_count),
        ValueType::String => T::parse_string(data, pos, value_count),
        ValueType::Vec => T::parse_list(data, pos, value_count),
        ValueType::Map => T::parse_map(data, pos, value_count),
        ValueType::None => T::parse_none(data, pos, value_count),
        ValueType::Bool => T::parse_bool(data, pos, value_count),
        ValueType::Time => T::parse_time(data, pos, value_count),
        ValueType::DateTime => T::parse_datetime(data, pos, value_count),
        ValueType::Angle => T::parse_angle(data, pos, value_count),
        ValueType::Literal => T::parse_literal(data, pos, value_count),
        ValueType::Length => T::parse_length(data, pos, value_count),
        ValueType::Color => T::parse_color(data, pos, value_count),
        ValueType::Bytes => T::parse_bytes(data, pos, value_count),
        ValueType::Invalid => Err({
            data.get(*pos).map_or_else(
                || CodecError::UnexpectedEOF {
                    offset: *pos,
                    origin: Some(ValueType::Invalid),
                    text: data.iter().collect(),
                },
                |c| CodecError::UnrecognizedType {
                    offset: *pos,
                    starting_char: *c,
                    text: data.iter().collect(),
                },
            )
        }),
    }
}

/// Try to determine the next type
/// 
/// # Errors
/// When a value is misshaped or an unexpected EOF is reached
pub fn figure_out_next_type<T: StaticParser>(
    data: &[char],
    pos: usize,
) -> Result<ValueType, CodecError> {
    if T::is_none(data, pos)? {
        Ok(ValueType::None)
    } else if T::is_bool(data, pos)? {
        Ok(ValueType::Bool)
    } else if T::is_datetime(data, pos)? {
        // datetime before time — datetime is a superset of time
        Ok(ValueType::DateTime)
    } else if T::is_time(data, pos)? {
        Ok(ValueType::Time)
    } else if T::is_number(data, pos)? {
        Ok(ValueType::Number)
    } else if T::is_string(data, pos)? {
        Ok(ValueType::String)
    } else if T::is_bytes(data, pos)? {
        Ok(ValueType::Bytes)
    } else if T::is_color(data, pos)? {
        Ok(ValueType::Color)
    } else if T::is_angle(data, pos)? {
        Ok(ValueType::Angle)
    } else if T::is_length(data, pos)? {
        Ok(ValueType::Length)
    } else if T::is_literal(data, pos)? {
        Ok(ValueType::Literal)
    } else if T::is_list(data, pos)? {
        Ok(ValueType::Vec)
    } else if T::is_map(data, pos)? {
        Ok(ValueType::Map)
    } else {
        Ok(ValueType::Invalid)
    }
}
/// Skips any whitespace and parses the next element
///
/// # Errors
/// Errors upon invalid/corrupt data
#[track_caller]
pub fn deal_with_data<T: StaticParser>(
    data: &[char],
    pos: &mut usize,
    value_count: &mut usize,
) -> Result<PositionedValue, CodecError> {
    T::skip_whitespace(data, pos, value_count);
    parse_next::<T>(
        data,
        pos,
        figure_out_next_type::<T>(data, *pos)?,
        value_count,
    )
}
#[must_use]
/// If the next value is a comment
pub fn is_comment(data: &[char], pos: usize) -> bool {
    for (start, _end, _) in ALLOWED_COMMENTS {
        let mut this = true;
        let characters: Vec<char> = start.chars().collect();
        //println!();
        for (idx, i) in characters.iter().enumerate() {
            let idx = pos + idx;
            match data.get(idx) {
                None => {
                    this = false;
                    break;
                }
                Some(val) => {
                    // println!(">{} {} ({})", val, i, idx);
                    if *val != *i {
                        this = false;
                        break;
                    }
                }
            }
        }
        if this {
            return true;
        }
    }
    false
}
// /// Removes and returns all comments
// ///
// /// # Errors
// pub fn purge_comments(
//     data: &mut [char],
//     pos: &mut usize,
// ) -> Result< ParsingError> {
//     let data_length = data.len();
//     let mut comments = Vec::new();
//     while *pos < data_length {
//         println!("{} < {}", pos, data_length);
//         comments.push(match purge_comment(data, pos) {
//             Ok(val) => val,
//             Err(err) => {
//                 if err == ParsingError::EmptyFile {
//                     //return Ok(comments);
//                     *pos += 1;
//                     continue;
//                 }
//                 println!("YEAH??? {}", err);
//                 return Err(err);
//             }
//         });
//     }
//     println!("{} < {}", pos, data_length);
//     Ok(comments)
// }
// /// Removes and returns a comment if a comment is present
// ///
// /// # Errors
// pub fn purge_comment(
//     data: &mut [char],
//     pos: &mut usize,
// ) -> Result< ParsingError> {
//     let mut possible = Vec::new();
//     println!("#>> {:?}", ALLOWED_COMMENTS);
//     for (start, end, error_when_end_missing) in ALLOWED_COMMENTS {
//         let mut this = true;
//         let characters: Vec<char> = start.chars().collect();
//         println!(">>> {:?}", characters);
//         for (e_idx, i) in characters.iter().enumerate() {
//             let idx = *pos + e_idx;

//             match data.get(idx) {
//                 None => {
//                     this = false;
//                     break;
//                 }
//                 Some(val) => {
//                     println!(
//                         ">{} {} ({} = pos {} + idx {})",
//                         val, i, idx, pos, e_idx
//                     );
//                     if *val != *i {
//                         this = false;
//                         break;
//                     }
//                 }
//             }
//         }
//         if this {
//             possible.push((start, end, error_when_end_missing));
//         }
//     }
//     possible.sort_by_key(|x| (x.0, x.1));
//     possible.reverse();
//     let Some((start, end, error_when_end_missing)) = possible.first() else {
//         println!("!>EMPTY");
//         return Err(ParsingError::EmptyFile);
//         // return Err(ParsingError::UnexpectedEOF {
//         //     offset: *pos,
//         //     origin: None,
//         //     text: data.iter().collect(),
//         // });
//     };
//     println!("!>FOUND {:?}", possible);
//     //println!("From '{start}' to '{end}'");
//     let temp = end.chars().collect::<Vec<char>>();
//     let end_chars: Vec<(usize, &char)> = temp.iter().enumerate().collect();
//     let starting_pos = *pos;
//     *pos += start.chars().count();

//     let mut comment = String::new();
//     let mut end_exists = false;

//     while let Some(char) = data.get(*pos) {
//         let mut stop = true;
//         for (idx, i) in &end_chars {
//             let Some(new_char) = data.get(*idx + *pos) else {
//                 continue;
//             };
//             let matches = *new_char == **i;
//             if !matches {
//                 stop = false;
//             }
//         }
//         if stop {
//             end_exists = true;
//             break;
//         }
//         println!("### I WAS HERE");
//         comment.push(*char);

//         data[*pos] = ' ';
//         *pos += 1;
//     }
//     if **error_when_end_missing && !end_exists {
//         Err(ParsingError::UnexpectedEOF {
//             offset: *pos,
//             origin: None,
//             text: data.iter().collect(),
//         })?;
//     }
//     let comment = Comment {
//         content:
//         position: PositionRange::new(starting_pos, *pos),
//         surrounding: ((**start).to_string(), (**end).to_string()),
//     };
//     *pos += end_chars.len();
//     Ok(comment)
// }
/// If the given data starts with a given pattern
/// Does 'bool == false' start with 'bool'?
///
/// # Errors
/// Upon an unexpected data stream end
#[allow(clippy::needless_pass_by_value)]
pub fn does_data_start_with_keyword(
    data: &[char],
    pos: &mut usize,
    original: String,
    value_type: ValueType,
) -> Result<PositionRange, CodecError> {
    let start = *pos;
    let word: Vec<char> = original.chars().collect();

    if data.len() >= word.len() {
        let string: String = if ALLOW_NON_LOWERCASE_KEYWORDS {
            data.iter()
                .skip(*pos)
                .take(word.len())
                .collect::<String>()
                .to_lowercase()
        } else {
            data.iter().skip(*pos).take(word.len()).collect()
        };

        if string == original {
            for _ in 0..word.len() {
                *pos += 1;
            }
            Ok(PositionRange::new(start, *pos))
        } else {
            for i in 0..word.len() {
                if data[i] != word[i] {
                    break;
                }
                *pos += 1;
            }
            Err(CodecError::UnexpectedEOF {
                offset: *pos,
                origin: Some(value_type),
                text: data.iter().collect::<String>(),
            })
        }
    } else {
        Err(CodecError::UnexpectedEOF {
            offset: *pos,
            origin: Some(value_type),
            text: data.iter().collect::<String>(),
        })
    }
}
