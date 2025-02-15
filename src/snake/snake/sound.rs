use crate::imports::BuildImports::*;

//AttackS4
unsafe extern "C" fn sound_snake_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_smash_s01"));
    }
    frame(fighter.lua_state_agent, 40.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_S4_IS_CHARGED) {
        frame(fighter.lua_state_agent, 41.0);
        if is_excute(fighter) {
            STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
            PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack_smash_s"));
            PLAY_SE(fighter, Hash40::new("se_snake_smash_s02"));
        }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_snake_smash_s02"));
        }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_snake_smash_s02"));
        }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_snake_smash_s02"));
        }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_snake_smash_s02"));
        }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_snake_smash_s02"));
        }
    }
    else {
        frame(fighter.lua_state_agent, 41.0);
        if is_excute(fighter) {
            STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
            PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack_smash_s"));
            PLAY_SE(fighter, Hash40::new("se_snake_smash_s02"));
        }
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_snake_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_snake_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_snake_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_snake_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_snake_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("snake")
    .sound_acmd("sound_attacks4", sound_snake_AttackS4, Low)
    .sound_acmd("sound_damageflyhi", sound_snake_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_snake_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_snake_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_snake_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_snake_DamageFlyRoll, Low)
    .install();
}