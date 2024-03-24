use crate::imports::BuildImports::*;

//Shot
unsafe extern "C" fn effect_peach_kinopiospore_Shot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("peach_kinopio_bullet"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 4.0, true);
    }
} 

pub fn install() {
    Agent::new("peach_kinopiospore")
    .effect_acmd("effect_shot", effect_peach_kinopiospore_Shot)
    .install();
}