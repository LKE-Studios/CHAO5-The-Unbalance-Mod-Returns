use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lua2cpp::L2CFighterCommon;
//use smash::phx::Hash40;
//use smash::hash40;
//use smash::phx::Vector3f;

#[fighter_frame( agent = FIGHTER_KIND_RICHTER )]
pub fn richter_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        //let kind = smash::app::utility::get_kind(boma);   
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if situation_kind == *SITUATION_KIND_AIR {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                }
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if MotionModule::frame(fighter.module_accessor) > 32.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        richter_opff
    );
}