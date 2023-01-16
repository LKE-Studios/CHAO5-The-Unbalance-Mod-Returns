use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    super::is_glider
};

#[skyline::hook(replace = L2CFighterCommon_status_FlySub)]
unsafe fn status_flysub(fighter: &mut L2CFighterCommon) {
    ControlModule::reset_flick_x(fighter.module_accessor);
    ControlModule::reset_flick_sub_x(fighter.module_accessor);
    fighter.global_table[0x1C].assign(&L2CValue::I32(0xfe));
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[0x1D].assign(&L2CValue::I32(0xfe));
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    fighter.sub_air_check_fall_common_pre();
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FLY);
    let mot = fighter.sub_getFlyMotion().get_u64();
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fly_uniq(false.into());
    }
    fighter.global_table[0x15].assign(&L2CValue::Ptr(L2CFighterCommon_sub_fly_uniq as *const () as _));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        FighterControlModuleImpl::reserve_on_attack_button(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    }
    // Enable Gliding
    if is_glider(fighter.global_table[0x2].get_i32()) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_AVAILABLE_GLIDE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_ENABLE);
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_Fly_Main)]
unsafe fn status_fly_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_glide_check().get_bool() {
        return 0.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 0.into();
    }
    let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let jump_count_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    let status = if jump_count != jump_count_max {
        FIGHTER_STATUS_KIND_FALL
    }
    else {
        FIGHTER_STATUS_KIND_FALL_AERIAL
    };
    
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(status.into(), false.into());
            return 0.into();
        }
    }

    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_flysub,
            status_fly_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}