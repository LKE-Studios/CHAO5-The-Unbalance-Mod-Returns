use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn game_sans_sidebone_Regular(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 45, 53, 200, 0, 7.2, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 18, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_MAGIC);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("palutena_sidebone")
    .game_acmd("game_regular", game_sans_sidebone_Regular, Low)
    .install();
}