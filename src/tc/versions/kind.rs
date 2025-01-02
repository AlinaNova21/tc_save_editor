use binrw::binrw;

#[binrw]
#[br(little,repr=u16)]
#[bw(little,repr=u16)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(u16)]
pub enum Kind {
    Off = 1,
    On = 2,
    NandBit = 3,
    Not = 4,
    AndBit3 = 5,
    OrBit = 6,
    NorBit = 7,
    XnorBit = 9,
    AndBit = 10,
    XorBit = 11,
    SwitchBit = 12,
    DelayBit = 13,
    MemoryBit = 14,
    AdderBit = 15,
    MakerBit8 = 16,
    SplitterBit8 = 17,
    Decoder1 = 43,
    Decoder2 = 44,
    Decoder3 = 45,
    Splitter2 = 47,
    DelayWord = 55,
    Custom = 78,
    Input = 79,
    Output = 81,
    Splitter4 = 99,
    Splitter8 = 100,
    SplitterBit2 = 109,
    SplitterBit4 = 110,
    MakerBit2 = 111,
    MakerBit4 = 112,
    Unmapped(u16),
    #[default]
    Unknown = u16::MAX,
}

impl Kind {
    pub fn is_memory(&self) -> bool {
        match self {
            // Kind::MemoryBit => true,
            _ => false,
        }
    }
    pub fn is_custom(&self) -> bool {
        match self {
            Kind::Custom => true,
            _ => false,
        }
    }
}

impl From<u16> for Kind {
    fn from(kind: u16) -> Self {
        match kind {
            1 => Kind::Off,
            2 => Kind::On,
            3 => Kind::NandBit,
            4 => Kind::Not,
            5 => Kind::AndBit3,
            6 => Kind::OrBit,
            7 => Kind::NorBit,
            9 => Kind::XnorBit,
            10 => Kind::AndBit,
            11 => Kind::XorBit,
            12 => Kind::SwitchBit,
            13 => Kind::DelayBit,
            14 => Kind::MemoryBit,
            15 => Kind::AdderBit,
            16 => Kind::MakerBit8,
            17 => Kind::SplitterBit8,
            43 => Kind::Decoder1,
            44 => Kind::Decoder2,
            45 => Kind::Decoder3,
            47 => Kind::Splitter2,
            55 => Kind::DelayWord,
            78 => Kind::Custom,
            79 => Kind::Input,
            81 => Kind::Output,
            99 => Kind::Splitter4,
            100 => Kind::Splitter8,
            109 => Kind::SplitterBit2,
            110 => Kind::SplitterBit4,
            111 => Kind::MakerBit2,
            112 => Kind::MakerBit4,
            _ => Kind::Unmapped(kind),
        }
    }
}

impl From<&Kind> for u16 {
    fn from(kind: &Kind) -> Self {
        kind.clone().into()
    }
}
impl From<Kind> for u16 {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Off => 1,
            Kind::On => 2,
            Kind::NandBit => 3,
            Kind::Not => 4,
            Kind::AndBit3 => 5,
            Kind::OrBit => 6,
            Kind::NorBit => 7,
            Kind::XnorBit => 9,
            Kind::AndBit => 10,
            Kind::XorBit => 11,
            Kind::SwitchBit => 12,
            Kind::DelayBit => 13,
            Kind::MemoryBit => 14,
            Kind::AdderBit => 15,
            Kind::MakerBit8 => 16,
            Kind::SplitterBit8 => 17,
            Kind::Decoder1 => 43,
            Kind::Decoder2 => 44,
            Kind::Decoder3 => 45,
            Kind::Splitter2 => 47,
            Kind::DelayWord => 55,
            Kind::Custom => 78,
            Kind::Input => 79,
            Kind::Output => 81,
            Kind::Splitter4 => 99,
            Kind::Splitter8 => 100,
            Kind::SplitterBit2 => 109,
            Kind::SplitterBit4 => 110,
            Kind::MakerBit2 => 111,
            Kind::MakerBit4 => 112,
            Kind::Unmapped(kind) => kind,
            Kind::Unknown => u16::MAX,
        }
    }
}
