use std::hint::unreachable_unchecked;

// use mirl::misc::Map;
use mirl_extensions::*;
use mirl_values::{
    settings::MapType,
    values::{ContainerValue, Value, ValueType, value::InnerCodecValue},
};

use crate::PositionRange;

// /// Represents a _thing_ from a single String to a List of Maps
// #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
// pub enum PureValue {
//     /// Null, None, Nothing. The third variant to bool
//     None,
//     /// True or False
//     Bool(bool),
//     /// A number, still in String form, to be converted to the requested output
//     Number(String),
//     /// A String
//     String(String),
//     /// List
//     Array(Vec<Self>),
//     /// Key: Value
//     Map(MapType<Self, Self>),
// }
// #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
// /// An optional description that doesn't influence any value
// pub struct Comment {
//     /// What the comment says
//     pub content: String,
//     /// Where the comment is located
//     pub position: PositionRange,
//     /// What the comment was wrapped in
//     pub surrounding: (String, String),
// }
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// /// The t
// pub struct StringValue {
//     value: String,
//     quotation: String,
// }
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A value at a specific location
pub struct PositionedValue {
    /// The value
    pub value: Value<PositionedValueInner>,
    /// From where to where
    pub position: PositionRange,
    /// The first item to be parsed has id 1, the second id 2, and so on
    pub item_id: usize,
    /// What encapsulated the value (For string its the quotations, for lists it's the brackets when the format language supports both tuples and lists)
    pub container: Option<String> = None,
}
#[allow(default_overrides_default_fields)]
impl Default for PositionedValue {
    /// Provides a positional struct in an invalid state
    fn default() -> Self {
        Self {
            value: Value::Simple(mirl_values::prelude::SimpleValue::None),
            position: PositionRange::new(usize::MAX, usize::MAX),
            item_id: usize::MAX,
            container: None,
        }
    }
}
impl PositionedValue {
    #[must_use]
    /// Convert any value into a [`PositionedValue`]
    /// TODO: Put this into the `From` and `FromPatch` traits
    pub fn positional_from_value<W: InnerCodecValue>(value: Value<W>) -> Self
    where
        <W as mirl_extensions::InnerCodecValue>::Inner: IntoPatch<Value<W>>,
    {
        let mut pos = Self::default();
        match value {
            Value::Simple(simple) => pos.value = Value::Simple(simple),
            Value::Container(container) => match container {
                ContainerValue::Map(map) => {
                    let mut new = MapType::new();
                    for (key, val) in map {
                        new.insert(
                            Self::positional_from_value(key.into_value()),
                            Self::positional_from_value(val.into_value()),
                        );
                    }
                    pos.value = Value::Container(ContainerValue::Map(new));
                }
                ContainerValue::Vec(vec) => {
                    let mut new = Vec::with_capacity(vec.len());
                    for i in vec {
                        new.push(Self::positional_from_value(i.into_value()));
                    }

                    pos.value = Value::Container(ContainerValue::Vec(new));
                }
            },
        }
        pos
    }
}
impl PositionedValue {
    #[must_use]
    /// Get the value type of the internal value
    pub fn get_value_type(&self) -> ValueType {
        self.value.get_value_type()
    }
}
impl CodecContainerSubValueRef for PositionedValue {
    type InnerValue = PositionedValueInner;
    fn as_container(&self) -> Option<&ContainerValue<Self>> {
        self.value.as_container()
    }
}
impl CodecSimpleSubValueRef for PositionedValue {
    fn as_simple(&self) -> Option<&mirl_values::prelude::SimpleValue> {
        self.value.as_simple()
    }
}
impl CodecContainerSubValueInto for PositionedValue {
    type InnerValue = PositionedValueInner;
    fn into_container(
        self,
    ) -> Option<ContainerValue<<Self::InnerValue as InnerCodecValue>::Inner>> {
        self.value.into_container()
    }
}
impl CodecSimpleSubValueInto for PositionedValue {
    fn into_simple(self) -> Option<mirl_values::prelude::SimpleValue> {
        self.value.into_simple()
    }
}
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PositionedValueInner {}
impl InnerCodecValue for PositionedValueInner {
    type Inner = PositionedValue;
}

// /// Represents a _thing_ from a single String to a List of Maps
// #[derive(Debug, Clone)]
// pub enum Value {
//     /// Null, None, Nothing. The third variant to bool
//     None(PositionRange, usize),
//     /// True or False
//     Bool(bool, PositionRange, usize),
//     /// A number, still in String form, to be converted to the requested output
//     Number(String, PositionRange, usize),
//     /// A String
//     String(String, PositionRange, String, usize),
//     /// List
//     Vec(Vec<Self>, PositionRange, usize),
//     /// Key: Value
//     Map(MapType<Self, Self>, PositionRange, usize),
// }
// use std::{
//     cmp::Ordering,
//     hash::{Hash, Hasher},
// };

// impl PartialEq for Value {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Self::None(_, _), Self::None(_, _)) => true,
//             (Self::Bool(a, _, _), Self::Bool(b, _, _)) => a == b,
//             (Self::Number(a, _, _), Self::Number(b, _, _))
//             | (Self::String(a, _, _, _), Self::String(b, _, _, _)) => a == b,
//             (Self::Vec(a, _, _), Self::Vec(b, _, _)) => a == b,
//             (Self::Map(a, _, _), Self::Map(b, _, _)) => a == b,
//             _ => false,
//         }
//     }
// }

// impl Eq for Value {}

