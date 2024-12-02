use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn game_kamek_beam_Regular(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 86, 0, 30, 10.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 666.0, 361, 100, 0, 30, 20.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_LEFT, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
        attack!(fighter, MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("ness_pinkmagic")
    .game_acmd("game_finalcutterregular", game_kamek_beam_Regular, Low)
    .install();
}