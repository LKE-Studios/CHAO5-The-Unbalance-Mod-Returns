use crate::imports::BuildImports::*;

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
    .effect_acmd("effect_specialn", effect_shizue_SpecialN)
    .effect_acmd("effect_specialairn", effect_shizue_SpecialAirN)
    .install();
}