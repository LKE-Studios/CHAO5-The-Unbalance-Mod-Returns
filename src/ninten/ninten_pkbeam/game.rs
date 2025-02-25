use crate::imports::BuildImports::*;

//Shoot
unsafe extern "C" fn game_ninten_pkbeam_Shoot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 82, 15, 0, 45, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PSI);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
        AREA_WIND_2ND_RAD_arg9(fighter, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
    }
}

//ShootAir
unsafe extern "C" fn game_ninten_pkbeam_ShootAir(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 82, 15, 0, 45, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PSI);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
        AREA_WIND_2ND_RAD_arg9(fighter, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
    }
}

//Pillar
unsafe extern "C" fn game_ninten_pkbeam_Pillar(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 60, 100, 0, 45, 11.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//PillarAir
unsafe extern "C" fn game_ninten_pkbeam_PillarAir(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 60, 100, 0, 45, 11.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("ness_pkfire")
    .game_acmd("game_shoot_ninten", game_ninten_pkbeam_Shoot, Low)
    .game_acmd("game_shootair_ninten", game_ninten_pkbeam_ShootAir, Low)
    .game_acmd("game_pillar_ninten", game_ninten_pkbeam_Pillar, Low)
    .game_acmd("game_pillarair_ninten", game_ninten_pkbeam_PillarAir, Low)
    .install();
}