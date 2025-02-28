use crate::imports::BuildImports::*;

//Fire
unsafe extern "C" fn game_sans_gaster_Fire(fighter: &mut L2CAgentBase) {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let rand_effect = [Hash40::new("collision_attr_elec"), Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_sleep"), Hash40::new("collision_attr_ink_hit"), Hash40::new("collision_attr_death")];
    let rng = sv_math::rand(hash40("palutena"), rand_effect.len() as i32);
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(owner_module_accessor, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_ANGLE) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 150.0, 60, 30, 0, 10, 4.0, 0.0, 7.0, 13.25, Some(0.0), Some(7.0), Some(146.5), 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -100, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, rand_effect[rng as usize], *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 0, /*Ink*/ 100.0);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 150.0, 60, 30, 0, 10, 4.0, 0.0, 7.0, 13.25, Some(0.0), Some(-49.0), Some(135.0), 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -100, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, rand_effect[rng as usize], *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 0, /*Ink*/ 100.0);
        }
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
    frame(fighter.lua_state_agent, 58.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Final
unsafe extern "C" fn game_sans_gaster_Final(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
}

//FinalFire
unsafe extern "C" fn game_sans_gaster_FinalFire(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 30, 0, 20, 20, 8.8, 0.0, 4.0, 2.25, Some(0.0), Some(4.0), Some(500.5), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8000.0, 361, 60, 0, 60, 100.0, 0.0, 4.0, 2.25, Some(0.0), Some(4.0), Some(500.5), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    frame(fighter.lua_state_agent, 58.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("palutena_gaster")
    .game_acmd("game_fire", game_sans_gaster_Fire, Low)
    .game_acmd("game_final", game_sans_gaster_Final, Low)
    .game_acmd("game_finalfire", game_sans_gaster_FinalFire, Low)
    .install();
}