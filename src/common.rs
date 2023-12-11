use std::{io, path::PathBuf};

pub const DEFAULT_BUFFER_SIZE: usize = 4096;

#[derive(PartialEq, Debug, Clone, Copy, clap::ValueEnum)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(PartialEq, Debug, Clone, clap::ValueEnum)]
pub enum DataType {
    I8,
    I16,
    I32,
    I64,
    I128,

    U8,
    U16,
    U32,
    U64,
    U128,

    F32,
    F64,
}

pub struct SourceFile<'a> {
    path: PathBuf,
    file: Box<dyn io::Read + 'a>,
}

impl<'a> SourceFile<'a> {
    #[must_use]
    pub fn new(path: PathBuf, file: impl io::Read + 'a) -> Self {
        return SourceFile {
            path,
            file: Box::new(file),
        };
    }

    #[must_use]
    pub fn path(&self) -> PathBuf {
        return self.path.clone();
    }

    // TODO(danilan): This unwraps the struct, maybe move to Rc
    #[must_use]
    pub fn file(self) -> Box<dyn io::Read + 'a> {
        return self.file;
    }
}
