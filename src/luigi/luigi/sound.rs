use crate::imports::BuildImports::*;

//SpecialSHold
unsafe extern "C" fn sound_luigi_SpecialSHold(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        sound!(fighter, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        PLAY_STATUS(fighter, Hash40::new("se_luigi_special_s02"));
    }
}

//SpecialAirSHold
unsafe extern "C" fn sound_luigi_SpecialAirSHold(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        sound!(fighter, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        PLAY_STATUS(fighter, Hash40::new("se_luigi_special_s02"));
    }
}


//DamageFlyHi
unsafe extern "C" fn sound_luigi_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_luigi_rnd_futtobi01"), Hash40::new("seq_luigi_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_luigi_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_luigi_rnd_futtobi01"), Hash40::new("seq_luigi_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_luigi_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_luigi_rnd_futtobi01"), Hash40::new("seq_luigi_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_luigi_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_luigi_rnd_futtobi01"), Hash40::new("seq_luigi_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_luigi_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_luigi_rnd_futtobi01"), Hash40::new("seq_luigi_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("luigi")
    .sound_acmd("sound_specialshold", sound_luigi_SpecialSHold, Low)
    .sound_acmd("sound_specialairshold", sound_luigi_SpecialAirSHold, Low)
    .sound_acmd("sound_damageflyhi", sound_luigi_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_luigi_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_luigi_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_luigi_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_luigi_DamageFlyRoll, Low)
    .install();
}