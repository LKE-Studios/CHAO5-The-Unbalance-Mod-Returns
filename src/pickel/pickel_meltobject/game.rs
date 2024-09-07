use crate::imports::BuildImports::*;

//AttackHi4
unsafe extern "C" fn game_pickel_meltobject_AttackHi4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 9.75, /*Z*/ 3.25, /*X2*/ Some(0.0), /*Y2*/ Some(9.75), /*Z2*/ Some(-3.25), /*Hitlag*/ 0.2, /*SDI*/ 0.2, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 113, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 3.25, /*Z*/ 3.25, /*X2*/ Some(0.0), /*Y2*/ Some(3.25), /*Z2*/ Some(-3.25), /*Hitlag*/ 0.0, /*SDI*/ 0.2, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_OBJECT);
    }
}

pub fn install() {
    Agent::new("pickel_meltobject")
    .game_acmd("game_attackhi4", game_pickel_meltobject_AttackHi4, Low)
    .install();
}