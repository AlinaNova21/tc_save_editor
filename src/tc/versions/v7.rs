use binrw::{BinRead, BinWrite, binrw, helpers::until};

use super::{CDString, Point, kind::Kind};

// #[derive(Debug, Clone)]
// enum SyncState {
//     Synced = 0,
//     Unsynced = 1,
// }

// static MEMORY_KINDS: [u16; 1] = [u16::from(Kind::Register1)];

#[binrw]
#[br(little)]
#[bw(little)]
#[derive(Debug, Clone, Default)]
pub struct CircuitData {
    pub custom_id: i64,
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
    dummy0: u16,
    // dummy0: u32,
    #[bw(try_calc(u16::try_from(player_data.len())))]
    player_data_len: u16,
    #[br(count = player_data_len)]
    pub player_data: Vec<u8>,
    pub hub_description: CDString,
    #[bw(try_calc(u64::try_from(components.len() as u64)))]
    components_len: u64,
    #[br(count = components_len)]
    pub components: Vec<Component>,
    #[bw(try_calc(u64::try_from(wires.len() as u64)))]
    wires_len: u64,
    #[br(count = wires_len)]
    pub wires: Vec<Wire>,
}

#[binrw]
#[br(little)]
#[bw(little)]
#[derive(Debug, Default, Clone)]
pub struct Component {
    // #[bw(try_calc(u16::from(kind.clone())))]
    // #[br(map = |k:u16| Kind::from(k))]
    pub kind: Kind,
    pub position: Point,
    pub rotation: u8,
    pub permanent_id: i64,
    pub custom_string: CDString,
    #[bw(try_calc(u16::try_from(settings.len())))]
    settings_len: u16,
    #[br(count = settings_len)]
    pub settings: Vec<u64>,
    pub buffer_size: i64,
    pub ui_order: i16,
    pub word_size: i64,
    dummy0: i64,
    // dummy0: u16,
    #[bw(if(kind.is_custom()))]
    #[br(if(kind.is_custom()))]
    pub custom: CustomInfo,
    #[bw(if(kind.is_memory()))]
    #[br(if(kind.is_memory()))]
    pub memory: MemoryInfo,
}

#[binrw]
#[br(little)]
#[bw(little)]
#[derive(Debug, Default, Clone)]
pub struct CustomInfo {
    pub id: i64,
    #[bw(try_calc(u16::try_from(explicit_word_sizes.len())))]
    pub explicit_word_sizes_len: u16,
    #[br(count = explicit_word_sizes_len)]
    pub explicit_word_sizes: Vec<ExplicitWordSize>,
    dummy0: u16,
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
    permanent_id: i64,
    inner_id: i64,
    name: CDString,
}

#[binrw]
#[br(little)]
#[bw(little)]
#[derive(Debug, Default, Clone)]
pub struct MemoryInfo {
    #[bw(try_calc(u16::try_from(selected_programs.len())))]
    selected_programs_len: u16,
    #[br(count = selected_programs_len)]
    pub selected_programs: Vec<SelectedProgram>,
    #[bw(try_calc(u16::try_from(watched_components.len())))]
    watched_components_len: u16,
    #[br(count = watched_components_len)]
    pub watched_components: Vec<WatchedComponent>,
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
    #[br(parse_with = until(|v| *v == 0))]
    pub segments: Vec<u8>,
}
