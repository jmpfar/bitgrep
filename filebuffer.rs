use std::cmp;
use std::collections::VecDeque;
use std::io::{Read, self};


pub(crate) struct FileBuffer<'a> {
    // File like implementation thorugh io::Read, make sure lifetime at least as parent
    file: Box<dyn io::Read + 'a>,

    ring_buf: VecDeque<u8>,
    reached_eof: bool,
    buffer_size: usize,
    position: usize
}

const DEFAULT_BUFFER_SIZE : usize = 4096;

impl<'a> FileBuffer<'a> {
    pub fn new(reader: impl io::Read + 'a) -> Self {
        return Self::with_buffer_size(reader, DEFAULT_BUFFER_SIZE);
    }

    fn with_buffer_size(reader: impl io::Read + 'a, buffer_size: usize) -> Self {
        // with_capacity usually actually doubles requested capacity (round(smallest 2^n-1)) 
        // This means that there'll be enough space for both the file read and leftovers
        let buffer = VecDeque::with_capacity(buffer_size);
        FileBuffer { file: Box::new(reader), ring_buf: buffer, reached_eof: false, buffer_size, position: 0 }
    }


    fn fill_buffer(&mut self) -> Result<(), io::Error> {
        let mut buffer = vec![0u8; self.buffer_size];
        
        let bytes_read = self.file.read(&mut buffer)?;
        buffer.truncate(bytes_read);
        
        self.ring_buf.extend(buffer);
        // This is used so we can get a single slice when we need to peek
        self.ring_buf.make_contiguous();
        
        if bytes_read == 0 {
            self.reached_eof = true;
        }

        Ok(())
    }

    /// Peeks ahead into the buffer without changing the current location
    /// If exceeds current buffer it fetches more from the file.
    /// If in EOF and there's no more data, the method will return available data
    pub fn peek(&mut self, n: usize) -> Result<Vec<u8>, io::Error> {
        if self.reached_eof {
            let data = self.ring_buf.as_slices().0;
            let size = cmp::min(n, self.ring_buf.len());
            return Ok(data[..size].to_vec());
        }
        
        if n <= self.ring_buf.len() {
            let data = self.ring_buf.as_slices().0;
            return Ok(data[..n].to_vec());
        } 

        self.fill_buffer()?;

        // Peek with new found buffer, or eof handling
        return self.peek(n);
    }

    fn drain(&mut self, size: usize) -> Vec<u8> {
        let result = self.ring_buf.drain(..size).collect();
        self.position += size;
        return result
    }

    pub fn pop(&mut self, n: usize) -> Result<Vec<u8>, io::Error> {
        if self.reached_eof {
            let size = cmp::min(n, self.ring_buf.len());
            let result = self.drain(size);
            return Ok(result)
        }
        
        if n <= self.ring_buf.len() {
            let result = self.drain(n);
            return Ok(result)
        }

        self.fill_buffer()?;
        
        // Try again with filled buffer
        return self.pop(n);
    }

    pub fn position(&self) -> usize {
        return self.position;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_simple() {
        let file = [1u8, 2u8, 3u8, 4u8].as_slice();
        const BUFFER_SIZE : usize = 4;


        let mut buffer = FileBuffer::with_buffer_size(file.clone(), BUFFER_SIZE);
        let result = buffer.peek(4).expect("peek to succeed");

        assert_eq!([1u8, 2u8, 3u8, 4u8], result.as_slice());  
    }

    #[test]
    fn peek_exceeds_buffer() {
        let file = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8].as_slice();
        const BUFFER_SIZE : usize = 4;


        let mut buffer = FileBuffer::with_buffer_size(file.clone(), BUFFER_SIZE);
        let result = buffer.peek(5).expect("peek to succeed");

        assert_eq!([1u8, 2u8, 3u8, 4u8, 5u8], result.as_slice());  
    }

    #[test]
    fn peek_eof_exact() {
        let file = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8].as_slice();
        const BUFFER_SIZE : usize = 5;


        let mut buffer = FileBuffer::with_buffer_size(file.clone(), BUFFER_SIZE);
        let result = buffer.peek(10).expect("peek to succeed");

