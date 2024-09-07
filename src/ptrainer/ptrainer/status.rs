use crate::imports::BuildImports::*;

static mut PTRAINER_NO_SWAP_DEAD : usize = 0xf96330;

//Removes the death swap from PT
#[skyline::hook(offset = PTRAINER_NO_SWAP_DEAD)]
unsafe fn ptrainer_no_swap_dead() {}

pub fn install() {
    skyline::install_hooks!(
        ptrainer_no_swap_dead
    );
}