/// Processes a fixed size of byte chunks
pub trait Processor<T> {
    fn consume(&self, bytes: &[u8]) -> Option<T>;
    fn chunk_size(&self) -> usize;
}
