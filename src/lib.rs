#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(unused_macros,
    warnings,
    unused_must_use,
    unused_unsafe,
    non_upper_case_globals,
    unused_assignments,
    unused_must_use,
    unused_mut,
    clippy::borrow_interior_mutable_const,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::absurd_extreme_comparisons,
    clippy::cmp_null,
    clippy::if_same_then_else)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
//#[macro_use]

use std::ffi::CStr;
use std::os::raw::c_int;
use skyline::{c_str, from_c_str, nn::ro::LookupSymbol};
use skyline::libc::c_char;
use skyline::nro::{self, NroInfo};
use skyline::hooks::{getRegionAddress, Region, InlineCtx};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::hash40;
use smashline::*;
use smash::app::sv_animcmd::*;
use crate::globals::*;
use crate::utils::*;
use crate::common::get_player_number;
use crate::common::FIGHTER_BOOL_1;
use crate::common::FIGHTER_BOOL_2;
use crate::common::FIGHTER_BOOL_3;

pub mod globals {
    pub const UNK1: i32 = 0x0; //void value
    pub const UNK2: i32 = 0x1; //void value
    pub const FIGHTER_KIND: i32 = 0x2; //fighter kind
    pub const OBJECT_ID: i32 = 0x3; //object id
    pub const UNK3: i32 = 0x4; //ptr value, very similar to 0x6
    pub const MODULE_ACCESSOR: i32 = 0x5; //module accessor
    pub const UNK4: i32 = 0x6; //void value
    pub const INIT_STATUS_FUNC: i32 = 0x7; //init status func
    pub const IS_STOP: i32 = 0x8; //is stop
    pub const STATUS_KIND_INTERRUPT: i32 = 0x9; //status kind interrupt
    pub const PREV_STATUS_KIND: i32 = 0xA; //prev status kind
    pub const STATUS_KIND: i32 = 0xB; //status kind
    pub const STATUS_COUNT: i32 = 0xC; //status count
    pub const UNK5: i32 = 0xD; //bool value
    pub const CURRENT_FRAME: i32 = 0xE; //current frame
    pub const CURRENT_FRAME_NO_INTERP: i32 = 0xF; //current frame no interp
    pub const UNK6: i32 = 0x10; //ptr value
    pub const UNK7: i32 = 0x11; //ptr value, equal to UNK8
    pub const UNK8: i32 = 0x12; //ptr value
    pub const SUB_STATUS3: i32 = 0x13; //sub status3
    pub const PREV_SUB_STATUS: i32 = 0x14; //prev sub status
    pub const SUB_STATUS: i32 = 0x15; //sub status
    pub const SITUATION_KIND: i32 = 0x16; //situation kind
    pub const PREV_SITUATION_KIND: i32 = 0x17; //prev situation kind
    pub const PREV_STATUS_FRAME: i32 = 0x18; //prev status frame
    pub const UNK9: i32 = 0x19; //i32 value
    pub const STICK_X: i32 = 0x1A; //stick x
    pub const STICK_Y: i32 = 0x1B; //stick y
    pub const FLICK_X: i32 = 0x1C; //flick x
    pub const FLICK_Y: i32 = 0x1D; //flick y
    pub const FLICK_Y_DIR: i32 = 0x1E; //flick y dir
    pub const PAD_FLAG: i32 = 0x1F; //pad flag
    pub const CMD_CAT1: i32 = 0x20; //cmd cat1
    pub const CMD_CAT2: i32 = 0x21; //cmd cat2
    pub const CMD_CAT3: i32 = 0x22; //cmd cat3
    pub const CMD_CAT4: i32 = 0x23; //cmd cat4
    pub const UNK10: i32 = 0x24;
    pub const UNK11: i32 = 0x25;
    pub const CHECK_AIR_SPECIAL_UNIQ: i32 = 0x26; //check air special uniq
    pub const CHECK_GROUND_SPECIAL_UNIQ: i32 = 0x27; //check ground special uniq
    pub const CHECK_GROUND_ATTACK_UNIQ: i32 = 0x28; //check ground attack uniq
    pub const DASH_COMMON_UNIQ: i32 = 0x29; //dash common uniq
    pub const RUN_MAIN_UNIQ: i32 = 0x2A; //run main uniq
    pub const JUMP_SQUAT_MAIN_UNIQ: i32 = 0x2B; //jump squat main uniq
    pub const CHECK_AIR_LANDING_UNIQ: i32 = 0x2C; //check air landing uniq
    pub const CHECK_AIR_ITEM_THROW_UNIQ: i32 = 0x2D; //check air item throw uniq
    pub const CHECK_AIR_ATTACK_UNIQ: i32 = 0x2E; //check air attack uniq
    pub const CHECK_AIR_ESCAPE_UNIQ: i32 = 0x2F; //check air escape uniq
    pub const CHECK_AIR_TREAD_JUMP_UNIQ: i32 = 0x30; //check air tread jump uniq
    pub const CHECK_AIR_WALL_JUMP_UNIQ: i32 = 0x31; //check air wall jump uniq
    pub const CHECK_AIR_JUMP_UNIQ: i32 = 0x32; //check air jump uniq
    pub const CHECK_AIR_JUMP_AERIAL_UNIQ: i32 = 0x33; //check air jump aerial uniq
    pub const GUARD_CONT_UNIQ: i32 = 0x34; //guard cont uniq
    pub const TURN_UNIQ: i32 = 0x35; //turn uniq
    pub const CHECK_AIR_CLIFF_LASSO_UNIQ: i32 = 0x36; //check air cliff lasso uniq
    pub const LANDING_UNIQ_CHECK_STRANS_UNIQ: i32 = 0x37; //landing uniq check strans uniq
    pub const CHECK_SPECIAL_N_UNIQ: i32 = 0x38; //check special n uniq
    pub const CHECK_SPECIAL_S_UNIQ: i32 = 0x39; //check special s uniq
    pub const CHECK_SPECIAL_HI_UNIQ: i32 = 0x3A; //check special hi uniq
    pub const CHECK_SPECIAL_LW_UNIQ: i32 = 0x3B; //check special lw uniq
    pub const CHECK_SPECIAL_COMMAND: i32 = 0x3C; //check special command
    pub const WAZA_CUSTOMIZE_CONTROL: i32 = 0x3D; //waza customize control
    pub const STATUS_END_CONTROL: i32 = 0x3E; //status end control
    pub const UNK12: i32 = 0x3F;
    pub const UNK13: i32 = 0x40;
    pub const UNK14: i32 = 0x41;
    pub const DAMAGE_MOTION_KIND_CALLBACK: i32 = 0x42;
    pub const SUB_UNIQ_DAMAGE_FLY_UNIQ: i32 = 0x43;
    pub const DOWN_DAMAGE_UNIQ: i32 = 0x44;
    pub const THROW_F_STATUS_KIND: i32 = 0x45;
    pub const THROW_B_STATUS_KIND: i32 = 0x46;
    pub const THROW_HI_STATUS_KIND: i32 = 0x47;
    pub const THROW_LW_STATUS_KIND: i32 = 0x48;
    pub const DAMAGE_STOP_MOTION_INTP_FRAME: i32 = 0x49;
    pub const SUB_REBIRTH_UNIQ_INIT_CORE_UNIQ: i32 = 0x4A;
    pub const SUB_REBIRTH_UNIQ_EXEC_UNIQ: i32 = 0x4B;
    pub const SUB_DEAD_UNIQ_INIT_UNIQ: i32 = 0x4C;
    pub const SUB_ROULETTE_SET_SETP_UNIQ: i32 = 0x4D;
    pub const FALL_BRAKE_UNIQ: i32 = 0x4E;
    pub const CHECK_GROUND_GUARD_UNIQ: i32 = 0x4F;
    pub const CHECK_GROUND_CATCH_UNIQ: i32 = 0x50;
    pub const CHECK_COMMAND_WALK_UNIQ: i32 = 0x51;
    pub const CHECK_GROUND_JUMP_MINI_ATTACK: i32 = 0x52;
    pub const CHECK_AIR_ITEM_THROW_POST: i32 = 0x53;
    pub const IS_ITEM_SHOOT_STATUS_UNIQ: i32 = 0x54;
    pub const CHECK_ATTACK_3_UNIQ: i32 = 0x55;
    pub const CHECK_ATTACK_N_UNIQ: i32 = 0x56;
    pub const CHECK_ATTACK_S4_UNIQ: i32 = 0x57;
    pub const CHECK_ATTACK_HI4_UNIQ: i32 = 0x58;
    pub const CHECK_ATTACK_LW4_UNIQ: i32 = 0x59;
    pub const SQUAT_COMMON_UNIQ: i32 = 0x5A;
}

