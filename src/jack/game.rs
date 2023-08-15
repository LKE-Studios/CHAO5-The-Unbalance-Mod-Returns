use crate::imports::BuildImports::*;

#[acmd_script(//Attack11
    agent = "jack", 
    script = "game_attack11", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attack11(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.44);
    frame(fighter.lua_state_agent, 3.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 180, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 12.5, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(15.2), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 12.5, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(15.2), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 4, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 361, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 33, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 7.5, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(10.8), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
            ATTACK(fighter, /*ID*/ 5, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 180, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 33, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 10.8, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(14.7), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

#[acmd_script(//Attack12
    agent = "jack", 
    script = "game_attack12", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.5, /*Angle*/ 361, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.5, /*Angle*/ 361, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(14.5), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.5, /*Angle*/ 180, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 18.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.5, /*Angle*/ 361, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 18.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 4, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 361, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 10.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(15.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
            ATTACK(fighter, /*ID*/ 5, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 180, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(18.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        }
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

#[acmd_script(//Attack13
    agent = "jack", 
    script = "game_attack13", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 13.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(14.5), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.95, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 13.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(14.5), /*Hitlag*/ 1.95, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(2.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.95, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 4, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 135, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 11.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(14.0), /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 300, /*Rehit*/ 30, /* Damage*/ 1.2, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 300, /*Rehit*/ 30, /* Damage*/ 1.2, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 300, /*Rehit*/ 30, /* Damage*/ 1.2, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 300, /*Rehit*/ 30, /* Damage*/ 1.2, /*Unk*/ false);
        }
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 15.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 17.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.95, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 5, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 135, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 17.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 300, /*Rehit*/ 30, /* Damage*/ 1.2, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 5, /*Frames*/ 300, /*Rehit*/ 30, /* Damage*/ 1.2, /*Unk*/ false);
        }
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//AttackDash
    agent = "jack", 
    script = "game_attackdash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attackdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 35, /*KBG*/ 95, /*FKB*/ 70, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 4.0, /*X2*/ Some(3.0), /*Y2*/ Some(9.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 8.0, /*Angle*/ 40, /*KBG*/ 95, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(2.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("footr"), /*Damage*/ 8.0, /*Angle*/ 50, /*KBG*/ 95, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 25, /*KBG*/ 95, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 4.0, /*X2*/ Some(3.0), /*Y2*/ Some(9.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 8.0, /*Angle*/ 10, /*KBG*/ 95, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(2.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("footr"), /*Damage*/ 8.0, /*Angle*/ 15, /*KBG*/ 95, /*FKB*/ 25, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.5, 4.0);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("legl"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 6.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(5.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("footl"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 6.8, /*X*/ 0.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 7.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(12.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 7.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(12.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 2.1);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 2.1);
        }
    }
    frame(fighter.lua_state_agent, 17.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 7.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(13.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 7.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(13.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 2.1);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 2.1);
        }
    }
    frame(fighter.lua_state_agent, 19.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 7.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(11.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 7.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(11.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 2.1);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 2.1);
        }
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//AttackS3Hi
    agent = "jack", 
    script = "game_attacks3hi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attacks3hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 5.0, /*Angle*/ 93, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 5.0, /*Angle*/ 40, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 12.0, /*Angle*/ 35, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            AttackModule::set_size(fighter.module_accessor, /*ID*/ 2, /*Size*/ 0.0);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 13.5, /*Angle*/ 35, /*KBG*/ 111, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(0.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("knife"), /*Damage*/ 13.5, /*Angle*/ 35, /*KBG*/ 111, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(0.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BOMB);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 4.0);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 4.0);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 2, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//AttackS3
    agent = "jack", 
    script = "game_attacks3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attacks3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 5.0, /*Angle*/ 93, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 5.0, /*Angle*/ 40, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 12.0, /*Angle*/ 35, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            AttackModule::set_size(fighter.module_accessor, /*ID*/ 2, /*Size*/ 0.0);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 13.5, /*Angle*/ 35, /*KBG*/ 111, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(0.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("knife"), /*Damage*/ 13.5, /*Angle*/ 35, /*KBG*/ 111, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(0.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BOMB);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 4.0);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 4.0);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 2, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//AttackS3Lw
    agent = "jack", 
    script = "game_attacks3lw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attacks3lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 5.0, /*Angle*/ 93, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 5.0, /*Angle*/ 40, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 12.0, /*Angle*/ 35, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            AttackModule::set_size(fighter.module_accessor, /*ID*/ 2, /*Size*/ 0.0);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 13.5, /*Angle*/ 35, /*KBG*/ 111, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(0.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("knife"), /*Damage*/ 13.5, /*Angle*/ 35, /*KBG*/ 111, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(0.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BOMB);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 4.0);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 4.0);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 2, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//AttackHi3
    agent = "jack", 
    script = "game_attackhi3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attackhi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 120, /*KBG*/ 100, /*FKB*/ 150, /*BKB*/ 0, /*Size*/ 12.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(7.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 7.0, /*Angle*/ 105, /*KBG*/ 100, /*FKB*/ 150, /*BKB*/ 0, /*Size*/ 12.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(1.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 7.0, /*Angle*/ 100, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 12.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(1.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 7.0, /*Angle*/ 110, /*KBG*/ 100, /*FKB*/ 165, /*BKB*/ 0, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 1.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 7.0, /*Angle*/ 115, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 1.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 7.0, /*Angle*/ 123, /*KBG*/ 100, /*FKB*/ 165, /*BKB*/ 0, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 3.3, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 6, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 7.0, /*Angle*/ 115, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 3.3, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 0, false);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 1, false);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 2, false);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 130, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 23.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(20.0), /*Z2*/ Some(5.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 23.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 133, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 7.5, /*X2*/ Some(0.0), /*Y2*/ Some(18.0), /*Z2*/ Some(7.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 6, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 2.5, /*Angle*/ 367, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 1.7, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 4, false);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 5, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.5, /*Angle*/ 367, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 25.0, /*Z*/ -1.0, /*X2*/ Some(0.0), /*Y2*/ Some(25.0), /*Z2*/ Some(1.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_SWORD);
        }
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 8.0, /*Angle*/ 90, /*KBG*/ 185, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 1.7, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 0, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 10.0, /*Angle*/ 90, /*KBG*/ 180, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 1.6, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_SWORD);
        }
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//AttackLw3
    agent = "jack", 
    script = "game_attacklw3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attacklw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 12.0, /*Angle*/ 65, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 9.6, /*X*/ -4.0, /*Y*/ 0.0, /*Z*/ 1.5, /*X2*/ Some(5.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1.5), /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 65, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 11.5, /*X*/ 0.0, /*Y*/ 3.2, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.2), /*Z2*/ Some(8.5), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 72, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 11.5, /*X*/ 0.0, /*Y*/ 3.2, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.2), /*Z2*/ Some(8.5), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.5);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.5);
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 12.0, /*Angle*/ 82, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 9.6, /*X*/ -4.0, /*Y*/ 0.0, /*Z*/ 1.5, /*X2*/ Some(5.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 82, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 11.5, /*X*/ 0.0, /*Y*/ 3.2, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.2), /*Z2*/ Some(8.5), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 82, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 11.5, /*X*/ 0.0, /*Y*/ 3.2, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.2), /*Z2*/ Some(8.5), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.5);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.5);
        }
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 12.0, /*Angle*/ 85, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 9.6, /*X*/ -4.0, /*Y*/ 0.0, /*Z*/ 1.5, /*X2*/ Some(5.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 85, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 11.5, /*X*/ 0.0, /*Y*/ 3.2, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.2), /*Z2*/ Some(8.5), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 82, /*KBG*/ 99, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 11.5, /*X*/ 0.0, /*Y*/ 3.2, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.2), /*Z2*/ Some(8.5), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.5);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.5);
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        JostleModule::set_status(fighter.module_accessor, true);
    }
}

