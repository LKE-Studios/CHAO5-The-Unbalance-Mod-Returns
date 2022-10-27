#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(unused_macros)]
#![allow(non_snake_case)]

use std::ffi::CStr;
use std::os::raw::c_int;
use skyline::hooks::{getRegionAddress, Region};
use smash::app::lua_bind::*;
use smash::app::*;

mod bayonetta;
mod brave;
mod buddy;
mod captain;
mod chrom;
mod cloud;
mod daisy;
mod dedede;
mod demon;
mod diddy;
mod dolly;
mod donkey;
mod duckhunt;
mod edge;
mod eflame;
mod elight;
mod falco;
mod fox;
mod gamewatch;
mod ganon;
mod gaogaen;
mod gekkouga;
mod ike;
mod inkling;
mod jack;
mod kamui;
mod ken;
mod kirby;
mod koopa;
mod koopajr;
mod krool;
mod link;
mod littlemac;
mod lucario;
mod lucas;
mod lucina;
mod luigi;
mod mario;
mod mariod;
mod marth;
mod master;
mod metaknight;
mod mewtwo;
mod miifighter;
mod miigunner;
mod miiswordsman;
mod murabito;
mod nana;
mod ness;
mod packun;
mod pacman;
mod palutena;
mod peach;
mod pfushigisou;
mod pichu;
mod pickel;
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
mod richter;
mod ridley;
mod robot;
mod rockman;
mod rosetta;
mod roy;
mod ryu;
mod samus;
mod samusd;
mod sheik;
mod shizue;
mod shulk;
mod simon;
mod snake;
mod sonic;
mod szerosuit;
mod toonlink;
mod trail;
mod wario;
mod wiifit;
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
    if str.contains("FIGHTER_RYU_STATUS_KIND_NUM") {
        value = 0x205;
    }
    original!()(unk,constant,value)
}

#[skyline::hook(replace = MotionModule::remove_motion_partial)]
pub unsafe fn log_remove_motion_partial(
    module_accessor: *mut BattleObjectModuleAccessor, 
    index: c_int, 
    arg3: bool
) -> u64 {
    let module_accessor_ptr = module_accessor as u64;
    let motion_module = *((module_accessor_ptr + 0x88) as *const u64);
    let partial_motions = *((motion_module + 0xe8) as *const u64);
    let data_ptr = (partial_motions + (0x40 * index as u64)) as *const [u8; 0x50];
    skyline::logging::hex_dump_ptr(data_ptr);
    original!()(module_accessor, index, arg3)
}

#[skyline::hook(replace = MotionModule::add_motion_partial)]
pub unsafe fn log_add_motion_partial(
    module_accessor: *mut BattleObjectModuleAccessor, 
    index: c_int, 
    arg3: u64,
    arg4: f32,
    arg5: f32,
    arg6: bool,
    arg7: bool,
    arg8: f32,
    arg9: bool,
    arg10: bool,
    arg11: bool,
) -> u64 {
    let module_accessor_ptr = module_accessor as u64;
    let motion_module = *((module_accessor_ptr + 0x88) as *const u64);
    let partial_motions = *((motion_module + 0xe8) as *const u64);
    let data_ptr = (partial_motions + (0x40 * index as u64)) as *const [u8; 0x50];
    skyline::logging::hex_dump_ptr(data_ptr);
    original!()(module_accessor, index, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

/*#[skyline::hook(offset = 0x3310760)]
pub unsafe fn update_selected_fighter(param_1: u64, player_id: u32, new_selection_info: u64){
    let ui_chara_hash: u64 = *((new_selection_info + 0x18) as *const u64) & 0xffffffffff;
    let selected_colour: *mut u8 = (new_selection_info + 0x20) as *mut u8;
    if ui_chara_hash == 0x0e7bbfb2e4 { //ui_chara_claus
        if *selected_colour < 8 {
            *selected_colour = *selected_colour + 8;
        }
    }
    println!("Hash: {:#x} - Colour {}", ui_chara_hash, *selected_colour);
    call_original!(param_1, player_id, new_selection_info);
}

#[skyline::hook(offset = 0x3237820)]
unsafe fn set_chara_colour_ui(param_1: u64, mut colour_slot: u32, param_3: u32) {
    let ui_chara_hash = param_1 & 0xffffffffff;
    if ui_chara_hash == 0x0e7bbfb2e4 {
        if *colour_slot >= 8 {
            *colour_slot = *colour_slot - 8;
        }
    }
    println!("UI_Hash: {:#x} - UI_Colour: {}", ui_chara_hash, colour_slot);
    call_original!(param_1, colour_slot, param_3);
}*/

//0xef9d43e1b = ui_chara_lucas

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
    murabito::install();
    rosetta::install();
    wiifit::install();
    duckhunt::install();
    koopajr::install();
    lucina::install();
    kamui::install();
    ridley::install();
    krool::install();
    gaogaen::install();
    packun::install();
    ken::install();
    inkling::install();
    simon::install();
    shizue::install();
    daisy::install();
    samusd::install();
    richter::install();
    chrom::install();
    dolly::install();
    jack::install();
    edge::install();
    brave::install();
    demon::install();
    master::install();
    pickel::install();
    eflame::install();
    elight::install();
    buddy::install();
    trail::install();
    skyline::install_hooks!(
        declare_const_hook, 
        log_remove_motion_partial,
        log_add_motion_partial,
        //update_selected_fighter,
        //set_chara_colour_ui
    );
    custom::install();
}
