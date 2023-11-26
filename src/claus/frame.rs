use crate::imports::BuildImports::*;

pub static mut CLAUS_WESS_DANCE : [bool; 8] = [false; 8];
pub static mut HOLD_TIME : [f32; 8] = [0.0; 8];

#[fighter_frame( agent = FIGHTER_KIND_LUCAS )]
pub fn frame_claus(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let CLAUS = color >= 64 && color <= 71;
        if CLAUS {
            platform_cancel_function(fighter);
            if CLAUS_WESS_DANCE[ENTRY_ID] == true {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("wess_dance"), 0.0, 1.0, false, 0.0, false, false);
            }
            if status_kind == *FIGHTER_STATUS_KIND_APPEAL && [hash40("appeal_hi_r"), hash40("appeal_hi_l")].contains(&motion_kind) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                    HOLD_TIME[ENTRY_ID] += 1.0;
                }
                if HOLD_TIME[ENTRY_ID] == 30.0 {
                    CLAUS_WESS_DANCE[ENTRY_ID] = true;
                }
            }
            else {
                HOLD_TIME[ENTRY_ID] = 0.0;
                CLAUS_WESS_DANCE[ENTRY_ID] = false;
            };
        }
        claus_classic_mode_image(fighter);
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

pub unsafe fn claus_classic_mode_image(fighter: &mut L2CFighterCommon) {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let LUCAS = color >= 0 && color <= 16;
    if get_stage_id() == BonusGame {
        if metadata("sd:/ultimate/mods/CHAO5 - The UN-Balance Mod Returns/standard/staffroll/texture/standard_staffroll_claus.nutexb").is_ok() {
            rename("sd:/ultimate/mods/CHAO5 - The UN-Balance Mod Returns/standard/staffroll/texture/standard_staffroll_lucas.nutexb", "sd:/ultimate/mods/CHAO5 - The UN-Balance Mod Returns/standard/staffroll/texture/standard_staffroll_claus.nutexb").unwrap();
            rename("sd:/ultimate/mods/CHAO5 - The UN-Balance Mod Returns/standard/staffroll/texture/standard_staffroll_claus.nutexb", "sd:/ultimate/mods/CHAO5 - The UN-Balance Mod Returns/standard/staffroll/texture/standard_staffroll_lucas.nutexb").unwrap();
        }

    }
    if LUCAS {
        if get_stage_id() == BonusGame {
            if metadata("sd:/ultimate/mods/CHAO5 - The UN-Balance Mod Returns/standard/staffroll/texture/standard_staffroll_lucas.nutexb").is_ok() {
                rename("sd:/ultimate/mods/CHAO5 - The UN-Balance Mod Returns/standard/staffroll/texture/standard_staffroll_claus.nutexb", "sd:/ultimate/mods/CHAO5 - The UN-Balance Mod Returns/standard/staffroll/texture/standard_staffroll_lucas.nutexb").unwrap();
                rename("sd:/ultimate/mods/CHAO5 - The UN-Balance Mod Returns/standard/staffroll/texture/standard_staffroll_lucas.nutexb", "sd:/ultimate/mods/CHAO5 - The UN-Balance Mod Returns/standard/staffroll/texture/standard_staffroll_claus.nutexb").unwrap();
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_claus
    );
}