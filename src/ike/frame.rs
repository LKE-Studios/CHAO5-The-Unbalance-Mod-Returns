use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_IKE )]
pub fn frame_ike(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        if status_kind == *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK {
            if MotionModule::frame(fighter.module_accessor) > 11.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                if is_grounded(fighter.module_accessor) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                }
            }
        };
        if status_kind == *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_ike
    );
}