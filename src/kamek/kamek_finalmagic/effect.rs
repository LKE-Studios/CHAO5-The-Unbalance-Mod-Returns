
use crate::imports::BuildImports::*;

//Move
unsafe extern "C" fn effect_kamek_finalmagic_Regular(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ness_final_main"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.55, 2.15, 0.0);
        EFFECT(fighter, Hash40::new("ness_final_stardust"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.55, 2.15, 0.0);
    }
}

pub fn install() {
    Agent::new("ness_finalmagic")
    .effect_acmd("effect_regular", effect_kamek_finalmagic_Regular, Low)
    .install();
}