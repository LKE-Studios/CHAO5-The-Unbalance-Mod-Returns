use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//AttackS4Hi
    agent = "mariod", 
    script = "sound_attacks4hi", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_mariod_attacks4hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        macros::PLAY_SE(fighter, Hash40::new("vc_mariod_attack05"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_mariod_smash_s01"));
    }
}

#[acmd_script(//AttackS4
    agent = "mariod", 
    script = "sound_attacks4", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_mariod_attacks4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        macros::PLAY_SE(fighter, Hash40::new("vc_mariod_attack05"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_mariod_smash_s01"));
    }
}

#[acmd_script(//AttackS4Lw
    agent = "mariod", 
    script = "sound_attacks4lw", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_mariod_attacks4lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        macros::PLAY_SE(fighter, Hash40::new("vc_mariod_attack05"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_mariod_smash_s01"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_mariod_attacks4hi,
        sound_mariod_attacks4,
        sound_mariod_attacks4lw
    );
}