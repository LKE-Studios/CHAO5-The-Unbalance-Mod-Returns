use smash::app::sv_animcmd::*;
use smash::hash40;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script(//SpecialLwLaugh
    agent = "diddy", 
    script = "sound_speciallwlaugh", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_diddy_speciallwlaugh(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_diddy_furafura"));
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
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_diddy_furafura"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("vc_diddy_furafura"), 1.1);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_diddy_speciallwlaugh,
        sound_diddy_specialairlwlaugh
    );
}