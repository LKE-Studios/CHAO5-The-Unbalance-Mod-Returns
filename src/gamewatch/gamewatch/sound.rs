use crate::imports::BuildImports::*;

//SpecialS6
unsafe extern "C" fn sound_gamewatch_SpecialS6(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gamewatch_special_s01"));
    }
}

//SpecialAirS6
unsafe extern "C" fn sound_gamewatch_SpecialAirS6(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gamewatch_special_s01"));
    }
}

//SpecialS7
unsafe extern "C" fn sound_gamewatch_SpecialS7(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gamewatch_special_s01"));
    }
}

//SpecialAirS7
unsafe extern "C" fn sound_gamewatch_SpecialAirS7(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gamewatch_special_s01"));
    }
}

pub fn install() {
    Agent::new("gamewatch")
    .sound_acmd("sound_specials6", sound_gamewatch_SpecialS6)
    .sound_acmd("sound_specialairs6", sound_gamewatch_SpecialAirS6)
    .sound_acmd("sound_specials7", sound_gamewatch_SpecialS7)
    .sound_acmd("sound_specialairs7", sound_gamewatch_SpecialAirS7)
    .install();
}