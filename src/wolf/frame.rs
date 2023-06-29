use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_WOLF )]
pub fn wolf_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);

        if status_kind == *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH_END {
            if MotionModule::frame(fighter.module_accessor) > 30.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        wolf_opff
    );
}