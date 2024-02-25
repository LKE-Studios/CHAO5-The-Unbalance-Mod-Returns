use crate::imports::BuildImports::*;

//SpecialLw
unsafe extern "C" fn sound_falco_SpecialLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_special_l01"));
        PLAY_SE(fighter, Hash40::new("se_item_item_get"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_falco_special_l01"));
    }
    wait(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_special_l03"));
    }
}

//SpecialAirLw
unsafe extern "C" fn sound_falco_SpecialAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_special_l01"));
        PLAY_SE(fighter, Hash40::new("se_item_item_get"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_falco_special_l01"));
    }
    wait(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_special_l03"));
    }
}

pub fn install() {
    Agent::new("falco")
    .sound_acmd("sound_speciallw", sound_falco_SpecialLw)
    .sound_acmd("sound_specialairlw", sound_falco_SpecialAirLw)
    .install();
}