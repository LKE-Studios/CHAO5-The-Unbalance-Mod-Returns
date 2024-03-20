use crate::imports::BuildImports::*;

//AttackS4Hi
unsafe extern "C" fn sound_mariod_AttackS4Hi(fighter: &mut L2CAgentBase) {
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

//AttackS4
unsafe extern "C" fn sound_mariod_AttackS4(fighter: &mut L2CAgentBase) {
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

//AttackS4Lw
unsafe extern "C" fn sound_mariod_AttackS4Lw(fighter: &mut L2CAgentBase) {
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
    Agent::new("mariod")
    .sound_acmd("sound_attacks4hi", sound_mariod_AttackS4Hi)
    .sound_acmd("sound_attacks4", sound_mariod_AttackS4)
    .sound_acmd("sound_attacks4lw", sound_mariod_AttackS4Lw)
    .install();
}