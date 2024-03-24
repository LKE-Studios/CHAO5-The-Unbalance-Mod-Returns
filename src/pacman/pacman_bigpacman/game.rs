use crate::imports::BuildImports::*;

//Eat
unsafe extern "C" fn game_pacman_bigpacman_Eat(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *WEAPON_PACMAN_BIGPACMAN_ATTACK_ABSOLUTE_KIND_EAT, /*ID*/ 0, /*Damage*/ 60.0, /*Angle*/ 45, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 30, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
        ATTACK_ABS(fighter, /*Kind*/ *WEAPON_PACMAN_BIGPACMAN_ATTACK_ABSOLUTE_KIND_FINISH_EAT, /*ID*/ 0, /*Damage*/ 80.0, /*Angle*/ 40, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 30, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, true);
    }
}

pub fn install() {
    Agent::new("pacman_bigpacman")
    .game_acmd("game_eat", game_pacman_bigpacman_Eat)
    .install();
}