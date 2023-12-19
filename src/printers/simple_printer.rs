use std::{error::Error, fmt::Display, io::Write, marker::PhantomData};

use super::{output::Output, output::Stringifier, printer::Printer};

// TODO(danilan): Move to io::write and add tests
pub struct SimplePrinter<T, S, W>
where
    T: Display,
    S: Stringifier<T>,
    W: Write,
{
    finished: bool,
    stringifier: S,
    io_writer: W,
    phantom: PhantomData<T>,
}

impl<T, S, W> Printer<T> for SimplePrinter<T, S, W>
where
    T: Display,
    S: Stringifier<T>,
    W: Write,
{
    fn feed(&mut self, output: Output<T>) -> Result<(), Box<dyn Error>> {
        assert!(!self.finished, "Cannot called feed() after end()");
        self.io_writer
            .write_fmt(format_args!("{}\n", self.stringifier.stringify(output)))?;

        Ok(())
    }

    fn end(&mut self) -> Result<(), Box<dyn Error>> {
        self.io_writer.flush()?;
        self.finished = true;
        Ok(())
    }
}

impl<T, S, W> Drop for SimplePrinter<T, S, W>
where
    T: Display,
    S: Stringifier<T>,
    W: Write,
{
    fn drop(&mut self) {
        if !cfg!(test) {
            debug_assert!(self.finished, "Must call end() when finishing printing");
        }
    }
}

impl<T, S, W> SimplePrinter<T, S, W>
where
    T: Display,
    S: Stringifier<T>,
    W: Write,
{
    pub fn new(stringifier: S, io_writer: W) -> Self {
        return Self {
            finished: false,
            stringifier,
            io_writer,
            phantom: PhantomData,
        };
    }
}

#[cfg(test)]
mod tests {

    use assertor::{assert_that, VecAssertion};

    use crate::printers::{
        output::{self, Output, Stringifier},
        printer::Printer,
    };

    use super::SimplePrinter;

    struct FakeStringifier {}

    impl Stringifier<i32> for FakeStringifier {
        fn stringify(&self, output: Output<i32>) -> String {
            return "OK!".into();
        }
    }

    #[test]
    fn printer_feed_expects_lines_written() {
        let vec = vec![];
        let first_string = "OK!\n".as_bytes();

        let mut expected_vec: Vec<u8> = vec![];
        expected_vec.extend(first_string);
        expected_vec.extend(first_string);

        let mut printer = SimplePrinter::new(FakeStringifier {}, vec);

        printer.feed(Output::default()).unwrap();
        printer.feed(Output::default()).unwrap();

        assert_that!(printer.io_writer).contains_exactly_in_order(expected_vec);
    }
}
