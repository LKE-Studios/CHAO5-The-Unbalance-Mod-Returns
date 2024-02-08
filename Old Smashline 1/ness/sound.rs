use crate::imports::BuildImports::*;

#[acmd_script(//EntryL
    agent = "ness", 
    script = "sound_entryl", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_ness_entryl(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//EntryR
    agent = "ness", 
    script = "sound_entryr", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_ness_entryr(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//Win3
    agent = "ness", 
    script = "sound_win3", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_ness_win3(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialLwStart
    agent = "ness", 
    script = "sound_speciallwstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_ness_speciallwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_004"));
    }
}

#[acmd_script(//SpecialAirLwStart
    agent = "ness", 
    script = "sound_specialairlwstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_ness_specialairlwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_004"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_ness_entryl,
        sound_ness_entryr,
        sound_ness_win3,
        sound_ness_speciallwstart,
        sound_ness_specialairlwstart
    );
}
