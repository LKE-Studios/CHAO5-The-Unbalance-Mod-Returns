use crate::imports::BuildImports::*;

//AttackS4Charge
unsafe extern "C" fn sound_shizue_AttackS4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
}

//AttackHi4Charge
unsafe extern "C" fn sound_shizue_AttackHi4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
}

//AttackLw4Charge
unsafe extern "C" fn sound_shizue_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_attackhard_h01"));
    }
}

//AttackAirB
unsafe extern "C" fn sound_shizue_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new_raw(0x1288a621ad), Hash40::new_raw(0x1288a621ad), Hash40::new("top"), -3, 0, 0, 13, 180, 180, 1.6, false, *EF_FLIP_YZ);
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
    Agent::new("shizue")
    .sound_acmd("sound_attacks4charge", sound_shizue_AttackS4Charge)
    .sound_acmd("sound_attackhi4charge", sound_shizue_AttackHi4Charge)
    .sound_acmd("sound_attacklw4charge", sound_shizue_AttackLw4Charge)
    .sound_acmd("sound_attackairb", sound_shizue_AttackAirB)
    .sound_acmd("sound_specialn", sound_shizue_SpecialN)
    .sound_acmd("sound_specialairn", sound_shizue_SpecialAirN)
    .install();
}