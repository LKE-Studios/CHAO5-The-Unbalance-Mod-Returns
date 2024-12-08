use crate::imports::BuildImports::*;

//Shoot
unsafe extern "C" fn effect_kamek_fire_Shoot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_shot"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
    }
}

//ShootAir
unsafe extern "C" fn effect_kamek_fire_ShootAir(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_starrod_shot"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
    }
}

//Pillar
unsafe extern "C" fn effect_kamek_fire_Pillar(fighter: &mut L2CAgentBase) {}

//PillarAir
unsafe extern "C" fn effect_kamek_fire_PillarAir(fighter: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("ness_pkfire")
    .effect_acmd("effect_shoot_kamek", effect_kamek_fire_Shoot, Low)
    .effect_acmd("effect_shootair_kamek", effect_kamek_fire_ShootAir, Low)
    .effect_acmd("effect_pillar_kamek", effect_kamek_fire_Pillar, Low)
    .effect_acmd("effect_pillarair_kamek", effect_kamek_fire_PillarAir, Low)
    .install();
}

