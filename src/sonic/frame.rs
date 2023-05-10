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
        //let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_N_FALL);
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
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
            ModelModule::set_scale(fighter.module_accessor, 1.05);
            AttackModule::set_attack_scale(fighter.module_accessor, 1.05, true);
            GrabModule::set_size_mul(fighter.module_accessor, 1.05);

            if [*FIGHTER_STATUS_KIND_DAMAGE, 
            *FIGHTER_STATUS_KIND_DAMAGE_AIR, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY, 
            *FIGHTER_STATUS_KIND_DAMAGE_FALL, 
            *FIGHTER_STATUS_KIND_DAMAGE_SONG, 
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, 
            *FIGHTER_STATUS_KIND_DAMAGE_SONG_FALL, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, 
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD, 
            *FIGHTER_STATUS_KIND_ICE
            ].contains(&status_kind) {
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_merikomi"), true, true);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_sonic
    );
}