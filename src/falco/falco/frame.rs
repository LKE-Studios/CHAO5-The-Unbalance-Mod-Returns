use crate::imports::BuildImports::*;

unsafe extern "C" fn falco_frame(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_LIGHT, true);
        }
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            if ControlModule::get_command_flag_cat(fighter.module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP != 0
                && ControlModule::get_stick_y(fighter.module_accessor) < -0.66
                && KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        fighter.sub_transition_group_check_air_cliff();
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -5.5, 0);
        }
    }
    if status_kind == *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH || status_kind == *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH_END {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -10.0, 0);
        }
    }
}

pub fn install() {
    Agent::new("falco")
    .on_line(Main, frame_falco)
    .install();
}