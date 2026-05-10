use mirl_values::values::ValueType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// What kind of sub error happened
pub enum ParserMishaps {
    /// For the number parse
    Number(ParserNumberMishap),
    /// For the string parser
    String(ParserStringMishap),
    /// For the array parser
    Array(ParserArrayMishap),
    /// For the map parser
    Map(ParserMapMishap),
    /// For the bool parser
    Bool(ParserBoolMishap),
    /// For the none parser
    None(ParserNoneMishap),
}

impl std::fmt::Display for ParserMishaps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(val) => write!(f, "{val}"),
            Self::String(val) => write!(f, "{val}"),
            Self::Array(val) => write!(f, "{val}"),
            Self::Map(val) => write!(f, "{val}"),
            Self::Bool(val) => write!(f, "{val}"),
            Self::None(val) => write!(f, "{val}"),
        }
    }
}

impl ParserMishaps {
    #[must_use]
    /// Get the value type of the current mishap
    pub const fn get_value_type(&self) -> ValueType {
        match self {
            Self::Number(_) => ValueType::Number,
            Self::String(_) => ValueType::String,
            Self::Array(_) => ValueType::Vec,
            Self::Map(_) => ValueType::Map,
            Self::None(_) => ValueType::None,
            Self::Bool(_) => ValueType::Bool,
        }
    }
}

mod array;
pub use array::*;

mod map;
pub use map::*;

mod string;
pub use string::*;

mod number;
pub use number::*;

mod none;
pub use none::*;

mod boolean;
pub use boolean::*;
