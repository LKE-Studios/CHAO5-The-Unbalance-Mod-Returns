use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn effect_sans_downbone_Regular(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -5, 0, 0, 0, 90, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.58);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, false);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
}

//Stick
unsafe extern "C" fn effect_sans_downbone_Stick(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    Agent::new("palutena_downbone")
    .effect_acmd("effect_regular", effect_sans_downbone_Regular, Low)
    .effect_acmd("effect_stick", effect_sans_downbone_Stick, Low)
    .install();
}