// impl Hash for Value {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         std::mem::discriminant(self).hash(state);
//         match self {
//             Self::None(_, _) => {}
//             Self::Bool(v, _, _) => v.hash(state),
//             Self::Number(v, _, _) => v.hash(state),
//             Self::String(a, _, b, _) => {
//                 a.hash(state);
//                 b.hash(state);
//             }
//             Self::Vec(v, _, _) => v.hash(state),
//             Self::Map(v, _, _) => v.hash(state),
//         }
//     }
// }

// impl PartialOrd for Value {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for Value {
//     fn cmp(&self, other: &Self) -> Ordering {
//         let d1 = std::mem::discriminant(self);
//         let d2 = std::mem::discriminant(other);

//         if d1 != d2 {
//             return self.cmp(other);
//         }

//         match (self, other) {
//             (Self::Bool(a, _, _), Self::Bool(b, _, _)) => a.cmp(b),
//             (Self::Number(a, _, _), Self::Number(b, _, _)) => a.cmp(b),
//             (Self::String(a, _, a2, _), Self::String(b, _, b2, _)) => {
//                 (a, a2).cmp((b, b2))
//             }
//             (Self::Vec(a, _, _), Self::Vec(b, _, _)) => a.cmp(b),
//             (Self::Map(a, _, _), Self::Map(b, _, _)) => a.cmp(b),
//             _ => Ordering::Equal,
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
// /// Empty value types that do not hold any data
// pub enum ValueType {
//     /// Refer to [`Value::None`]
//     None,
//     /// Refer to [`Value::Bool`]
//     Bool,
//     /// Refer to [`Value::Number`]
//     Number,
//     /// Refer to [`Value::String`]
//     String,
//     /// Refer to [`Value::Vec`]
//     Vec,
//     /// Refer to [`Value::Map`]
//     Map,
//     #[default]
//     /// When no type could be determined
//     ///
//     /// When used in an error, it means the error originated outside the parser
//     Invalid,
// }

// impl Value {
//     #[must_use]
//     /// Get the value as a String if it's a String
//     pub const fn as_string(&self) -> Option<&String> {
//         match self {
//             Self::String(val, _, _, _) => Some(val),
//             _ => None,
//         }
//     }
//     /// Get the value as a bool if it's a bool
//     #[must_use]
//     pub const fn as_bool(&self) -> Option<&bool> {
//         match self {
//             Self::Bool(val, _, _) => Some(val),
//             _ => None,
//         }
//     }

//     /// Get the value as a Vec if it's a Vec
//     #[must_use]
//     pub const fn as_vec(&self) -> Option<&Vec<Self>> {
//         match self {
//             Self::Vec(val, _, _) => Some(val),
//             _ => None,
//         }
//     }
//     /// Get the value as a Map if it's a Map
//     #[must_use]
//     pub const fn as_map(&self) -> Option<&MapType<Self, Self>> {
//         match self {
//             Self::Map(val, _, _) => Some(val),
//             _ => None,
//         }
//     }
//     /// Get the value as None if it's None
//     #[must_use]
//     pub const fn as_none(&self) -> Option<&Option<()>> {
//         match self {
//             Self::None(_, _) => Some(&None),
//             _ => None,
//         }
//     }
//     #[must_use]
//     /// Parse the number string into an actual number
//     pub fn to_number<T: TryFromPatch<String>>(&self) -> Option<T> {
//         match self {
//             Self::Number(val, _, _) => Some(T::try_from_value(val.clone())?),
//             _ => None,
//         }
//     }
//     #[must_use]
//     /// Convert the map to another
//     pub fn to_map<T: TryFromPatch<crate::settings::MapType<Self, Self>>>(
//         &self,
//     ) -> Option<T> {
//         match self {
//             Self::Map(val, _, _) => Some(T::try_from_value(val.clone())?),
//             _ => None,
//         }
//     }
// }
// impl Value {
//     #[must_use]
//     /// Turns Map<Value, Value> into Map<String, Value>
//     pub fn to_map_with_only_string_key(
//         &self,
//     ) -> Option<crate::settings::MapType<&String, &Self>> {
//         let map = self.as_map()?;
//         let mut new = MapType::new();
//         for (value, item) in map {
//             let key = value.as_string()?;
//             new.insert(key, item);
//         }
//         Some(new)
//     }
// }

// TODO: Turn this into a trait that auto impl
impl PositionedValue {
    #[must_use]
    /// Checks if the underlying value is equivalent
    pub fn is_value_eq(&self, other: &Self) -> bool {
        let me = &self.value;
        let other = &other.value;
        if me.get_value_type() != other.get_value_type() {
            return false;
        }
        match (me, other) {
            (Value::Simple(val), Value::Simple(val2)) => val.eq(val2),
            (Value::Container(val), Value::Container(val2)) => match (val, val2) {
                (ContainerValue::Map(m), ContainerValue::Map(m2)) => {
                    for (k, v1) in m.iter() {
                        let Some(v2) = m2.get(k) else {
                            return false;
                        };
                        if !v1.is_value_eq(v2) {
                            return false;
                        }
                    }
                    true
                }
                (ContainerValue::Vec(v), ContainerValue::Vec(v2)) => {
                    if v.len() != v2.len() {
                        return false;
                    }
                    for (idx, val1) in v.iter().enumerate() {
                        let val2 = unsafe { v2.get_unchecked(idx) };
                        if !val1.is_value_eq(val2) {
                            return false;
                        }
                    }
                    true
                }
                _ => unsafe { unreachable_unchecked() },
            },
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl PositionedValue {
    #[must_use]
    /// Get the position of the value
    pub const fn get_position(&self) -> PositionRange {
        self.position
    }
    #[must_use]
    /// Wether the position data is valid
    pub fn in_invalid_position(self) -> bool {
        self.position.offset == usize::MAX && self.position.width == usize::MAX
    }
}

mod conversion;
