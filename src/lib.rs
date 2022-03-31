#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(non_snake_case)]

pub static mut FIGHTER_CUTIN_MANAGER : *mut smash::app::FighterCutInManager = 0 as _;

use std::ffi::CStr;
use skyline::nn::ro::LookupSymbol;

mod captain;
mod donkey;
mod falco;
mod fox;
mod gamewatch;
mod ganon;
mod kirby;
mod koopa;
mod link;
mod luigi;
mod mario;
mod mariod;
mod marth;
mod ness;
mod peach;
mod pikachu;
mod purin;
mod samus;
mod yoshi;
mod zelda;
<<<<<<< HEAD
=======
mod peach;
>>>>>>> a7e1434b22331661b1cfb10ffb063699368a9835
mod utils;
// mod custom;

static mut CONSTANT_OFFSET : usize = 0x3727390; //13.0.1


#[skyline::hook(offset = CONSTANT_OFFSET)]
unsafe fn declare_const_hook(unk: u64, constant: *const u8, mut value: u32) {
    let str = CStr::from_ptr(constant as _).to_str().unwrap();
    if str.contains("FIGHTER_PEACH_STATUS_KIND_MAX") {
        value = 0x1ee //494 for now
    }
    original!()(unk,constant,value)
}

#[skyline::main(name = "smashline_test")]
pub fn main() {
    mario::install();
    donkey::install();
    link::install();
    samus::install();
    yoshi::install();
    kirby::install();
    pikachu::install();
    fox::install();
    luigi::install();
    ness::install();
    captain::install();
    purin::install();
    koopa::install();
    marth::install();
    ganon::install();
    gamewatch::install();
    mariod::install();
    zelda::install();
    falco::install();
    peach::install();
    skyline::install_hooks!(declare_const_hook);
    // custom::install();
}