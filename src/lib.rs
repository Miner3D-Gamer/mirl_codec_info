#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::expect_used)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::format_push_string)]
#![allow(clippy::result_large_err)]
#![feature(default_field_values)]
#![feature(const_trait_impl)]
//! ### Mici, a library for parsing text formats
//! >Mirl Codec Info (Mici)
//!
//! This library records the source locations of all parsed elements.
//!
//! Entry points: `from_str::<{static_parser}>`, `{static_parser}_from_str`, or `{dyn_parser}.from_str`
//!
//! # Supported formats
//!
//! **JSON**
//! - Passes all but 2* tests from [JSONTestSuite](https://github.com/nst/JSONTestSuite) (Failed `n_structure_100000_opening_arrays.json` and `n_structure_open_array_object.json` as this lib uses a recursive structure and stack overflows. 2 tests have also been removed because of their unusual file encoding, their contents would have been parsed perfectly fine)
//!
//! **CSS**
//! - No "@" support and no "[*]" support. Otherwise passes the 300 line testing css.
//!
//! ## Disclaimer:
//! This lib fast and has no known errors but its not a projected with years of optimization nor millions of testing users.
//! If you encounter an issue or want to contribute, open a GitHub issue.
//!
//! ### TODO:
//! - Add support for comments
//! - Add more parsers (yalm)
//! - Add more marshals
//! - Clean up other TODO

/// The Parsers, Marshals, and codecs available
pub mod available;
/// Possible errors
pub mod error;
/// Reuseable settings
pub mod settings;
/// The possible values a thing can be
pub mod values;

#[cfg(test)]
/// Tests
pub mod test;
// #[cfg(not(feature = "strum"))]
// mod test {
//     #[test]
//     fn caller() {
//         panic!(">Use the 'strum' flag for testing")
//     }
// }

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Where something is located
pub struct PositionRange {
    /// Initial position
    pub offset: usize,
    /// Ending: offset + width
    pub width: usize,
}

impl Default for PositionRange {
    fn default() -> Self {
        Self {
            offset: usize::MAX,
            width: usize::MAX,
        }
    }
}

impl PositionRange {
    /// Create a new range based on start and end points
    #[must_use]
    pub const fn new(offset: usize, end: usize) -> Self {
        Self {
            offset,
            width: end - offset,
        }
    }
}

/// Everything anyone would need to import to use this lib
pub mod prelude;

use crate::{
    error::CodecError,
    settings::*,
    traits::{MarshalError, StaticCompactMarshal, StaticFormattedMarshal, StaticParser},
    values::PositionedValue,
};
/// Parsers of supported languages
pub mod parsers;
/// The Parsing and Marshalling traits
pub mod traits;

/// Insert values into text
pub mod inserter;
/// Parse json from the given string
///
/// # Errors
/// Errors upon invalid/corrupt data
pub fn json_from_str(data: &str) -> Result<Option<PositionedValue>, CodecError> {
    from_str::<parsers::DefaultJson>(data)
}

/// Parse json text into [Value]
///
/// Only returns Ok(None) when the file contains no value (empty file)
///
/// # Errors
/// Errors upon invalid/corrupt data
pub fn from_str<T: StaticParser>(data: &str) -> Result<Option<PositionedValue>, CodecError> {
    let chars: Vec<char> = data.chars().collect();
    let mut pos = 0;
    let mut value_count = 0;
    // let mut comment_pos = 0;

    // let comments: Vec<Comment> = if ALLOW_COMMENTS {
    //     T::purge_comments(&mut chars, &mut comment_pos)?
    // } else {
    //     Vec::new()
    // };

    // println!("After purge: {}", chars.iter().collect::<String>());

    let val = match parsers::helper::deal_with_data::<T>(&chars, &mut pos, &mut value_count) {
        Ok(val) => val,
        Err(err) => {
            if err == CodecError::EmptyFile {
                return Ok(None);
            }
            Err(err)?
        }
    };
    if ERROR_ON_EXPECTED_EOF {
        let temp = chars.clone();
        let temp_pos = pos;
        T::skip_whitespace(&chars, &mut pos, &mut value_count);
        if chars.len() != pos {
            return Err(CodecError::ExpectedEOF {
                offset: temp_pos,
                origin: None,
                text: temp.iter().collect(),
            });
        }
    }
    Ok(Some(val))
}

/// Turns the given value into a string representation
///
/// # Errors
/// [`MarshalError`]
pub fn to_compact_string<T: StaticCompactMarshal>(
    value: &PositionedValue,
) -> Result<String, MarshalError> {
    T::to_compact_string(value, 0)
}

/// Turns the given value into a string representation
///
/// # Errors
/// [`MarshalError`]
pub fn to_formatted_string<T: StaticFormattedMarshal>(
    value: &PositionedValue,
) -> Result<String, MarshalError> {
    T::to_formatted_string(value, 0)
}
