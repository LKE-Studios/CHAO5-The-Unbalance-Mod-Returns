use crate::imports::BuildImports::*;

#[acmd_script(//SpecialS6
    agent = "gamewatch", 
    script = "sound_specials6", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_gamewatch_specials6(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gamewatch_special_s01"));
    }
}

#[acmd_script(//SpecialAirS6
    agent = "gamewatch", 
    script = "sound_specialairs6", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_gamewatch_specialairs6(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gamewatch_special_s01"));
    }
}

#[acmd_script(//SpecialS7
    agent = "gamewatch",
    script = "sound_specials7", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_gamewatch_specials7(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gamewatch_special_s01"));
    }
}

#[acmd_script(//SpecialAirS7
    agent = "gamewatch", 
    script = "sound_specialairs7", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_gamewatch_specialairs7(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gamewatch_special_s01"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_gamewatch_specials6,
        sound_gamewatch_specialairs6,
        sound_gamewatch_specials7,
        sound_gamewatch_specialairs7
    );
}