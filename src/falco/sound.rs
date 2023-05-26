use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script(//SpecialLw
    agent = "falco", 
    script = "sound_speciallw", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_falco_speciallw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_falco_special_l01"));
        macros::PLAY_SE(fighter, Hash40::new("se_item_item_get"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_falco_special_l01"));
    }
    wait(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_falco_special_l03"));
    }
}

#[acmd_script(//SpecialAirLw
    agent = "falco", 
    script = "sound_specialairlw", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_falco_specialairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_falco_special_l01"));
        macros::PLAY_SE(fighter, Hash40::new("se_item_item_get"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_falco_special_l01"));
    }
    wait(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_falco_special_l03"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_falco_speciallw,
        sound_falco_specialairlw
    );
}