use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_fox_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    platform_cancel_function(fighter);
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
            DamageModule::heal(fighter.module_accessor, -5.0, 0);
        }
    }
    if status_kind == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH || status_kind == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -50.0, 0);
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        }
    }
    if motion_kind == hash40("appeal_hi_r") || motion_kind == hash40("appeal_hi_l") {
        if frame > 41.0 && frame < 44.0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, false);
            }
        }
    }
}

unsafe fn platform_cancel_function(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let flick_y = ControlModule::get_flick_y(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if [hash40("special_s_end"), hash40("special_air_s_end"), hash40("special_lw_end"), hash40("special_air_lw_end")].contains(&motion_kind) || 
    status_kind == *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END || status_kind == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END {
        if situation_kind == *SITUATION_KIND_GROUND {
            if GroundModule::is_passable_ground(fighter.module_accessor) {
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
                if flick_y < -1 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                }
            }
        }
    }
}

pub fn install() {
    Agent::new("fox")
    .on_line(Main, frame_fox_Main)
    .install();
}