use crate::imports::BuildImports::*;

//SpecialHiL
unsafe extern "C" fn effect_purin_SpecialHiL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("purin_utau"), Hash40::new("hip"), 2, 0, 0, 0, 0, 0, 2.25, true);
    }
}

//SpecialHiR
unsafe extern "C" fn effect_purin_SpecialHiR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("purin_utau"), Hash40::new("hip"), 2, 0, 0, 0, 0, 0, 2.25, true);
    }
}

//SpecialAirHiL
unsafe extern "C" fn effect_purin_SpecialAirHiL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("purin_utau"), Hash40::new("hip"), 2, 0, 0, 0, 0, 0, 2.25, true);
    }
}

//SpecialAirHiR
unsafe extern "C" fn effect_purin_SpecialAirHiR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("purin_utau"), Hash40::new("hip"), 2, 0, 0, 0, 0, 0, 2.25, true);
    }
}

pub fn install() {
    Agent::new("purin")
    .effect_acmd("effect_specialhil", effect_purin_SpecialHiL, Low)
    .effect_acmd("effect_specialhir", effect_purin_SpecialHiR, Low)
    .effect_acmd("effect_specialairhil", effect_purin_SpecialAirHiL, Low)
    .effect_acmd("effect_specialairhir", effect_purin_SpecialAirHiR, Low)
    .install();
}
