use binrw::{BinRead, BinWrite, binrw, helpers::until_eof};

#[binrw]
#[br(little)]
#[bw(little)]
#[derive(Debug, Default, Clone)]
pub struct CircuitDataFile {
    pub version: u8,
    #[br(parse_with = until_eof, map = |bytes: Vec<u8>| snap::raw::Decoder::new().decompress_vec(&bytes).unwrap())]
    #[bw(try_map(|data| snap::raw::Encoder::new().compress_vec(data)))]
    pub data: Vec<u8>,
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

pub fn new_permament_id() -> i64 {
    rand::random()
}
