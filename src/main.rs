extern crate core;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod tests;

use std::fs::{File, OpenOptions, Permissions};
use std::io::{BufWriter, Read, Seek, Write};
use std::str::FromStr;
use crate::args::*;
use anyhow::Result;
use clap::{Parser};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::PNG;

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();
    println!("{:?} file: {:?}", cli.command, cli.png);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .open(cli.png.as_path())?;
    let mut data: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize);
    file.read_to_end(&mut data)?;
    file.rewind()?;
    let mut png = PNG::try_from(data.as_slice())?;
    match cli.command {
        Command::Encode { chunk_type, message, output } => {
            let msg_chunk_type = ChunkType::from_str(&chunk_type)?;
            let msg_chunk = Chunk::new(msg_chunk_type, message.into_bytes());
            png.append_chunk(msg_chunk);
            if let Some(output_path) = output {
                let output_file = File::create(output_path)?;
                let mut writer = BufWriter::new(output_file);
                writer.write_all(png.as_bytes().as_slice())?;
            } else {
                println!("Trying to overwrite original file: {:?}", cli.png.as_path().canonicalize()?);
                file.write_all(png.as_bytes().as_slice())?;
            }
        }
        Command::Decode { chunk_type } => {
            if let Some(msg_chunk) = png.chunk_by_type(&chunk_type) {
                println!("{}", String::from_utf8(msg_chunk.data().into())?);
            }
        }
        Command::Remove { chunk_type } => {
            let _msg_chunk = png.remove_chunk(&chunk_type)?;
            let bytes = png.as_bytes();
            file.set_len(bytes.len() as u64)?;
            file.write_all(bytes.as_slice())?;
            println!("One message of type {} has been removed", chunk_type);
        }
        Command::Print => {
            println!("{:?}", data);
        }
    }
    Ok(())
}
