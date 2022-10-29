use smash::lib::lua_const::*;
//use smash::app::lua_bind::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
//use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_TANTAN )]
pub fn tantan_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
            shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 12.0, 0.0, 0.0, 0.0, 0.0, 0.0, 15.0, 3.0, 2.0, 5000, false, 7, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        }
        else {
            shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        else {
            damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        tantan_opff
    );
}