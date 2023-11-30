use std::{fmt::Write, num::ParseIntError};

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum DecodeHexError {
    #[error("Could not parse hex number: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Invalid hex string format: {0}")]
    InvalidHexFormat(String),
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, DecodeHexError> {
    if s.len() % 2 != 0 {
        return Err(DecodeHexError::InvalidHexFormat(s.to_string()));
    }

    let odd_chars = s.chars().step_by(2);
    let even_chars = s.chars().skip(1).step_by(2);

    let hex_pairs = odd_chars
        .zip(even_chars)
        .map(|(c1, c2)| String::from(c1) + &String::from(c2));

    let result: Result<Vec<u8>, ParseIntError> = hex_pairs
        .map(|pair| u8::from_str_radix(&pair, 16))
        .collect();

    return Ok(result?);
}

pub fn encode_hex(bytes: Vec<u8>) -> String {
    let mut result = String::new();
    for byte in bytes {
        write!(&mut result, "{:x}", byte).expect("Failed writing to perfectly good string");
    }

    return result;
}

pub fn encode_hex_borrowed(bytes: &[u8]) -> String {
    let mut result = String::new();
    for byte in bytes {
        write!(&mut result, "{:x}", byte).expect("Failed writing to perfectly good string");
    }

    return result;
}
