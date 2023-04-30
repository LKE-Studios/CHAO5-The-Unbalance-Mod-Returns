use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Vector3f;

#[fighter_frame( agent = FIGHTER_KIND_MIIGUNNER )]
pub fn frame_miigunner(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        
        if status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL {
            if situation_kind == *SITUATION_KIND_AIR {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                    WorkModule::set_int(fighter.module_accessor, *STATUS_KIND_NONE, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
                }
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_miigunner
    );
}