#[acmd_script(//AttackS4
    agent = "jack", 
    script = "game_attacks4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attacks4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 2.0, 5.0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 23.0, /*Angle*/ 361, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(19.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 23.0, /*Angle*/ 361, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ -9.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ -15.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(21.5), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.5), /*Z2*/ Some(10.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.5, /*Angle*/ 361, /*KBG*/ 101, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ -15.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(21.5), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_NONE);
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.5, /*Angle*/ 361, /*KBG*/ 101, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.5), /*Z2*/ Some(10.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_NONE);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.25);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 3, /*ShieldstunMul*/ 1.25);
        }
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 23.0, /*Angle*/ 361, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(19.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 23.0, /*Angle*/ 361, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ -9.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ -15.5, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(21.5), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.5, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.5), /*Z2*/ Some(10.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.5, /*Angle*/ 361, /*KBG*/ 101, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ -15.5, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(21.5), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_NONE);
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.5, /*Angle*/ 361, /*KBG*/ 101, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.5), /*Z2*/ Some(10.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_NONE);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.25);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 3, /*ShieldstunMul*/ 1.25);
        }
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, /*ID*/ 0, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(fighter.module_accessor, /*ID*/ 1, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(fighter.module_accessor, /*ID*/ 0, /*Size*/ 0.0);
        AttackModule::set_size(fighter.module_accessor, /*ID*/ 1, /*Size*/ 0.0);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.5, /*Angle*/ 361, /*KBG*/ 92, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ -15.5, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(21.5), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.5, /*Angle*/ 361, /*KBG*/ 92, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.5), /*Z2*/ Some(10.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//AttackHi4
    agent = "jack", 
    script = "game_attackhi4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attackhi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 22.0, /*Angle*/ 83, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 22.0, /*Angle*/ 83, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 7.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 22.0, /*Angle*/ 83, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.0), /*Z2*/ Some(6.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 21.0, /*Angle*/ 83, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 21.0, /*Angle*/ 83, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 7.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 83, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.0), /*Z2*/ Some(6.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 1, /*Bone*/ Hash40::new("knife"), /*Damage*/ 17.0, /*Angle*/ 83, /*KBG*/ 99, /*FKB*/ 0, /*BKB*/ 59, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 4, /*Part*/ 1, /*Bone*/ Hash40::new("armr"), /*Damage*/ 17.0, /*Angle*/ 83, /*KBG*/ 99, /*FKB*/ 0, /*BKB*/ 59, /*Size*/ 8.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 5, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 83, /*KBG*/ 99, /*FKB*/ 0, /*BKB*/ 59, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.0), /*Z2*/ Some(6.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 3.25);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 3.25);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 3.25);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 3, /*ShieldstunMul*/ 3.25);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 4, /*ShieldstunMul*/ 3.25);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 5, /*ShieldstunMul*/ 3.25);

        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 2, false);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 5, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 83, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 28.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(16.0), /*Z2*/ Some(9.5), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 5, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 83, /*KBG*/ 99, /*FKB*/ 0, /*BKB*/ 59, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 28.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(16.0), /*Z2*/ Some(9.5), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 5, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
        }
    }
    frame(fighter.lua_state_agent, 18.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 83, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 12.2, /*X*/ 0.0, /*Y*/ 28.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 5, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 83, /*KBG*/ 99, /*FKB*/ 0, /*BKB*/ 59, /*Size*/ 12.2, /*X*/ 0.0, /*Y*/ 28.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 5, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
        }
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script(//AttackLw4
    agent = "jack", 
    script = "game_attacklw4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attacklw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 20.0, /*Angle*/ 32, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 1.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 20.0, /*Angle*/ 32, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.0, /*X*/ 1.0, /*Y*/ 4.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 20.0, /*Angle*/ 32, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 20.0, /*Angle*/ 32, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 19.0, /*Angle*/ 25, /*KBG*/ 87, /*FKB*/ 0, /*BKB*/ 64, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 1.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 19.0, /*Angle*/ 25, /*KBG*/ 87, /*FKB*/ 0, /*BKB*/ 64, /*Size*/ 7.0, /*X*/ 1.0, /*Y*/ 4.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 19.0, /*Angle*/ 25, /*KBG*/ 87, /*FKB*/ 0, /*BKB*/ 64, /*Size*/ 7.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 19.0, /*Angle*/ 25, /*KBG*/ 87, /*FKB*/ 0, /*BKB*/ 64, /*Size*/ 7.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 25, /*KBG*/ 87, /*FKB*/ 0, /*BKB*/ 64, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 6.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(15.5), /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 5, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 25, /*KBG*/ 87, /*FKB*/ 0, /*BKB*/ 64, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 6.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(15.5), /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 4, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 5, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
        }
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 21.0, /*Angle*/ 30, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 1.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 21.0, /*Angle*/ 30, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.0, /*X*/ 1.0, /*Y*/ 4.5, /*Z*/ 0.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 21.0, /*Angle*/ 30, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 21.0, /*Angle*/ 30, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 19.4, /*Angle*/ 30, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 1.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 19.4, /*Angle*/ 30, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 7.0, /*X*/ 1.0, /*Y*/ 4.5, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 19.4, /*Angle*/ 30, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 7.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 19.4, /*Angle*/ 30, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 7.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.4, /*Angle*/ 30, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -8.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(-15.0), /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 5, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.4, /*Angle*/ 30, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -8.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(-15.0), /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 4, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 5, /*Frames*/ 600, /*Rehit*/ 45, /* Damage*/ 1.5, /*Unk*/ false);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 2.33);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 2.33);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 2.33);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 3, /*ShieldstunMul*/ 2.33);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 4, /*ShieldstunMul*/ 2.33);
        }
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//AttackAirN
    agent = "jack", 
    script = "game_attackairn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attackairn(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 15.0, /*Angle*/ 46, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 10.8, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 1, /*Bone*/ Hash40::new("throw"), /*Damage*/ 15.5, /*Angle*/ 46, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 10.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script(//AttackAirF 
    agent = "jack", 
    script = "game_attackairf", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attackairf(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 0.9);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("legr"), /*Damage*/ 10.0, /*Angle*/ 367, /*KBG*/ 25, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 3.2, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(3.2), /*Y2*/ Some(-0.5), /*Z2*/ Some(1.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("legr"), /*Damage*/ 10.0, /*Angle*/ 76, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 73, /*Size*/ 8.0, /*X*/ 3.2, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(3.2), /*Y2*/ Some(0.0), /*Z2*/ Some(1.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 10.0, /*Angle*/ 367, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 4.2, /*Y*/ -0.7, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 10.0, /*Angle*/ 80, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 73, /*Size*/ 8.0, /*X*/ 4.2, /*Y*/ -0.7, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 367, /*KBG*/ 25, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 76, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 73, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 4, false);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 5, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 46, /*Size*/ 9.5, /*X*/ 4.4, /*Y*/ -0.7, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("legl"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 46, /*Size*/ 9.5, /*X*/ 3.2, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, /*ID*/ 1, false);
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 34, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(13.5), /*Z2*/ Some(7.5), /*Hitlag*/ 1.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 34, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(13.5), /*Z2*/ Some(7.5), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 34, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 14.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(17.0), /*Z2*/ Some(6.5), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script(//AttackAirB
    agent = "jack", 
    script = "game_attackairb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attackairb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 43, /*KBG*/ 102, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 3.3, /*Z*/ -6.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.7), /*Z2*/ Some(-8.4), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 43, /*KBG*/ 102, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 4.7, /*Z*/ -11.1, /*X2*/ Some(0.0), /*Y2*/ Some(11.1), /*Z2*/ Some(-14.2), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 43, /*KBG*/ 102, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ -3.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(-5.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.5, /*Angle*/ 35, /*KBG*/ 102, /*FKB*/ 0, /*BKB*/ 41, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ -12.5, /*X2*/ Some(0.0), /*Y2*/ Some(15.0), /*Z2*/ Some(-12.5), /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.5, /*Angle*/ 35, /*KBG*/ 102, /*FKB*/ 0, /*BKB*/ 41, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.5), /*Z2*/ Some(-8.5), /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 35, /*KBG*/ 102, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.5), /*Z2*/ Some(-8.5), /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 35, /*KBG*/ 102, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ -12.5, /*X2*/ Some(0.0), /*Y2*/ Some(15.0), /*Z2*/ Some(-12.5), /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 4, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 35, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 16.5, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(20.5), /*Z2*/ Some(-9.0), /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 17.0, /*Angle*/ 43, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 54, /*Size*/ 7.5, /*X*/ -1.0, /*Y*/ 2.5, /*Z*/ 1.0, /*X2*/ Some(-3.0), /*Y2*/ Some(-0.5), /*Z2*/ Some(8.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 17.0, /*Angle*/ 43, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 54, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ 2.5, /*X2*/ Some(-4.5), /*Y2*/ Some(-6.0), /*Z2*/ Some(10.5), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 43, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 54, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -6.5, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(-5.5), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.5, /*Angle*/ 35, /*KBG*/ 102, /*FKB*/ 0, /*BKB*/ 41, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ -12.5, /*X2*/ Some(0.0), /*Y2*/ Some(15.0), /*Z2*/ Some(-12.5), /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.5, /*Angle*/ 35, /*KBG*/ 102, /*FKB*/ 0, /*BKB*/ 41, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.5), /*Z2*/ Some(-8.5), /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 35, /*KBG*/ 102, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.5), /*Z2*/ Some(-8.5), /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 35, /*KBG*/ 102, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ -12.5, /*X2*/ Some(0.0), /*Y2*/ Some(15.0), /*Z2*/ Some(-12.5), /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 4, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 35, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 16.5, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(20.5), /*Z2*/ Some(-9.0), /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}        

#[acmd_script(//AttackAirHi
    agent = "jack", 
    script = "game_attackairhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attackairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 3.5, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ 4.7, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.2, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 3.5, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 8.0, /*X*/ 4.7, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.2, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 9.0, /*Angle*/ 80, /*KBG*/ 136, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 11.5, /*X*/ 4.8, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(2.5), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 11.0, /*X*/ 4.3, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(1.5), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 20.0, /*Z*/ 1.5, /*X2*/ Some(0.0), /*Y2*/ Some(24.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script(//AttackAirLw
    agent = "jack", 
    script = "game_attackairlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_attackairlw(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 20.0, /*Angle*/ 270, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ -1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 20.0, /*Angle*/ 270, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 23.0, /*Angle*/ 270, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ -1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_flower"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 23.0, /*Angle*/ 270, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_flower"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 23.0, /*Angle*/ 270, /*KBG*/ 96, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(-4.0), /*Z2*/ Some(1.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_flower"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 23.0, /*Angle*/ 270, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ -1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_flower"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("knife"), /*Damage*/ 23.0, /*Angle*/ 270, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_flower"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 4, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script(//Catch
    agent = "jack", 
    script = "game_catch", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_catch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 5.3, /*X*/ 0.0, /*Y*/ 8.1, /*Z*/ 4.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.1), /*Z2*/ Some(13.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 4.65, /*X*/ 0.0, /*Y*/ 8.1, /*Z*/ 2.85, /*X2*/ Some(0.0), /*Y2*/ Some(8.1), /*Z2*/ Some(14.65), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        game_CaptureCutCommon(fighter);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

#[acmd_script(//CatchDash
    agent = "jack", 
    script = "game_catchdash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_catchdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 5.6, /*X*/ 0.0, /*Y*/ 6.6, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.6), /*Z2*/ Some(15.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 4.3, /*X*/ 0.0, /*Y*/ 6.6, /*Z*/ 2.7, /*X2*/ Some(0.0), /*Y2*/ Some(6.6), /*Z2*/ Some(16.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        game_CaptureCutCommon(fighter);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

#[acmd_script(//CatchTurn
    agent = "jack", 
    script = "game_catchturn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_catchturn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 5.3, /*X*/ 0.0, /*Y*/ 6.6, /*Z*/ -7.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.6), /*Z2*/ Some(-21.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 4.65, /*X*/ 0.0, /*Y*/ 6.6, /*Z*/ -5.35, /*X2*/ Some(0.0), /*Y2*/ Some(6.6), /*Z2*/ Some(-22.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        game_CaptureCutCommon(fighter);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

#[acmd_script(//CatchAttack
    agent = "jack", 
    script = "game_catchattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_catchattack(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.4, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KNEE);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 720, /*Rehit*/ 40, /* Damage*/ 1.6, /*Unk*/ false);
        DamageModule::heal(fighter.module_accessor, -4.0, 0);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//ThrowF
    agent = "jack", 
    script = "game_throwf", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_throwf(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 17.0, /*Angle*/ 45, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 65, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 15, 0);
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.0);
        FighterCutInManager::set_throw_finish_offset(FighterCutInManager(), Vector3f{x:6.0,y:-2.0,z:0.0});
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}  

#[acmd_script(//ThrowB
    agent = "jack", 
    script = "game_throwb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_throwb(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 19.0, /*Angle*/ 131, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 68, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, -7, 0);
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.2);
        FighterCutInManager::set_throw_finish_offset(FighterCutInManager(), Vector3f{x:-2.0,y:0.0,z:0.0});
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}  

