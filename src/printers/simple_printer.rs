use std::{fmt::Display, marker::PhantomData};

use super::{output::Output, output::Stringifier, printer::Printer};

// TODO(danilan): Move to io::write and add tests
pub struct SimplePrinter<T, S>
where
    T: Display,
    S: Stringifier<T>,
{
    finished: bool,
    stringifier: S,
    phantom: PhantomData<T>,
}

impl<T, S> Printer<T> for SimplePrinter<T, S>
where
    T: Display,
    S: Stringifier<T>,
{
    fn feed(&mut self, output: Output<T>) {
        println!("{}", self.stringifier.stringify(output));
    }

    fn end(&mut self) {
        self.finished = true;
    }
}

impl<T, S> Drop for SimplePrinter<T, S>
where
    T: Display,
    S: Stringifier<T>,
{
    fn drop(&mut self) {
        debug_assert!(self.finished, "Must call end() when finishing printing");
    }
}

impl<T, S> SimplePrinter<T, S>
where
    T: Display,
    S: Stringifier<T>,
{
    pub fn new(stringifier: S) -> Self {
        return Self {
            finished: false,
            stringifier,
            phantom: PhantomData,
        };
    }
}
