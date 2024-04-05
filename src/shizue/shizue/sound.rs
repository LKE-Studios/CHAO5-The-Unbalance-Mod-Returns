use crate::imports::BuildImports::*;

//AttackS4Charge
unsafe extern "C" fn sound_samusd_AttackS4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
}

//AttackHi4Charge
unsafe extern "C" fn sound_samusd_AttackHi4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
}

//AttackLw4Charge
unsafe extern "C" fn sound_samusd_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
}

//SpecialN
unsafe extern "C" fn sound_shizue_SpecialN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_attackdash_01"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_special_n02"));
    }
}

//SpecialAirN
unsafe extern "C" fn sound_shizue_SpecialAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_attackdash_01"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_special_n02"));
    }
}

pub fn install() {
    Agent::new("samusd")
    .sound_acmd("sound_attacks4charge", sound_samusd_AttackS4Charge)
    .sound_acmd("sound_attackhi4charge", sound_samusd_AttackHi4Charge)
    .sound_acmd("sound_attacklw4charge", sound_samusd_AttackLw4Charge)
    .sound_acmd("sound_specialn", sound_shizue_SpecialN)
    .sound_acmd("sound_specialairn", sound_shizue_SpecialAirN)
    .install();
}