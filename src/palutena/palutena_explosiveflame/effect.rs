use crate::imports::BuildImports::*;

//Explode
unsafe extern "C" fn effect_palutena_explosiveflame_Explode(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("palutena_bomb"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("palutena_bomb_appear"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("palutena_bomb_finish"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    Agent::new("palutena_explosiveflame")
    .effect_acmd("effect_explode", effect_palutena_explosiveflame_Explode, Low)
    .install();
}