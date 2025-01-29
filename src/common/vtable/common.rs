use crate::imports::BuildImports::*; 

const COMMON_VTABLE_ON_SEARCH_EVENT_OFFSET: usize = 0x68d8a0;

#[skyline::hook(offset = COMMON_VTABLE_ON_SEARCH_EVENT_OFFSET)]
unsafe extern "C" fn vtable_common_OnSearch(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    original!()(vtable, fighter, log)
}

pub fn install() {
    skyline::install_hooks!(
        vtable_common_OnSearch
    );
}