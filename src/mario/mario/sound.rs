use crate::imports::BuildImports::*;
use crate::mario::mario::frame::*;

//SpecialN
unsafe extern "C" fn game_mario_SpecialN(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_SPECIAL_N_GIANT_FIREBALL) {
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
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_SPECIAL_N_GIANT_FIREBALL) {
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

pub fn install() {
    Agent::new("mario")
    .sound_acmd("sound_specialn", game_mario_SpecialN)
    .sound_acmd("sound_specialairn", game_mario_SpecialAirN)
    .install();
}
