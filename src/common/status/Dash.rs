use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_common_dash(fighter : &mut L2CFighterCommon) {
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
    smashline::api::install_line_callback(None, StatusLine::Main, frame_common_dash as _);
}