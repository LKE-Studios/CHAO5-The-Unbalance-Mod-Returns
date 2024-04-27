use crate::imports::BuildImports::*;

//SBurst
unsafe extern "C" fn effect_samusd_supermissile_SBurst(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_BRANCH_SITUATION(fighter, Hash40::new("samusd_bomb_a"), Hash40::new("samusd_bomb_b"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.04, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
    }
}

pub fn install() {
    Agent::new("samusd_supermissile")
    .effect_acmd("effect_sburst", effect_samusd_supermissile_SBurst, Low)
    .install();
}