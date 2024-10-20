use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn effect_silver_box_Regular(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_box00_break"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_box00_break"), Hash40::new("top"), 0, 8, 0, 0, 0, 180, 3.0, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    Agent::new("mewtwo_box")
    .effect_acmd("effect_regular", effect_silver_box_Regular, Low)
    .install();
}