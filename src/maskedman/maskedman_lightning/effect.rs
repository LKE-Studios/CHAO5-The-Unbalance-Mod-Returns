use crate::imports::BuildImports::*;

//Shoot
unsafe extern "C" fn effect_maskedman_lightning_Shoot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pikachu_dengeki"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
}

pub fn install() {
    Agent::new("lucas_pkfire")
    .effect_acmd("effect_shoot_maskedman", effect_maskedman_lightning_Shoot, Low)
    .install();
}