mod bayonetta;
mod brave;
mod buddy;
mod captain;
mod chrom;
mod claus;
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
mod koopag;
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
mod silver;
mod simon;
mod snake;
mod sonic;
mod szerosuit;
mod tantan;
mod toonlink;
mod trail;
mod waluigi;
mod wario;
mod wiifit;
mod wolf;
mod yoshi;
mod younglink;
mod zelda;

pub mod singletons;
pub mod helper;
pub mod imports;
mod utils;
mod common;

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

#[skyline::hook(offset=0x3f0028, inline)]
pub unsafe fn offset_dump(ctx: &InlineCtx) {
	let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
	println!("Function Offset: {:#X}", ctx.registers[8].x.as_ref() - text);
}

#[skyline::hook(replace = MotionModule::change_motion)]
pub unsafe fn motionmodule_change_motion_replace(
    module_accessor: &mut smash::app::BattleObjectModuleAccessor,
    motion_kind: u64,
    entry_frame: f32,
    rate: f32,
    arg5: bool,
    arg6: f32,
    arg7: bool,
    arg8: bool
) -> u64 {
	let fighter_kind = smash::app::utility::get_kind(module_accessor);

    if fighter_kind == *FIGHTER_KIND_KOOPAG {
		let GIGA_DTILT = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
		let GIGA_DASH_ATTACK = &mut FIGHTER_BOOL_2[get_player_number(module_accessor)];
		let DTILT_INPUT = &mut FIGHTER_BOOL_3[get_player_number(module_accessor)];
		let mut new_motion = motion_kind;
        if motion_kind == hash40("attack_s3_s") {
			if *GIGA_DASH_ATTACK {
				new_motion = hash40("attack_dash");
			}
			if *GIGA_DTILT {
				new_motion = hash40("attack_lw3");
			}
			*DTILT_INPUT = false;
		}
		if motion_kind == hash40("special_n") || motion_kind == hash40("special_n_end") {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
			return 0;	
		}
		if motion_kind == hash40("special_air_n") || motion_kind == hash40("special_air_n_end") {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
			return 0;
		}
		*GIGA_DASH_ATTACK = false;
		*GIGA_DTILT = false;
		original!()(module_accessor, new_motion, entry_frame, rate, arg5, arg6, arg7, arg8)
    }
    else {
        original!()(module_accessor, motion_kind, entry_frame, rate, arg5, arg6, arg7, arg8)
    }
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

extern "C" {
	pub fn change_version_string(arg: u64, string: *const c_char);
}

#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const c_char) {
	let original_string = unsafe {from_c_str(string)};
	if original_string.contains("Ver.") {
		let version_string = format!("\nSmash {} \nCHAO5: The UN-Balance Mod Returns! | Ver. 1.8.0 \0", original_string);
		call_original!(arg, skyline::c_str(&version_string));
	}
	else {
		call_original!(arg, string);
	}
}


#[skyline::main(name = "chao5")]
pub fn main() {
    unsafe {
        extern "C" { fn allow_ui_chara_hash_online(ui_chara_hash: u64); }
        allow_ui_chara_hash_online(0xe7bbfb2e4); //Claus
        allow_ui_chara_hash_online(0xfec9738f6); //Silver
        allow_ui_chara_hash_online(0x108658e080); //Waluigi
    }
    unsafe {
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
    tantan::install();
    claus::install();
    koopag::install();
    silver::install();
    waluigi::install();
    skyline::install_hooks!(
        declare_const_hook, 
        offset_dump,
        log_remove_motion_partial,
        log_add_motion_partial,
        motionmodule_change_motion_replace,
        change_version_string_hook
    );
    common::install();
}
