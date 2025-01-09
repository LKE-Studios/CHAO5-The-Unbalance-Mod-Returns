use crate::imports::BuildImports::*;

//AttackLw3
unsafe extern "C" fn sound_daisy_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_daisy_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_daisy_attackhard_l01"));
    }
}

//AttackAirN
unsafe extern "C" fn sound_daisy_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_daisy_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_daisy_attackair_n01"));
    }
}

//AttackAirF
unsafe extern "C" fn sound_daisy_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_daisy_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_daisy_attackair_f01"));
    }
}

//AttackAirLw
unsafe extern "C" fn sound_daisy_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_daisy_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_daisy_attackair_l01"));
    }
}    

//DamageFlyHi
unsafe extern "C" fn sound_daisy_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_daisy_rnd_futtobi01"), Hash40::new("seq_daisy_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_daisy_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_daisy_rnd_futtobi01"), Hash40::new("seq_daisy_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_daisy_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_daisy_rnd_futtobi01"), Hash40::new("seq_daisy_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_daisy_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_daisy_rnd_futtobi01"), Hash40::new("seq_daisy_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_daisy_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_daisy_rnd_futtobi01"), Hash40::new("seq_daisy_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("daisy")
    .sound_acmd("sound_attacklw3", sound_daisy_AttackLw3, Low)
    .sound_acmd("sound_attackairn", sound_daisy_AttackAirN, Low)
    .sound_acmd("sound_attackairf", sound_daisy_AttackAirF, Low)
    .sound_acmd("sound_attackairlw", sound_daisy_AttackAirLw, Low)
    .sound_acmd("sound_damageflyhi", sound_daisy_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_daisy_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_daisy_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_daisy_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_daisy_DamageFlyRoll, Low)
    .install();
}