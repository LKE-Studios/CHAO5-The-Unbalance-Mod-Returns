use crate::imports::BuildImports::*;

#[acmd_script(//GuardSpecial
    agent = "pzenigame", 
    script = "effect_guardspecial", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pzenigame_guardspecial(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_water_walk"), Hash40::new("head"), 0.0, 3.8, 0, 0.0, 100.0, 0, 1.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("head"), 0.0, 0, 0, 0.0, 90.0, 0.0, 2.0, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, 5.0, 5.0, 90.0, 0.0, 0, 1.9, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, 5.5, 10.0, 90.0, 0.0, 0, 1.8, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, 6.0, 15.0, 90.0, 0.0, 0, 1.7, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, 6.5, 20.0, 90.0, 0.0, 0, 1.6, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, 7.0, 25.0, 90.0, 0.0, 0, 1.5, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, 7.0, 30.0, 90.0, 0.0, 0, 1.4, false);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, 7.0, 35.0, 90.0, 0.0, 0, 1.3, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, 7.0, 40.0, 90.0, 0.0, 0, 1.2, false);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, 7.0, 45.0, 90.0, 0.0, 0, 1.15, false);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, 7.0, 50.0, 90.0, 0.0, 0, 1.1, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_drown_out"), false, false);
    }
}

#[acmd_script(//EscapeAirSpecial
    agent = "pzenigame", 
    script = "effect_escapeairspecial", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pzenigame_escapeairspecial(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_water_walk"), Hash40::new("head"), 0.0, 6.5, 0, 0.0, 100.0, 0, 1.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("head"), 0.0, 0, 0, 0.0, 90.0, 0.0, 2.0, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, 5.0, 5.0, 90.0, 0.0, 0, 1.9, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, 3.5, 10.0, 90.0, 0.0, 0, 1.8, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, 2.0, 15.0, 90.0, 0.0, 0, 1.7, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, 0.5, 20.0, 90.0, 0.0, 0, 1.6, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, -1.0, 25.0, 90.0, 0.0, 0, 1.5, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, -2.5, 30.0, 90.0, 0.0, 0, 1.4, false);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, -4.0, 35.0, 90.0, 0.0, 0, 1.3, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, -5.5, 40.0, 90.0, 0.0, 0, 1.2, false);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, -7.0, 45.0, 90.0, 0.0, 0, 1.15, false);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sleep"), Hash40::new("top"), 0.0, -8.5, 50.0, 90.0, 0.0, 0, 1.1, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_drown_out"), false, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_pzenigame_guardspecial,
        effect_pzenigame_escapeairspecial
    );
}