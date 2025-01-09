use crate::imports::BuildImports::*;

//SpecialS
unsafe extern "C" fn sound_wolf_SpecialS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.33);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_wolf_special_s01"));
        PLAY_SE(fighter, Hash40::new("se_wolf_special_s02"));
    }
}

//SpecialAirS
unsafe extern "C" fn sound_wolf_SpecialAirS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.33);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_wolf_special_s01"));
        PLAY_SE(fighter, Hash40::new("se_wolf_special_s02"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_wolf_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_wolf_rnd_futtobi01"), Hash40::new("seq_wolf_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_wolf_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_wolf_rnd_futtobi01"), Hash40::new("seq_wolf_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_wolf_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_wolf_rnd_futtobi01"), Hash40::new("seq_wolf_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_wolf_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_wolf_rnd_futtobi01"), Hash40::new("seq_wolf_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_wolf_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_wolf_rnd_futtobi01"), Hash40::new("seq_wolf_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("wolf")
    .sound_acmd("sound_specials", sound_wolf_SpecialS, Low)
    .sound_acmd("sound_specialairs", sound_wolf_SpecialAirS, Low)
    .sound_acmd("sound_damageflyhi", sound_wolf_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_wolf_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_wolf_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_wolf_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_wolf_DamageFlyRoll, Low)
    .install();
}