use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_SHIZUE )]
pub fn frame_shizue(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);

        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                if situation_kind == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                }
            }
        };
        if status_kind == *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END {
            if MotionModule::frame(fighter.module_accessor) > 3.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                    }
                }
            }
        }
    };
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_shizue
    );
}