use crate::imports::BuildImports::*;

#[acmd_script(//SpecialLwLaugh
    agent = "diddy", 
    script = "sound_speciallwlaugh", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_diddy_speciallwlaugh(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_diddy_furafura"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("vc_diddy_furafura"), 1.1);
    }
}

#[acmd_script(//SpecialAirLwLaugh
    agent = "diddy", 
    script = "sound_specialairlwlaugh", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_diddy_specialairlwlaugh(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_diddy_furafura"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("vc_diddy_furafura"), 1.1);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_diddy_speciallwlaugh,
        sound_diddy_specialairlwlaugh
    );
}