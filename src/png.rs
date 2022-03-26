#![allow(dead_code)]
//! # PNG
//! Png file structure according to its spec.
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

/// The PNG file structure
pub struct PNG {
    /// Signature of a png file will always be `89 50 4E 47 0D 0A 1A 0A`
    signature: [u8; 8],
    /// A list of chunks, a valid png file must start with IHDR chunk and end with IEND chunk
    chunks: Vec<Chunk>,
}

impl PNG {
    pub const STANDARD_HEADER: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    /// Constructor for a png structure, be ware that this method does not check
    /// if given chunks are all valid. For example, "IHDR" and "IEND" chunk can appear anywhere in
    /// given chunk sequence.
    pub fn from_chunks(chunks: Vec<Chunk>) -> PNG {
        PNG {
            signature: Self::STANDARD_HEADER,
            chunks,
        }
    }

    /// Append chunk to the tail of the png but before the IEND chunk.
    /// It assumes that the png being manipulated has IEND as its last chunk.
    pub fn append_chunk(&mut self, chunk: Chunk) {
        if let Some(last_chunk) = self.chunks.pop() {
            self.chunks.push(chunk);
            self.chunks.push(last_chunk);
        } else {
            self.chunks.push(chunk);
        }
    }

    /// Removes the first chunk that matches given `chunk_type`
    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        let chunk_type = ChunkType::from_str(chunk_type)?;
        let result = self
            .chunks
            .iter()
            .enumerate()
            .find(|(_, item)| item.chunk_type().eq(&chunk_type));
        match result {
            None => {}
            Some((index, _)) => {
                return Ok(self.chunks.remove(index));
            }
        }
        Err(anyhow!("No such type"))
    }

    pub fn header(&self) -> &[u8; 8] {
        &self.signature
    }

    pub fn chunks(&self) -> &[Chunk] {
        self.chunks.as_slice()
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        let chunk_type = ChunkType::from_str(chunk_type);
        if chunk_type.is_err() {
            return None;
        }
        let chunk_type = chunk_type.unwrap();
        for chunk in self.chunks.iter() {
            if chunk.chunk_type().eq(&chunk_type) {
                return Some(&chunk);
            }
        }
        None
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.signature
            .iter()
            .copied()
            .chain(self.chunks.iter().flat_map(|chunk| chunk.as_bytes()))
            .collect()
    }
}

impl TryFrom<&[u8]> for PNG {
    type Error = anyhow::Error;

    /// This implementation always tries to construct a chunk,
    /// doesn't care if it starts with a IHDR chunk and ends with a IEND chunk.
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);
        let mut signature: [u8; 8] = [0; 8];
        reader.read_exact(&mut signature)?;
        if !signature.eq(&PNG::STANDARD_HEADER) {
            return Err(anyhow!("Header signature does not match PNG spec"));
        }
        let mut len_four_bytes: [u8; 4] = [0; 4];
        let mut type_four_bytes: [u8; 4] = [0; 4];
        let mut crc_four_bytes: [u8; 4] = [0; 4];
        let mut chunks: Vec<Chunk> = Vec::new();
        while reader.fill_buf().map(|b| !b.is_empty())? {
            reader.read_exact(&mut len_four_bytes)?;
            reader.read_exact(&mut type_four_bytes)?;
            let length = u32::from_be_bytes(len_four_bytes);
            let chunk_type = ChunkType::try_from(type_four_bytes)?;
            let mut data: Vec<u8> = vec![0; length as usize];
            reader.read_exact(data.as_mut_slice())?;
            reader.read_exact(&mut crc_four_bytes)?;
            let crc = u32::from_be_bytes(crc_four_bytes);
            let chunk = Chunk::new(chunk_type, data);
            if chunk.length() != length {
                return Err(Self::Error::msg("Length does not match actual data size"));
            }
            if chunk.crc() != crc {
                return Err(Self::Error::msg("CRC check failed"));
            }
            chunks.push(chunk);
        }
        Ok(PNG { signature, chunks })
    }
}

impl Display for PNG {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_bytes())
    }
}
