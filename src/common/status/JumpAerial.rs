use crate::imports::BuildImports::*;
use super::super::can_use_glide;

#[skyline::hook(replace = L2CFighterCommon_status_JumpAerialSub)]
unsafe fn status_JumpAerialSub(fighter: &mut L2CFighterCommon, motion: L2CValue, keep_jump: L2CValue) -> L2CValue {
    let ret = call_original!(fighter, motion, keep_jump);
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    // Enable Gliding
    if can_use_glide(fighter_kind) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_AVAILABLE_GLIDE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_ENABLE);
    }
    ret
}

#[skyline::hook(replace = L2CFighterCommon_status_JumpAerial_Main)]
unsafe fn status_JumpAerial_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_glide_check().get_bool() {
        return 0.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 0.into();
    }
    if fighter.sub_air_check_stop_ceil().get_bool() {
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
            return 1.into();
        }
    }
    fighter.sub_air_check_superleaf_fall_slowly();
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_JumpAerialSub,
            status_JumpAerial_Main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}