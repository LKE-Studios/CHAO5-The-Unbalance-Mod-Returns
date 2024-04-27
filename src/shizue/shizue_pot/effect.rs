use crate::imports::BuildImports::*;

//Burst
unsafe extern "C" fn effect_shizue_pot_Burst(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    Agent::new("shizue_pot")
    .effect_acmd("effect_burst", effect_shizue_pot_Burst, Low)
    .install();
}