use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn game_bandana_apple_Fly(fighter: &mut L2CAgentBase) {
    let rand_effect = [Hash40::new("collision_attr_elec"), Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_sleep"), Hash40::new("collision_attr_curse_poison"), Hash40::new("collision_attr_death"), Hash40::new("collision_attr_ice"), Hash40::new("collision_attr_fire"), Hash40::new("collision_attr_flower"), Hash40::new("collision_attr_purple"), Hash40::new("collision_attr_bury"), Hash40::new("collision_attr_bind")];
    let rng = sv_math::rand(hash40("fighter"), rand_effect.len() as i32);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("have"), 24.0, 30, 70, 0, 20, 8.0, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, rand_effect[rng as usize], *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 600, /*Rehit*/ 30, /*Damage*/ 1.5, /*Unk*/ false);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
}

//Haved
unsafe extern "C" fn game_bandana_apple_Haved(fighter: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("edge_apple")
    .game_acmd("game_fly", game_bandana_apple_Fly, Low)
    .game_acmd("game_haved", game_bandana_apple_Haved, Low)
    .install();
}