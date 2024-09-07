use crate::imports::BuildImports::*;

#[acmd_script(//SpecialLwStart
    agent = "fox", 
    script = "sound_speciallwstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_fox_speciallwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_fox_special_l01"));
        PLAY_SE(fighter, Hash40::new("se_item_item_get"));
    }
}

#[acmd_script(//SpecialAirLwStart
    agent = "fox", 
    script = "sound_specialairlwstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_fox_specialairlwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_fox_special_l01"));
        PLAY_SE(fighter, Hash40::new("se_item_item_get"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_fox_speciallwstart,
        sound_fox_specialairlwstart,
    );
}