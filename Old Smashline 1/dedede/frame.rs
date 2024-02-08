use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_DEDEDE )]
fn frame_dedede(fighter: &mut L2CFighterCommon) {
    unsafe {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("jump_aerial_f9") {
            if MotionModule::frame(fighter.module_accessor) == 55.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_dedede
    );
}