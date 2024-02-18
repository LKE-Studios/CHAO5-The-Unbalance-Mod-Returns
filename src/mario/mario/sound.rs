use crate::imports::BuildImports::*;
use crate::mario::mario::frame::*;

//SpecialN
unsafe extern "C" fn game_mario_SpecialN(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if MARIO_GIANT_FIREBALL[ENTRY_ID] {
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_mario_special_n01"));
            SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_mario_special_n01"), 0.7);
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
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if MARIO_GIANT_FIREBALL[ENTRY_ID] {
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_mario_special_n01"));
            SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_mario_special_n01"), 0.7);
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
