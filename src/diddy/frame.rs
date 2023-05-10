use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Vector3f;

#[fighter_frame( agent = FIGHTER_KIND_DIDDY )]
pub fn frame_diddy(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let frame = MotionModule::frame(fighter.module_accessor);
        
        if status_kind == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_SHOOT {
            if frame > 5.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                }
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_diddy
    );
}