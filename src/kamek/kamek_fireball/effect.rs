use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn effect_kamek_fireball_Regular(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_l"), Hash40::new("rot"), 0, 1.8, 0, 0, 0, 0, 1, true);
    }
}

//Bound
unsafe extern "C" fn effect_kamek_fireball_Bound(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("mario_fb_bound"), Hash40::new("top"), 0, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    Agent::new("ness_fire")
    .effect_acmd("effect_regular", effect_kamek_fireball_Regular, Low)
    .effect_acmd("effect_bound", effect_kamek_fireball_Bound, Low)
    .install();
}