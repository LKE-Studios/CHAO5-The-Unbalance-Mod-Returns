use crate::imports::BuildImports::*;

//BurstAttack
unsafe extern "C" fn effect_samusd_bomb_BurstAttack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_BRANCH_SITUATION(fighter, Hash40::new("samusd_bomb_a"), Hash40::new("samusd_bomb_b"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.12, /*G*/ 0.1, /*B*/ 1.85);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    Agent::new("samusd_bomb")
    .effect_acmd("effect_burstattack", effect_samusd_bomb_BurstAttack)
    .install();
}