#[acmd_script(//ThrowHi
    agent = "jack", 
    script = "game_throwhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_throwhi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 16.0, /*Angle*/ 90, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 36, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 3, 10);
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.5);
        FighterCutInManager::set_throw_finish_offset(FighterCutInManager(), Vector3f{x:2.0,y:6.0,z:0.0});
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}   

#[acmd_script(//ThrowLw
    agent = "jack", 
    script = "game_throwlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_throwlw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 16.5, /*Angle*/ 79, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 35, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 12.8, 0);
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.3);
        FighterCutInManager::set_throw_finish_offset(FighterCutInManager(), Vector3f{x:9.5,y:2.0,z:0.0});
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    }
}  

#[acmd_script(//CliffAttack
    agent = "jack", 
    script = "game_cliffattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_cliffattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 45, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 26.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(1.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//SlipAttack
    agent = "jack", 
    script = "game_slipattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_slipattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(-3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 9.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//DownAttackD
    agent = "jack", 
    script = "game_downattackd", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_downattackd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 14.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -17.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-10.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//DownAttackU
    agent = "jack", 
    script = "game_downattacku", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_downattacku(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 14.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -17.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-10.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//SpecialN1
    agent = "jack", 
    script = "game_specialn1", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialn1(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(1184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        }
    }
    if is_excute(fighter) {
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialN2
    agent = "jack", 
    script = "game_specialn2", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialn2(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(1184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        }
    }
    if is_excute(fighter) {
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialN3
    agent = "jack", 
    script = "game_specialn3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialn3(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(1184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        }
    }
    if is_excute(fighter) {
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialN1_Ex
    agent = "jack", 
    script = "game_specialn1_ex", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialn1_ex(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 12.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -14, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -11, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -9, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(fighter.lua_state_agent, 18.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -8.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -9, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialN2_Ex
    agent = "jack", 
    script = "game_specialn2_ex", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialn2_ex(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 12.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -14, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -11, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -9, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(fighter.lua_state_agent, 18.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -8.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -9, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialN3_Ex
    agent = "jack", 
    script = "game_specialn3_ex", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialn3_ex(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 12.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -14, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -11, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -9, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 18.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -8.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -9, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialNEscapeB
    agent = "jack", 
    script = "game_specialnescapeb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialnescapeb(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(74.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(174.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(1169.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(74.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(174.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(1169.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        }
    }
    if is_excute(fighter) {
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialNEscapeF
    agent = "jack", 
    script = "game_specialnescapef", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialnescapef(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(84.0), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 4, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.5, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(1184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(84.0), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 4, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(1184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        }
    }
    if is_excute(fighter) {
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialNEscapeB_Ex
    agent = "jack", 
    script = "game_specialnescapeb_ex", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialnescapeb_ex(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(24.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(34.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(69.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 28.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(24.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(34.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(69.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 34.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(24.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(34.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(69.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialNEscapeF_Ex
    agent = "jack", 
    script = "game_specialnescapef_ex", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialnescapef_ex(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 30, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.5), /*Z2*/ Some(69.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 8, 4);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 4, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(44.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(89.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 26.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(44.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(89.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 32.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(44.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(89.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialNJump
    agent = "jack", 
    script = "game_specialnjump", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialnjump(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor,&Vector3f{x:0.75,y:0.4,z:0.0});
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 365, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-50.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.3, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 365, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-100.0), /*Z2*/ Some(-9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 365, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-150.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 365, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-200.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 365, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-50.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.3, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 365, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-100.0), /*Z2*/ Some(-9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 365, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-150.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 365, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-200.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
    }
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 2.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 365, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-50.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.3, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 365, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-100.0), /*Z2*/ Some(-9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 365, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-150.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 365, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-200.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
    }
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.5, /*Angle*/ 365, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-50.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.3, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 365, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-100.0), /*Z2*/ Some(-9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 365, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-150.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 365, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-200.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.5, /*Angle*/ 365, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-50.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.3, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 365, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-100.0), /*Z2*/ Some(-9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 365, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-150.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 365, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-200.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
    }
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 2.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.5, /*Angle*/ 365, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-50.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.3, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.5, /*Angle*/ 365, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-100.0), /*Z2*/ Some(-9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.5, /*Angle*/ 365, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-150.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 365, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-200.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
    }
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.5, /*Angle*/ 365, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-50.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.3, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 50, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-100.0), /*Z2*/ Some(-9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 365, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-150.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 365, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-200.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.5, /*Angle*/ 365, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-50.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.3, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 50, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-100.0), /*Z2*/ Some(-9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 365, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-150.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 365, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-200.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
    }
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 2.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 365, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-50.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.3, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 50, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-100.0), /*Z2*/ Some(-9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 365, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-150.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 365, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-200.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
    }
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        KineticModule::add_speed(fighter.module_accessor,&Vector3f{x:0.0,y:-0.8,z:0.0});
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.5, /*Angle*/ 125, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-50.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.3, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 90, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-100.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 45, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-150.0), /*Z2*/ Some(-9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 45, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-200.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 2, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.5, /*Angle*/ 125, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-50.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.3, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 90, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-100.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 45, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-150.0), /*Z2*/ Some(-9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 45, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-200.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 2, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
    }
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//SpecialAirN1
    agent = "jack", 
    script = "game_specialairn1", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairn1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(1184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        }
    }
    if is_excute(fighter) {
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialAirN2
    agent = "jack", 
    script = "game_specialairn2", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairn2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(1184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        }
    }
    if is_excute(fighter) {
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialAirN3
    agent = "jack", 
    script = "game_specialairn3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairn3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(1184.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 1.1, /*Unk*/ false);
        }
    }
    if is_excute(fighter) {
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialAirN1_Ex
    agent = "jack", 
    script = "game_specialairn1_ex", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairn1_ex(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 12.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -14, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -11, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -9, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(fighter.lua_state_agent, 18.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -8.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -9, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialAirN2_Ex
    agent = "jack", 
    script = "game_specialairn2_ex", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairn2_ex(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 12.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -14, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -11, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -9, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(fighter.lua_state_agent, 18.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -8.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -9, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialAirN3_Ex
    agent = "jack", 
    script = "game_specialairn3_ex", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairn3_ex(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 12.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -14, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -11, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -9, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 18.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -12, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -8.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(84.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -9, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(1189.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialAirNDown
    agent = "jack", 
    script = "game_specialairndown", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairndown(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 1.0);
    for _ in 0..20 {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 50, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-50.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 45, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 29, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-100.0), /*Z2*/ Some(-2.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 55, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 29, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-150.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 45, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 29, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-200.0), /*Z2*/ Some(2.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 50, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-50.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
                ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 45, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 29, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-100.0), /*Z2*/ Some(-2.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
                ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 55, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 29, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-150.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
                ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 45, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 29, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-200.0), /*Z2*/ Some(2.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
                AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
                AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
                AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
                AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
            }
        }
        if is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
        }
        wait(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 6.0);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 50, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-50.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
                ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 45, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-100.0), /*Z2*/ Some(-2.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
                ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 55, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-150.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
                ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 45, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-200.0), /*Z2*/ Some(2.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
                AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
                AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
                AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
                AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
            }
        }
        if is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
        }
        wait(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 6.0);
    }
}

#[acmd_script(//SpecialAirNDownEnd
    agent = "jack", 
    script = "game_specialairndownend", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairndownend(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 125, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-50.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-100.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 45, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-150.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 45, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-200.0), /*Z2*/ Some(5.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 2, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 125, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-50.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 90, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-100.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 45, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-150.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 45, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(-200.0), /*Z2*/ Some(5.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 2, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
    }
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//SpecialAirNEscapeB
    agent = "jack", 
    script = "game_specialairnescapeb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairnescapeb(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(74.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(174.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(1169.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(74.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(174.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(1169.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        }
    }
    if is_excute(fighter) {
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialAirNEscapeF
    agent = "jack", 
    script = "game_specialairnescapef", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairnescapef(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(84.0), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 4, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.5, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(1184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(84.0), /*Hitlag*/ 0.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 4, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(1184.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        }
    }
    if is_excute(fighter) {
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialAirNEscapeB_Ex
    agent = "jack", 
    script = "game_specialairnescapeb_ex", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairnescapeb_ex(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(24.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(34.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(69.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 28.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(24.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(34.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(69.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 34.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(24.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 1, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(34.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(69.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialAirNEscapeF_Ex
    agent = "jack", 
    script = "game_specialairnescapef_ex", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairnescapef_ex(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 30, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.5), /*Z2*/ Some(69.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 8, 4);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 4, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(44.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(89.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 26.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(44.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(89.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 32.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(44.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(89.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script(//SpecialAirNShoot
    agent = "jack", 
    script = "game_specialairnshoot", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairnshoot(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 12, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.9, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 12, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.9, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.9, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.9, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.9, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.9, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.9, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.9, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.9, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.9, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.9, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 43.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("gunl"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.9, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(1000.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 15, /* Damage*/ 1.5, /*Unk*/ false);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//SpecialS1
    agent = "jack", 
    script = "game_specials1", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specials1(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_FIRE, false, 0);
    }
}

#[acmd_script(//SpecialAirS1
    agent = "jack", 
    script = "game_specialairs1", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairs1(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_FIRE, false, 0);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_SET_FALL_NORMAL);
    }
}

#[acmd_script(//Fly
    agent = "jack_fire", 
    script = "game_fly", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_fire_fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 5.2, /*X*/ 0.0, /*Y*/ -0.7, /*Z*/ 1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_MAGIC);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 5.2, /*X*/ 2.4, /*Y*/ -0.7, /*Z*/ 1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_MAGIC);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 5.2, /*X*/ -2.4, /*Y*/ -0.7, /*Z*/ 1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 720, /*Rehit*/ 20, /*Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 720, /*Rehit*/ 20, /*Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 720, /*Rehit*/ 20, /*Damage*/ 2.0, /*Unk*/ false);
    }
}

#[acmd_script(//Hit
    agent = "jack_fire", 
    script = "game_hit", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_fire_hit(fighter: &mut L2CAgentBase) {   
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.5, /*Angle*/ 75, /*KBG*/ 167, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 16.5, /*X*/ 0.0, /*Y*/ 1.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(0.0), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 960, /*Rehit*/ 40, /*Damage*/ 5.0, /*Unk*/ false);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
}

#[acmd_script(//Fly
    agent = "jack_fire2", 
    script = "game_fly", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_fire2_fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 90, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ -0.5, /*Z*/ 0.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_MAGIC);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 90, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 7.5, /*X*/ 3.0, /*Y*/ -0.5, /*Z*/ 0.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_MAGIC);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 90, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 7.5, /*X*/ -3.0, /*Y*/ -0.5, /*Z*/ 0.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 960, /*Rehit*/ 20, /*Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 960, /*Rehit*/ 20, /*Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 960, /*Rehit*/ 20, /*Damage*/ 1.5, /*Unk*/ false);
    }
}

#[acmd_script(//Hit
    agent = "jack_fire2", 
    script = "game_hit", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_fire2_hit(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.5, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 1.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1420, /*Rehit*/ 40, /*Damage*/ 5.0, /*Unk*/ false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.5, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 1.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1420, /*Rehit*/ 40, /*Damage*/ 5.0, /*Unk*/ false);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.5, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 1.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1420, /*Rehit*/ 40, /*Damage*/ 5.0, /*Unk*/ false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 73, /*KBG*/ 117, /*FKB*/ 0, /*BKB*/ 105, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 1.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1420, /*Rehit*/ 40, /*Damage*/ 5.0, /*Unk*/ false);
    }
}

#[acmd_script(//SpecialHi
    agent = "jack", 
    script = "game_specialhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialhi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, Hash40::new("special_hi"), false, 0.0);
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, 0.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_HI_FLAG_REVERSE_LR);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_EXTEND, smash::app::ArticleOperationTarget(0));
        AreaModule::reset_area(fighter.module_accessor, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        ENABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        AreaModule::reset_area(fighter.module_accessor, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
        ENABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("throw"), 10.0, 1.5, 1.5, 0.7, Some(1.5), Some(-30.0), Some(-6.4), *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
        SEARCH(fighter, 1, 0, Hash40::new("throw"), 15.5, 1.5, 1.5, 0.7, None, None, None, *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
        SEARCH(fighter, 2, 0, Hash40::new("throw"), 15.5, 1.2, 1.5, 0.7, None, None, None, *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 1, false);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        SET_SEARCH_SIZE_EXIST(fighter, 2, 8);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        UNABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_BACK, smash::app::ArticleOperationTarget(0));
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        UNABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
    }
}

#[acmd_script(//SpecialHiThrow
    agent = "jack", 
    script = "game_specialhithrow", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialhithrow(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, Hash40::new("special_hi_throw"), false, 0.0);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 35.0, /*Angle*/ 270, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 25, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, -20, 10);
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.5);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, smash::app::ArticleOperationTarget(0));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script(//SpecialAirHi
    agent = "jack", 
    script = "game_specialairhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairhi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, 0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_HI_FLAG_REVERSE_LR);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_JACK_CLIFF_HANG_DATA_AIR_LASSO as u32);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_EXTEND, smash::app::ArticleOperationTarget(0));
        AreaModule::reset_area(fighter.module_accessor, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        ENABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        AreaModule::reset_area(fighter.module_accessor, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
        ENABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("throw"), /*Damage*/ 48.0, /*Angle*/ 55, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 18, /*Size*/ 5.7, /*X*/ 1.5, /*Y*/ 2.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 48.0, /*Angle*/ 55, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 18, /*Size*/ 5.7, /*X*/ 0.0, /*Y*/ 22.0, /*Z*/ 3.2, /*X2*/ Some(0.0), /*Y2*/ Some(38.0), /*Z2*/ Some(10.6), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_OBJECT);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 1, false);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        UNABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_BACK, smash::app::ArticleOperationTarget(0));
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        UNABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
    }
}

#[acmd_script(//SpecialAirHiF
    agent = "jack", 
    script = "game_specialairhif", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairhif(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WING, Hash40::new("special_hi2_f"), false, 0.0);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 60.0, /*Angle*/ 80, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 18, /*Size*/ 9.3, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_HI2_FLAG_APPEAR_DOYLE);
    }
}

#[acmd_script(//SpecialAirHiB
    agent = "jack", 
    script = "game_specialairhib", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairhib(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WING, Hash40::new("special_hi2_b"), false, 0.0);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 60.0, /*Angle*/ 80, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 18, /*Size*/ 9.3, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_HI2_FLAG_APPEAR_DOYLE);
    }
}

#[acmd_script(//SpecialLwAttack
    agent = "jack", 
    script = "game_speciallwattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_speciallwattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 28.0, /*Angle*/ 46, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 18.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -3.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(3.0), /*Hitlag*/ 1.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script(//SpecialLwCounter
    agent = "jack", 
    script = "game_speciallwcounter", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_speciallwcounter(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 666.0, /*Angle*/ 361, /*KBG*/ 51, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ -5.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.5), /*Z2*/ Some(10.0), /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//SpecialAirLwAttack
    agent = "jack", 
    script = "game_specialairlwattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairlwattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 28.0, /*Angle*/ 46, /*KBG*/ 85, /*FKB*/ 82, /*BKB*/ 30, /*Size*/ 18.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -3.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(3.0), /*Hitlag*/ 1.25, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script(//SpecialAirLwCounter
    agent = "jack", 
    script = "game_specialairlwcounter", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_specialairlwcounter(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 666.0, /*Angle*/ 361, /*KBG*/ 51, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ -5.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.5), /*Z2*/ Some(10.0), /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//AppealHiR
    agent = "jack", 
    script = "game_appealhir", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_appealhir(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_STAFF), 0, 0, false, false);  
    }
}

#[acmd_script(//AppealHiL
    agent = "jack", 
    script = "game_appealhil", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_appealhil(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_STAFF), 0, 0, false, false);  
    }
}

#[acmd_script(//AppealSR
    agent = "jack", 
    script = "game_appealsr", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_appealsr(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_DEATHSCYTHE), 0, 0, false, false);
    }
}

