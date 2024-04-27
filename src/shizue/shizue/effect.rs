use crate::imports::BuildImports::*;

//AttackAirB
unsafe extern "C" fn effect_shizue_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new_raw(0x1288a621ad), Hash40::new_raw(0x1288a621ad), Hash40::new("top"), -3, 0, 0, 13, 180, 180, 1.6, false, *EF_FLIP_YZ);
    }
}

//SpecialN
unsafe extern "C" fn effect_shizue_SpecialN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new_raw(0x1408ec2771), Hash40::new("top"), 0, 9, 7.3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//SpecialAirN
unsafe extern "C" fn effect_shizue_SpecialAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new_raw(0x1408ec2771), Hash40::new("top"), 0, 9, 7.3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    Agent::new("shizue")
    .effect_acmd("effect_attackairb", effect_shizue_AttackAirB, Low)
    .effect_acmd("effect_specialn", effect_shizue_SpecialN, Low)
    .effect_acmd("effect_specialairn", effect_shizue_SpecialAirN, Low)
    .install();
}