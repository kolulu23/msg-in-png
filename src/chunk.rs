//! # Chunk
//! Implementation of a PNG chunk
//!
//! A PNG chunk contains four components:
//! 1. A 4 byte length(big endian)
//! 2. A 4 byte type, see [ChunkType](crate::chunk_type::ChunkType)
//! 3. Chunk data, could be 0 byte to N byte
//! 4. A 4 byte CRC field
//!
//! A chunk is not only responsible for holding image data, but also controls how a decoder can read
//! the PNG file.

use crate::chunk_type::ChunkType;
use anyhow::Result;
use crc32fast::Hasher;
use std::fmt::{Display, Formatter};
use std::io::{BufReader, Read};

#[derive(Debug)]
pub struct Chunk {
    /// A 4-byte unsigned integer giving the number of bytes in the chunk's data field.
    /// The `length` counts only the `data` field, not itself, the `chunk_type` code, or the `crc`.
    /// Zero is a valid length.
    length: u32,
    /// A 4-byte chunk type code. Only ASCII A-Z(65-90) and a-z(97-122) are acceptable.
    chunk_type: ChunkType,
    /// 0..N Bytes depending on what type this chunk is
    data: Vec<u8>,
    /// ISO-3309 Cyclic Redundancy Check  
    /// The 32-bit CRC register is initialized to all 1's, and then the data from each byte
    /// is processed from the least significant bit (1) to the most significant bit (128).
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let mut crc_hasher = Hasher::new();
        crc_hasher.update(chunk_type.inner.as_slice());
        crc_hasher.update(data.as_slice());
        let crc = crc_hasher.finalize();
        Self {
            length: data.len() as u32,
            chunk_type,
            data,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    /// Convert data into UTF-8 string, if given data can not be represented as valid UTF-8 string,
    /// error will be returned instead
    pub fn data_as_string(&self) -> Result<String> {
        std::str::from_utf8(self.data.as_slice())
            .map_err(|e| anyhow::Error::from(e))
            .map(|slice| String::from(slice))
    }

    /// Returns the entire chunk as a sequence of bytes in the order required by the PNG spec.
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.inner.iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);
        let mut four_bytes: [u8; 4] = [0; 4];

        reader.read_exact(&mut four_bytes)?;
        let length = u32::from_be_bytes(four_bytes);

        reader.read_exact(&mut four_bytes)?;
        let chunk_type = ChunkType::try_from(four_bytes)?;

        let mut data: Vec<u8> = vec![0; length as usize];
        reader.read_exact(data.as_mut_slice())?;

        reader.read_exact(&mut four_bytes)?;
        let crc = u32::from_be_bytes(four_bytes);

        let chunk = Chunk::new(chunk_type, data);
        if chunk.length() != length {
            return Err(Self::Error::msg("Length does not match actual data size"));
        }
        if chunk.crc() != crc {
            return Err(Self::Error::msg("CRC check failed"));
        }
        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}
