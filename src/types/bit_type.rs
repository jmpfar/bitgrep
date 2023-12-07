use std::fmt::Display;
use std::str::FromStr;

use super::endian::{FromBigEndian, FromLittleEndian};

/// A general marker trait that represents a type that bitgrep supports.
/// Used as part of the generics black magic
pub trait BitType:
    FromStr + Copy + PartialOrd + Display + FromLittleEndian + FromBigEndian
{
}

impl BitType for f32 {}
impl BitType for f64 {}

impl BitType for i8 {}
impl BitType for i16 {}
impl BitType for i32 {}
impl BitType for i64 {}
impl BitType for i128 {}

impl BitType for u8 {}
impl BitType for u16 {}
impl BitType for u32 {}
impl BitType for u64 {}
impl BitType for u128 {}

pub trait Float: BitType + approx::UlpsEq {
    fn is_nan(self) -> bool;
    fn is_pos_infinity(self) -> bool;
    fn is_neg_infinity(self) -> bool;
}

impl Float for f32 {
    fn is_nan(self) -> bool {
        return self.is_nan();
    }

    fn is_pos_infinity(self) -> bool {
        return self.is_infinite() && self.is_sign_positive();
    }

    fn is_neg_infinity(self) -> bool {
        return self.is_infinite() && self.is_sign_negative();
    }
}
impl Float for f64 {
    fn is_nan(self) -> bool {
        return self.is_nan();
    }

    fn is_pos_infinity(self) -> bool {
        return self.is_infinite() && self.is_sign_positive();
    }

    fn is_neg_infinity(self) -> bool {
        return self.is_infinite() && self.is_sign_negative();
    }
}
