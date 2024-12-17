use crate::imports::BuildImports::*;

//Shoot
unsafe extern "C" fn sound_maskedman_lightning_Shoot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_special_s02"));
    }
}

pub fn install() {
    Agent::new("lucas_pkfire")
    .sound_acmd("sound_shoot_maskedman", sound_maskedman_lightning_Shoot, Low)
    .install();
}