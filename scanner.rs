use std::{error::Error, fs::File};

use crate::filebuffer::FileBuffer;
use crate::filters::and::And;
use crate::filters::filter::Filter;
use crate::filters::max::Max;
use crate::filters::min::Min;
use crate::hex;
use crate::workers::native_processor::NativeProcessor;
use crate::workers::processors::{Endianness, Processor};

pub(crate) struct Scanner<'a> {
    file_path: String,
    filebuffer: FileBuffer<'a>,
    filter: Box<dyn Filter<f64>>,
    processor: NativeProcessor<f64>,
}

impl<'a> Scanner<'a> {
    pub fn new(
        file_path: String,
        minimum: Option<f64>,
        maximum: Option<f64>,
        endianness: Endianness,
    ) -> Self {
        let file = File::open(file_path.as_str()).expect("File should be opened");

        return Self {
            // TODO(danilan): Return float support from the dead
            file_path: file_path.clone(),
            filebuffer: FileBuffer::new(file),
            filter: Self::create_filters(minimum, maximum),
            processor: Self::create_processor(endianness),
        };
    }

    pub fn scan(&mut self) -> Result<usize, Box<dyn Error>> {
        let position = self.scan_buffer()?;
        Ok(position)
    }

    fn create_filters(minimum: Option<f64>, maximum: Option<f64>) -> Box<dyn Filter<f64>> {
        let mut filters: Vec<Box<dyn Filter<f64>>> = vec![];
        if let Some(min) = minimum {
            filters.push(Min::with_box(min));
        }

        if let Some(max) = maximum {
            filters.push(Max::with_box(max));
        }

        if filters.len() == 1 {
            return filters.remove(0);
        }

        return Box::new(And::with_filters(filters));
    }

    fn create_processor(endianness: Endianness) -> NativeProcessor<f64> {
        return NativeProcessor::new(endianness);
    }

    fn scan_buffer(&mut self) -> Result<usize, Box<dyn Error>> {
        let chunk_size = self.processor.chunk_size();
        loop {
            let data = self.filebuffer.peek(chunk_size)?;

            // TODO: Change to vector
            let result = self.processor.consume(&data);

            if result.is_none() {
                break; // EOF
            }

            if self.filter.include_unwrap(result) {
                let cur_pos = self.filebuffer.position();
                println!(
                    "{}: [{cur_pos:#01X}] double: {} [{}]",
                    self.file_path,
                    result.unwrap(),
                    hex::encode_hex_borrowed(&data),
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
        filebuffer::FileBuffer,
        filters::and::And,
        workers::{native_processor::NativeProcessor, processors::Endianness},
    };

    #[test]
    fn scan_buffer() {
        let buf = vec![1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8];
        let mut double_grepper = Scanner {
            file_path: "ok".into(),
            filter: Box::new(And::new()), // Empty filter
            processor: NativeProcessor::new(Endianness::Little),
            filebuffer: FileBuffer::new(buf.as_slice()),
        };

        let bytes_scanned = double_grepper.scan().expect("scan to complete successfuly");
        assert_eq!(bytes_scanned, 3);
    }

    #[test]
    fn scan_buffer_big() {
        let buf = vec![1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8];
        let mut double_grepper = Scanner {
            file_path: "ok".into(),
            filter: Box::new(And::new()), // Empty filter
            processor: NativeProcessor::new(Endianness::Big),
            filebuffer: FileBuffer::new(buf.as_slice()),
        };

        let bytes_scanned = double_grepper.scan().expect("scan to complete successfuly");
        assert_eq!(bytes_scanned, 3);
    }
}
