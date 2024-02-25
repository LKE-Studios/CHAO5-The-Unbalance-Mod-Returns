use crate::imports::BuildImports::*;

//SpecialSDash
unsafe extern "C" fn game_inkling_roller_SpecialSDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("neck"), /*Damage*/ 25.0, /*Angle*/ 60, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 8.6, /*X*/ 0.5, /*Y*/ 3.8, /*Z*/ 5.0, /*X2*/ Some(0.5), /*Y2*/ Some(3.8), /*Z2*/ Some(-5.0), /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 20, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G_d, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bury"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("neck"), /*Damage*/ 25.0, /*Angle*/ 80, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 7.0, /*X*/ 0.5, /*Y*/ 3.8, /*Z*/ 5.0, /*X2*/ Some(0.5), /*Y2*/ Some(3.8), /*Z2*/ Some(-5.0), /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 20, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ink_hit"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 0, /*Ink*/ 150.0);
        AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 1, /*Ink*/ 150.0);
    }
}

//SpecialSDashNoInk
unsafe extern "C" fn game_inkling_roller_SpecialSDashNoInk(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("neck"), /*Damage*/ 20.0, /*Angle*/ 60, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.0, /*X*/ 0.5, /*Y*/ 3.8, /*Z*/ 5.0, /*X2*/ Some(0.5), /*Y2*/ Some(3.8), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 20, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ink_hit"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("neck"), /*Damage*/ 20.0, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.8, /*X*/ 0.5, /*Y*/ 3.8, /*Z*/ 5.0, /*X2*/ Some(0.5), /*Y2*/ Some(3.8), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 20, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ink_hit"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
    }
}

//SpecialSRun
unsafe extern "C" fn game_inkling_roller_SpecialSRun(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("neck"), /*Damage*/ 30.0, /*Angle*/ 60, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 8.6, /*X*/ 0.5, /*Y*/ 3.8, /*Z*/ 5.0, /*X2*/ Some(0.5), /*Y2*/ Some(3.8), /*Z2*/ Some(-5.0), /*Hitlag*/ 3.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 20, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G_d, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bury"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("neck"), /*Damage*/ 30.0, /*Angle*/ 80, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 7.0, /*X*/ 0.5, /*Y*/ 3.8, /*Z*/ 5.0, /*X2*/ Some(0.5), /*Y2*/ Some(3.8), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 20, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ink_hit"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 0, /*Ink*/ 160.0);
        AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 1, /*Ink*/ 160.0);
    }
}

//SpecialSRunNoInk
unsafe extern "C" fn game_inkling_roller_SpecialSRunNoInk(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("neck"), /*Damage*/ 22.0, /*Angle*/ 60, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.0, /*X*/ 0.5, /*Y*/ 3.8, /*Z*/ 5.0, /*X2*/ Some(0.5), /*Y2*/ Some(3.8), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 20, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ink_hit"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("neck"), /*Damage*/ 22.0, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.8, /*X*/ 0.5, /*Y*/ 3.8, /*Z*/ 5.0, /*X2*/ Some(0.5), /*Y2*/ Some(3.8), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 20, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ink_hit"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
    }
}

//SpecialSWalk
unsafe extern "C" fn game_inkling_roller_SpecialSWalk(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("neck"), /*Damage*/ 22.0, /*Angle*/ 60, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.6, /*X*/ 0.5, /*Y*/ 3.8, /*Z*/ 0.0, /*X2*/ Some(0.5), /*Y2*/ Some(3.8), /*Z2*/ Some(-2.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 30, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G_d, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ink_hit"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("neck"), /*Damage*/ 22.0, /*Angle*/ 80, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 8.0, /*X*/ 0.5, /*Y*/ 3.8, /*Z*/ 0.0, /*X2*/ Some(0.5), /*Y2*/ Some(3.8), /*Z2*/ Some(-2.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 30, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ink_hit"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 0, /*Ink*/ 170.0);
        AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 1, /*Ink*/ 170.0);
    }
}

//SpecialSWalkNoInk
unsafe extern "C" fn game_inkling_roller_SpecialSWalkNoInk(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("neck"), /*Damage*/ 17.0, /*Angle*/ 60, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.0, /*X*/ 0.5, /*Y*/ 3.8, /*Z*/ 0.0, /*X2*/ Some(0.5), /*Y2*/ Some(3.8), /*Z2*/ Some(-2.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 30, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ink_hit"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("neck"), /*Damage*/ 17.0, /*Angle*/ 80, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.8, /*X*/ 0.5, /*Y*/ 3.8, /*Z*/ 0.0, /*X2*/ Some(0.5), /*Y2*/ Some(3.8), /*Z2*/ Some(-2.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 30, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ink_hit"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
    }
}

//SpecialAirSDash
unsafe extern "C" fn game_inkling_roller_SpecialAirSDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("roll"), /*Damage*/ 28.0, /*Angle*/ 270, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 8.6, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 40, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G_d, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ink_hit"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("roll"), /*Damage*/ 28.0, /*Angle*/ 270, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 40, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ink_hit"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 0, /*Ink*/ 150.0);
        AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 1, /*Ink*/ 150.0);
    }
}

//SpecialAirSRun
unsafe extern "C" fn game_inkling_roller_SpecialAirSRun(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("roll"), /*Damage*/ 32.0, /*Angle*/ 280, /*KBG*/ 48, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 8.6, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 40, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G_d, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ink_hit"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("roll"), /*Damage*/ 32.0, /*Angle*/ 280, /*KBG*/ 48, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 40, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ink_hit"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 0, /*Ink*/ 160.0);
        AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 1, /*Ink*/ 160.0);
    }
}

//SpecialAirSWalk
unsafe extern "C" fn game_inkling_roller_SpecialAirSWalk(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("roll"), /*Damage*/ 4.0, /*Angle*/ 280, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.6, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 40, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G_d, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ink_hit"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("roll"), /*Damage*/ 4.0, /*Angle*/ 280, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 40, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ink_hit"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 0, /*Ink*/ 170.0);
        AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 1, /*Ink*/ 170.0);
    }
}

pub fn install() {
    Agent::new("inkling_roller")
    .game_acmd("game_specialsdash", game_inkling_roller_SpecialSDash)
    .game_acmd("game_specialsdashnoink", game_inkling_roller_SpecialSDashNoInk)
    .game_acmd("game_specialsrun", game_inkling_roller_SpecialSRun)
    .game_acmd("game_specialsrunnoink", game_inkling_roller_SpecialSRunNoInk)
    .game_acmd("game_specialswalk", game_inkling_roller_SpecialSWalk)
    .game_acmd("game_specialswalknoink", game_inkling_roller_SpecialSWalkNoInk)
    .game_acmd("game_specialairsdash", game_inkling_roller_SpecialAirSDash)
    .game_acmd("game_specialairsrun", game_inkling_roller_SpecialAirSRun)
    .game_acmd("game_specialairswalk", game_inkling_roller_SpecialAirSWalk)
    .install();
}