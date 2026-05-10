mod compact_dyn;
mod compact_static;
mod formatted_dyn;
mod formatted_static;
pub use compact_dyn::*;
pub use compact_static::*;
pub use formatted_dyn::*;
pub use formatted_static::*;

/// If [`DynFormattedMarshal`] and [`StaticFormattedMarshal`] should automatically implemented for you struct if available
pub trait AutoImplFormatted {}
