use mirl_values::prelude::*;

use crate::{
    PositionRange,
    settings::MapType,
    traits::{AutoImplFormatted, MarshalError},
    values::{PositionedValue, PositionedValueInner},
};

/// Opposite of a Parser
///
/// Implement [`MarshalBase`] for this trait to automatically be implemented.
///
/// For values like [`Vec<Value>`], use the [`marshal_to_string`](super::StaticFormattedMarshal::marshal_to_string) of this trait for the sub values
pub trait StaticFormattedMarshal {
    /// Converts the value into a string. Set depth to 0 for the formatting to start at ground level
    ///
    /// # Errors
    /// [`MarshalError`]
    fn to_formatted_string(value: &PositionedValue, depth: usize) -> Result<String, MarshalError>;
}
/// Convert individual types to string
///
/// You don't need to use this trait as you can implement [`StaticFormattedMarshal`] yourself. This is just a helper trait,
///
/// When implemented, [`StaticFormattedMarshal`], [`DynFormattedMarshalBase`](super::DynFormattedMarshalBase), and [`DynFormattedMarshal`](super::DynFormattedMarshal) will automatically be implemented
pub trait StaticFormattedMarshalBase<W: InnerCodecValue> {
    /// Convert the given string into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_formatted_string(
        input: &str,
        quotation: &Option<String>,
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError>;
    /// Convert the given number into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_formatted_number(
        input: &Number,
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError>;
    /// Convert the given array into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_formatted_array(
        input: &[W::Inner],
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError>;
    /// Convert the given bool into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_formatted_bool(
        input: bool,
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError>;
    /// Get the None equivalent of the current formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_formatted_none(
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError>;
    /// Convert the given map into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_formatted_map(
        input: &MapType<W::Inner, W::Inner>,
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError>;
}

impl<T: StaticFormattedMarshalBase<PositionedValueInner> + AutoImplFormatted> StaticFormattedMarshal
    for T
{
    fn to_formatted_string(value: &PositionedValue, depth: usize) -> Result<String, MarshalError> {
        let depth = depth + 1;
        match &value.value {
            Value::Simple(simple) => match simple {
                SimpleValue::String(input) => T::marshal_formatted_string(
                    &input.escape_debug().to_string(),
                    &value.container,
                    depth,
                    value.get_position(),
                ),
                SimpleValue::Bool(input) => {
                    T::marshal_formatted_bool(*input, depth, value.get_position())
                }
                SimpleValue::None => T::marshal_formatted_none(depth, value.get_position()),
                SimpleValue::Number(input) => {
                    T::marshal_formatted_number(input, depth, value.get_position())
                }
                _ => Err(MarshalError::UnsupportedType {
                    value_type: value.get_value_type(),
                    id: value.item_id,
                }),
            },
            Value::Container(container) => match container {
                ContainerValue::Vec(input) => {
                    T::marshal_formatted_array(input, depth, value.get_position())
                }
                ContainerValue::Map(input) => {
                    T::marshal_formatted_map(input, depth, value.get_position())
                }
            },
        }
    }
}
