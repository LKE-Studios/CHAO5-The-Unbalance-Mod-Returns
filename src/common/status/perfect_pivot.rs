use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash_script::*;
use smashline::*;
use smash::app::*;
use smash::hash40;
use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::Vector3f;
use smash::phx::Vector2f;
use smash::lib::L2CValue;

#[fighter_frame_callback]
pub fn frame_perfect_pivot(fighter : &mut L2CFighterCommon) {
    unsafe {
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        let lr = PostureModule::lr(fighter.module_accessor);

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("dash") && MotionModule::frame(fighter.module_accessor) <= (40.0) {
            if stick_x * lr < -0.25 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN, true);
            }
        };
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("turn_dash") && MotionModule::frame(fighter.module_accessor) <= (40.0) {
            if stick_x * lr < -0.25 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN, true);
            }
        };
    }
}

pub fn install() {
    install_agent_frame_callbacks!(
        frame_perfect_pivot
    );
}