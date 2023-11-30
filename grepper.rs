use std::mem;
use std::{error::Error, fs::File};

use crate::filebuffer::FileBuffer;
use crate::hex;

pub(crate) struct DoubleGrepper<'a> {
    file_path: String,
    minimum: Option<f64>,
    maximum: Option<f64>,
    show_floats: bool,

    filebuffer: FileBuffer<'a>,
}

impl<'a> DoubleGrepper<'a> {
    pub fn new(file_path: String, minimum: f64, maximum: f64, show_floats: bool) -> Self {
        let file = File::open(file_path.as_str()).expect("Expect file to be opened");

        return Self {
            file_path: file_path.clone(),
            minimum: minimum.into(),
            maximum: maximum.into(),
            show_floats: show_floats,
            filebuffer: FileBuffer::new(file),
        };
    }

    pub fn scan(&mut self) -> Result<usize, Box<dyn Error>> {
        let position = self.scan_buffer()?;
        Ok(position)
    }

    fn scan_buffer(&mut self) -> Result<usize, Box<dyn Error>> {
        const SEEKED_SIZE: usize = mem::size_of::<f64>();
        loop {
            let data = self.filebuffer.peek(SEEKED_SIZE)?;
            if data.len() < SEEKED_SIZE {
                // eof or only unprocessable leftovers remain
                break;
            }

            // TODO(danilan): Fix byte order
            let value = f64::from_le_bytes(
                data.as_slice()
                    .try_into()
                    .expect("Buffer should be large enough to contain {SEEKED_SIZE}"),
            );

            if self.is_needle(value) {
                let cur_pos = self.filebuffer.position();
                println!(
                    "{}: [{cur_pos:#01X}] double: {value} [{}]",
                    self.file_path,
                    hex::encode_hex_borrowed(&data)
                )
            }

            let bytes = &data[0..4];
            // TODO(danilan): temp f32 handling.
            let value = f32::from_le_bytes(bytes.try_into().expect("Expect valid buffer"));

            if self.show_floats && self.is_needle_32(value) {
                let cur_pos = self.filebuffer.position();
                println!(
                    "{}: [{cur_pos:#01X}] float: {value} [{}]",
                    self.file_path,
                    hex::encode_hex_borrowed(bytes)
                )
            }

            // move carret to the next byte
            let _ = self.filebuffer.pop(1);
        }

        Ok(self.filebuffer.position())
    }

    fn is_needle(&self, value: f64) -> bool {
        if !value.is_finite() {
            return false;
        }

        if self.minimum.is_some_and(|min| value < min) {
            return false;
        }

        if self.maximum.is_some_and(|max| value > max) {
            return false;
        }

        true
    }

    fn is_needle_32(&self, value: f32) -> bool {
        if !value.is_finite() {
            return false;
        }

        if self.minimum.is_some_and(|min| value < min as f32) {
            return false;
        }

        if self.maximum.is_some_and(|max| value > max as f32) {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::DoubleGrepper;
    use crate::filebuffer::FileBuffer;

    #[test]
    fn scan_buffer() {
        let buf = vec![1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8];
        let mut double_grepper = DoubleGrepper {
            file_path: "ok".into(),
            show_floats: false,
            minimum: f64::MIN.into(),
            maximum: f64::MAX.into(),
            filebuffer: FileBuffer::new(buf.as_slice()),
        };

        let bytes_scanned = double_grepper.scan().expect("scan to complete successfuly");
        assert_eq!(bytes_scanned, 3);
    }
}
