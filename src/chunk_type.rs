#![allow(dead_code)]
//! # Chunk Type
//! PNG files are essentially just a list of "chunks", each containing their own data.
//! Each chunk has a type that can be represented as a 4 character string.
//! See more on [PNG spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Contents.html)

use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct ChunkType {
    /// Stores four bytes in the order of `critical byte`, `public/private byte`, `reserved byte` and
    /// `state-of-copy byte`.
    pub inner: [u8; 4],
}

impl ChunkType {
    /// Get bytes from this type, bytes are copied.
    pub fn bytes(&self) -> [u8; 4] {
        return self.inner;
    }

    /// Determine whether this chunk type is valid according to PNG spec.
    /// A chunk type is valid if:
    /// 1. All four bytes are decimal number 65-90 or 97-122 (ASCII A-Z and a-z)
    /// 2. Reserved bit is 0 (the third letter is uppercase)
    pub fn is_valid(&self) -> bool {
        for item in self.inner {
            if !item.is_ascii_alphabetic() {
                return false;
            }
        }
        return self.is_reserved_bit_valid();
    }

    pub fn is_critical(&self) -> bool {
        self.inner[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.inner[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.inner[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.inner[3].is_ascii_lowercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = anyhow::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let chunk_type = ChunkType { inner: value };
        if chunk_type.is_valid() {
            Ok(chunk_type)
        } else {
            Err(Self::Error::msg("Not a valid chunk type value"))
        }
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let source_bytes = s.as_bytes();
        if source_bytes.len() != 4 {
            return Err(Self::Err::msg("Chunk type str must be 4 bytes"));
        }
        let mut chunk_bytes: [u8; 4] = [0; 4];
        for (index, byte) in source_bytes[0..4].iter().enumerate() {
            if !byte.is_ascii_alphabetic() {
                let err_msg = format!(
                    "Source string contains non ASCII alphabetic letter {}",
                    byte
                );
                return Err(Self::Err::msg(err_msg));
            }
            chunk_bytes[index] = *byte;
        }
        // This chunk type could be invalid, but we still construct an instance for it
        Ok(ChunkType { inner: chunk_bytes })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // UTF-8 has the same code page as ASCII in first 128 characters
        let chunk_type_string =
            String::from_utf8(self.inner.to_vec()).unwrap_or(String::from("Unknown"));
        write!(f, "{}", chunk_type_string)
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl Eq for ChunkType {}
