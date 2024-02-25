use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_gamewatch_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_LIGHT, true);
        }
        if ControlModule::get_command_flag_cat(fighter.module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP != 0
            && ControlModule::get_stick_y(fighter.module_accessor) < -0.66 
            && KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
        if StatusModule::is_changing(fighter.module_accessor) {
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 0.9, y: 1.0, z: 1.0}, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
}

pub fn install() {
    Agent::new("gamewatch")
    .on_line(Main, frame_gamewatch_Main)
    .install();
}