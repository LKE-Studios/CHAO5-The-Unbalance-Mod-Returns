use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    super::is_glider
};

#[skyline::hook(replace = L2CFighterCommon_status_JumpAerialSub)]
unsafe fn status_jumpaerialsub(fighter: &mut L2CFighterCommon, motion: L2CValue, keep_jump: L2CValue) -> L2CValue {
    let ret = call_original!(fighter, motion, keep_jump);
    // Enable Gliding
    if is_glider(fighter.global_table[0x2].get_i32()) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_AVAILABLE_GLIDE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_ENABLE);
    }
    ret
}

#[skyline::hook(replace = L2CFighterCommon_status_JumpAerial_Main)]
unsafe fn status_jumpaerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
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
            status_jumpaerialsub,
            status_jumpaerial_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}