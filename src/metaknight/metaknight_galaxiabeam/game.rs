use crate::imports::BuildImports::*;

//Shoot
unsafe extern "C" fn game_metaknight_galaxiabeam_Shoot(fighter: &mut L2CAgentBase) {
    let rand_effect = [Hash40::new("collision_attr_fire"), Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_elec"), Hash40::new("collision_attr_ice")];
    let rng = smash::app::sv_math::rand(hash40("metaknight"), rand_effect.len() as i32);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 80, 0, 60, 9.0, 0.0, 7.0, 0.9, Some(0.0), Some(1.0), Some(3.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, rand_effect[rng as usize], *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 361, 80, 0, 60, 9.0, 0.0, -3.7, 2.2, Some(0.0), Some(1.0), Some(3.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, rand_effect[rng as usize], *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
}

pub fn install() {
    Agent::new("metaknight_galaxiabeam")
    .game_acmd("game_shoot", game_metaknight_galaxiabeam_Shoot, Low)
    .install();
}