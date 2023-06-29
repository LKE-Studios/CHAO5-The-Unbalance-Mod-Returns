use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_SIMON )]
pub fn frame_simon(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);  
        
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if MotionModule::frame(fighter.module_accessor) > 32.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_simon
    );
}