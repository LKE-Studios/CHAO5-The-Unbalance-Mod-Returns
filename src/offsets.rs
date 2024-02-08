extern "C" {
    #[link_name = "offsets_get_battle_object_from_id"]
    fn offsets_get_battle_object_from_id() -> usize;

    #[link_name = "offsets_map_controls"]
    fn offsets_map_controls() -> usize;

    #[link_name = "offsets_analog_trigger_l"]
    fn offsets_analog_trigger_l() -> usize;

    #[link_name = "offsets_analog_trigger_r"]
    fn offsets_analog_trigger_r() -> usize;
}

pub fn get_battle_object_from_id() -> usize {
    unsafe {
        offsets_get_battle_object_from_id()
    }
}

pub fn map_controls() -> usize {
    unsafe {
        offsets_map_controls()
    }
}

pub fn analog_trigger_l() -> usize {
    unsafe {
        offsets_analog_trigger_l()
    }
}

pub fn analog_trigger_r() -> usize {
    unsafe {
        offsets_analog_trigger_r()
    }
}

#[cfg(not(feature = "no-offset-search"))]
mod offsets_impl {
    pub fn offset_to_addr<T>(offset: usize) -> *const T {
        unsafe {
            (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8).add(offset) as _
        }
    }
    use crate::utils::byte_search;
    pub struct CoreOffsets {
        pub get_battle_object_from_id: usize,
        pub map_controls: usize,
        pub analog_trigger_l: usize,
        pub analog_trigger_r: usize,
    }
    static GET_BATTLE_OBJECT_FROM_ID_SEARCH_CODE: &[u8] = &[
        0x1f, 0x60, 0x02, 0x39, // strb wzr, [x0, #0x98]
        0xc0, 0x03, 0x5f, 0xd6, // ret
        0x00, 0x00, 0x00, 0x00, // ??
        0x08, 0x7c, 0x1c, 0x53, // lsr w9, x0, #0x1C
        0x1f, 0x11, 0x00, 0x71, // cmp w8, #0x4
    ];
    const GET_BATTLE_OBJECT_FROM_ID_OFFSET_TO_START: usize = 0xC;
    static MAP_CONTROLS_SEARCH_CODE: &[u8] = &[
        0xff, 0x03, 0x02, 0xd1, // sub sp, sp, #0x80
        0xf7, 0x23, 0x00, 0xf9, // str x23, [sp, #local_40]
        0xf6, 0x57, 0x05, 0xa9, // stp x22, x21, [sp, #local_30]
        0xf4, 0x4f, 0x06, 0xa9, // stp x20, x19, [sp, #local_20]
        0xfd, 0x7b, 0x07, 0xa9, // stp x29, x30, [sp, #local_10]
        0xfd, 0xc3, 0x01, 0x91, // add x29, sp, #0x70
        0x3f, 0x04, 0x00, 0x31, // cmn w1, #0x1
    ];
    static ANALOG_TRIGGER_L_SEARCH_CODE: &[u8] = &[
        0x29, 0x01, 0x7a, 0xb2, // orr x9, x9, #0x40
        0x1f, 0x01, 0x0b, 0x6b, // cmp w8, w11
        0x28, 0xc1, 0x8a, 0x9a, // csel x8, x9, x10, gt
        0xe9, 0x2b, 0x40, 0xb9, // ldr w9, [sp, #0x28]
        0x0a, 0xf9, 0x78, 0x92, // and x10, x8, #0xffffffffffffff7f
        0x08, 0x01, 0x79, 0xb2, // orr x8, x8, #0x80
        0x3f, 0x01, 0x0b, 0x6b, // cmp w9, w11
    ];
    const ANALOG_TRIGGER_R_OFFSET_FROM_L: usize = 0x14;
    fn offset_from_adrp(adrp_offset: usize) -> usize {
        unsafe {
            let adrp = *offset_to_addr::<u32>(adrp_offset);
            let immhi = (adrp & 0b0_00_00000_1111111111111111111_00000) >> 3;
            let immlo = (adrp & 0b0_11_00000_0000000000000000000_00000) >> 29;
            let imm = ((immhi | immlo) << 12) as i32 as usize;
            let base = adrp_offset & 0xFFFF_FFFF_FFFF_F000;
            base + imm
        }
    }
    fn offset_from_ldr(ldr_offset: usize) -> usize {
        unsafe {
            let ldr = *offset_to_addr::<u32>(ldr_offset);
            let size = (ldr & 0b11_000_0_00_00_000000000000_00000_00000) >> 30;
            let imm = (ldr & 0b00_000_0_00_00_111111111111_00000_00000) >> 10;
            (imm as usize) << size
        }
    }
    fn offset_from_bl(bl_offset: usize) -> usize {
        unsafe {
            let bl = *offset_to_addr::<u32>(bl_offset);
            let imm = bl & 0b0_00000_11111111111111111111111111;
            (imm * 4) as usize
        }
    }
    lazy_static::lazy_static! {
        pub static ref CORE_OFFSETS: CoreOffsets = {
            let mut offsets = CoreOffsets {
                get_battle_object_from_id: 0,
                map_controls: 0,
                analog_trigger_l: 0,
                analog_trigger_r: 0
            };
            offsets.get_battle_object_from_id = byte_search(GET_BATTLE_OBJECT_FROM_ID_SEARCH_CODE).expect("Unable to find Item class constructor hook!") + GET_BATTLE_OBJECT_FROM_ID_OFFSET_TO_START;
            offsets.map_controls = byte_search(MAP_CONTROLS_SEARCH_CODE).expect("Unable to find control mapping function!");
            offsets.analog_trigger_l = byte_search(ANALOG_TRIGGER_L_SEARCH_CODE).expect("Unable to find the analog trigger l");
            offsets.analog_trigger_r = offsets.analog_trigger_l + ANALOG_TRIGGER_R_OFFSET_FROM_L;
            offsets
        };
    }

    #[export_name = "offsets_get_battle_object_from_id"]
    pub fn get_battle_object_from_id() -> usize {
        CORE_OFFSETS.get_battle_object_from_id
    }

    #[export_name = "offsets_map_controls"]
    pub fn map_controls() -> usize {
        CORE_OFFSETS.map_controls
    }

    #[export_name = "offsets_analog_trigger_l"]
    pub fn analog_trigger_l() -> usize {
        CORE_OFFSETS.analog_trigger_l
    }

    #[export_name = "offsets_analog_trigger_r"]
    pub fn analog_trigger_r() -> usize {
        CORE_OFFSETS.analog_trigger_r
    }
}

pub use offsets_impl::*;