use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script(//EntryL
    agent = "ness", 
    script = "sound_entryl", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn ness_entryl(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_appear01"));
    }
    wait(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
        macros::PLAY_SE(fighter, Hash40::new("vc_ness_heavyget"));
    }
}

#[acmd_script(//EntryR
    agent = "ness", 
    script = "sound_entryr", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn ness_entryr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_appear01"));
    }
    wait(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
        macros::PLAY_SE(fighter, Hash40::new("vc_ness_heavyget"));
    }
}

#[acmd_script(//Win3
    agent = "ness", 
    script = "sound_win3", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn ness_win(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_smash_s02"));
    }
    frame(fighter.lua_state_agent, 125.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_04"));
    }
    frame(fighter.lua_state_agent, 129.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_ness_damage01"));
    }
}

#[acmd_script(//SpecialLwStart
    agent = "ness", 
    script = "sound_speciallwstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn ness_downbsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_ness_004"));
    }
}

#[acmd_script(//SpecialAirLwStart
    agent = "ness", 
    script = "sound_specialairlwstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn ness_downbairsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_ness_004"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        ness_entryl,
        ness_entryr,
        ness_win,
        ness_downbsfx,
        ness_downbairsfx
    );
}
