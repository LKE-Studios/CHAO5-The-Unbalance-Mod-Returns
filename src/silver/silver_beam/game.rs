use crate::imports::BuildImports::*;

//Shoot 
unsafe extern "C" fn game_silver_beam_Shoot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 361, 90, 0, 40, 9.0, 0.0, -5.0, 0.0, Some(0.0), Some(5.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -0.6, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("mewtwo_shadowball")
    .game_acmd("game_shoot_silver", game_silver_beam_Shoot, Low)
    .install();
}