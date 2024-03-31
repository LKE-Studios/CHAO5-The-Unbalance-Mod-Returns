use crate::imports::BuildImports::*;

unsafe extern "C" fn status_ridley_SpecialHiStopWall_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge_status = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_STATUS);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    if charge_status == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B { 
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_wall_b"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_wall"), 0.0, 1.0, false, 0.0, false, false);
    }
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    let speed_x_mul_on_stop_wall = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("speed_x_mul_on_stop_wall"));
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, speed_x * speed_x_mul_on_stop_wall * -1.0, 0.0, 0.0, 0.0, 0.0);
    let mut speed_y_on_stop_wall = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("speed_y_on_stop_wall"));
    if speed_y_on_stop_wall < 0.0 {
        speed_y_on_stop_wall = 0.0;
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y_on_stop_wall, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.sub_shift_status_main(L2CValue::Ptr(ridley_SpecialHiStopWall_Main_loop as *const () as _))
}

unsafe extern "C" fn ridley_SpecialHiStopWall_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return true.into()
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        return true.into()
    }
    else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    false.into()
}

pub fn install() {
    Agent::new("ridley")
    .status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_WALL, status_ridley_SpecialHiStopWall_Main)
    .install();
}