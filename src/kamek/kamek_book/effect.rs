use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn effect_kamek_book_Regular(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, -10, 0, 0, 90, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
}

//Stick
unsafe extern "C" fn effect_kamek_book_Stick(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    Agent::new("ness_book")
    .effect_acmd("effect_regular", effect_kamek_book_Regular, Low)
    .effect_acmd("effect_stick", effect_kamek_book_Stick, Low)
    .install();
}