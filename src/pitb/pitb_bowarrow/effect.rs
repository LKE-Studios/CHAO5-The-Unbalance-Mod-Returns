use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn effect_pitb_bowarrow_Fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_pa_fly_arrow"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_pa_fly_arrow2"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 2.2, true);
    }
}

pub fn install() {
    Agent::new("pitb_bowarrow")
    .game_acmd("game_fly", game_pitb_bowarrow_Fly)
    .install();
}