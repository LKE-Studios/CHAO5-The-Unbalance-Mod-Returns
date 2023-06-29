use crate::imports::BuildImports::*;

#[acmd_script(//SpecialHiL
    agent = "purin", 
    script = "effect_specialhil", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_purin_specialhil(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("purin_utau"), Hash40::new("hip"), 2, 0, 0, 0, 0, 0, 2.25, true);
    }
}

#[acmd_script(//SpecialHiR
    agent = "purin", 
    script = "effect_specialhir", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_purin_specialhir(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("purin_utau"), Hash40::new("hip"), 2, 0, 0, 0, 0, 0, 2.25, true);
    }
}

#[acmd_script(//SpecialAirHiL
    agent = "purin", 
    script = "effect_specialairhil", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_purin_specialairhil(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("purin_utau"), Hash40::new("hip"), 2, 0, 0, 0, 0, 0, 2.25, true);
    }
}

#[acmd_script(//SpecialAirHiR
    agent = "purin", 
    script = "effect_specialairhir", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_purin_specialairhir(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("purin_utau"), Hash40::new("hip"), 2, 0, 0, 0, 0, 0, 2.25, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_purin_specialhil,
        effect_purin_specialhir,
        effect_purin_specialairhil,
        effect_purin_specialairhir,
    );
}
