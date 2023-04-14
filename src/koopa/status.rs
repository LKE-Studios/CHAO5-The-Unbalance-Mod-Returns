use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;
use crate::utils::*;
use smashline::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
//use smash::app::sv_battle_object::module_accessor;
use smash::hash40;
use smash::phx::{Vector3f, Hash40};
use smash_script::*;

#[status_script(agent = "koopa", status = FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_koopa_special_lw_g(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Midbus
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw") && MotionModule::is_end(fighter.module_accessor) {   
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
        }
    } 
    else {//Bowser
        call_original!()(fighter);
    }
    L2CValue::I32(0)
}

#[status_script(agent = "koopa", status = FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_koopa_special_lw_a(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Midbus
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw") && MotionModule::is_end(fighter.module_accessor) {   
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        }
    } else {//Bowser
        call_original!()(fighter);
    }
    L2CValue::I32(0)
}

pub fn install() {
    smashline::install_status_scripts!(
        status_koopa_special_lw_g,
        status_koopa_special_lw_a
    );
}