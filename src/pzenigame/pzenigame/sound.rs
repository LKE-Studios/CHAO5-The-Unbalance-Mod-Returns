use crate::imports::BuildImports::*;

//SpecialZ
unsafe extern "C" fn sound_pzenigame_SpecialZ(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pzenigame_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pzenigame_bubble"));
        PLAY_SE(fighter, Hash40::new("se_pzenigame_swing_m"));
    }
}

//SpecialAirZ
unsafe extern "C" fn sound_pzenigame_SpecialAirZ(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pzenigame_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pzenigame_bubble"));
        PLAY_SE(fighter, Hash40::new("se_pzenigame_swing_m"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_pzenigame_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pzenigame_rnd_futtobi01"), Hash40::new("seq_pzenigame_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_pzenigame_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pzenigame_rnd_futtobi01"), Hash40::new("seq_pzenigame_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_pzenigame_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pzenigame_rnd_futtobi01"), Hash40::new("seq_pzenigame_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_pzenigame_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pzenigame_rnd_futtobi01"), Hash40::new("seq_pzenigame_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_pzenigame_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pzenigame_rnd_futtobi01"), Hash40::new("seq_pzenigame_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("pzenigame")
    .sound_acmd("sound_specialz", sound_pzenigame_SpecialZ, Low)
    .sound_acmd("sound_specialairz", sound_pzenigame_SpecialAirZ, Low)
    .sound_acmd("sound_damageflyhi", sound_pzenigame_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_pzenigame_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_pzenigame_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_pzenigame_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_pzenigame_DamageFlyRoll, Low)
    .install();
}