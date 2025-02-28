use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn effect_sans_sidebone_Regular(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 78.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, false);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
}

pub fn install() {
    Agent::new("palutena_sidebone")
    .effect_acmd("effect_regular", effect_sans_sidebone_Regular, Low)
    .install();
}