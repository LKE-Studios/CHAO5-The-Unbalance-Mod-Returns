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

pub fn install() {
    Agent::new("fox")
    .sound_acmd("sound_speciallwstart", sound_fox_SpecialLwStart, Low)
    .sound_acmd("sound_specialairlwstart", sound_fox_SpecialAirLwStart, Low)
    .install();
}