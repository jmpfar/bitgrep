use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use std::{error::Error, fs::File};

use crate::filebuffer::FileBuffer;
use crate::filters::filter::Filter;
use crate::hex;
use crate::workers::processors::Processor;

type EntropyProcessorRef<T> = Option<Rc<RefCell<dyn Processor<T>>>>;

/// Scans a file for data types that match a filter
/// T is the type to be scanned
pub struct Scanner<'a, T> {
    file_path: String,
    filebuffer: FileBuffer<'a>,

    // TODO(danilan): Move to static dispatch
    filter: Box<dyn Filter<T>>,
    processor: Box<dyn Processor<T>>,
    entropy_processor: EntropyProcessorRef<T>,
}

impl<'a, T> Scanner<'a, T>
where
    T: Display + Copy,
{
    #[must_use]
    pub fn new(
        file_path: &str,
        processor: Box<dyn Processor<T>>,
        filter: Box<dyn Filter<T>>,
    ) -> Self {
        return Self::with_entropy_processor(file_path, processor, filter, None);
    }

    // TODO(danilan): Add a generic interface for handling different processors
    #[must_use]
    pub fn with_entropy_processor(
        file_path: &str,
        processor: Box<dyn Processor<T>>,
        filter: Box<dyn Filter<T>>,
        entropy_processor: EntropyProcessorRef<T>,
    ) -> Self {
        let file = File::open(file_path).expect("File should be opened");

        return Self {
            file_path: file_path.to_owned(),
            filebuffer: FileBuffer::new(file),
            filter,
            processor,
            entropy_processor,
        };
    }

    pub fn scan(&mut self) -> Result<usize, Box<dyn Error>> {
        let position = self.scan_buffer()?;
        Ok(position)
    }

    fn scan_buffer(&mut self) -> Result<usize, Box<dyn Error>> {
        let type_name = std::any::type_name::<T>();
        let chunk_size = self.processor.chunk_size().unwrap();
        loop {
            let cur_pos = self.filebuffer.position();
            let data = self.filebuffer.peek(chunk_size)?;

            let result = self.processor.consume(data);

            if let Some(entropy_processor) = &self.entropy_processor {
                entropy_processor.borrow_mut().consume(data);
            }

            if result.is_none() {
                break; // EOF
            }

            if self.filter.include_unwrap(result) {
                println!(
                    "{}: [{cur_pos:#01X}] {}: {} [{}]",
                    self.file_path,
                    type_name,
                    result.unwrap(),
                    hex::encode_borrowed(data),
                );
            }

            // move carret to the next byte
            self.filebuffer.pop_drop(1)?;
        }

        Ok(self.filebuffer.position())
    }
}

#[cfg(test)]
mod tests {
    use super::Scanner;
    use crate::{
        common::Endianness, filebuffer::FileBuffer, filters::filter::Filter,
        workers::native_processor::NativeProcessor,
    };

    struct TrueFilter;

    impl<T> Filter<T> for TrueFilter {
        fn include(&self, _: T) -> bool {
            return true;
        }
    }

    // TODO(danilan): Fix tests to use mocks and public interface
    // TODO(danilan): Test entropy_processor

    #[test]
    fn scan_buffer() {
        let buf = vec![1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8];
        let mut double_grepper = Scanner::<f64> {
            file_path: "ok".into(),
            filter: Box::new(TrueFilter {}),
            processor: Box::new(NativeProcessor::new(Endianness::Little)),
            filebuffer: FileBuffer::new(buf.as_slice()),
            entropy_processor: None,
        };

        let bytes_scanned = double_grepper.scan().expect("scan to complete successfuly");
        assert_eq!(bytes_scanned, 3);
    }

    #[test]
    fn scan_buffer_big() {
        let buf = vec![1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8];
        let mut double_grepper = Scanner::<i64> {
            file_path: "ok".into(),
            filter: Box::new(TrueFilter {}), // Empty filter
            processor: Box::new(NativeProcessor::new(Endianness::Big)),
            filebuffer: FileBuffer::new(buf.as_slice()),
            entropy_processor: None,
        };

        let bytes_scanned = double_grepper.scan().expect("scan to complete successfuly");
        assert_eq!(bytes_scanned, 3);
    }
}
