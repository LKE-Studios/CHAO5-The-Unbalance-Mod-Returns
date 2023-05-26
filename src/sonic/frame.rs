use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lib::L2CValue;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Hash40;
use smash::phx::Vector3f;

#[fighter_frame( agent = FIGHTER_KIND_SONIC )]
pub fn frame_sonic(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);

        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_N_FALL);
        if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT || status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
            AttackModule::set_power_up(fighter.module_accessor, 1.25);
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -25.0, 0);
            }
        }
        if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW ||
        status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 50);
        }
        else {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            AttackModule::set_power_up(fighter.module_accessor, 1.0);
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_sonic
    );
}