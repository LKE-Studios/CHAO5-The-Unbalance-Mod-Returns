use crate::imports::BuildImports::*;

//Shoot
unsafe extern "C" fn game_palutena_reflectionboard_Shoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 100, 50, 0, 9.0, 0.0, 8.5, 0.0, Some(0.0), Some(-4.5), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

pub fn install() {
    Agent::new("palutena_reflectionboard")
    .game_acmd("game_shoot", game_palutena_reflectionboard_Shoot, Low)
    .install();
}