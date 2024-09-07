use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_captain_Main(fighter : &mut L2CFighterCommon) {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw") {
        let cat = fighter.global_table[CMD_CAT1].get_int() as i32;
        if (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_LEFT) != 0 && GroundModule::get_touch_flag(fighter.module_accessor) == *GROUND_TOUCH_FLAG_LEFT as u64 || 
        (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_RIGHT) != 0 && GroundModule::get_touch_flag(fighter.module_accessor) == *GROUND_TOUCH_FLAG_RIGHT as u64 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
        }
    }
}

pub fn install() {
    Agent::new("captain")
    .on_line(Main, frame_captain_Main)
    .install();
}