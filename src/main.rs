mod args;
mod chunk;
mod chunk_type;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
use clap::Parser;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
use std::str::FromStr;

use crate::args::{
    Cli,
    Commands::{Decode, Encode, Print, Remove},
};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

fn read_path(path: &Option<OsString>) -> &str {
    path.as_deref()
        .unwrap_or_else(|| OsStr::new(""))
        .to_str()
        .unwrap()
}

fn read_chunk_type(chunk_type: &Option<OsString>) -> &str {
    chunk_type
        .as_deref()
        .unwrap_or_else(|| OsStr::new(""))
        .to_str()
        .unwrap()
}

fn read_message(message: Option<OsString>) -> Vec<u8> {
    message
        .as_deref()
        .unwrap_or_else(|| OsStr::new(""))
        .to_str()
        .unwrap()
        .as_bytes()
        .to_vec()
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Encode {
            path,
            chunk,
            message,
        } => {
            let path = read_path(&path);
            let chunk_type = read_chunk_type(&chunk);
            let message = read_message(message);
            let data = match fs::read(path) {
                Ok(data) => data,
                Err(_) => {
                    panic!("Failed to read file with path: {}", path)
                }
            };

            let mut png = Png::try_from(data.as_ref()).expect("Failed to read png file");
            let chunk_type = match ChunkType::from_str(chunk_type) {
                Ok(data) => data,
                Err(_) => panic!("Invalid chunk type"),
            };
            png.append_chunk(Chunk::new(chunk_type, message));
            fs::write(path, png.as_bytes()).expect("Failed to write to file");
        }
        Decode { path, chunk } => {
            let path = read_path(&path);
            let chunk_type = read_chunk_type(&chunk);

            let data = match fs::read(path) {
                Ok(data) => data,
                Err(_) => panic!("Failed to read file"),
            };

            let png = Png::try_from(data.as_ref()).expect("Failed to read png file");
            let decoded_chunk = match png.chunk_by_type(chunk_type) {
                Some(data) => data,
                None => panic!("Chunk {} does not exists in this file", chunk_type),
            };
            print!(
                "Chunk: {} has secret message: {}",
                chunk_type, decoded_chunk
            )
        }
        Remove { path, chunk } => {
            let path = read_path(&path);
            let chunk_type = read_chunk_type(&chunk);

            let data = match fs::read(path) {
                Ok(data) => data,
                Err(_) => panic!("Failed to read file"),
            };
            let mut png = Png::try_from(data.as_ref()).expect("Failed to read png file");
            match png.remove_chunk(chunk_type) {
                Ok(chunk) => print!(
                    "deleted chunk: {} with secret message: {}",
                    chunk_type, chunk
                ),
                Err(_) => print!("Can't delete chunk"),
            }
            fs::write(path, png.as_bytes()).expect("Failed to write to file");
        }
        Print { path } => {
            let path = read_path(&path);

            let data = match fs::read(path) {
                Ok(data) => data,
                Err(_) => panic!("Failed to read file"),
            };
            let png = Png::try_from(data.as_ref()).expect("Failed to read png file");

            println!("{}", png)
        }
    }
}
