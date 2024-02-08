use crate::imports::BuildImports::*;

#[acmd_script(//AttackS4Hi
    agent = "mariod", 
    script = "sound_attacks4hi", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_mariod_attacks4hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        PLAY_SE(fighter, Hash40::new("vc_mariod_attack05"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mariod_smash_s01"));
    }
}

#[acmd_script(//AttackS4
    agent = "mariod", 
    script = "sound_attacks4", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_mariod_attacks4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        PLAY_SE(fighter, Hash40::new("vc_mariod_attack05"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mariod_smash_s01"));
    }
}

#[acmd_script(//AttackS4Lw
    agent = "mariod", 
    script = "sound_attacks4lw", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_mariod_attacks4lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        PLAY_SE(fighter, Hash40::new("vc_mariod_attack05"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mariod_smash_s01"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_mariod_attacks4hi,
        sound_mariod_attacks4,
        sound_mariod_attacks4lw
    );
}