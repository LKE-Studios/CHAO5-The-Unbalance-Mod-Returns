use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_ZELDA )]
pub fn frame_zelda(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if situation_kind == *SITUATION_KIND_GROUND {
                if StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR && frame < 55.0 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_LIGHT, false);
                    MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 56.0, true, true, false);
                }
            }
            else if situation_kind == *SITUATION_KIND_AIR {
                if frame >= 31.0 {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                    if ControlModule::get_command_flag_cat(fighter.module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP != 0 
                    && ControlModule::get_stick_y(fighter.module_accessor) < -0.66 
                    && KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                        WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                    }
                }
            }
        }
        if status_kind == *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END {
            if situation_kind == *SITUATION_KIND_GROUND {
                if StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING, false);
                }
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && fighter.global_table[CURRENT_FRAME].get_i32() > 9 {
            if ControlModule::get_stick_y(fighter.module_accessor) < -0.66 && GroundModule::is_passable_ground(fighter.module_accessor) {
                GroundModule::pass_floor(fighter.module_accessor);
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_zelda
    );
}