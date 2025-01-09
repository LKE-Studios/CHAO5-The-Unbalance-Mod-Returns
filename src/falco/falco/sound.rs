use crate::imports::BuildImports::*;

//SpecialLw
unsafe extern "C" fn sound_falco_SpecialLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_special_l01"));
        PLAY_SE(fighter, Hash40::new("se_item_item_get"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_falco_special_l01"));
    }
    wait(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_special_l03"));
    }
}

//SpecialAirLw
unsafe extern "C" fn sound_falco_SpecialAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_special_l01"));
        PLAY_SE(fighter, Hash40::new("se_item_item_get"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_falco_special_l01"));
    }
    wait(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_special_l03"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_falco_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_falco_rnd_futtobi01"), Hash40::new("seq_falco_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_falco_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_falco_rnd_futtobi01"), Hash40::new("seq_falco_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_falco_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_falco_rnd_futtobi01"), Hash40::new("seq_falco_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_falco_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_falco_rnd_futtobi01"), Hash40::new("seq_falco_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_falco_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_falco_rnd_futtobi01"), Hash40::new("seq_falco_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("falco")
    .sound_acmd("sound_speciallw", sound_falco_SpecialLw, Low)
    .sound_acmd("sound_specialairlw", sound_falco_SpecialAirLw, Low)
    .sound_acmd("sound_damageflyhi", sound_falco_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_falco_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_falco_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_falco_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_falco_DamageFlyRoll, Low)
    .install();
}