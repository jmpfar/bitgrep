use std::{cmp, collections::VecDeque, marker::PhantomData};

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
    histogram: [u8; 256],

    /// Window size used to calculate entropy
    window_size: usize,

    /// Do not return entropy until scanned at least this amount of bytes
    minimum_consumed_bytes: usize,

    // Stores values and probabilities of elements of the sliding window
    buffer: VecDeque<u8>,
    phantom: PhantomData<T>, // TODO(danilan): Remove
}

impl<T> Processor<T> for EntropyProcessor<T> {
    fn consume(&mut self, bytes: &[u8]) -> Option<T> {
        // Some of our buffer might exceed the window size, the part the exceeds will be handled differently.
        let mut leftover_size = (self.buffer.len() + bytes.len()).saturating_sub(self.window_size);

        // try to remove existing bytes to make space but don't exceed existing amount of elements.
        let remove_from_existing: usize = cmp::min(leftover_size, self.buffer.len());

        leftover_size -= self.remove_bytes(remove_from_existing);

        // Handle case where bytes.len() > window_size
        // In this case trim from the start of the slice
        if leftover_size > 0 {
            self.add_bytes(&bytes[leftover_size..]);
            return None;
        }

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
    fn entropy(&self) -> Option<f64> {
        if self.buffer.len() < self.minimum_consumed_bytes {
            return None;
        }

        let mut entropy = 0.0;
        for count in self.histogram {
            if count == 0 {
                continue;
            }

            let p = f64::from(count) / (self.buffer.len() as f64);
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
            histogram: [0u8; 256],
            buffer: VecDeque::with_capacity(window_size),
            phantom: PhantomData,
        }
    }

    /// Add bytes to queue and calculates entropy
    /// Assumes sliding window already has space for the new items
    fn add_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            let value = *byte as usize;
            self.histogram[value] += 1;
            self.buffer.push_front(*byte);
        }
        debug_assert!(self.buffer.len() <= self.window_size);
    }

    /// Removes n bytes from the entropy sliding window
    /// Returns amount of bytes removed
    fn remove_bytes(&mut self, count: usize) -> usize {
        for _ in 0..count {
            let byte = self.buffer.pop_back().unwrap();
            let value = byte as usize;
            self.histogram[value] -= 1;
        }

        return count;
    }
}

#[cfg(test)]
mod tests {
    use std::os::unix::process;

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
