use crate::imports::BuildImports::*;

//Move 
unsafe extern "C" fn sound_silver_search_Move(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_final02"));
    }
}

pub fn install() {
    Agent::new("mewtwo_search")
    .sound_acmd("sound_move_silver", sound_silver_search_Move, Low)
    .install();
}