use std::io::{Cursor, Read, Seek};

use binrw::{BinRead, BinResult, BinWrite, binrw, parser, writer};

use crate::{v7, v8, v9};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Binrw(binrw::Error),
    UnsupportedVersion(u8, Vec<u8>),
}

impl From<binrw::Error> for Error {
    fn from(e: binrw::Error) -> Self {
        Self::Binrw(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug, Clone)]
pub enum CircuitDataVersion {
    // V7(v7::CircuitData),
    V8(v8::CircuitData),
    V9(v9::CircuitData),
    Unknown(Vec<u8>),
}

impl Default for CircuitDataVersion {
    fn default() -> Self {
        self::CircuitDataVersion::V9(v9::CircuitData::default())
    }
}

impl CircuitDataVersion {
    #[parser(reader)]
    fn parse(version: u8) -> BinResult<Self> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let data = snap::raw::Decoder::new().decompress_vec(&data).unwrap();
        let mut cursor = Cursor::new(&data);
        match version {
            // 7 => Ok(Self::V7(v7::CircuitData::read(&mut cursor)?)),
            8 => Ok(Self::V8(v8::CircuitData::read(&mut cursor)?)),
            9 => Ok(Self::V9(v9::CircuitData::read(&mut cursor)?)),
            _ => Ok(Self::Unknown(data)),
        }
    }

    #[writer(writer)]
    fn write(&self) -> BinResult<()> {
        let data = match self {
            // Self::V7(data) => data.get_bytes(),
            Self::V8(data) => data.get_bytes(),
            Self::V9(data) => data.get_bytes(),
            Self::Unknown(data) => data.clone(),
        };
        let data = snap::raw::Encoder::new().compress_vec(&data).unwrap();
        writer.write_all(&data)?;
        Ok(())
    }
}

#[binrw]
#[br(little)]
#[bw(little)]
#[derive(Debug, Default, Clone)]
pub struct CircuitDataFile {
    pub version: u8,
    #[br(parse_with = CircuitDataVersion::parse, args (version))]
    #[bw(write_with = CircuitDataVersion::write)]
    pub circuit: CircuitDataVersion,
}

impl CircuitDataFile {
    pub fn load(path: &str) -> Result<Self, Error> {
        let mut fh = std::fs::File::open(path)?;
        let cdf = Self::read(&mut fh)?;
        Ok(cdf)
    }

    pub fn save(&self, path: &str) -> Result<(), Error> {
        let mut fh = std::fs::File::create(path)?;
        self.write(&mut fh)?;
        Ok(())
    }

    pub fn debug_dump(path: &str) -> Result<Vec<u8>, Error> {
        let mut fh = std::fs::File::open(path)?;
        let mut data = Vec::new();
        fh.seek(std::io::SeekFrom::Start(1))?;
        fh.read_to_end(&mut data)?;
        let data = snap::raw::Decoder::new().decompress_vec(&data).unwrap();
        Ok(data)
    }
}

#[binrw]
#[br(little)]
#[bw(little)]
#[derive(Debug, Clone, Default)]
pub struct CDString {
    #[bw(try_calc(u16::try_from(value.len())))]
    len: u16,
    #[br(count = len)]
    #[br(map = |s: Vec<u8>| String::from_utf8_lossy(&s).to_string())]
    #[bw(map = |s| s.clone().into_bytes())]
    pub value: String,
}

impl From<&str> for CDString {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

#[derive(BinRead, BinWrite, Debug, Default, Clone)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

pub fn new_permament_id() -> u64 {
    rand::random()
}
