use crate::imports::BuildImports::*;

//SpecialHi2
unsafe extern "C" fn sound_donkey_SpecialHi2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h01"));
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h04"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h05"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h06"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h07"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h07"));
    }
}

//SpecialAirHi
unsafe extern "C" fn sound_donkey_SpecialAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_donkey_special_h02"));
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h04"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h05"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h06"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h07"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h07"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h07"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h07"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h07"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h07"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_donkey_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_donkey_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_donkey_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_donkey_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_donkey_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("donkey")
    .sound_acmd("sound_specialhi2", sound_donkey_SpecialHi2, Low)
    .sound_acmd("sound_specialairhi", sound_donkey_SpecialAirHi, Low)
    .sound_acmd("sound_damageflyhi", sound_donkey_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_donkey_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_donkey_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_donkey_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_donkey_DamageFlyRoll, Low)
    .install();
}