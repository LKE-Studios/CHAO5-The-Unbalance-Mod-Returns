use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn effect_richter_axe_Fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("richter_ax_blur"), Hash40::new("top"), 0.0, 6.0, 0.0, 0.0, -90.0, 0.0, 3.7, true);
    }
}

pub fn install() {
    Agent::new("richter_axe")
    .game_acmd("effect_fly", effect_richter_axe_Fly)
    .install();
}