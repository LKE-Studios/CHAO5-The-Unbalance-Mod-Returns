use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_SHIZUE )]
pub fn shizue_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                if situation_kind == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                }
            }
        };
        if status_kind == *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END {
            if MotionModule::frame(fighter.module_accessor) > 3.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                    }
                }
            }
        }
    };
}

pub fn install() {
    smashline::install_agent_frames!(
        shizue_opff
    );
}