use crate::imports::BuildImports::*;

//Move 
unsafe extern "C" fn game_silver_search_Move(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 50.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE, *COLLISION_PART_MASK_ALL, false);
    }
}

pub fn install() {
    Agent::new("mewtwo_search")
    .game_acmd("game_move_silver", game_silver_search_Move, Low)
    .install();
}