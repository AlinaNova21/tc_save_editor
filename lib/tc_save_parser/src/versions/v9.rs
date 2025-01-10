use std::{default, io::Cursor};

use binrw::{BinRead, BinWrite, binrw, helpers::until};
use modular_bitfield::{BitfieldSpecifier, bitfield, prelude::B5};

use crate::{CDString, Point, kind::Kind};

#[binrw]
#[br(little)]
#[bw(little)]
#[derive(Debug, Clone, Default)]
pub struct CircuitData {
    pub custom_id: u64,
    pub hub_id: u32,
    pub gate: i64,
    pub delay: i64,
    #[bw(map = |b| (if *b { 1 } else { 0 }) as u8)]
    #[br(map = |b: u8| b != 0)]
    pub menu_visible: bool,
    pub clock_speed: u64,
    #[bw(try_calc(u16::try_from(dependencies.len())))]
    dependencies_len: u16,
    #[br(count = dependencies_len)]
    pub dependencies: Vec<i64>,
    pub description: CDString,
    pub camera_position: Point,
    pub synced: u8, // SyncState
    dummy0: u32,
    #[bw(try_calc(u16::try_from(player_data.len())))]
    player_data_len: u16,
    #[br(count = player_data_len)]
    pub player_data: Vec<u8>,
    #[bw(try_calc(u64::try_from(components.len() as u64)))]
    components_len: u64,
    #[br(count = components_len)]
    pub components: Vec<Component>,
    #[bw(try_calc(u64::try_from(wires.len() as u64)))]
    wires_len: u64,
    #[br(count = wires_len)]
    pub wires: Vec<Wire>,
}

impl CircuitData {
    pub fn get_bytes(&self) -> Vec<u8> {
        let mut buf = Cursor::new(vec![0u8; 8192]);
        self.write(&mut buf).unwrap();
        let len = buf.position();
        buf.into_inner()[..len as usize].to_vec()
    }
}

#[binrw]
#[br(little)]
#[bw(little)]
#[derive(Debug, Default, Clone)]
pub struct Component {
    pub kind: Kind,
    pub position: Point,
    pub rotation: u8,
    pub permanent_id: u64,
    pub custom_string: CDString,
    #[bw(try_calc(u16::try_from(settings.len())))]
    settings_len: u16,
    #[br(count = settings_len)]
    pub settings: Vec<u64>,
    pub buffer_size: i64,
    pub ui_order: i16,
    pub word_size: i64,
    // dummy0: i64,
    #[bw(if(kind.has_linked_components()))]
    #[br(if(kind.has_linked_components()))]
    pub linked_components: LinkedComponents,
    #[bw(try_calc(u16::try_from(watched_components.len())))]
    watched_components_len: u16,
    #[br(count = watched_components_len)]
    pub watched_components: Vec<WatchedComponent>,
    #[bw(try_calc(u16::try_from(selected_programs.len())))]
    selected_programs_len: u16,
    #[br(count = selected_programs_len)]
    pub selected_programs: Vec<SelectedProgram>,
    #[bw(if(kind.is_custom()))]
    #[br(if(kind.is_custom()))]
    pub custom: CustomInfo,
    // pub assembler_info: AssemblerInfo,
}

// Hex View  00 01 02 03 04 05 06 07  08 09 0A 0B 0C 0D 0E 0F

// 00000090                           02 00 00 00 00 00 00 00          ........
// 000000A0  00 00 00 00 00 00 00 00  00 00 00 00 D8 FD F7 EC  ................
// 000000B0  C8 49 E4 7A 00 00 00 00  00 00 00 00 05 00 72 65  .I.z..........re
// 000000C0  67 20 30 01 00 07 00 73  61 6E 64 62 6F 78 17 00  g 0....sandbox..
// 000000D0  73 61 6E 64 62 6F 78 2F  6E 65 77 5F 70 72 6F 67  sandbox/new_prog
// 000000E0  72 61 6D 2E 61 73 6D 03  00 FC FF 15 00 00 42 F8  ram.asm.......B.
// 000000F0  40 A8 0F 60 52 7F 00 00  00 00 00 00 00 00 00 00  @..`R...........
// 00000100  00 00 00 00                                      ....

#[binrw]
#[br(little)]
#[bw(little)]
#[derive(Debug, Default, Clone)]
pub struct CustomInfo {
    pub id: u64,
    #[bw(try_calc(u16::try_from(explicit_word_sizes.len())))]
    pub explicit_word_sizes_len: u16,
    #[br(count = explicit_word_sizes_len)]
    pub explicit_word_sizes: Vec<ExplicitWordSize>,
}

