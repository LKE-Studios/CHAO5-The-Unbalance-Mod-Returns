use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
//use smash::lib::L2CValue;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_SONIC )]
pub fn sonic_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        if [
            *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING_START,
            *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING,
            *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT,
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD,
            *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH,
            *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP,
            *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD
        ].contains(&status_kind) {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        };
        if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT || status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
            AttackModule::set_power_up(fighter.module_accessor, 1.25);
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -25.0, 0);
            }
        }
        else {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            AttackModule::set_power_up(fighter.module_accessor, 1.0);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        sonic_opff
    );
}