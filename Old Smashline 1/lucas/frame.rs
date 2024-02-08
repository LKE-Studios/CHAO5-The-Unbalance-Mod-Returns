use crate::imports::BuildImports::*;

pub static mut LUCAS_WESS_DANCE : [bool; 8] = [false; 8];
pub static mut HOLD_TIME : [f32; 8] = [0.0; 8];

#[fighter_frame( agent = FIGHTER_KIND_LUCAS )]
pub fn frame_lucas(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let LUCAS = color >= 0 && color <= 16;
        let CLAUS = color >= 64 && color <= 71;
        if LUCAS {
            platform_cancel_function(fighter);
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
                if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
                && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_LIGHT, true);
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
            if LUCAS_WESS_DANCE[ENTRY_ID] == true {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("wess_dance"), 0.0, 1.0, false, 0.0, false, false);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
            if status_kind == *FIGHTER_STATUS_KIND_APPEAL && [hash40("appeal_hi_r"), hash40("appeal_hi_l")].contains(&motion_kind) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                    HOLD_TIME[ENTRY_ID] += 1.0;
                }
                if HOLD_TIME[ENTRY_ID] == 30.0 {
                    LUCAS_WESS_DANCE[ENTRY_ID] = true;
                }
            }
            else {
                HOLD_TIME[ENTRY_ID] = 0.0;
                LUCAS_WESS_DANCE[ENTRY_ID] = false;
            };
        }
    }
}

unsafe fn platform_cancel_function(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_GROUND {
            if GroundModule::is_passable_ground(fighter.module_accessor) {
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
                if stick_y < -0.66 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                }
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_lucas
    );
}