#[binrw]
#[br(little)]
#[bw(little)]
#[derive(Debug, Default, Clone)]
pub struct SelectedProgram {
    level: CDString,
    program: CDString,
}

#[binrw]
#[br(little)]
#[bw(little)]
#[derive(Debug, Default, Clone)]
pub struct WatchedComponent {
    permanent_id: u64,
    inner_id: u64,
    name: CDString,
}

#[binrw]
#[br(little)]
#[bw(little)]
#[derive(Debug, Default, Clone)]
pub struct AssemblerInfo {
    #[bw(try_calc(u16::try_from(watched_components.len())))]
    watched_components_len: u16,
    #[br(count = watched_components_len)]
    pub watched_components: Vec<WatchedComponent>,

    #[bw(try_calc(u16::try_from(selected_programs.len())))]
    selected_programs_len: u16,
    #[br(count = selected_programs_len)]
    pub selected_programs: Vec<SelectedProgram>,
}

#[binrw]
#[br(little)]
#[bw(little)]
#[derive(Debug, Default, Clone)]
pub struct LinkedComponents {
    #[bw(try_calc(u16::try_from(linked_components.len())))]
    linked_components_len: u16,
    #[br(count = linked_components_len)]
    pub linked_components: Vec<u64>,
    #[bw(calc = 0)]
    #[br(temp)]
    #[unused]
    _dummy0: u64,
}

#[binrw]
#[br(little)]
#[bw(little)]
#[derive(Debug, Clone)]
pub struct ExplicitWordSize {
    pub a: i64,
    pub b: i64,
}

#[binrw]
#[br(little)]
#[bw(little)]
#[derive(Debug, Clone)]
pub struct Wire {
    pub color: u8,
    pub comment: CDString,
    pub start: Point,
    #[br(parse_with = until(|v: &WireSegment| v.direction() == WireDirection::Right && v.length() == 0))]
    pub segments: Vec<WireSegment>,
}

#[bitfield(bits = 8)]
#[derive(BitfieldSpecifier, Debug, Clone, Copy, Default)]
pub struct WireSegment {
    pub length: B5,
    #[bits = 3]
    pub direction: WireDirection,
}

impl BinRead for WireSegment {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let mut buf = [0u8; 1];
        reader
            .read_exact(&mut buf)
            .expect("Failed to read WireSegment");
        Ok(WireSegment::from_bytes(buf))
    }
}

impl BinWrite for WireSegment {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        let buf = WireSegment::into_bytes(*self);
        writer.write_all(&buf).expect("Failed to write WireSegment");
        Ok(())
    }
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy, PartialEq)]
#[bits = 3]
#[repr(u8)]
pub enum WireDirection {
    Right = 0,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    Up,
    UpRight,
}

pub struct WireBuilder(Wire);

impl WireBuilder {
    pub fn new(start: Point) -> Self {
        Self(Wire {
            color: 0,
            comment: CDString::default(),
            start,
            segments: vec![],
        })
    }
    pub fn comment(mut self, comment: &str) -> Self {
        self.0.comment = CDString::from(comment);
        self
    }
    pub fn color(mut self, color: u8) -> Self {
        self.0.color = color;
        self
    }
    pub fn go(mut self, dir: WireDirection, length: u8) -> Self {
        self.0
            .segments
            .push(WireSegment::new().with_direction(dir).with_length(length));
        self
    }
    pub fn right(self, length: u8) -> Self {
        self.go(WireDirection::Right, length)
    }
    pub fn down_right(self, length: u8) -> Self {
        self.go(WireDirection::DownRight, length)
    }
    pub fn down(self, length: u8) -> Self {
        self.go(WireDirection::Down, length)
    }
    pub fn down_left(self, length: u8) -> Self {
        self.go(WireDirection::DownLeft, length)
    }
    pub fn left(self, length: u8) -> Self {
        self.go(WireDirection::Left, length)
    }
    pub fn up_left(self, length: u8) -> Self {
        self.go(WireDirection::UpLeft, length)
    }
    pub fn up(self, length: u8) -> Self {
        self.go(WireDirection::Up, length)
    }
    pub fn up_right(self, length: u8) -> Self {
        self.go(WireDirection::UpRight, length)
    }
    pub fn build(self) -> Wire {
        self.0
    }
}
