use crate::imports::BuildImports::*;

//Pillar
unsafe extern "C" fn game_lucas_pkfire_Pillar(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 42, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_PSI);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 42, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 9.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_PSI);
    }
    wait(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        AREA_WIND_2ND_RAD_arg9(fighter, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
    }
}

//Shoot
unsafe extern "C" fn game_lucas_pkfire_Shoot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_PSI);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("lucas_pkfire")
    .game_acmd("game_pillar", game_lucas_pkfire_Pillar)
    .game_acmd("game_shoot", game_lucas_pkfire_Shoot)
    .install();
}