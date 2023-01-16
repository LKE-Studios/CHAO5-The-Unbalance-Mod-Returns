//use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::phx::Hash40;
use smash::hash40;
//use smash::phx::Vector3f;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_EFLAME )]
fn eflame_opff(fighter: &mut L2CFighterCommon) {
    unsafe {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_l") {
            if MotionModule::frame(fighter.module_accessor) >= 1.0 && MotionModule::frame(fighter.module_accessor) < 2.0 {
                DamageModule::heal(fighter.module_accessor, -30.0, 0);
            }
        }
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_r") {
            if MotionModule::frame(fighter.module_accessor) >= 1.0 && MotionModule::frame(fighter.module_accessor) < 2.0 {
                DamageModule::heal(fighter.module_accessor, -30.0, 0);
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        eflame_opff
    );
}
