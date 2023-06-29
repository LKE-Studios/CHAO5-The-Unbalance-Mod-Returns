use crate::imports::BuildImports::*;

pub static mut CLAUS_WESS_DANCE : [bool; 8] = [false; 8];
pub static mut HOLD_TIME : [f32; 8] = [0.0; 8];

#[fighter_frame( agent = FIGHTER_KIND_CLAUS )]
pub fn frame_claus(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        if CLAUS_WESS_DANCE[ENTRY_ID] == true {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("wess_dance"), 0.0, 1.0, false, 0.0, false, false);
        }
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL && [hash40("appeal_hi_r"), hash40("appeal_hi_l")].contains(&motion_kind) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                HOLD_TIME[ENTRY_ID] += 1.0;
            }
            if HOLD_TIME[ENTRY_ID] == 30.0 {
                CLAUS_WESS_DANCE[ENTRY_ID] = true;
            }
        }
        else {
            HOLD_TIME[ENTRY_ID] = 0.0;
            CLAUS_WESS_DANCE[ENTRY_ID] = false;
        };
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
        WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
        }
    }
}        

pub fn install() {
    smashline::install_agent_frames!(
        frame_claus
    );
}