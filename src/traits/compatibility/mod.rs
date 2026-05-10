// use crate::values::Value;
// // mod support;

// /// A trait for converting T into [Option<Value>]
// pub trait TryToCodecInfoValue {
//     /// Try to convert self into [Value]
//     fn try_into_codec_value(&self) -> Option<Value>;
// }
// #[allow(clippy::wrong_self_convention)]
// /// A trait for converting T into [Value]
// pub trait ToCodecInfoValue {
//     /// Convert self into [Value]
//     fn into_codec_value(&self) -> Value;
// }

// impl<T: ToCodecInfoValue> TryToCodecInfoValue for T {
//     fn try_into_codec_value(&self) -> Option<Value> {
//         Some(self.into_codec_value())
//     }
// }
