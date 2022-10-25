use std::{
    fmt::{self, Display},
    io::{BufReader, Read},
    str::FromStr,
    string::FromUtf8Error,
};

use crate::chunk_type::{ChunkType, ChunkTypeError};
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}
#[derive(thiserror::Error, Debug, Clone)]
pub enum ChunkError {
    #[error("I/O Error.")]
    IOError(String),
    #[error("Invalid CRC sum.")]
    InvalidCrcSum,
    #[error("Invalid Chunk Type.")]
    InvalidChunkType(ChunkTypeError),
}

impl std::convert::From<std::io::Error> for ChunkError {
    fn from(err: std::io::Error) -> Self {
        match err {
            _ => ChunkError::IOError(err.to_string()),
        }
    }
}

impl std::convert::From<ChunkTypeError> for ChunkError {
    fn from(err: ChunkTypeError) -> Self {
        ChunkError::InvalidChunkType(err)
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::with_capacity(value.len() as usize, value);
        let mut length: [u8; 4] = [0; 4];
        let mut chunk_type: [u8; 4] = [0; 4];
        let mut crc_bytes: [u8; 4] = [0; 4];
        reader.read_exact(&mut length)?;
        let length = u32::from_be_bytes(length);
        let mut data: Vec<u8> = vec![0; length as usize];
        reader.read_exact(&mut chunk_type)?;
        reader.read_exact(&mut data)?;
        reader.read_exact(&mut crc_bytes)?;
        let chunk_type = ChunkType::try_from(chunk_type)?;
        let crc = u32::from_be_bytes(crc_bytes);
        let data_with_chunk_type: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.as_slice().iter())
            .copied()
            .collect();
        let calculated_crc = crc::crc32::checksum_ieee(data_with_chunk_type.as_slice());
        if crc != calculated_crc {
            return Err(ChunkError::InvalidCrcSum);
        }
        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8(self.data.clone()).unwrap())
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let chunk_type_bytes = chunk_type.bytes();
        let data_with_chunk_type: Vec<u8> = chunk_type_bytes
            .iter()
            .chain(data.as_slice().iter())
            .copied()
            .collect();
        Chunk {
            length: data.len() as u32,
            chunk_type: chunk_type,
            crc: crc::crc32::checksum_ieee(data_with_chunk_type.as_slice()),
            data,
        }
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

fn testing_chunk() -> Chunk {
    let data_length: u32 = 42;
    let chunk_type = "RuSt".as_bytes();
    let message_bytes = "This is where your secret message will be!".as_bytes();
    let crc: u32 = 2882656334;

    let chunk_data: Vec<u8> = data_length
        .to_be_bytes()
        .iter()
        .chain(chunk_type.iter())
        .chain(message_bytes.iter())
        .chain(crc.to_be_bytes().iter())
        .copied()
        .collect();

    Chunk::try_from(chunk_data.as_ref()).unwrap()
}

#[test]
fn test_new_chunk() {
    let chunk_type = ChunkType::from_str("RuSt").unwrap();
    let data = "This is where your secret message will be!"
        .as_bytes()
        .to_vec();
    let chunk = Chunk::new(chunk_type, data);
    assert_eq!(chunk.length(), 42);
    assert_eq!(chunk.crc(), 2882656334);
}

#[test]
fn test_chunk_length() {
    let chunk = testing_chunk();
    assert_eq!(chunk.length(), 42);
}

#[test]
fn test_chunk_type() {
    let chunk = testing_chunk();
    assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
}

#[test]
fn test_chunk_string() {
    let chunk = testing_chunk();
    let chunk_string = chunk.data_as_string().unwrap();
    let expected_chunk_string = String::from("This is where your secret message will be!");
    assert_eq!(chunk_string, expected_chunk_string);
}

#[test]
fn test_chunk_crc() {
    let chunk = testing_chunk();
    assert_eq!(chunk.crc(), 2882656334);
}

#[test]
fn test_valid_chunk_from_bytes() {
    let data_length: u32 = 42;
    let chunk_type = "RuSt".as_bytes();
    let message_bytes = "This is where your secret message will be!".as_bytes();
    let crc: u32 = 2882656334;

    let chunk_data: Vec<u8> = data_length
        .to_be_bytes()
        .iter()
        .chain(chunk_type.iter())
        .chain(message_bytes.iter())
        .chain(crc.to_be_bytes().iter())
        .copied()
        .collect();

    let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

    let chunk_string = chunk.data_as_string().unwrap();
    let expected_chunk_string = String::from("This is where your secret message will be!");

    assert_eq!(chunk.length(), 42);
    assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    assert_eq!(chunk_string, expected_chunk_string);
    assert_eq!(chunk.crc(), 2882656334);
}

#[test]
fn test_invalid_chunk_from_bytes() {
    let data_length: u32 = 42;
    let chunk_type = "RuSt".as_bytes();
    let message_bytes = "This is where your secret message will be!".as_bytes();
    let crc: u32 = 2882656333;

    let chunk_data: Vec<u8> = data_length
        .to_be_bytes()
        .iter()
        .chain(chunk_type.iter())
        .chain(message_bytes.iter())
        .chain(crc.to_be_bytes().iter())
        .copied()
        .collect();

    let chunk = Chunk::try_from(chunk_data.as_ref());

    assert!(chunk.is_err());
}

#[test]
pub fn test_chunk_trait_impls() {
    let data_length: u32 = 42;
    let chunk_type = "RuSt".as_bytes();
    let message_bytes = "This is where your secret message will be!".as_bytes();
    let crc: u32 = 2882656334;

    let chunk_data: Vec<u8> = data_length
        .to_be_bytes()
        .iter()
        .chain(chunk_type.iter())
        .chain(message_bytes.iter())
        .chain(crc.to_be_bytes().iter())
        .copied()
        .collect();

    let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

    let _chunk_string = format!("{}", chunk);
}
