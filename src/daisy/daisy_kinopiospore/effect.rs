use crate::imports::BuildImports::*;

//Shot
unsafe extern "C" fn effect_daisy_kinopiospore_Shot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("daisy_kinopio_bullet"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 6.0, true);
    }
} 

pub fn install() {
    Agent::new("daisy_kinopiospore")
    .effect_acmd("effect_shot", effect_daisy_kinopiospore_Shot)
    .install();
}