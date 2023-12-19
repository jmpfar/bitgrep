use std::cell::RefCell;
use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;
use std::rc::Rc;

use crate::common::SourceFile;
use crate::filebuffer::FileBuffer;
use crate::filters::filter::Filter;
use crate::printers::output::{DataContext, Output};
use crate::printers::printer::Printer;
use crate::workers::processors::Processor;

type EntropyProcessorRef<T> = Option<Rc<RefCell<dyn Processor<T>>>>;

/// Scans a file for data types that match a filter
/// T is the type to be scanned
pub struct Scanner<'a, T, P>
where
    T: Display + Copy,
    P: Printer<T>,
{
    file_path: PathBuf,
    filebuffer: FileBuffer<'a>,
    printer: P,

    // TODO(danilan): Move to static dispatch
    filter: Box<dyn Filter<T>>,
    processor: Box<dyn Processor<T>>,
    entropy_processor: EntropyProcessorRef<T>,
}

impl<'a, T, P> Scanner<'a, T, P>
where
    T: Display + Copy,
    P: Printer<T>,
{
    #[must_use]
    pub fn new(
        file: SourceFile<'a>,
        processor: Box<dyn Processor<T>>,
        filter: Box<dyn Filter<T>>,
        printer: P,
    ) -> Self {
        return Self::with_entropy_processor(file, processor, filter, printer, None);
    }

    // TODO(danilan): Add a generic interface for handling different processors
    #[must_use]
    pub fn with_entropy_processor(
        file: SourceFile<'a>,
        processor: Box<dyn Processor<T>>,
        filter: Box<dyn Filter<T>>,
        printer: P,
        entropy_processor: EntropyProcessorRef<T>,
    ) -> Self {
        return Self {
            file_path: file.path(),
            filebuffer: FileBuffer::new(file.file()),
            printer,
            filter,
            processor,
            entropy_processor,
        };
    }

    pub fn scan(mut self) -> Result<usize, Box<dyn Error>> {
        return self.scan_file();
    }

    /// Wrapper method that calls printer.end(), also does not
    /// consume self so can be used for testing.
    fn scan_file(&mut self) -> Result<usize, Box<dyn Error>> {
        let position = self.scan_buffer()?;
        self.printer.end()?;
        Ok(position)
    }

    fn scan_buffer(&mut self) -> Result<usize, Box<dyn Error>> {
        let type_name = std::any::type_name::<T>();
        let chunk_size = self.processor.chunk_size().unwrap();
        loop {
            let cur_pos = self.filebuffer.position();
            let data = self.filebuffer.peek(chunk_size)?;

            let result = self.processor.consume(data);

            // TODO(danilan): Need to improve here, cause it doesn't make sense to scan byte by byte
            if let Some(entropy_processor) = &self.entropy_processor {
                entropy_processor.borrow_mut().consume(&data[..1]);
            }

            if result.is_none() {
                break; // EOF
            }

            if self.filter.include_unwrap(result) {
                let output = Output::new(
                    &self.file_path,
                    result.unwrap(),
                    type_name.into(),
                    DataContext::new(data.to_vec(), cur_pos),
                );
                self.printer.feed(output)?;
            }

            // move carret to the next byte
            self.filebuffer.pop_drop(1)?;
        }

        Ok(self.filebuffer.position())
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fmt::Display, path::Path, vec};

    use assertor::{assert_that, VecAssertion};

    use super::Scanner;
    use crate::{
        common::{Endianness, SourceFile},
        filters::filter::Filter,
        printers::{
            output::{DataContext, Output},
            printer::Printer,
        },
        workers::native_processor::NativeProcessor,
    };

    struct TrueFilter;

    impl<T> Filter<T> for TrueFilter {
        fn include(&self, _: T) -> bool {
            return true;
        }
    }

    struct FakePrinter<T>
    where
        T: Display + Clone,
    {
        outputs: Vec<Output<T>>,
        finished: bool,
    }

    impl<T> FakePrinter<T>
    where
        T: Display + Clone,
    {
        fn new() -> Self {
            return FakePrinter {
                outputs: Vec::new(),
                finished: false,
            };
        }
    }

    impl<T> Printer<T> for FakePrinter<T>
    where
        T: Display + Clone,
    {
        fn feed(&mut self, output: Output<T>) -> Result<(), Box<(dyn Error)>> {
            self.outputs.push(output);
            Ok(())
        }

        fn end(&mut self) -> Result<(), Box<(dyn Error)>> {
            self.finished = true;
            Ok(())
        }
    }

    // TODO(danilan): Test entropy_processor

    #[test]
    fn scan_buffer_f64() {
        let buf = vec![1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8];
        let file = SourceFile::new("ok".into(), buf.as_slice());

        let mut scanner = Scanner::new(
            file,
            Box::new(NativeProcessor::<f64>::new(Endianness::Little)),
            Box::new(TrueFilter {}),
            FakePrinter::<f64>::new(),
        );

        let bytes_scanned = scanner.scan_file().expect("scan to complete successfuly");
        assert_eq!(bytes_scanned, 3);

        let expected = vec![
            Output::new(
                Path::new("ok"),
                f64::from_le_bytes(buf[..8].try_into().unwrap()),
                "f64".into(),
                DataContext::new(buf[..8].to_vec(), 0),
            ),
            Output::new(
                Path::new("ok"),
                f64::from_le_bytes(buf[1..9].try_into().unwrap()),
                "f64".into(),
                DataContext::new(buf[1..9].to_vec(), 1),
            ),
            Output::new(
                Path::new("ok"),
                f64::from_le_bytes(buf[2..10].try_into().unwrap()),
                "f64".into(),
                DataContext::new(buf[2..10].to_vec(), 2),
            ),
        ];
        assert_that!(scanner.printer.outputs).contains_exactly_in_order(expected);
        assert_that!(scanner.printer.finished);
    }

    #[test]
    fn scan_buffer_big() {
        let buf = vec![1u8, 2u8, 3u8, 4u8, 5u8, 6u8];
        let file = SourceFile::new("ok".into(), buf.as_slice());

        let mut scanner = Scanner::new(
            file,
            Box::new(NativeProcessor::<i32>::new(Endianness::Big)),
            Box::new(TrueFilter {}), // Empty filter
            FakePrinter::<i32>::new(),
        );

        let bytes_scanned = scanner.scan_file().expect("scan to complete successfuly");
        assert_eq!(bytes_scanned, 3);

        let expected = vec![
            Output::new(
                Path::new("ok"),
                i32::from_be_bytes(buf[..4].try_into().unwrap()),
                "i32".into(),
                DataContext::new(buf[..4].to_vec(), 0),
            ),
            Output::new(
                Path::new("ok"),
                i32::from_be_bytes(buf[1..5].try_into().unwrap()),
                "i32".into(),
                DataContext::new(buf[1..5].to_vec(), 1),
            ),
            Output::new(
                Path::new("ok"),
                i32::from_be_bytes(buf[2..6].try_into().unwrap()),
                "i32".into(),
                DataContext::new(buf[2..6].to_vec(), 2),
            ),
        ];

        assert_that!(scanner.printer.outputs).contains_exactly_in_order(expected);
        assert_that!(scanner.printer.finished);
    }
}
