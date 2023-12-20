use std::{cmp, collections::VecDeque, marker::PhantomData};

use crate::utils::ringbuffer::RingBuffer;

use super::processors::{ChunkSize, Processor};

// Do not return entropy until scanned enough bytes
const DEFAULT_MIN_CONSUMED_BYTES: usize = 512;

/// Calculates the absolute Shanon's entropy of the last consumed buffer.
/// We're interested in the entropy around the matched data, so the idea here
/// is to always consume the full read buffer.
///
/// Could have went with a sliding window to get a more exact context but consuming
/// the entire buffer is good enough.
pub struct EntropyProcessor<T> {
    /// Counts of every byte occurences
    histogram: [usize; 256],

    /// Window size used to calculate entropy
    window_size: usize,

    /// Do not return entropy until scanned at least this amount of bytes
    minimum_consumed_bytes: usize,

    // Stores values and probabilities of elements of the sliding window
    buffer: RingBuffer<u8>,
    phantom: PhantomData<T>, // TODO(danilan): Remove
}

impl<T> Processor<T> for EntropyProcessor<T> {
    fn consume(&mut self, bytes: &[u8]) -> Option<T> {
        self.add_bytes(bytes);
        return None;
    }

    fn chunk_size(&self) -> ChunkSize {
        return ChunkSize::Any;
    }
}

pub trait EntropyProducer {
    fn entropy(&self) -> Option<f64>;
}

impl<T> EntropyProducer for EntropyProcessor<T> {
    #[allow(clippy::cast_precision_loss)]
    fn entropy(&self) -> Option<f64> {
        if self.buffer.len() < self.minimum_consumed_bytes {
            return None;
        }

        let mut entropy = 0.0;
        for count in self.histogram {
            if count == 0 {
                continue;
            }

            let p = (count as f64) / (self.buffer.len() as f64);
            entropy -= p * p.log2();
        }

        return Some(entropy);
    }
}

impl<T> EntropyProcessor<T> {
    #[must_use]
    pub fn new(window_size: usize) -> Self {
        return Self::with_minimum_consumed(window_size, DEFAULT_MIN_CONSUMED_BYTES);
    }

    #[must_use]
    fn with_minimum_consumed(window_size: usize, minimum_consumed_bytes: usize) -> Self {
        assert!(
            minimum_consumed_bytes <= window_size,
            "minimum_consumed_bytes must be lesser or equal than window_size"
        );
        EntropyProcessor {
            window_size,
            minimum_consumed_bytes,
            histogram: [0usize; 256],
            buffer: RingBuffer::new(window_size),
            phantom: PhantomData,
        }
    }

    /// Add bytes to queue and calculates histogram
    fn add_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            let value = *byte as usize;
            self.histogram[value] += 1;
            let removed = self.buffer.push_front(*byte);

            if let Some(byte) = removed {
                let value = byte as usize;
                self.histogram[value] -= 1;
            }
        }
        debug_assert!(self.buffer.len() <= self.window_size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_size_returns_any() {
        assert_eq!(
            EntropyProcessor::<f64>::new(512).chunk_size(),
            ChunkSize::Any
        );
    }

    #[test]
    fn consume_less_than_minimum_returns_none() {
        let mut processor: EntropyProcessor<()> = EntropyProcessor::with_minimum_consumed(100, 100);

        processor.consume(&[1u8, 2u8, 3u8, 4u8, 5u8, 6u8]);
        assert_eq!(processor.entropy(), None);
    }

    #[test]
    fn consume_less_than_window_returns_entropy() {
        let mut processor: EntropyProcessor<()> = EntropyProcessor::with_minimum_consumed(100, 1);

        processor.consume(&[1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8]);
        assert_eq!(processor.entropy(), Some(3.321928094887362));
    }

    #[test]
    fn consume_more_than_window_returns_window_entropy() {
        let mut processor: EntropyProcessor<()> = EntropyProcessor::with_minimum_consumed(8, 1);

        processor.consume(&[1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8]);

        // entropy for 030405060708090A
        assert_eq!(processor.entropy(), Some(3.0));
    }

    #[test]
    fn consume_more_than_window_two_iterations_returns_window_entropy() {
        let mut processor: EntropyProcessor<()> = EntropyProcessor::with_minimum_consumed(8, 1);

        processor.consume(&[1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8]);
        processor.consume(&[5u8, 6u8, 7u8, 8u8]);

        // entropy for 0506070805060708
        assert_eq!(processor.entropy(), Some(2.0));
    }
}
