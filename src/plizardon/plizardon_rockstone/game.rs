use crate::imports::BuildImports::*;

//Move
unsafe extern "C" fn game_plizardon_rockstone_Move(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("have"), 16.0, 70, 48, 0, 50, 6.6, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);  
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("plizardon_rockstone")
    .game_acmd("game_move", game_plizardon_rockstone_Move, Low)
    .install();
}