use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script(//SpecialS2SFX
    agent = "gamewatch", 
    script = "sound_specials2", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn gamewatch_sideb2sfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_gamewatch_special_s01"));
    }
}

#[acmd_script(//SpecialAirS2SFX
    agent = "gamewatch", 
    script = "sound_specialairs2", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn gamewatch_sidebair2sfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_gamewatch_special_s01"));
    }
}

#[acmd_script(//SpecialS6SFX
    agent = "gamewatch", 
    script = "sound_specials6", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn gamewatch_sideb6sfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_gamewatch_special_s01"));
    }
}

#[acmd_script(//SpecialAirS6SFX
    agent = "gamewatch", 
    script = "sound_specialairs6", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn gamewatch_sidebair6sfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_gamewatch_special_s01"));
    }
}

#[acmd_script(//SpecialS7SFX
    agent = "gamewatch", 
    script = "sound_specials7", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn gamewatch_sideb7sfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_gamewatch_special_s01"));
    }
}

#[acmd_script(//SpecialAirS7SFX
    agent = "gamewatch", 
    script = "sound_specialairs7", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn gamewatch_sidebair7sfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_gamewatch_special_s01"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        gamewatch_sideb2sfx,
        gamewatch_sidebair2sfx,
        gamewatch_sideb6sfx,
        gamewatch_sidebair6sfx,
        gamewatch_sideb7sfx,
        gamewatch_sidebair7sfx
    );
}