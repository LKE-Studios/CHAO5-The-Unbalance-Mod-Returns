use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smashline::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;

pub static mut DONKEY_64_DANCE : [bool; 8] = [false; 8];
pub static mut HOLD_TIME : [f32; 8] = [0.0; 8];

#[fighter_frame( agent = FIGHTER_KIND_DONKEY )]
pub fn frame_donkey(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let frame = MotionModule::frame(fighter.module_accessor);

        if DONKEY_64_DANCE[ENTRY_ID] == true {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_r_2"), 0.0, 1.0, false, 0.0, false, false);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        };
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL && [hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                HOLD_TIME[ENTRY_ID] += 1.0;
            }
            if HOLD_TIME[ENTRY_ID] == 30.0 {
                DONKEY_64_DANCE[ENTRY_ID] = true;
            }
        }
        else {
            HOLD_TIME[ENTRY_ID] = 0.0;
            DONKEY_64_DANCE[ENTRY_ID] = false;
        };
        if motion_kind == hash40("appeal_lw_r_2") {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                if frame >= 134.0 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_r_2"), 33.0, 1.0, false, 0.0, false, false);
                }
            }
        }
        if motion_kind == hash40("appeal_lw_l_2") {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                if frame >= 134.0 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_l_2"), 33.0, 1.0, false, 0.0, false, false);
                }
            }
        }
    }
}        

pub fn install() {
    smashline::install_agent_frames!(
        frame_donkey
    );
}