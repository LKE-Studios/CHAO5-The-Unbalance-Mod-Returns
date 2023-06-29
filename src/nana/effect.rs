use crate::imports::BuildImports::*;

#[acmd_script(//Fly
    agent = "nana_blizzard", 
    script = "effect_fly", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_nana_blizzard_fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("popo_blizzerd_bullet"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.5, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_nana_blizzard_fly
    );
}