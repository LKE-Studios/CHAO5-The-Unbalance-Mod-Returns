use crate::imports::BuildImports::*;

//Shoot
unsafe extern "C" fn effect_ninten_pkbeam_Shoot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ness_pkfi_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, true);
    }
}

//ShootAir
unsafe extern "C" fn effect_ninten_pkbeam_ShootAir(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ness_pkfi_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, true);
    }
}

//Pillar
unsafe extern "C" fn effect_ninten_pkbeam_Pillar(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("ness_pkfl_bomb"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

//PillarAir
unsafe extern "C" fn effect_ninten_pkbeam_PillarAir(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("ness_pkfl_bomb"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

pub fn install() {
    Agent::new("ness_pkfire")
    .effect_acmd("effect_shoot_ninten", effect_ninten_pkbeam_Shoot, Low)
    .effect_acmd("effect_shootair_ninten", effect_ninten_pkbeam_ShootAir, Low)
    .effect_acmd("effect_pillar_ninten", effect_ninten_pkbeam_Pillar, Low)
    .effect_acmd("effect_pillarair_ninten", effect_ninten_pkbeam_PillarAir, Low)
    .install();
}

