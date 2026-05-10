/// What character is used for separating list elements
pub const ELEMENT_SEPARATOR: char = ',';
/// What character is used for starting a list
pub const LIST_START: char = '[';
/// What character is used for ending a list
pub const LIST_END: char = ']';
/// What character is used for starting a map
pub const MAP_START: char = '{';
/// What character is used for ending a map
pub const MAP_END: char = '}';
/// The character between the key and
pub const MAP_POINTER: char = ':';
/// What character signifies that the next character should be escaped
pub const ESCAPE_CHARACTER: char = '\\';
/// Characters to be skipped without repercussions
pub const WHITESPACE_CHARACTERS: &[char] = &[' ', '\t', '\r', '\n', '\u{feff}'];
/// What denotes the start and end of a string
pub const STRING_INDICATOR: &[char] = &['"', '\''];
/// What character make sense to escape
pub const ALLOWED_ESCAPED: &[char] =
    &['\\', '"', 'u', 'n', 't', 'r', '/', 'b', 'f'];
/// The keyword used for None
pub const NONE_KEYWORD: &str = "null";
/// The keyword used for true
pub const TRUE_KEYWORD: &str = "true";
/// The keyword used for false
pub const FALSE_KEYWORD: &str = "false";
/// What control characters are allowed to parsed without an escape character
pub const CONTROL_CHARACTERS_ALLOWED_TO_BE_UNESCAPED: &[char] = &[''];

/// Should True, False, and None, also be accepted when true, false, and none are the correct keywords?
pub const ALLOW_NON_LOWERCASE_KEYWORDS: bool = false;
/// If it should error when more file is available than expected
pub const ERROR_ON_EXPECTED_EOF: bool = true;
/// If for example [1, 2, 3,] should error bc it ends with a comma
pub const ALLOW_TRAILING_ELEMENT_SEPARATOR: bool = false;
/// Will not error upon '.5' but instead replace it with '0.5' or '1.' to '1.0'
pub const MISSING_INTEGER_AUTOMATICALLY_PLACED: bool = false;
#[allow(clippy::doc_link_with_quotes)]
/// If {["some", "data"]: 100} or {100: "hi"} is also allowed
pub const ALLOW_NON_STRING_KEYS: bool = false;
/// If 00001 should ve allowed when it could be simplified to just 1
pub const ALLOW_UNNECESSARY_ZEROS: bool = false;
/// If the null character \u0000 is allowed
pub const ALLOW_NULL_CHARACTER: bool = false;
/// If all control characters like \u0001 must be escaped
pub const CONTROL_CHARACTER_MUST_BE_ESCAPED: bool = true;
/// All number chars
pub const NUMBER_CHARS: &[char] =
    &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// What characters are allowed to end an element like a number
pub const ALLOWED_ELEMENT_INTERRUPTIONS: &[char] =
    &[LIST_END, MAP_END, ELEMENT_SEPARATOR, ' '];

/// If [100 "hi"] should be fixed to [100, "hi"] and {"first" "second" "third" "fourth"} to {"first": "second", "third": "fourth"}
pub const AUTOMATIC_SEPARATOR_INSERTION: bool = false;

/// If `100_000_000` should be read as `100000000`
pub const NUMBER_ALLOW_UNDERSCORE: bool = false;

/// What comment types exist and are allowed
pub const ALLOWED_COMMENTS: &[(&str, &str, bool)] = &[
    ("//", "\n", false),
    ("/*", "*/", true),
    ("#", "\n", false),
    ("###", "###", true),
];

// /// If comments are allowed
// pub const ALLOW_COMMENTS: bool = true;

pub use mirl_values::settings::*;
