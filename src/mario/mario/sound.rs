use crate::imports::BuildImports::*;
use crate::mario::mario::frame::*;

//SpecialN
unsafe extern "C" fn game_mario_SpecialN(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_SPECIAL_N_GIANT_FIREBALL) {
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_mario_special_n01"));
            SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_mario_special_n01"), 0.8);
        }
    }
    else { 
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_mario_special_n01"));
        }
    }
}

//SpecialAirN
unsafe extern "C" fn game_mario_SpecialAirN(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_SPECIAL_N_GIANT_FIREBALL) {
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_mario_special_n01"));
            SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_mario_special_n01"), 0.8);
        }
    }
    else { 
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_mario_special_n01"));
        }
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_mario_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_mario_rnd_futtobi01"), Hash40::new("seq_mario_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_mario_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_mario_rnd_futtobi01"), Hash40::new("seq_mario_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_mario_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_mario_rnd_futtobi01"), Hash40::new("seq_mario_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_mario_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_mario_rnd_futtobi01"), Hash40::new("seq_mario_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_mario_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_mario_rnd_futtobi01"), Hash40::new("seq_mario_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("mario")
    .sound_acmd("sound_specialn", game_mario_SpecialN, Low)
    .sound_acmd("sound_specialairn", game_mario_SpecialAirN, Low)
    .sound_acmd("sound_damageflyhi", sound_mario_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_mario_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_mario_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_mario_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_mario_DamageFlyRoll, Low)
    .install();
}
