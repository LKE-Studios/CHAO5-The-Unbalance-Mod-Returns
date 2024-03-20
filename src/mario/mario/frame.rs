use crate::imports::BuildImports::*;

pub static mut MARIO_GIANT_FIREBALL : [bool; 8] = [false; 8];
pub static mut HOLD_TIME : [f32; 8] = [0.0; 8];

unsafe extern "C" fn frame_mario_Main(fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let mut on_frame = WorkModule::get_int(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_BUTTON_ON_FRAME);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_SPECIAL_N_GIANT_FIREBALL) {
        MotionModule::set_rate(fighter.module_accessor, 0.4);
        if frame > 14.0 {
            MotionModule::set_rate(fighter.module_accessor, 1.0); 
        } 
        if frame > 16.0 {
            MotionModule::set_rate(fighter.module_accessor, 0.6);
            if situation_kind == *SITUATION_KIND_AIR {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            }
        } 
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::inc_int(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_BUTTON_ON_FRAME);
        }
        if on_frame >= 30 {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_SPECIAL_N_GIANT_FIREBALL);
        }
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_BUTTON_ON_FRAME)
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 9 {//Ice Mario Costume c09
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_SPECIAL_N_GIANT_FIREBALL);
    };
}

pub fn install() {
    Agent::new("mario")
    .on_line(Main, frame_mario_Main)
    .install();
}