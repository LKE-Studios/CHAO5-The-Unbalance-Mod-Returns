use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
//use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_JACK )]
pub fn jack_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        if status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW2_COUNTER {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -66.0, 0);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        jack_opff
    );
}