use binrw::binrw;

#[binrw]
#[br(little,repr=u16)]
#[bw(little,repr=u16)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(u16)]
pub enum Kind {
    None = 0,
    Off = 1,
    On = 2,
    NotBit = 3,
    AndBit = 4,
    And3Bit = 5,
    NandBit = 6,
    OrBit = 7,
    Or3Bit = 8,
    NorBit = 9,
    XorBit = 10,
    XnorBit = 11,
    SwitchBit = 12,
    DelayLineBit = 13,
    RegisterBit = 14,
    FullAdder = 15,
    MakerBit8 = 16,
    SplitterBit8 = 17,
    NotWord = 18,
    OrWord = 19,
    AndWord = 20,
    NandWord = 21,
    NorWord = 22,
    XorWord = 23,
    XnorWord = 24,
    SwitchWord = 25,
    Equal = 26,
    LessU = 27,
    LessS = 28,
    Neg = 29,
    Add = 30,
    Mul = 31,
    Div = 32,
    Lsl = 33,
    Lsr = 34,
    Rol = 35,
    Ror = 36,
    Asr = 37,
    Counter = 38,
    RegisterWord = 39,
    ImmRegisterWord = 40,
    ImmDelayLineBit = 41,
    Mux = 42,
    Decoder1 = 43,
    Decoder2 = 44,
    Decoder3 = 45,
    Constant = 46,
    SplitterWord2 = 47,
    MakerWord2 = 48,
    FrontPanel = 49,
    Assembler = 50,
    Ssd = 51,
    Ram = 52,
    RamLatency = 53,
    RamFast = 54,
    DelayLineWord = 55,
    RamDualLoad = 56,
    FileLoader = 57,
    CcLevelOutput = 58,
    LevelGate = 59,
    LevelInput1 = 60,
    LevelInputWord = 61,
    LevelInputSwitched = 62,
    LevelInput2Pin = 63,
    LevelInput3Pin = 64,
    LevelInput4Pin = 65,
    LevelInputCustom = 66,
    LevelInputArch = 67,
    LevelOutput1 = 68,
    LevelOutputWord = 69,
    LevelOutputSwitched = 70,
    LevelOutput1Sum = 71,
    LevelOutput1Car = 72,
    LevelOutput2Pin = 73,
    LevelOutput3Pin = 74,
    LevelOutput4Pin = 75,
    LevelOutputArch = 76,
    LevelOutputCounter = 77,
    Custom = 78,
    CcInput = 79,
    CcInputBuffer = 80,
    CcOutput = 81,
    ProbeMemoryBit = 82,
    ProbeMemoryWord = 83,
    ProbeWireBit = 84,
    ProbeWireWord = 85,
    ConfigDelay = 86,
    Halt = 87,
    Console = 88,
    SegmentDisplay = 89,
    StaticValue = 90,
    PixelScreen = 91,
    Time = 92,
    Keyboard = 93,
    StaticEval = 94,
    VerilogInput = 95,
    VerilogOutput = 96,
    MakerWord4 = 97,
    MakerWord8 = 98,
    SplitterWord4 = 99,
    SplitterWord8 = 100,
    StaticIndexer = 101,
    ImmProbeMemoryBit = 102,
    ImmDelayLineWord = 103,
    Inc = 104,
    CcLevelInputCustom = 105,
    CcLevelInput = 106,
    ImmRegisterBit = 107,
    Mod = 108,
    SplitterBit2 = 109,
    SplitterBit4 = 110,
    MakerBit2 = 111,
    MakerBit4 = 112,
    ImmProbeMemoryWord = 113,
    Concatenator2 = 114,
    Concatenator4 = 115,
    Concatenator8 = 116,
    StaticIndexerConfig = 117,
    Rom = 118,
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
            3 => Kind::NotBit,
            4 => Kind::AndBit,
            5 => Kind::And3Bit,
            6 => Kind::NandBit,
            7 => Kind::OrBit,
            8 => Kind::Or3Bit,
            9 => Kind::NorBit,
            10 => Kind::XorBit,
            11 => Kind::XnorBit,
            12 => Kind::SwitchBit,
            13 => Kind::DelayLineBit,
            14 => Kind::RegisterBit,
            15 => Kind::FullAdder,
            16 => Kind::MakerBit8,
            17 => Kind::SplitterBit8,
            18 => Kind::NotWord,
            19 => Kind::OrWord,
            20 => Kind::AndWord,
            21 => Kind::NandWord,
            22 => Kind::NorWord,
            23 => Kind::XorWord,
            24 => Kind::XnorWord,
            25 => Kind::SwitchWord,
            26 => Kind::Equal,
            27 => Kind::LessU,
            28 => Kind::LessS,
            29 => Kind::Neg,
            30 => Kind::Add,
            31 => Kind::Mul,
            32 => Kind::Div,
            33 => Kind::Lsl,
            34 => Kind::Lsr,
            35 => Kind::Rol,
            36 => Kind::Ror,
            37 => Kind::Asr,
            38 => Kind::Counter,
            39 => Kind::RegisterWord,
            40 => Kind::ImmRegisterWord,
            41 => Kind::ImmDelayLineBit,
            42 => Kind::Mux,
            43 => Kind::Decoder1,
            44 => Kind::Decoder2,
            45 => Kind::Decoder3,
            46 => Kind::Constant,
            47 => Kind::SplitterWord2,
            48 => Kind::MakerWord2,
            49 => Kind::FrontPanel,
            50 => Kind::Assembler,
            51 => Kind::Ssd,
            52 => Kind::Ram,
            53 => Kind::RamLatency,
            54 => Kind::RamFast,
            55 => Kind::DelayLineWord,
            56 => Kind::RamDualLoad,
            57 => Kind::FileLoader,
            58 => Kind::CcLevelOutput,
            59 => Kind::LevelGate,
            60 => Kind::LevelInput1,
            61 => Kind::LevelInputWord,
            62 => Kind::LevelInputSwitched,
            63 => Kind::LevelInput2Pin,
            64 => Kind::LevelInput3Pin,
            65 => Kind::LevelInput4Pin,
            66 => Kind::LevelInputCustom,
            67 => Kind::LevelInputArch,
            68 => Kind::LevelOutput1,
            69 => Kind::LevelOutputWord,
            70 => Kind::LevelOutputSwitched,
            71 => Kind::LevelOutput1Sum,
            72 => Kind::LevelOutput1Car,
            73 => Kind::LevelOutput2Pin,
            74 => Kind::LevelOutput3Pin,
            75 => Kind::LevelOutput4Pin,
            76 => Kind::LevelOutputArch,
            77 => Kind::LevelOutputCounter,
            78 => Kind::Custom,
            79 => Kind::CcInput,
            80 => Kind::CcInputBuffer,
            81 => Kind::CcOutput,
            82 => Kind::ProbeMemoryBit,
            83 => Kind::ProbeMemoryWord,
            84 => Kind::ProbeWireBit,
            85 => Kind::ProbeWireWord,
            86 => Kind::ConfigDelay,
            87 => Kind::Halt,
            88 => Kind::Console,
            89 => Kind::SegmentDisplay,
            90 => Kind::StaticValue,
            91 => Kind::PixelScreen,
            92 => Kind::Time,
            93 => Kind::Keyboard,
            94 => Kind::StaticEval,
            95 => Kind::VerilogInput,
            96 => Kind::VerilogOutput,
            97 => Kind::MakerWord4,
            98 => Kind::MakerWord8,
            99 => Kind::SplitterWord4,
            100 => Kind::SplitterWord8,
            101 => Kind::StaticIndexer,
            102 => Kind::ImmProbeMemoryBit,
            103 => Kind::ImmDelayLineWord,
            104 => Kind::Inc,
            105 => Kind::CcLevelInputCustom,
            106 => Kind::CcLevelInput,
            107 => Kind::ImmRegisterBit,
            108 => Kind::Mod,
            109 => Kind::SplitterBit2,
            110 => Kind::SplitterBit4,
            111 => Kind::MakerBit2,
            112 => Kind::MakerBit4,
            113 => Kind::ImmProbeMemoryWord,
            114 => Kind::Concatenator2,
            115 => Kind::Concatenator4,
            116 => Kind::Concatenator8,
            117 => Kind::StaticIndexerConfig,
            118 => Kind::Rom,
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
            Kind::None => 0,
            Kind::Off => 1,
            Kind::On => 2,
            Kind::NotBit => 3,
            Kind::AndBit => 4,
            Kind::And3Bit => 5,
            Kind::NandBit => 6,
            Kind::OrBit => 7,
            Kind::Or3Bit => 8,
            Kind::NorBit => 9,
            Kind::XorBit => 10,
            Kind::XnorBit => 11,
            Kind::SwitchBit => 12,
            Kind::DelayLineBit => 13,
            Kind::RegisterBit => 14,
            Kind::FullAdder => 15,
            Kind::MakerBit8 => 16,
            Kind::SplitterBit8 => 17,
            Kind::NotWord => 18,
            Kind::OrWord => 19,
            Kind::AndWord => 20,
            Kind::NandWord => 21,
            Kind::NorWord => 22,
            Kind::XorWord => 23,
            Kind::XnorWord => 24,
            Kind::SwitchWord => 25,
            Kind::Equal => 26,
            Kind::LessU => 27,
            Kind::LessS => 28,
            Kind::Neg => 29,
            Kind::Add => 30,
            Kind::Mul => 31,
            Kind::Div => 32,
            Kind::Lsl => 33,
            Kind::Lsr => 34,
            Kind::Rol => 35,
            Kind::Ror => 36,
            Kind::Asr => 37,
            Kind::Counter => 38,
            Kind::RegisterWord => 39,
            Kind::ImmRegisterWord => 40,
            Kind::ImmDelayLineBit => 41,
            Kind::Mux => 42,
            Kind::Decoder1 => 43,
            Kind::Decoder2 => 44,
            Kind::Decoder3 => 45,
            Kind::Constant => 46,
            Kind::SplitterWord2 => 47,
            Kind::MakerWord2 => 48,
            Kind::FrontPanel => 49,
            Kind::Assembler => 50,
            Kind::Ssd => 51,
            Kind::Ram => 52,
            Kind::RamLatency => 53,
            Kind::RamFast => 54,
            Kind::DelayLineWord => 55,
            Kind::RamDualLoad => 56,
            Kind::FileLoader => 57,
            Kind::CcLevelOutput => 58,
            Kind::LevelGate => 59,
            Kind::LevelInput1 => 60,
            Kind::LevelInputWord => 61,
            Kind::LevelInputSwitched => 62,
            Kind::LevelInput2Pin => 63,
            Kind::LevelInput3Pin => 64,
            Kind::LevelInput4Pin => 65,
            Kind::LevelInputCustom => 66,
            Kind::LevelInputArch => 67,
            Kind::LevelOutput1 => 68,
            Kind::LevelOutputWord => 69,
            Kind::LevelOutputSwitched => 70,
            Kind::LevelOutput1Sum => 71,
            Kind::LevelOutput1Car => 72,
            Kind::LevelOutput2Pin => 73,
            Kind::LevelOutput3Pin => 74,
            Kind::LevelOutput4Pin => 75,
            Kind::LevelOutputArch => 76,
            Kind::LevelOutputCounter => 77,
            Kind::Custom => 78,
            Kind::CcInput => 79,
            Kind::CcInputBuffer => 80,
            Kind::CcOutput => 81,
            Kind::ProbeMemoryBit => 82,
            Kind::ProbeMemoryWord => 83,
            Kind::ProbeWireBit => 84,
            Kind::ProbeWireWord => 85,
            Kind::ConfigDelay => 86,
            Kind::Halt => 87,
            Kind::Console => 88,
            Kind::SegmentDisplay => 89,
            Kind::StaticValue => 90,
            Kind::PixelScreen => 91,
            Kind::Time => 92,
            Kind::Keyboard => 93,
            Kind::StaticEval => 94,
            Kind::VerilogInput => 95,
            Kind::VerilogOutput => 96,
            Kind::MakerWord4 => 97,
            Kind::MakerWord8 => 98,
            Kind::SplitterWord4 => 99,
            Kind::SplitterWord8 => 100,
            Kind::StaticIndexer => 101,
            Kind::ImmProbeMemoryBit => 102,
            Kind::ImmDelayLineWord => 103,
            Kind::Inc => 104,
            Kind::CcLevelInputCustom => 105,
            Kind::CcLevelInput => 106,
            Kind::ImmRegisterBit => 107,
            Kind::Mod => 108,
            Kind::SplitterBit2 => 109,
            Kind::SplitterBit4 => 110,
            Kind::MakerBit2 => 111,
            Kind::MakerBit4 => 112,
            Kind::ImmProbeMemoryWord => 113,
            Kind::Concatenator2 => 114,
            Kind::Concatenator4 => 115,
            Kind::Concatenator8 => 116,
            Kind::StaticIndexerConfig => 117,
            Kind::Rom => 118,
            Kind::Unmapped(kind) => kind,
            Kind::Unknown => u16::MAX,
        }
    }
}
