use mirl_values::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Things that could go wrong during marshaling
pub enum MarshalError {
    /// The given type is not supported by the selected language
    UnsupportedType {
        /// The value type that is unsupported
        value_type: ValueType,
        /// The id of the unsupported type
        id: usize,
    },
    /// The given type is supported but not where it is located
    InvalidStructure {
        /// The value type
        value_type: ValueType,
        /// The id
        id: usize,
    },
}

use crate::{
    settings::MapType,
    traits::StaticCompactMarshalBase,
    values::{PositionedValue, PositionedValueInner},
};
/// Opposite of a Parser
///
/// Implement [`DynCompactMarshalBase`] for this trait to automatically be implemented.
/// You do not need to implement that trait as it is but a helper.
///
/// For values like [`Vec<Value>`], use the [`marshal_to_string`](super::StaticMarshal::marshal_to_string) of this trait for the sub values
pub trait DynCompactMarshal<W: InnerCodecValue> {
    /// Converts the value into a string. Set depth to 0 for the formatting to start at ground level
    ///
    /// # Errors
    /// [`MarshalError`]
    fn to_compact_string(
        &mut self,
        value: &PositionedValue,
        depth: usize,
    ) -> Result<String, MarshalError>;
}

/// Convert individual types to string
///
/// This trait will automatically implemented when [`StaticCompactMarshalBase`] is implemented
///
/// You don't need to use this trait as you can implement [`DynCompactMarshal`] yourself. This is just a helper trait,
///
/// When implemented, [`DynCompactMarshal`] will automatically be implemented
pub trait DynCompactMarshalBase<W: InnerCodecValue> {
    /// Convert the given string into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_compact_string(
        &mut self,
        input: &str,
        depth: usize,
    ) -> Result<String, MarshalError>;
    /// Convert the given number into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_compact_number(
        &mut self,
        input: &Number,
        depth: usize,
    ) -> Result<String, MarshalError>;
    /// Convert the given array into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_compact_vec(
        &mut self,
        input: &[W::Inner],
        depth: usize,
    ) -> Result<String, MarshalError>;
    /// Convert the given bool into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_compact_bool(
        &mut self,
        input: bool,
        depth: usize,
    ) -> Result<String, MarshalError>;
    /// Get the None equivalent of the current formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_compact_none(
        &mut self,
        depth: usize,
    ) -> Result<String, MarshalError>;
    /// Convert the given map into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_compact_map(
        &mut self,
        input: &MapType<W::Inner, W::Inner>,
        depth: usize,
    ) -> Result<String, MarshalError>;
}

impl<T: StaticCompactMarshalBase<W>, W: InnerCodecValue>
    DynCompactMarshalBase<W> for T
{
    fn marshal_compact_string(
        &mut self,
        input: &str,
        depth: usize,
    ) -> Result<String, MarshalError> {
        T::marshal_compact_string(input, depth)
    }

    fn marshal_compact_number(
        &mut self,
        input: &Number,
        depth: usize,
    ) -> Result<String, MarshalError> {
        T::marshal_compact_number(input, depth)
    }

    fn marshal_compact_vec(
        &mut self,
        input: &[W::Inner],
        depth: usize,
    ) -> Result<String, MarshalError> {
        T::marshal_compact_array(input, depth)
    }

    fn marshal_compact_bool(
        &mut self,
        input: bool,
        depth: usize,
    ) -> Result<String, MarshalError> {
        T::marshal_compact_bool(input, depth)
    }

    fn marshal_compact_none(
        &mut self,
        depth: usize,
    ) -> Result<String, MarshalError> {
        T::marshal_compact_none(depth)
    }

    fn marshal_compact_map(
        &mut self,
        input: &MapType<W::Inner, W::Inner>,
        depth: usize,
    ) -> Result<String, MarshalError> {
        T::marshal_compact_map(input, depth)
    }
}

impl<T: DynCompactMarshalBase<PositionedValueInner>>
    DynCompactMarshal<PositionedValueInner> for T
{
    fn to_compact_string(
        &mut self,
        value: &PositionedValue,
        depth: usize,
    ) -> Result<String, MarshalError> {
        match &value.value {
            Value::Simple(simple) => match &simple {
                SimpleValue::String(input) => self.marshal_compact_string(
                    &input.escape_debug().to_string(),
                    depth,
                ),
                SimpleValue::Bool(input) => {
                    self.marshal_compact_bool(*input, depth)
                }
                SimpleValue::None => self.marshal_compact_none(depth),
                SimpleValue::Number(input) => {
                    self.marshal_compact_number(input, depth)
                }
                _ => Err(MarshalError::UnsupportedType {
                    value_type: value.get_value_type(),
                    id: value.item_id,
                }),
            },
            Value::Container(container) => match &container {
                ContainerValue::Map(input) => {
                    self.marshal_compact_map(input, depth)
                }
                ContainerValue::Vec(input) => {
                    self.marshal_compact_vec(input, depth)
                }
            },
        }
    }
}
