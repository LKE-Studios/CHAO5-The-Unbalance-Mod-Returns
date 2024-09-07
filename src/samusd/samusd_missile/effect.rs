use crate::imports::BuildImports::*;

//HBurst 
unsafe extern "C" fn effect_samusd_missile_HBurst(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_BRANCH_SITUATION(fighter, Hash40::new("samusd_bomb_a"), Hash40::new("samusd_bomb_b"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
    }
}

pub fn install() {
    Agent::new("samusd_missile")
    .effect_acmd("effect_hburst", effect_samusd_missile_HBurst, Low)
    .install();
}