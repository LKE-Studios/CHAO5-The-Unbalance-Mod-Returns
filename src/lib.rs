#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(unused_macros)]
#![allow(non_snake_case)]

use std::ffi::CStr;
use skyline::hooks::{getRegionAddress, Region};

mod bayonetta;
mod captain;
mod cloud;
mod dedede;
mod diddy;
mod donkey;
mod falco;
mod fox;
mod gamewatch;
mod ganon;
mod gekkouga;
mod ike;
mod kirby;
mod koopa;
mod link;
mod littlemac;
mod lucario;
mod lucas;
mod luigi;
mod mario;
mod mariod;
mod marth;
mod metaknight;
mod mewtwo;
mod miifighter;
mod miigunner;
mod miiswordsman;
mod nana;
mod ness;
mod pacman;
mod palutena;
mod peach;
mod pfushigisou;
mod pichu;
mod pikachu;
mod pikmin;
mod pit;
mod pitb;
mod plizardon;
mod popo;
mod purin;
mod ptrainer;
mod pzenigame;
mod reflet;
mod robot;
mod rockman;
mod roy;
mod ryu;
mod samus;
mod sheik;
mod shulk;
mod snake;
mod sonic;
mod szerosuit;
mod toonlink;
mod wario;
mod wolf;
mod yoshi;
mod younglink;
mod zelda;
mod utils;
mod custom;

const DECLARE_CONST_SEARCH_CODE: &[u8] = &[
    0xfc, 0x67, 0xbb, 0xa9, 0xf8, 0x5f, 0x01, 0xa9, 0xf6, 0x57, 0x02, 0xa9, 0xf4,
    0x4f, 0x03, 0xa9, 0xfd, 0x7b, 0x04, 0xa9, 0xfd, 0x03, 0x01, 0x91, 0xff, 0x83,
    0x20, 0xd1, 0x97, 0x10, 0x01, 0xd0, 0xf7, 0xe2, 0x0c, 0x91, 0x16, 0x04, 0x40,
    0xf9, 0xe8, 0xfe, 0xdf, 0x08, 0xf4, 0x03, 0x02, 0x2a, 0xf5, 0x03, 0x01,
    0xaa, 0xf3, 0x03, 0x00, 0xaa, 0x88, 0x06, 0x00, 0x36
];
static mut DECLARE_CONST_OFFSET : usize = 0x3727390; //13.0.1

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

#[skyline::hook(offset = DECLARE_CONST_OFFSET)]
unsafe fn declare_const_hook(unk: u64, constant: *const u8, mut value: u32) {
    let str = CStr::from_ptr(constant as _).to_str().unwrap();
    if str.contains("FIGHTER_PEACH_STATUS_KIND_MAX") {
        value = 0x1ee; //494 for now
    }
    original!()(unk,constant,value)
}

#[skyline::main(name = "chao5")]
pub fn main() {
    unsafe{
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, DECLARE_CONST_SEARCH_CODE){
            DECLARE_CONST_OFFSET = offset;
        }
    }
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
    sheik::install();
    falco::install();
    peach::install();
    popo::install();
    nana::install();
    younglink::install();
    mewtwo::install();
    pichu::install();
    roy::install();
    metaknight::install();
    pit::install();
    wario::install();
    diddy::install();
    ptrainer::install();
    pzenigame::install();
    pfushigisou::install();
    plizardon::install();
    dedede::install();
    lucas::install();
    lucario::install();
    sonic::install();
    snake::install();
    wolf::install();
    szerosuit::install();
    robot::install();
    ike::install();
    toonlink::install();
    pikmin::install();
    littlemac::install();
    shulk::install();
    palutena::install();
    rockman::install();
    ryu::install();
    miifighter::install();
    miiswordsman::install();
    miigunner::install();
    cloud::install();
    bayonetta::install();
    reflet::install();
    gekkouga::install();
    pitb::install();
    pacman::install();
    skyline::install_hooks!(declare_const_hook);
    custom::install();
}
