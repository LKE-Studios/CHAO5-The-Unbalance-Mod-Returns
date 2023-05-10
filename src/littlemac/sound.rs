use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script(//AppealHiL, AppealHiR
    agent = "littlemac", 
    scripts = ["sound_appealhil", "sound_appealhir"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn littlemac_uptauntsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_littlemac_appeal04"));
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_littlemac_landing03"));
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_littlemac_swing_m"));
    }
}

#[acmd_script(//AppealLwL, AppealLwR
    agent = "littlemac", 
    scripts = ["sound_appeallwl", "sound_appeallwr"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn littlemac_downtauntsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_littlemac_appeal06"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_littlemac_appeal03"));
	}
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_littlemac_swing_m"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_littlemac_smash_s02"));
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_littlemac_swing_m"));
    }
    frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_littlemac_special_l01"));
    }
}

#[acmd_script(//AppealSL, AppealSR
    agent = "littlemac", 
    scripts = ["sound_appealsl", "sound_appealsr"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn littlemac_sidetauntsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_littlemac_appeal05"));
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_littlemac_appeal_s01"));
    }
    frame(fighter.lua_state_agent, 63.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_littlemac_appeal_s02"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        littlemac_uptauntsfx,
        littlemac_downtauntsfx,
        littlemac_sidetauntsfx
    );
}