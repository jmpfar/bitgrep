use super::processors::ChunkSize::Size;
use super::processors::{ChunkSize, Processor};
use crate::common::Endianness;
use crate::types::bit_type::BitType;
use crate::types::endian::{FromBigEndian, FromLittleEndian};
use std::marker::PhantomData;
use std::mem;

pub struct NativeProcessor<T> {
    endianness: Endianness,
    phantom: PhantomData<T>, // TODO(danilan): Remove
}

impl<T> Processor<T> for NativeProcessor<T>
where
    T: BitType,
{
    // TODO(danilan): change interface to not return directly
    fn consume(&mut self, bytes: &[u8]) -> Option<T> {
        if bytes.len() < self.chunk_size().unwrap() {
            // Not enough bytes supplied
            return None;
        }

        if self.endianness == Endianness::Big {
            let result = <T as FromBigEndian>::from_bytes(bytes);
            return Some(result);
        }

        let result = <T as FromLittleEndian>::from_bytes(bytes);
        return Some(result);
    }

    fn chunk_size(&self) -> ChunkSize {
        return Size(mem::size_of::<T>());
    }
}

impl<T> NativeProcessor<T>
where
    T: BitType,
{
    #[must_use]
    pub fn with_little_endian() -> NativeProcessor<T> {
        Self::new(Endianness::Little)
    }

    #[must_use]
    pub fn with_big_endian() -> NativeProcessor<T> {
        Self::new(Endianness::Big)
    }

    #[must_use]
    pub fn new(endianness: Endianness) -> NativeProcessor<T> {
        NativeProcessor {
            endianness,
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_sizes() {
        assert_eq!(
            NativeProcessor::<i16>::with_big_endian().chunk_size(),
            Size(2)
        );
        assert_eq!(
            NativeProcessor::<u16>::with_big_endian().chunk_size(),
            Size(2)
        );

        assert_eq!(
            NativeProcessor::<i32>::with_big_endian().chunk_size(),
            Size(4)
        );
        assert_eq!(
            NativeProcessor::<f32>::with_big_endian().chunk_size(),
            Size(4)
        );
        assert_eq!(
            NativeProcessor::<u32>::with_big_endian().chunk_size(),
            Size(4)
        );

        assert_eq!(
            NativeProcessor::<i64>::with_little_endian().chunk_size(),
            Size(8)
        );
        assert_eq!(
            NativeProcessor::<f64>::with_little_endian().chunk_size(),
            Size(8)
        );
        assert_eq!(
            NativeProcessor::<u64>::with_little_endian().chunk_size(),
            Size(8)
        );

        assert_eq!(
            NativeProcessor::<i128>::with_little_endian().chunk_size(),
            Size(16)
        );
        assert_eq!(
            NativeProcessor::<u128>::with_little_endian().chunk_size(),
            Size(16)
        );
    }

    #[test]
    fn consume_big() {
        let mut processor: NativeProcessor<_> = NativeProcessor::new(Endianness::Big);
        let result: i32 = processor.consume(&[0u8, 0u8, 0x10u8, 0xf8u8]).unwrap();

        assert_eq!(result, 4344);
    }

    #[test]
    fn consume_little() {
        let mut processor: NativeProcessor<_> = NativeProcessor::new(Endianness::Little);
        let result: i32 = processor.consume(&[0xf8u8, 0x10u8, 0u8, 0u8]).unwrap();

        assert_eq!(result, 4344);
    }

    #[test]
    fn consume_not_enough_bytes_returns_none() {
        let mut processor: NativeProcessor<i128> = NativeProcessor::new(Endianness::Little);
        let result = processor.consume(&[0u8, 0u8, 0x10u8, 0xf8u8]);

        assert!(result.is_none());
    }
}
