use crate::imports::BuildImports::*;

//Move 
unsafe extern "C" fn effect_silver_search_Move(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_final_ball"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
}

pub fn install() {
    Agent::new("mewtwo_search")
    .effect_acmd("effect_move_silver", effect_silver_search_Move, Low)
    .install();
}