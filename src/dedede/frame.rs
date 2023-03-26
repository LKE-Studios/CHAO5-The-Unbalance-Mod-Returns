use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::phx::Vector2f;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smashline::*;
use smash_script::*;
use smash::hash40;

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