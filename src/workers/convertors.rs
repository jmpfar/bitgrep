pub(super) trait FromLittleEndian {
    type Output;

    fn from_bytes(bytes: &[u8]) -> Self::Output;
}

impl FromLittleEndian for f64 {
    type Output = f64;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return f64::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for f32 {
    type Output = f32;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return f32::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for i8 {
    type Output = i8;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return i8::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for i16 {
    type Output = i16;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return i16::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for i32 {
    type Output = i32;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return i32::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for i64 {
    type Output = i64;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return i64::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for i128 {
    type Output = i128;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return i128::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for u8 {
    type Output = u8;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return u8::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for u16 {
    type Output = u16;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return u16::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for u32 {
    type Output = u32;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return u32::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for u64 {
    type Output = u64;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return u64::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for u128 {
    type Output = u128;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return u128::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

pub(super) trait FromBigEndian {
    type Output;

    fn from_bytes(bytes: &[u8]) -> Self::Output;
}

impl FromBigEndian for f64 {
    type Output = f64;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return f64::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for f32 {
    type Output = f32;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return f32::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for i8 {
    type Output = i8;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return i8::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for i16 {
    type Output = i16;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return i16::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for i32 {
    type Output = i32;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return i32::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for i64 {
    type Output = i64;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return i64::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for i128 {
    type Output = i128;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return i128::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for u8 {
    type Output = u8;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return u8::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for u16 {
    type Output = u16;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return u16::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for u32 {
    type Output = u32;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return u32::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for u64 {
    type Output = u64;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return u64::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for u128 {
    type Output = u128;
    fn from_bytes(bytes: &[u8]) -> Self::Output {
        return u128::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}