        assert_eq!([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8], result.as_slice());  
    }

    #[test]
    fn peek_eof_exceeds() {
        let file = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8].as_slice();
        const BUFFER_SIZE : usize = 4;


        let mut buffer = FileBuffer::with_buffer_size(file.clone(), BUFFER_SIZE);
        let result = buffer.peek(12).expect("peek to succeed");

        assert_eq!([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8], result.as_slice());  
    }


    #[test]
    fn peek_eof_multiple() {
        let file = [].as_slice();

        let mut buffer = FileBuffer::new(file.clone());
        let result = buffer.peek(12).expect("peek to succeed");
        let result2 = buffer.peek(4).expect("peek to succeed");

        assert!(result.is_empty());
        assert!(result2.is_empty());
    }    


    #[test]
    fn pop_simple() {
        let file = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8].as_slice();
        const BUFFER_SIZE : usize = 4;


        let mut buffer = FileBuffer::with_buffer_size(file.clone(), BUFFER_SIZE);
        let result = buffer.pop(4).expect("pop to succeed");

        assert_eq!([1u8, 2u8, 3u8, 4u8], result.as_slice());
    }

    #[test]
    fn pop_exceeds_buffer() {
        let file = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8].as_slice();
        const BUFFER_SIZE : usize = 4;


        let mut buffer = FileBuffer::with_buffer_size(file.clone(), BUFFER_SIZE);
        let result = buffer.pop(5).expect("pop to succeed");

        assert_eq!([1u8, 2u8, 3u8, 4u8, 5u8], result.as_slice());  
    }

    #[test]
    fn pop_eof_exact() {
        let file = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8].as_slice();
        const BUFFER_SIZE : usize = 5;


        let mut buffer = FileBuffer::with_buffer_size(file.clone(), BUFFER_SIZE);
        let result = buffer.pop(10).expect("pop to succeed");

        assert_eq!([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8], result.as_slice());  
    }

    #[test]
    fn pop_eof_exceeds() {
        let file = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8].as_slice();
        const BUFFER_SIZE : usize = 4;

        let mut buffer = FileBuffer::with_buffer_size(file.clone(), BUFFER_SIZE);
        let result = buffer.pop(12).expect("pop to succeed");

        assert_eq!([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8], result.as_slice());  
    }
 
   #[test]
    fn pop_eof_multiple() {
        let file = [].as_slice();

        let mut buffer = FileBuffer::new(file.clone());
        let result = buffer.pop(12).expect("pop to succeed");
        let result2 = buffer.pop(4).expect("pop to succeed");

        assert!(result.is_empty());
        assert!(result2.is_empty());
    }    

    #[test]
    fn pop_zero() {
        let file = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8].as_slice();
        const BUFFER_SIZE : usize = 4;

        let mut buffer = FileBuffer::with_buffer_size(file.clone(), BUFFER_SIZE);
        let result = buffer.pop(0).expect("pop to succeed");

        assert!(result.is_empty());
    }

    #[test]
    fn peek_zero() {
        let file = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8].as_slice();
        const BUFFER_SIZE : usize = 4;

        let mut buffer = FileBuffer::with_buffer_size(file.clone(), BUFFER_SIZE);
        let result = buffer.peek(0).expect("peek to succeed");

        assert!(result.is_empty());
    }

    #[test]
    fn position_start() {
        let file = [].as_slice();

        let buffer = FileBuffer::new(file.clone());

        assert_eq!(buffer.position(), 0);
    }

    #[test]
    fn position_middle() {
        let file = [1u8, 2u8, 3u8, 4u8].as_slice();
        const BUFFER_SIZE : usize = 4;


        let mut buffer = FileBuffer::with_buffer_size(file.clone(), BUFFER_SIZE);
        buffer.pop(3).expect("pop to succeed");

        assert_eq!(buffer.position(), 3);
    }

    #[test]
    fn position_eof() {
        let file = [1u8, 2u8, 3u8, 4u8, 5u8].as_slice();
        const BUFFER_SIZE : usize = 4;


        let mut buffer = FileBuffer::with_buffer_size(file.clone(), BUFFER_SIZE);
        buffer.pop(5).expect("pop to succeed");

        assert_eq!(buffer.position(), 5);
    }    

}