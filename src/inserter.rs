use mirl_std_exposed::str::multi_replace_non_overlapping;
use mirl_extensions::{InnerCodecValue, IntoPatch};
use mirl_values::values::Value;

use crate::{
    traits::{MarshalError, StaticCompactMarshal},
    values::PositionedValue,
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Insert values into text
pub struct Inserter<W: InnerCodecValue> {
    /// The Values
    pub values: std::collections::HashMap<String, Value<W>>,
    /// The file to insert the values
    pub file: String,
}
impl<W: InnerCodecValue> Inserter<W> {
    /// Set a value
    pub fn set_value<K: Into<String>>(&mut self, key: K, value: Value<W>) {
        self.values.insert(key.into(), value);
    }
    /// Inserts all values into the inside file
    ///
    /// # Errors
    /// When a value type is not supported for the given format
    ///
    /// Safety:
    /// We give all keys a prefix and suffix.
    /// It is possible for this to UB when a key contains another key and includes both the pre- and suffix
    pub fn insert_into_chat_compact<T: StaticCompactMarshal>(
        &self,
    ) -> Result<String, MarshalError>
    where
        <W as mirl_extensions::InnerCodecValue>::Inner: IntoPatch<Value<W>>,
    {
        let mut stuff = Vec::with_capacity(self.values.len());
        for (key, val) in &self.values {
            stuff.push((
                format!("{{${key}}}"),
                T::to_compact_string(
                    &PositionedValue::positional_from_value(val.clone()),
                    0,
                )?,
            ));
        }
        let output =
            unsafe { multi_replace_non_overlapping(&self.file, &stuff) };
        Ok(output)
    }
}

// pub fn is_value_allowed_for_codec<
//     T: StaticCompactMarshal,
//     W: mirl_extensions::InnerCodecValue,
// >(
//     value: Value<W>,
// ) -> bool
// where
//     <W as mirl_extensions::InnerCodecValue>::Inner: IntoPatch<Value<W>>,
// {
//     T::to_compact_string(&PositionedValue::positional_from_value(val), 0)
//         .is_ok()
// }
