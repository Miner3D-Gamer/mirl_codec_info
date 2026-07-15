use mirl_values::prelude::*;

use crate::{
    settings::MapType,
    traits::MarshalError,
    values::{PositionedValue, PositionedValueInner},
};

/// Opposite of a Parser
///
/// Implement [`MarshalBase`] for this trait to automatically be implemented.
///
/// For values like [`Vec<Value>`], use the [`marshal_to_string`](StaticCompactMarshal::marshal_to_string) of this trait for the sub values
pub trait StaticCompactMarshal {
    /// Converts the value into a string. Set depth to 0 for the formatting to start at ground level
    ///
    /// # Errors
    /// [`MarshalError`]
    fn to_compact_string(value: &PositionedValue, depth: usize) -> Result<String, MarshalError>;
}
/// Convert individual types to string
///
/// You don't need to use this trait as you can implement [`StaticCompactMarshal`] yourself. This is just a helper trait,
///
/// When implemented, [`StaticCompactMarshal`], [`DynCompactMarshalBase`](super::DynCompactMarshalBase), and [`DynCompactMarshal`](super::DynCompactMarshal) will automatically be implemented
pub trait StaticCompactMarshalBase<W: InnerCodecValue> {
    /// Convert the given string into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_compact_string(input: &str, depth: usize) -> Result<String, MarshalError>;
    /// Convert the given number into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_compact_number(input: &Number, depth: usize) -> Result<String, MarshalError>;
    /// Convert the given array into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_compact_array(input: &[W::Inner], depth: usize) -> Result<String, MarshalError>;
    /// Convert the given bool into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_compact_bool(input: bool, depth: usize) -> Result<String, MarshalError>;
    /// Get the None equivalent of the current formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_compact_none(depth: usize) -> Result<String, MarshalError>;
    /// Convert the given map into a string of the correct formatting
    ///
    /// # Errors
    /// [`MarshalError`]
    fn marshal_compact_map(
        input: &MapType<W::Inner, W::Inner>,
        depth: usize,
    ) -> Result<String, MarshalError>;
}

impl<T: StaticCompactMarshalBase<PositionedValueInner>> StaticCompactMarshal for T {
    fn to_compact_string(value: &PositionedValue, depth: usize) -> Result<String, MarshalError> {
        match &value.value {
            Value::Simple(simple) => match simple {
                SimpleValue::String(input) => {
                    T::marshal_compact_string(&input.escape_debug().to_string(), depth)
                }
                SimpleValue::Bool(input) => T::marshal_compact_bool(*input, depth),
                SimpleValue::None => T::marshal_compact_none(depth),
                SimpleValue::Number(input) => T::marshal_compact_number(input, depth),
                _ => Err(MarshalError::UnsupportedType {
                    value_type: value.get_value_type(),
                    id: value.item_id,
                }),
            },
            Value::Container(container) => match container {
                ContainerValue::Vec(input) => T::marshal_compact_array(input, depth),
                ContainerValue::Map(input) => T::marshal_compact_map(input, depth),
            },
        }
    }
}
