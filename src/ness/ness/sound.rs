use crate::imports::BuildImports::*;

//EntryL
unsafe extern "C" fn sound_ness_EntryL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appear01"));
    }
    wait(fighter.lua_state_agent, 51.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
}

//EntryR
unsafe extern "C" fn sound_ness_EntryR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appear01"));
    }
    wait(fighter.lua_state_agent, 51.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
}

//Win3
unsafe extern "C" fn sound_ness_Win3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_smash_s02"));
    }
    frame(fighter.lua_state_agent, 125.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_04"));
    }
    frame(fighter.lua_state_agent, 129.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_damage01"));
    }
}

//SpecialLwStart
unsafe extern "C" fn sound_ness_SpecialLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_004"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_ness_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ness_rnd_futtobi01"), Hash40::new("seq_ness_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_ness_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ness_rnd_futtobi01"), Hash40::new("seq_ness_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_ness_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ness_rnd_futtobi01"), Hash40::new("seq_ness_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_ness_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ness_rnd_futtobi01"), Hash40::new("seq_ness_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_ness_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ness_rnd_futtobi01"), Hash40::new("seq_ness_rnd_futtobi02"));
    }
}

//SpecialAirLwStart
unsafe extern "C" fn sound_ness_SpecialAirLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_004"));
    }
}

pub fn install() {
    Agent::new("ness")
    .sound_acmd("sound_entryl", sound_ness_EntryL, Low)
    .sound_acmd("sound_entryr", sound_ness_EntryR, Low)
    .sound_acmd("sound_speciallwstart", sound_ness_SpecialLwStart, Low)
    .sound_acmd("sound_specialairlwstart", sound_ness_SpecialAirLwStart, Low)
    .sound_acmd("sound_damageflyhi", sound_ness_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_ness_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_ness_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_ness_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_ness_DamageFlyRoll, Low)
    .sound_acmd("sound_win3", sound_ness_Win3, Low)
    .install();
}
