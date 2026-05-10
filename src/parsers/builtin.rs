use crate::{
    error::ParsingError,
    traits::{DynInfo, DynParser},
    values::PositionedValue,
};
/// Dyn parser with dyn info
pub trait DynParserWithSelfInfo: DynParser + DynInfo {}
impl<T: DynParser + DynInfo> DynParserWithSelfInfo for T {}

#[allow(clippy::wrong_self_convention)]
// Fuck you clippy, if you wanna complain then suggest what naming convention would be correct
/// Parse a piece of text using a list of parsers
pub trait ListCodecParsing {
    /// Iterate through all parsers and parse with the one that doesn't error, otherwise return None
    fn from_str(
        &mut self,
        data: &str,
    ) -> Option<Result<Option<PositionedValue>, ParsingError>>;
    /// Parse the text with the parser that supports the given extension, return None is none support the extension
    fn from_str_with_extension(
        &mut self,
        data: &str,
        extension: &str,
    ) -> Option<Result<Option<PositionedValue>, ParsingError>>;
    /// Parse the text with the parser that has the given name, return None if none have the name
    fn from_str_with_name(
        &mut self,
        data: &str,
        name: &str,
    ) -> Option<Result<Option<PositionedValue>, ParsingError>>;
}
impl ListCodecParsing for [&mut dyn DynParserWithSelfInfo] {
    fn from_str(
        &mut self,
        data: &str,
    ) -> Option<Result<Option<PositionedValue>, ParsingError>> {
        for parser in self.iter_mut() {
            let parser_mut: &mut dyn DynParserWithSelfInfo = *parser;

            if let Ok(val) = parser_mut.from_str(data) {
                return Some(Ok(val));
            }
        }
        None
    }

    fn from_str_with_extension(
        &mut self,
        data: &str,
        extension: &str,
    ) -> Option<Result<Option<PositionedValue>, ParsingError>> {
        for parser in self.iter_mut() {
            let parser_mut: &mut dyn DynParserWithSelfInfo = *parser;
            if parser_mut
                .get_supported_extension()
                .iter()
                .map(std::string::String::as_str)
                .any(|x| x.eq(extension))
            {
                return Some(parser_mut.from_str(data));
            }
        }
        None
    }

    fn from_str_with_name(
        &mut self,
        data: &str,
        name: &str,
    ) -> Option<Result<Option<PositionedValue>, ParsingError>> {
        for parser in self.iter_mut() {
            let parser_mut: &mut dyn DynParserWithSelfInfo = *parser;
            if parser_mut.get_name().as_str() == name {
                return Some(parser_mut.from_str(data));
            }
        }
        None
    }
}

// #[cfg_attr(feature = "strum", derive(strum::EnumIter))]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// /// What parsers are provided by default
// pub enum DefaultBuiltinCodecs {
//     /// JSON
//     DefaultJson,
// }

// impl DefaultBuiltinCodecs {
//     #[allow(clippy::missing_errors_doc)]
//     /// Parse with the current parser
//     pub fn from_str(
//         &self,
//         data: &str,
//     ) -> Result<Option<PositionedValue>, ParsingError> {
//         match self {
//             Self::DefaultJson => crate::from_str::<DefaultJson>(data),
//         }
//     }
//     #[allow(clippy::missing_errors_doc)]
//     /// Parse with the current parser
//     pub fn to_compact_string(
//         &self,
//         value: &PositionedValue,
//     ) -> Result<String, MarshalError> {
//         match self {
//             Self::DefaultJson => DefaultJson::to_compact_string(value, 0),
//         }
//     }
//     #[must_use]
//     /// Get the name of the current parser
//     pub fn get_name(&self) -> &'static str {
//         match self {
//             Self::DefaultJson => DefaultJson::get_name(),
//         }
//     }
// }
