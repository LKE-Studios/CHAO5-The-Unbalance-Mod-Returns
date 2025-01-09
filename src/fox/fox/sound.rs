use crate::imports::BuildImports::*;

//SpecialLwStart
unsafe extern "C" fn sound_fox_SpecialLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_fox_special_l01"));
        PLAY_SE(fighter, Hash40::new("se_item_item_get"));
    }
}

//SpecialAirLwStart
unsafe extern "C" fn sound_fox_SpecialAirLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_fox_special_l01"));
        PLAY_SE(fighter, Hash40::new("se_item_item_get"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_fox_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_fox_rnd_futtobi01"), Hash40::new("seq_fox_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_fox_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_fox_rnd_futtobi01"), Hash40::new("seq_fox_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_fox_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_fox_rnd_futtobi01"), Hash40::new("seq_fox_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_fox_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_fox_rnd_futtobi01"), Hash40::new("seq_fox_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_fox_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_fox_rnd_futtobi01"), Hash40::new("seq_fox_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("fox")
    .sound_acmd("sound_speciallwstart", sound_fox_SpecialLwStart, Low)
    .sound_acmd("sound_specialairlwstart", sound_fox_SpecialAirLwStart, Low)
    .sound_acmd("sound_damageflyhi", sound_fox_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_fox_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_fox_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_fox_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_fox_DamageFlyRoll, Low)
    .install();
}