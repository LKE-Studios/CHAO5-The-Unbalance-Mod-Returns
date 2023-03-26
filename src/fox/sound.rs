use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script(//SpecialLwStart
    agent = "fox", 
    script = "sound_speciallwstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_fox_speciallwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_fox_special_l01"));
        macros::PLAY_SE(fighter, Hash40::new("se_item_item_get"));
    }
}

#[acmd_script(//SpecialAirLwStart
    agent = "fox", 
    script = "sound_specialairlwstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_fox_specialairlwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_fox_special_l01"));
        macros::PLAY_SE(fighter, Hash40::new("se_item_item_get"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_fox_speciallwstart,
        sound_fox_specialairlwstart,
    );
}