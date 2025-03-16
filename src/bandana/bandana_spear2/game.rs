use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn game_bandana_spear2_Fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 35.0, 78, 54, 0, 15, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -25.0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 900, /*Rehit*/ 30, /*Damage*/ 2.0, /*Unk*/ false);
    }
}

//Stick
unsafe extern "C" fn game_bandana_spear2_Stick(fighter: &mut L2CAgentBase) {
    let rand_effect = [Hash40::new("collision_attr_slip"), Hash40::new("collision_attr_sleep"), Hash40::new("collision_attr_ice"), Hash40::new("collision_attr_saving"), Hash40::new("collision_attr_bury"), Hash40::new("collision_attr_bind")];
    let rng = sv_math::rand(hash40("fighter"), rand_effect.len() as i32);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 30.0, 361, 75, 0, 15, 6.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 1.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, rand_effect[rng as usize], *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
    }
}

pub fn install() {
    Agent::new("edge_spear2")
    .game_acmd("game_fly", game_bandana_spear2_Fly, Low)
    .game_acmd("game_stick", game_bandana_spear2_Stick, Low)
    .install();
}