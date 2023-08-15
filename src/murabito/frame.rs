use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_MURABITO )]
pub fn frame_murabito(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_FALL {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        }
        if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_JUMP {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        }
        if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_AIR, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_DASH_B, 
            *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_DASH_F, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP,
            *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP_SQUAT,  *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_LANDING,
            *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WAIT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_B,
            *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_BRAKE_B, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_BRAKE_F,
            *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_F].contains(&status_kind) {
            if frame > 12.0 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD){
                if situation_kind == *SITUATION_KIND_AIR {
                    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                    ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                }
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                }
            }
        }
    };
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_murabito
    );
}