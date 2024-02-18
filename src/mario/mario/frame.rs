use crate::imports::BuildImports::*;

pub static mut MARIO_GIANT_FIREBALL : [bool; 8] = [false; 8];
pub static mut HOLD_TIME : [f32; 8] = [0.0; 8];

unsafe extern "C" fn frame_mario_Main(fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    frame_common(fighter);
    if MARIO_GIANT_FIREBALL[ENTRY_ID] == true {
        MotionModule::set_rate(fighter.module_accessor, 0.41);
        if MotionModule::frame(fighter.module_accessor) > 14.0 {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        } 
        if MotionModule::frame(fighter.module_accessor) > 16.0 {
            MotionModule::set_rate(fighter.module_accessor, 0.6);
        } 
    };
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            HOLD_TIME[ENTRY_ID] += 1.0;
        }
        if HOLD_TIME[ENTRY_ID] == 30.0 {
            MARIO_GIANT_FIREBALL[ENTRY_ID] = true;
        }
    }
    else {
        HOLD_TIME[ENTRY_ID] = 0.0;
        MARIO_GIANT_FIREBALL[ENTRY_ID] = false;
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    };
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 9 {//Ice Mario Costume c09
        MARIO_GIANT_FIREBALL[ENTRY_ID] = false;
    };
}

pub fn install() {
    Agent::new("mario")
    .on_line(Main, frame_mario_Main)
    .install();
}