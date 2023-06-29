use crate::imports::BuildImports::*;

#[acmd_script(//Shot
    agent = "peach_kinopiospore", 
    script = "effect_shot", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_peach_kinopiospore_shot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("peach_kinopio_bullet"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 4.0, true);
    }
} 

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_peach_kinopiospore_shot
    );
}