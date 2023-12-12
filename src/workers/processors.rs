/// Processes a fixed size of byte chunks
// TODO(danilan): change interface to not return directly
pub trait Processor<T> {
    fn consume(&mut self, bytes: &[u8]) -> Option<T>;
    fn chunk_size(&self) -> ChunkSize;
}

/// Represents the chunk size the Processor works in.
/// Could have been an Option but might have more values in future.
#[derive(PartialEq, Debug)]
pub enum ChunkSize {
    /// Pass a chunk size of a determined size
    Size(usize),
    /// Pass a chunk size of your chosing
    Any,
}

impl ChunkSize {
    #[must_use]
    pub fn unwrap(self) -> usize {
        match self {
            ChunkSize::Size(val) => val,
            ChunkSize::Any => panic!("called `ChunkSize::unwrap()` on an `Any` value"),
        }
    }

    /// Returns requested chunk size or revert to default
    #[must_use]
    pub fn or_default(self, default_size: usize) -> usize {
        if let ChunkSize::Size(size) = self {
            return size;
        }

        return default_size;
    }
}
