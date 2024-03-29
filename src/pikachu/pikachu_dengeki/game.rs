use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn game_pikachu_dengeki_Regular(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("sphere"), 10.0, 361, 20, 0, 18, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("sphere"), 9.0, 361, 20, 0, 13, 2.8, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -2.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("sphere"), 9.0, 361, 20, 0, 18, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -2.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("sphere"), 9.0, 361, 20, 0, 13, 2.8, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -1.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("sphere"), 8.0, 361, 20, 0, 18, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -1.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("sphere"), 8.0, 361, 20, 0, 13, 2.8, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -1.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
} 

pub fn install() {
    Agent::new("pikachu_dengeki")
    .game_acmd("game_regular", game_pikachu_dengeki_Regular)
    .install();
}