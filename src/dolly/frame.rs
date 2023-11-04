use crate::imports::BuildImports::*;
use crate::waluigi::frame::*;

#[fighter_frame( agent = FIGHTER_KIND_DOLLY )]
pub fn frame_dolly(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let DOLLY = color >= 0 && color <= 16; 
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW);
        if DOLLY {
            if status_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP {
                fighter.sub_air_check_fall_common();
                if MotionModule::frame(fighter.module_accessor) > 40.0 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                    if is_grounded(fighter.module_accessor) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                    }
                }
            };
            if status_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND {
                fighter.sub_air_check_fall_common();
                if MotionModule::frame(fighter.module_accessor) > 40.0 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                    if is_grounded(fighter.module_accessor) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                    }
                }
            };
        }
        //WALUIGI
        let WALUIGI = color >= 120 && color <= 130; 
        if WALUIGI {
            scale_waluigi(fighter);
            attack_dash_waluigi(fighter);
            special_n_waluigi(fighter);
            special_s_waluigi(fighter);
            final_start_waluigi(fighter);
            //slap_cancel_waluigi(fighter);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_dolly
    );
}