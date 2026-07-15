use mirl_values::prelude::*;

use crate::{
    PositionRange,
    settings::MapType,
    traits::{AutoImplFormatted, MarshalError, StaticFormattedMarshalBase},
    values::{PositionedValue, PositionedValueInner},
};
/// Opposite of a Parser
///
/// Implement [`DynFormattedMarshalBase`] for this trait to automatically be implemented.
/// You do not need to implement that trait as it is but a helper.
///
/// For values like [`Vec<Value>`], use the [`marshal_to_string`](super::StaticFormattedMarshal::marshal_to_string) of this trait for the sub values
pub trait DynFormattedMarshal {
    /// Converts the value into a string. Set depth to 0 for the formatting to start at ground level
    ///
    /// # Errors
    /// [`MarshalError`]
    fn to_formatted_string(
        &mut self,
        value: &PositionedValue,
        depth: usize,
    ) -> Result<String, MarshalError>;
}

/// Convert individual types to string
///
/// This trait will automatically implemented when [`StaticFormattedMarshalBase`] is implemented
///
/// You don't need to use this trait as you can implement [`DynFormattedMarshal`] yourself. This is just a helper trait,
///
/// When implemented, [`DynFormattedMarshal`] will automatically be implemented
pub trait DynFormattedMarshalBase<W: InnerCodecValue> {
    /// Convert the given string into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn to_formatted_string(
        &mut self,
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
        &mut self,
        input: &Number,
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError>;
    /// Convert the given array into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_formatted_array(
        &mut self,
        input: &[W::Inner],
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError>;
    /// Convert the given bool into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_formatted_bool(
        &mut self,
        input: bool,
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError>;
    /// Get the None equivalent of the current formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_formatted_none(
        &mut self,
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError>;
    /// Convert the given map into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_formatted_map(
        &mut self,
        input: &MapType<W::Inner, W::Inner>,
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError>;
}

impl<T: StaticFormattedMarshalBase<W>, W: InnerCodecValue> DynFormattedMarshalBase<W> for T {
    fn to_formatted_string(
        &mut self,
        input: &str,
        quotation: &Option<String>,
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError> {
        T::marshal_formatted_string(input, quotation, depth, position)
    }

    fn marshal_formatted_number(
        &mut self,
        input: &Number,
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError> {
        T::marshal_formatted_number(input, depth, position)
    }

    fn marshal_formatted_array(
        &mut self,
        input: &[W::Inner],
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError> {
        T::marshal_formatted_array(input, depth, position)
    }

    fn marshal_formatted_bool(
        &mut self,
        input: bool,
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError> {
        T::marshal_formatted_bool(input, depth, position)
    }

    fn marshal_formatted_none(
        &mut self,
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError> {
        T::marshal_formatted_none(depth, position)
    }

    fn marshal_formatted_map(
        &mut self,
        input: &MapType<W::Inner, W::Inner>,
        depth: usize,
        position: PositionRange,
    ) -> Result<String, MarshalError> {
        T::marshal_formatted_map(input, depth, position)
    }
}

impl<T: DynFormattedMarshalBase<PositionedValueInner> + AutoImplFormatted> DynFormattedMarshal
    for T
{
    fn to_formatted_string(
        &mut self,
        value: &PositionedValue,
        depth: usize,
    ) -> Result<String, MarshalError> {
        match &value.value {
            Value::Simple(simple) => match &simple {
                SimpleValue::String(input) => self.to_formatted_string(
                    &input.escape_debug().to_string(),
                    &value.container,
                    depth,
                    value.get_position(),
                ),
                SimpleValue::Bool(input) => {
                    self.marshal_formatted_bool(*input, depth, value.get_position())
                }
                SimpleValue::None => self.marshal_formatted_none(depth, value.get_position()),
                SimpleValue::Number(input) => {
                    self.marshal_formatted_number(input, depth, value.get_position())
                }
                _ => Err(MarshalError::UnsupportedType {
                    value_type: value.get_value_type(),
                    id: value.item_id,
                }),
            },
            Value::Container(container) => match &container {
                ContainerValue::Vec(input) => {
                    self.marshal_formatted_array(input, depth, value.get_position())
                }
                ContainerValue::Map(input) => {
                    self.marshal_formatted_map(input, depth, value.get_position())
                }
            },
        }
    }
}
