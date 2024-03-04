use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn game_krool_backpack_Fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("wingl1"), /*Damage*/ 24.0, /*Angle*/ 80, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 14.0, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-4.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.2, /*SDI*/ 1.2, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 15, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
    }
}

//FlyWind
unsafe extern "C" fn game_krool_backpack_FlyWind(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("wingl1"), /*Damage*/ 24.0, /*Angle*/ 80, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 14.0, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-4.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.2, /*SDI*/ 1.2, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 15, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
    }
}

pub fn install() {
    Agent::new("krool_backpack")    
    .game_acmd("game_fly", game_krool_backpack_Fly)
    .game_acmd("game_flywind", game_krool_backpack_FlyWind)
    .install();
}