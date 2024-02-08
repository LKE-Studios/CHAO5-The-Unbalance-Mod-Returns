use crate::imports::BuildImports::*;

#[acmd_script(//SpecialHi
    agent = "eflame_firepillar", 
    script = "effect_specialhi",
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_eflame_specialhi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("eflame_promrevolt_firepillar_ground"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 3.3, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("eflame_promrevolt_firepillar"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 3.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        EFFECT(fighter, Hash40::new("eflame_promrevolt_firepillar_impact"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 3.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_eflame_specialhi
    );
}