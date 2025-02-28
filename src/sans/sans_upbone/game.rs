use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn game_sans_upbone_Regular(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 32.0, 361, 66, 0, 25, 9.2, 0.0, 0.0, 0.0, None,None,None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 420, 40, 3.0, false);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    } 
}

pub fn install() {
    Agent::new("palutena_upbone")
    .game_acmd("game_regular", game_sans_upbone_Regular, Low)
    .install();
}
