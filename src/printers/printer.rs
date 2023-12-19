use std::fmt::Display;

use super::output::Output;

/// Implements a printing output sink for bitgrep
pub trait Printer<T>
where
    T: Display,
{
    /// Feed a result to print
    fn feed(&mut self, output: Output<T>);

    /// Call when ending all processing, this allows
    /// printers to print a footer or flush buffered output.
    fn end(&mut self);
}
