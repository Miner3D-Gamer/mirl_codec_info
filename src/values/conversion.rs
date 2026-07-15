use mirl_extensions::*;

use crate::values::PositionedValue;

impl TryFromPatch<PositionedValue> for std::string::String {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.into_string()
    }
}
impl<T: TryFromPatch<PositionedValue>> TryFromPatch<PositionedValue> for Vec<T> {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.into_vec().and_then(|list| {
            list.iter()
                .map(|x| T::try_from_value(x.clone()))
                .collect::<Vec<Option<T>>>()
                .collect_options()
        })
    }
}
impl TryFromPatch<PositionedValue> for bool {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.into_bool()
    }
}

impl TryFromPatch<PositionedValue> for u8 {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.to_number()
    }
}

impl TryFromPatch<PositionedValue> for u16 {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.to_number()
    }
}

impl TryFromPatch<PositionedValue> for u32 {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.to_number()
    }
}

impl TryFromPatch<PositionedValue> for u64 {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.to_number()
    }
}

impl TryFromPatch<PositionedValue> for u128 {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.to_number()
    }
}

impl TryFromPatch<PositionedValue> for usize {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.to_number()
    }
}

impl TryFromPatch<PositionedValue> for i8 {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.to_number()
    }
}

impl TryFromPatch<PositionedValue> for i16 {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.to_number()
    }
}

impl TryFromPatch<PositionedValue> for i32 {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.to_number()
    }
}

impl TryFromPatch<PositionedValue> for i64 {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.to_number()
    }
}

impl TryFromPatch<PositionedValue> for i128 {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.to_number()
    }
}

impl TryFromPatch<PositionedValue> for isize {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.to_number()
    }
}

impl TryFromPatch<PositionedValue> for f32 {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.to_number()
    }
}

impl TryFromPatch<PositionedValue> for f64 {
    fn try_from_value(value: PositionedValue) -> Option<Self> {
        value.to_number()
    }
}