#[acmd_script(//AppealSL
    agent = "jack", 
    script = "game_appealsl", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_appealsl(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_DEATHSCYTHE), 0, 0, false, false);
    }
}

#[acmd_script(//AppealLwR
    agent = "jack", 
    script = "game_appeallwr", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_appeallwr(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BACKSHIELD), 0, 0, false, false);
    }
}

#[acmd_script(//AppealLwL
    agent = "jack", 
    script = "game_appeallwl", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_appeallwl(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BACKSHIELD), 0, 0, false, false);
    }
}

#[acmd_script(//FinalEnd
    agent = "jack", 
    script = "game_finalend", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_finalend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        CANCEL_FILL_SCREEN(fighter, 0, 0);
        CANCEL_FILL_SCREEN(fighter, 1, 0);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_JACK_ATTACK_ABSOLUTE_KIND_FINAL, /*ID*/ 0, /*Damage*/ 999.0, /*Angle*/ 60, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 73, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_FINAL_FLAG_ABS_SET);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_FINAL_FLAG_END_FINAL);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_VISUAL_SCENE_FLAG_END_EXIT);
    }
}

#[acmd_script(//FinalAirEnd
    agent = "jack", 
    script = "game_finalairend", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn game_jack_finalairend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        CANCEL_FILL_SCREEN(fighter, 0, 0);
        CANCEL_FILL_SCREEN(fighter, 1, 0);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_JACK_ATTACK_ABSOLUTE_KIND_FINAL, /*ID*/ 0, /*Damage*/ 999.0, /*Angle*/ 60, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 73, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_FINAL_FLAG_ABS_SET);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_FINAL_FLAG_END_FINAL);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_VISUAL_SCENE_FLAG_END_EXIT);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        game_jack_attack11,
        game_jack_attack12,
        game_jack_attack13,
        game_jack_attackdash,
        game_jack_attacks3hi,
        game_jack_attacks3,
        game_jack_attacks3lw,
        game_jack_attackhi3,
        game_jack_attacklw3,
        game_jack_attacks4,
        game_jack_attackhi4,
        game_jack_attacklw4,
        game_jack_attackairn,
        game_jack_attackairf,
        game_jack_attackairb,
        game_jack_attackairhi,
        game_jack_attackairlw,
        game_jack_catch,
        game_jack_catchdash,
        game_jack_catchturn,
        game_jack_catchattack,
        game_jack_throwf,
        game_jack_throwb,
        game_jack_throwhi,
        game_jack_throwlw,
        game_jack_downattacku,
        game_jack_downattackd,
        game_jack_cliffattack,
        game_jack_slipattack,
        game_jack_specialn1,
        game_jack_specialn2,
        game_jack_specialn3,
        game_jack_specialn1_ex,
        game_jack_specialn2_ex,
        game_jack_specialn3_ex,
        game_jack_specialnescapeb,
        game_jack_specialnescapeb_ex,
        game_jack_specialnescapef,
        game_jack_specialnescapef_ex,
        game_jack_specialnjump,
        game_jack_specialairn1,
        game_jack_specialairn2,
        game_jack_specialairn3,
        game_jack_specialairn1_ex,
        game_jack_specialairn2_ex,
        game_jack_specialairn3_ex,
        game_jack_specialairnescapeb,
        game_jack_specialairnescapeb_ex,
        game_jack_specialairnescapef,
        game_jack_specialairnescapef_ex,
        game_jack_specialairndown,
        game_jack_specialairndownend,
        game_jack_specialairnshoot,
        game_jack_specials1,
        game_jack_specialairs1,
        game_jack_fire_fly,
        game_jack_fire_hit,
        game_jack_fire2_fly,
        game_jack_fire2_hit,
        game_jack_specialhi,
        game_jack_specialhithrow,
        game_jack_specialairhi,
        game_jack_specialairhif,
        game_jack_specialairhib,
        game_jack_speciallwattack,
        game_jack_speciallwcounter,
        game_jack_specialairlwattack,
        game_jack_specialairlwcounter,
        game_jack_appealsr,
        game_jack_appealsl,
        game_jack_appealhir,
        game_jack_appealhil,
        game_jack_appeallwr,
        game_jack_appeallwl,
        game_jack_finalend,
        game_jack_finalairend
    );
}
