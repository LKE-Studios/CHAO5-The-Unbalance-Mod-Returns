use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn effect_bandana_spear_Fly(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("younglink_arrow_trace"), Hash40::new("top"), 0, 0, -14, 0, 0, 0, 1.05, true);
    }
}

//Stick
unsafe extern "C" fn effect_bandana_spear_Stick(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("younglink_arrow_trace"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("younglink_arrow"), true, true);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -1, 90, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    Agent::new("edge_spear")
    .effect_acmd("effect_fly", effect_bandana_spear_Fly, Low)
    .effect_acmd("effect_stick", effect_bandana_spear_Stick, Low)
    .install();
}