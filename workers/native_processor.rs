use super::convertors::{FromLittleEndian, FromBigEndian};
use super::processors::{Endianness, Processor};
use std::marker::PhantomData;
use std::mem;

pub(crate) struct NativeProcessor<T> {
    endianness: Endianness,
    phantom: PhantomData<T>, // TODO(danilan): Remove
}

impl<T> Processor<T> for NativeProcessor<T>
where
    T: FromLittleEndian<Output = T>,
    T: FromBigEndian<Output = T>
{
    fn consume(&self, bytes: &Vec<u8>) -> Option<T> {
        if bytes.len() < self.chunk_size() {
            // Not enough bytes supplied
            return None;
        }

        if self.endianness == Endianness::Big {
            let result = <T as FromBigEndian>::from_bytes(&bytes);
            return Some(result);            
        }

        let result = <T as FromLittleEndian>::from_bytes(&bytes);
        return Some(result);
    }

    fn chunk_size(&self) -> usize {
        return mem::size_of::<T>();
    }
}

impl<T> NativeProcessor<T> {
    pub fn with_little_endian() -> NativeProcessor<T> {
        return Self::new(Endianness::Little);
    }

    pub fn with_big_endian() -> NativeProcessor<T> {
        return Self::new(Endianness::Big);
    }

    pub fn new(endianness: Endianness) -> NativeProcessor<T> {
        return NativeProcessor {
            endianness: endianness,
            phantom: PhantomData,
        };
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_sizes() {
        assert_eq!(NativeProcessor::<i32>::with_big_endian().chunk_size(), 4);
        assert_eq!(NativeProcessor::<f32>::with_big_endian().chunk_size(), 4);
        assert_eq!(NativeProcessor::<u32>::with_big_endian().chunk_size(), 4);

        assert_eq!(NativeProcessor::<i64>::with_little_endian().chunk_size(), 8);
        assert_eq!(NativeProcessor::<f64>::with_little_endian().chunk_size(), 8);
        assert_eq!(NativeProcessor::<u64>::with_little_endian().chunk_size(), 8);
    }

    #[test]
    fn consume() {
        todo!("write consume test and fix interface");
    }
}
