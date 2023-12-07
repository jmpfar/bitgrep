pub trait FromLittleEndian {
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl FromLittleEndian for f64 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return f64::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for f32 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return f32::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for i8 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return i8::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for i16 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return i16::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for i32 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return i32::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for i64 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return i64::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for i128 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return i128::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for u8 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return u8::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for u16 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return u16::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for u32 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return u32::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for u64 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return u64::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromLittleEndian for u128 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return u128::from_le_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

pub trait FromBigEndian {
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl FromBigEndian for f64 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return f64::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for f32 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return f32::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for i8 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return i8::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for i16 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return i16::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for i32 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return i32::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for i64 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return i64::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for i128 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return i128::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for u8 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return u8::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for u16 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return u16::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for u32 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return u32::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for u64 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return u64::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}

impl FromBigEndian for u128 {
    fn from_bytes(bytes: &[u8]) -> Self {
        return u128::from_be_bytes(bytes.try_into().unwrap_or_else(|_| {
            panic!(
                "Amount of bytes should be larger than data type size. bytes_len={}",
                bytes.len()
            )
        }));
    }
}
