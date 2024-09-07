use crate::imports::BuildImports::*;

unsafe extern "C" fn status_ridley_SpecialHiStopCeil_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CEIL_COUNT);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_ceil"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let speed_x_mul_on_stop_ceil = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("speed_x_mul_on_stop_ceil"));
    let deccel_x_on_stop_ceil = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("deccel_x_on_stop_ceil"));
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, speed_x * (speed_x_mul_on_stop_ceil * deccel_x_on_stop_ceil), 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    if !StopModule::is_stop(fighter.module_accessor) {
        ridley_SpecialHiStopCeil_Sub_Status(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ridley_SpecialHiStopCeil_Main_loop as *const () as _))
}

unsafe extern "C" fn ridley_SpecialHiStopCeil_Sub_Status(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CEIL_COUNT);
        let ceil_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CEIL_COUNT);
        let fall_frame_on_stop_ceil = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_frame_on_stop_ceil"));
        if ceil_count >= fall_frame_on_stop_ceil {
            if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) {
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
                sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn ridley_SpecialHiStopCeil_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("ridley")
    .status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_CEIL, status_ridley_SpecialHiStopCeil_Main)
    .install();
}