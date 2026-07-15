mod parser;
use mirl_values::values::value::InnerCodecValue;
pub use parser::*;
mod info;
pub use info::*;
mod marshals;
pub use marshals::*;
/// When a struct can Parse and Marshal it is considered a codec
///
/// This is a static implementation for better rust optimizations
///
/// When Marshaling, will try to produce the smallest output possible
pub trait StaticCompactCodec<W: InnerCodecValue>:
    StaticParser + StaticCompactMarshal + StaticInfo
{
}

impl<T: StaticParser + StaticCompactMarshal + StaticInfo, W: InnerCodecValue> StaticCompactCodec<W>
    for T
{
}

/// When a struct can Parse and Marshal it is considered a codec
///
/// This is a static implementation for dynamic parsing/marshaling
///
/// When Marshaling, will try to produce the smallest output possible
pub trait DynCompactCodec<W: InnerCodecValue>: DynParser + DynCompactMarshal<W> + DynInfo {}

impl<T: DynParser + DynCompactMarshal<W> + DynInfo, W: InnerCodecValue> DynCompactCodec<W> for T {}

/// When a struct can Parse and Marshal it is considered a codec
///
/// This is a static implementation for better rust optimizations
///
/// When Marshaling, will try to produce a formatted (often human readable) output
pub trait StaticFormattedCodec<W: InnerCodecValue>:
    StaticParser + StaticCompactMarshal + StaticInfo
{
}

impl<T: StaticParser + StaticCompactMarshal + StaticInfo, W: InnerCodecValue>
    StaticFormattedCodec<W> for T
{
}

/// When a struct can Parse and Marshal it is considered a codec
///
/// This is a static implementation for dynamic parsing/marshaling
///
/// When Marshaling, will try to produce a formatted (often human readable) output
pub trait DynFormattedCodec<W: InnerCodecValue>: DynParser + DynFormattedMarshal + DynInfo {}

impl<T: DynParser + DynFormattedMarshal + DynInfo, W: InnerCodecValue> DynFormattedCodec<W> for T {}

// mod compatibility;
// pub use compatibility::*;
