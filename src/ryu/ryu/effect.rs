use crate::imports::BuildImports::*;

//Kamehameha_Fire
unsafe extern "C" fn effect_ryu_Kamehameha_Fire(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("top"), 10, 9, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 67.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_genesis_beam"), true, true);
    }
}

pub fn install() {
    Agent::new("ryu")
    .effect_acmd("effect_kamehameha_fire", effect_ryu_Kamehameha_Fire)
    .install();
}