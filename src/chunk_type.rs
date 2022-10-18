use std::{str::FromStr, str::from_utf8, fmt::{Display, self}};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ChunkType {
    chunks: [u8;4]
}

impl TryFrom<[u8; 4]> for ChunkType{
    type Error = &'static str;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType { chunks: value })
    }
}

impl FromStr for ChunkType{
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 || !s.chars().all(|x| x.is_alphabetic()) {
            return Err("Invalid input");
        }
        Ok(ChunkType { chunks: s.as_bytes().try_into().unwrap() })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", from_utf8(&self.chunks).unwrap())
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.chunks
    }
    fn is_valid(&self) -> bool {
        self.chunks[2].is_ascii_uppercase()
    }
    fn is_critical(&self) -> bool {
        self.chunks[0].is_ascii_uppercase()
    }
    fn is_public(&self) -> bool {
        self.chunks[1].is_ascii_uppercase()
    }
    fn is_reserved_bit_valid(&self) -> bool {
        self.is_valid()
    }
    fn is_safe_to_copy(&self) -> bool {
        self.chunks[3].is_ascii_lowercase()
    }
}

#[test]
pub fn test_chunk_type_from_bytes() {
    let expected = [82, 117, 83, 116];
    let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

    assert_eq!(expected, actual.bytes());
}

#[test]
pub fn test_chunk_type_from_str() {
    let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
    let actual = ChunkType::from_str("RuSt").unwrap();
    assert_eq!(expected, actual);
}

#[test]
pub fn test_chunk_type_is_critical() {
    let chunk = ChunkType::from_str("RuSt").unwrap();
    assert!(chunk.is_critical());
}

#[test]
pub fn test_chunk_type_is_not_critical() {
    let chunk = ChunkType::from_str("ruSt").unwrap();
    assert!(!chunk.is_critical());
}

#[test]
pub fn test_chunk_type_is_public() {
    let chunk = ChunkType::from_str("RUSt").unwrap();
    assert!(chunk.is_public());
}

#[test]
pub fn test_chunk_type_is_not_public() {
    let chunk = ChunkType::from_str("RuSt").unwrap();
    assert!(!chunk.is_public());
}

#[test]
pub fn test_chunk_type_is_reserved_bit_valid() {
    let chunk = ChunkType::from_str("RuSt").unwrap();
    assert!(chunk.is_reserved_bit_valid());
}

#[test]
pub fn test_chunk_type_is_reserved_bit_invalid() {
    let chunk = ChunkType::from_str("Rust").unwrap();
    assert!(!chunk.is_reserved_bit_valid());
}

#[test]
pub fn test_chunk_type_is_safe_to_copy() {
    let chunk = ChunkType::from_str("RuSt").unwrap();
    assert!(chunk.is_safe_to_copy());
}

#[test]
pub fn test_chunk_type_is_unsafe_to_copy() {
    let chunk = ChunkType::from_str("RuST").unwrap();
    assert!(!chunk.is_safe_to_copy());
}

#[test]
pub fn test_valid_chunk_is_valid() {
    let chunk = ChunkType::from_str("RuSt").unwrap();
    assert!(chunk.is_valid());
}

#[test]
pub fn test_invalid_chunk_is_valid() {
    let chunk = ChunkType::from_str("Rust").unwrap();
    assert!(!chunk.is_valid());

    let chunk = ChunkType::from_str("Ru1t");
    assert!(chunk.is_err());
}

#[test]
pub fn test_chunk_type_string() {
    let chunk = ChunkType::from_str("RuSt").unwrap();
    assert_eq!(&chunk.to_string(), "RuSt");
}

#[test]
pub fn test_chunk_type_trait_impls() {
    let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
    let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
    let _chunk_string = format!("{}", chunk_type_1);
    let _are_chunks_equal = chunk_type_1 == chunk_type_2;
}
