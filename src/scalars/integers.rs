use crate::{impl_scalar_internal, Result, Scalar, Value};

macro_rules! impl_integer_scalars {
    ($($ty:ty),*) => {
        $(
        impl Scalar for $ty {
            fn type_name() -> &'static str {
                "Int"
            }

            fn description() -> Option<&'static str> {
                Some("The `Int` scalar type represents non-fractional signed whole numeric values. Int can represent values between -(2^31) and 2^31 - 1.")
            }

            fn parse(value: &Value) -> Option<Self> {
                match value {
                    Value::Int(n) => Some(n.as_i64().unwrap() as Self),
                    _ => None
                }
            }

            fn to_json(&self) -> Result<serde_json::Value> {
                Ok((*self).into())
            }
        }

        impl_scalar_internal!($ty);
        )*
    };
}

impl_integer_scalars!(i8, i16, i32, u8, u16, u32);

macro_rules! impl_int64_scalars {
    ($($ty:ty),*) => {
        $(
        impl Scalar for $ty {
            fn type_name() -> &'static str {
                "Int64"
            }

            fn description() -> Option<&'static str> {
                Some("The `Int64` scalar type represents non-fractional signed whole numeric values. Int can represent values between -(2^64) and 2^64 - 1.")
            }

            fn parse(value: &Value) -> Option<Self> {
                match value {
                    Value::Int(n) => Some(n.as_i64().unwrap() as Self),
                    Value::String(s) => s.parse().ok(),
                    _ => None
                }
            }

            fn to_json(&self) -> Result<serde_json::Value> {
                Ok(self.to_string().into())
            }
        }

        impl_scalar_internal!($ty);
        )*
    };
}

impl_int64_scalars!(i64, u64);
