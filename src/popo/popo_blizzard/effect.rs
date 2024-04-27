use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn effect_popo_blizzard_Fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("popo_blizzerd_bullet"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.5, false);
    }
}

pub fn install() {
    Agent::new("popo_blizzard")
    .effect_acmd("effect_fly", effect_popo_blizzard_Fly, Low)
    .install();
}