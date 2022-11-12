mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
use clap::Parser;
use std::ffi::OsStr;
use std::fs;
use std::str::FromStr;


use crate::args::{Cli, Commands::{Encode, Decode, Remove, Print}};
use crate::png::Png;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

fn main() {
    let args = Cli::parse();

    match args.command {
        Encode { path, chunk, message } => {
            let path = path.as_deref().unwrap_or_else(|| OsStr::new(""));
            let chunk_type = chunk.as_deref().unwrap().to_str().unwrap();
            let message = message.as_deref().unwrap().to_str().unwrap().as_bytes().to_vec();
            let data = match fs::read(path) {
                Ok(data) => data,
                Err(_) => {
                    fs::write(path, Png::STANDARD_HEADER);
                    fs::read(path).unwrap()
                }
            };

            let mut png = Png::try_from(data.as_ref()).unwrap();
            png.append_chunk(Chunk::new(ChunkType::from_str(chunk_type).unwrap(), message));
        },
        Decode { path, chunk } => {
            let path = path.as_deref().unwrap_or_else(|| OsStr::new(""));
            let chunk_type = chunk.as_deref().unwrap().to_str().unwrap();

            let data = match fs::read(path) {
                Ok(data) => data,
                Err(_) => panic!("File does not exists!")
            };

            let png = Png::try_from(data.as_ref()).unwrap();
            print!("{:}", png.chunk_by_type(chunk_type).unwrap())
        }
        Remove { path, chunk } => {
            let path = path.as_deref().unwrap_or_else(|| OsStr::new(""));
            let chunk_type = chunk.as_deref().unwrap().to_str().unwrap();

            let data = match fs::read(path) {
                Ok(data) => data,
                Err(_) => panic!("File does not exists!")
            };
            let mut png = Png::try_from(data.as_ref()).unwrap();
            match png.remove_chunk(chunk_type) {
                Ok(chunk) => print!("{:?}", chunk),
                Err(_) => print!("Can't delete chunk")
            }
        },
        Print { path } => {
            let path = path.as_deref().unwrap_or_else(|| OsStr::new(""));

            let data = match fs::read(path) {
                Ok(data) => data,
                Err(_) => panic!("File does not exists!")
            };
            let png = Png::try_from(data.as_ref()).unwrap();

            println!("{}", png)
        },
    }
}
