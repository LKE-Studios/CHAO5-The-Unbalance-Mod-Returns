use crate::imports::BuildImports::*;

//SpecialS6
unsafe extern "C" fn effect_gamewatch_SpecialS6(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 0..12 {
        wait(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("rot"), 0, 2, 0, 0, 0, 0, 1.2, 16, 20, 16, 0, 0, 0, false);
        }
    }
}

//SpecialAirS6
unsafe extern "C" fn effect_gamewatch_SpecialAirS6(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    for _ in 0..12 {
        wait(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("rot"), 0, 2, 0, 0, 0, 0, 1.2, 16, 20, 16, 0, 0, 0, false);
        }
    }
}

//SpecialS7
unsafe extern "C" fn effect_gamewatch_SpecialS7(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 0..12 {
        wait(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("rot"), 0, 2, 0, 0, 0, 0, 1.2, 16, 20, 16, 0, 0, 0, false);
        }
    }
}

//SpecialAirS7
unsafe extern "C" fn effect_gamewatch_SpecialAirS7(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    for _ in 0..12 {
        wait(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("rot"), 0, 2, 0, 0, 0, 0, 1.2, 16, 20, 16, 0, 0, 0, false);
        }
    }
}

pub fn install() {
    Agent::new("gamewatch")
    .effect_acmd("effect_specials6", effect_gamewatch_SpecialS6)
    .effect_acmd("effect_specialairs6", effect_gamewatch_SpecialAirS6)
    .effect_acmd("effect_specials7", effect_gamewatch_SpecialS7)
    .effect_acmd("effect_specialairs7", effect_gamewatch_SpecialAirS7)
    .install();
}