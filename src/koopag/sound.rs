use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script(//Run
    agent = "koopag", 
    script = "sound_run", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn koopag_runsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_koopag_001"));
    }
}

#[acmd_script(//SpecialNStart
    agent = "koopag", 
    script = "sound_specialnstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn koopag_neutralbsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_koopag_special_n01"));
    }
    wait(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_koopag_step_left_m"));
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_koopag_special_n03"));
    }
}

#[acmd_script(//SpecialAirNStart
    agent = "koopag", 
    script = "sound_specialairnstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn koopag_neutralbairsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_koopag_special_n01"));
    }
    wait(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_koopag_special_n03"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        koopag_neutralbsfx,
        koopag_neutralbairsfx,
        koopag_runsfx
    );
}