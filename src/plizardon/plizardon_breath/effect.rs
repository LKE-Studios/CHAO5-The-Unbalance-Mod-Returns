use crate::imports::BuildImports::*;

//Move
unsafe extern "C" fn effect_plizardon_breath_Move(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_RND(fighter, Hash40::new("plizardon_kaenhousya"), Hash40::new("top"), 0.0, 0.0, 0.0, 10.0, -3.0, 0.0, 1.0, 0.0, 2.0, 2.0, 10.0, 0.0, 0.0, true);
    }
}

pub fn install() {
    Agent::new("plizardon_breath")
    .effect_acmd("effect_move", effect_plizardon_breath_Move)
    .install();
}