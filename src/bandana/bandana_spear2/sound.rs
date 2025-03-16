use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn sound_bandana_spear2_Fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_n02_02"));
    }
}

//Stick
unsafe extern "C" fn sound_bandana_spear2_Stick(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_n02_01"));
    }
}

pub fn install() {
    Agent::new("edge_spear2")
    .sound_acmd("sound_fly", sound_bandana_spear2_Fly, Low)
    .sound_acmd("sound_stick", sound_bandana_spear2_Stick, Low)
    .install();
}