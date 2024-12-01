use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn sound_kamek_book_Regular(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_landing_akuukan"));
    }
}

//Stick
unsafe extern "C" fn sound_kamek_book_Stick(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_landing_ringmat")); 
    }
}

pub fn install() {
    Agent::new("ness_book")
    .sound_acmd("sound_regular", sound_kamek_book_Regular, Low)
    .sound_acmd("sound_stick", sound_kamek_book_Stick, Low)
    .install();
}