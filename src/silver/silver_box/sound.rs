use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn sound_silver_box_Regular(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_item_box_break"));
    }
}

pub fn install() {
    Agent::new("mewtwo_box")
    .sound_acmd("sound_regular", sound_silver_box_Regular, Low)
    .install();
}