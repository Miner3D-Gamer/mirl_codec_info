/// All functions used in the json parser
pub mod json;
pub use json::DefaultJson;

/// All functions used in the css parser
pub mod css;

mod builtin;
pub use builtin::*;

/// Functions to be reused
pub mod helper;

// /// All functios related to parsing/marshaling a gen ex format
// pub mod gen_ex;
// pub use gen_ex::GenEx;
