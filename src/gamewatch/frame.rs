use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_GAMEWATCH )]
pub fn frame_gamewatch(fighter : &mut L2CFighterCommon) {
    unsafe {
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
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW || status_kind == *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_CATCH || status_kind == *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_SHOOT ||
        status_kind == *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_REFLECT || status_kind == *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_WAIT || 
        status_kind == *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_END || status_kind == *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_WAIT_START {
            KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_gamewatch
    );
}