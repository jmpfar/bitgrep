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
        debug_assert!(self.finished, "Must call end() when finishing printing");
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
