use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_WOLF )]
pub fn wolf_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        //let kind = smash::app::utility::get_kind(boma); 
        if status_kind == *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH_END {
            if MotionModule::frame(fighter.module_accessor) > 30.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        wolf_opff
    );
}