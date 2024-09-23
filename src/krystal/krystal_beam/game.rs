use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn game_krystal_beam_Fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 36.0, 40, 72, 0, 40, 1.3, 0.0, -2.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PALUTENA);
    }
}

pub fn install() {
    Agent::new("pitb_bowarrow")
    .game_acmd("game_fly_krystal", game_krystal_beam_Fly, Low)
    .install();
}