use crate::imports::BuildImports::*;

//SpecialLwLaugh
unsafe extern "C" fn sound_diddy_SpecialLwLaugh(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_diddy_furafura"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("vc_diddy_furafura"), 1.1);
    }
}

//SpecialAirLwLaugh
unsafe extern "C" fn sound_diddy_SpecialAirLwLaugh(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_diddy_furafura"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("vc_diddy_furafura"), 1.1);
    }
}

pub fn install() {
    Agent::new("diddy")
    .sound_acmd("sound_speciallwlaugh", sound_diddy_SpecialLwLaugh)
    .sound_acmd("sound_specialairlwlaugh", sound_diddy_SpecialAirLwLaugh)
    .install();
}