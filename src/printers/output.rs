use std::path::Path;
use std::string::String;
use std::{fmt::Display, path::PathBuf};

use crate::hex;

pub(crate) enum Content {
    Simple,
    Context,
}

pub(crate) struct Output<'a, T: Display> {
    file_path: PathBuf,
    value_type: String, // Enum?
    value: T,           // Remove T and convert on own?
    data_context: DataContext<'a>,
}

impl<'a, T: Display> Output<'a, T> {
    pub fn new(path: &Path, value: T, value_type: String, data_context: DataContext<'a>) -> Self {
        return Self {
            file_path: path.to_owned(),
            value,
            value_type,
            data_context,
        };
    }
}

pub(crate) struct DataContext<'a> {
    data: &'a [u8],
    offset: usize,

    /// zero index of value in data
    value_index: usize,
    /// size of value
    value_size: usize,
}

impl<'a> DataContext<'a> {
    pub fn new(data: &'a [u8], offset: usize) -> Self {
        return DataContext {
            data,
            offset,
            value_index: 0,
            value_size: data.len(),
        };
    }

    // TODO(danilan): add with_context()

    fn value_as_slice(&self) -> &[u8] {
        let range = self.value_index..(self.value_index + self.value_size);
        return &self.data[range];
    }
}

// TODO(danilan): Consider moving to newtypes implementing display

pub struct SimpleOutput {}

impl SimpleOutput {
    pub fn new() -> Self {
        return SimpleOutput {};
    }
}

pub trait Stringifier<T: Display> {
    fn stringify(&self, output: Output<T>) -> String;
}

impl<T: Display> Stringifier<T> for SimpleOutput {
    fn stringify(&self, output: Output<T>) -> String {
        return format!(
            "{}: [{:#01X}] {}: {} [{}]",
            output.file_path.display(),
            output.data_context.offset,
            output.value_type,
            output.value,
            hex::encode(output.data_context.value_as_slice()),
        );
    }
}
