/// Processes a fixed size of byte chunks
pub(crate) trait Processor<T> {
    fn consume(&self, bytes: &Vec<u8>) -> Option<T>;
    fn chunk_size(&self) -> usize;
}
