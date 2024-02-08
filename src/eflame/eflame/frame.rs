use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_eflame(fighter: &mut L2CFighterCommon) {
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

pub fn install() {
    Agent::new("eflame")
    .on_line(Main, frame_eflame)
    .install();
}