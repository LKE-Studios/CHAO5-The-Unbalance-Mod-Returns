use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_PACKUN )]
pub fn frame_packun(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            fighter.sub_air_check_fall_common();
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
            }
        };
        if status_kind == *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_WORK_INT_CANCEL_TYPE) == *FIGHTER_PACKUN_SPECIAL_S_CANCEL_TYPE_AIR_ESCAPE_AIR {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_PACKUN_SPECIAL_S_CANCEL_TYPE_NONE, *FIGHTER_PACKUN_STATUS_SPECIAL_S_WORK_INT_CANCEL_TYPE);
                }
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_packun
    );
}