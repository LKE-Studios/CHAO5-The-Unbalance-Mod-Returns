#![allow(dead_code)]

use smash::{
    *,
    phx::Hash40,
    lib::lua_const::*,
    app::{lua_bind::*, *}
};

// Transition Hook static muts:
// 0 - Don't change 
// 1 - Force off
// 2 - Force on (doesnt really work)
pub static mut CAN_UPB: [i32; 8] = [0; 8];
pub static mut CAN_SIDEB: [i32; 8] = [0; 8];
pub static mut CAN_DOWNB: [i32; 8] = [0; 8];
pub static mut CAN_NEUTRALB: [i32; 8] = [0; 8];
pub static mut CAN_JUMP_SQUAT: [i32; 8] = [0; 8];
pub static mut CAN_DOUBLE_JUMP: [i32; 8] = [0; 8];
pub static mut CAN_CLIFF: [i32; 8] = [0; 8];
pub static mut CAN_ATTACK_AIR: [i32; 8] = [0; 8];
pub static mut CAN_AIRDODGE: [i32; 8] = [0; 8];
pub static mut CAN_RAPID_JAB: [i32; 8] = [0; 8];
pub static mut CAN_JAB: [i32; 8] = [0; 8];

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_hook(boma: &mut smash::app::BattleObjectModuleAccessor, flag: i32) -> bool {
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if CAN_UPB[ENTRY_ID] != 0 && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
        if CAN_UPB[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else if CAN_SIDEB[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S {
        if CAN_SIDEB[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else if CAN_DOWNB[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
        if CAN_DOWNB[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else if CAN_CLIFF[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH{
        if CAN_CLIFF[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else if CAN_AIRDODGE[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR {
        if CAN_AIRDODGE[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else if CAN_NEUTRALB[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N {
        if CAN_NEUTRALB[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else if CAN_JUMP_SQUAT[ENTRY_ID] != 0  && (flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT || flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON) {
        if CAN_JUMP_SQUAT[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else if CAN_DOUBLE_JUMP[ENTRY_ID] != 0  && (flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL || flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON) {
        if CAN_DOUBLE_JUMP[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else if CAN_ATTACK_AIR[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR  {
        if CAN_ATTACK_AIR[ENTRY_ID] == 1 {
            return false
        } else {
            return true 
        }
    } else {
        original!()(boma, flag)
    }
}

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E"]
    pub static FIGHTER_MANAGER: *mut smash::app::FighterManager;
}
extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"]
    pub static FIGHTER_CUTIN_MANAGER: *mut smash::app::FighterCutInManager;
}

pub fn get_module_accessor_by_entry_id(entry_id: i32) -> *mut smash::app::BattleObjectModuleAccessor {
    unsafe {
        &mut *smash::app::sv_battle_object::module_accessor(smash::app::Fighter::get_id_from_entry_id(entry_id))
    }
}

pub fn is_grounded(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    let situation_kind = unsafe { StatusModule::situation_kind(module_accessor) as i32 };
    situation_kind == SITUATION_KIND_GROUND
}

pub unsafe fn set_position_lock(entry_id: i32){
    lua_bind::FighterManager::set_position_lock(FIGHTER_MANAGER, FighterEntryID(entry_id), true);
}

pub unsafe fn unset_position_lock(entry_id: i32){
    lua_bind::FighterManager::set_position_lock(FIGHTER_MANAGER, FighterEntryID(entry_id), false);
}


pub unsafe fn change_motion(module_accessor: *mut BattleObjectModuleAccessor, anim: &str){
    MotionModule::change_motion(module_accessor, Hash40::new(anim), 0.0, 1.0, false, 0.0, false, false);
}
pub unsafe fn get_entry_id(module_accessor: *mut BattleObjectModuleAccessor) -> usize{
    WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
}

pub unsafe fn disable_gravity(module_accessor: *mut BattleObjectModuleAccessor){
    KineticModule::unable_energy(module_accessor,  *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
}

pub unsafe fn enable_gravity(module_accessor: *mut BattleObjectModuleAccessor){
    KineticModule::enable_energy(module_accessor,  *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
}

pub fn install() {
	skyline::install_hook!(
        is_enable_transition_term_hook
    );
}

/*
todo
pub unsafe fn get_nearest_opponent(module_accessor: *mut BattleObjectModuleAccessor) -> i32{

    let entry_id = get_entry_id(module_accessor);
    let mut lowestavg = 0.0;
    let entry_count = FighterManager::entry_count(FIGHTER_MANAGER);
    for i in 0..entry_count{
        let curr_boma = get_module_accessor_by_entry_id(i);
        let avg = (PostureModule::pos_x(curr_boma) + PostureModule::pos_y(curr_boma)) / 2;
    }
}
 */
