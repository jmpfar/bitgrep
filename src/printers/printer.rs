use std::fmt::Display;

use super::output::Output;

/// Implements a printing output sink for bitgrep
pub trait Printer<T: Display> {
    /// Feed a result to print
    fn feed(&self, output: Output<T>);

    /// Call when ending all processing, this allows
    /// printers to print a footer or flush buffered output.
    fn end(&mut self);
}
