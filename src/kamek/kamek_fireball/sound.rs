use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn sound_kamek_fireball_Regular(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_throw_pk02"));
        PLAY_SE(fighter, Hash40::new("se_ness_throw_pk01"));
    }
}

pub fn install() {
    Agent::new("ness_fireball")
    .sound_acmd("sound_regular_kamek", sound_kamek_fireball_Regular, Low)
    .install();
}