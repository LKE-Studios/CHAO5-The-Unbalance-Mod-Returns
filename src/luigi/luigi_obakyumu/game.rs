use crate::imports::BuildImports::*;

//FinalShootReady
unsafe extern "C" fn game_luigi_obakyumu_FinalShootReady(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 200.0, /*Angle*/ 60, /*KBG*/ 142, /*FKB*/ 0, /*BKB*/ 70, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, true);
    }
}

//FinalAirShootReady
unsafe extern "C" fn game_luigi_obakyumu_FinalAirShootReady(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 200.0, /*Angle*/ 60, /*KBG*/ 142, /*FKB*/ 0, /*BKB*/ 70, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, true);
    }
}

pub fn install() {
    Agent::new("luigi_obakyumu")
    .game_acmd("game_finalshootready", game_luigi_obakyumu_FinalShootReady)
    .game_acmd("game_finalairshootready", game_luigi_obakyumu_FinalAirShootReady)
    .install();
}