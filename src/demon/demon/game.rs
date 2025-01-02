use crate::imports::BuildImports::*;

//Attack11
unsafe extern "C" fn game_demon_Attack11(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.444);
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(fighter.lua_state_agent, 5.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 14.5, /*Z*/ 9.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.2, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 8.75, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.2, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 3.25, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.2, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 8.75, /*X2*/ Some(0.0), /*Y2*/ Some(13.5), /*Z2*/ Some(8.75), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.2, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 3.25, /*X2*/ Some(0.0), /*Y2*/ Some(13.5), /*Z2*/ Some(3.25), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 33, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 5.6, /*X*/ 0.0, /*Y*/ 13.2, /*Z*/ 3.25, /*X2*/ Some(0.0), /*Y2*/ Some(13.2), /*Z2*/ Some(9.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 15.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 15.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 15.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 15.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 15.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.3);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.3);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.3);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
}

//Attack12
unsafe extern "C" fn game_demon_Attack12(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        JostleModule::set_push_speed_x_overlap_rate(fighter.module_accessor, 20.0);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_ENABLE_COMBO);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 14.25, /*Z*/ 9.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.4, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 8.75, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.4, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 3.25, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.4, /*X*/ 0.0, /*Y*/ 10.25, /*Z*/ 8.75, /*X2*/ Some(0.0), /*Y2*/ Some(13.0), /*Z2*/ Some(8.75), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.4, /*X*/ 0.0, /*Y*/ 10.25, /*Z*/ 3.25, /*X2*/ Some(0.0), /*Y2*/ Some(13.0), /*Z2*/ Some(3.25), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 30, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 12.9, /*Z*/ 3.25, /*X2*/ Some(0.0), /*Y2*/ Some(12.9), /*Z2*/ Some(9.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 16.0, /*Unk*/ false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        JostleModule::set_push_speed_x_overlap_rate(fighter.module_accessor, 0.0);
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH);
    }
}

//Attack13
unsafe extern "C" fn game_demon_Attack13(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.66);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 14.5, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 5.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 9.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 5.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 3.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 5.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 8.75, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(13.5), /*Z2*/ Some(9.0), /*Hitlag*/ 0.5, /*SDI*/ 5.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(13.5), /*Z2*/ Some(3.0), /*Hitlag*/ 0.5, /*SDI*/ 5.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 33, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(15.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

//FlashPunch
unsafe extern "C" fn game_demon_FlashPunch(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 35, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA_d, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 43, /*KBG*/ 112, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 9.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 43, /*KBG*/ 112, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(8.5), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 43, /*KBG*/ 112, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.5), /*Z2*/ Some(8.5), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Attack14
unsafe extern "C" fn game_demon_Attack14(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.5, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 4.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.5, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 9.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 4.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.5, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 4.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.5, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(13.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.5, /*SDI*/ 4.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.5, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(13.0), /*Z2*/ Some(4.0), /*Hitlag*/ 0.5, /*SDI*/ 4.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.5, /*Angle*/ 36, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(14.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

//Attack15
unsafe extern "C" fn game_demon_Attack15(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 42, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(15.0), /*Z2*/ Some(11.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 0, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 4.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 0, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 4.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 0, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 9.75, /*Z*/ 7.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 4.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 0, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 10.75, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 4.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 0, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 14.5, /*Z*/ 10.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 4.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 15.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 15.0, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

//Attack16
unsafe extern "C" fn game_demon_Attack16(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 0, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.9, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 0, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.9, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 7.25, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 0, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.9, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 0, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 0, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 10.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 9.0, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 0, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.9, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 0, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.9, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 7.25, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 0, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.9, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 0, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 10.5, /*X2*/ Some(0.0), /*Y2*/ Some(10.1), /*Z2*/ Some(10.5), /*Hitlag*/ 0.4, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 0, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 10.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 9.0, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

//Attack17
unsafe extern "C" fn game_demon_Attack17(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.66);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 0, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.3, /*X*/ 0.0, /*Y*/ 6.25, /*Z*/ 6.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 0, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.3, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 0, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 0, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 15.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 15.0, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

//Attack18
unsafe extern "C" fn game_demon_Attack18(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.5, /*Angle*/ 0, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.3, /*X*/ 0.0, /*Y*/ 6.25, /*Z*/ 8.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.5, /*Angle*/ 0, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.3, /*X*/ 0.0, /*Y*/ 6.75, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.5, /*Angle*/ 0, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.3, /*X*/ 0.0, /*Y*/ 7.25, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.5, /*Angle*/ 0, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 6.25, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.5, /*Angle*/ 0, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 5.75, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 3.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 12.0, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

//Attack19
unsafe extern "C" fn game_demon_Attack19(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 11.5, /*Z*/ 7.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x1985267897), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(7.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x1985267897), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_2 as u8);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

//Attack110
unsafe extern "C" fn game_demon_Attack110(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 53, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 7.0, /*X2*/ Some(0.0), /*Y2*/ Some(14.0), /*Z2*/ Some(6.5), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 53, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 5.5, /*X2*/ Some(0.0), /*Y2*/ Some(16.0), /*Z2*/ Some(7.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 53, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(5.5), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Attack100 
unsafe extern "C" fn game_demon_Attack100(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    for _ in 0..i32::MAX {
        frame(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.5, /*Angle*/ 85, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 8.2, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 13.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 2.5, /*Angle*/ 75, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 8.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 2.5, /*Angle*/ 90, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 8, /*Size*/ 8.0, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        }
        wait(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.5, /*Angle*/ 85, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 8.2, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 13.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 2.5, /*Angle*/ 75, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 8.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 2.5, /*Angle*/ 90, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 8, /*Size*/ 8.0, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        }
        wait(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.5, /*Angle*/ 85, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 8.2, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 13.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 2.5, /*Angle*/ 75, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 8.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 2.5, /*Angle*/ 90, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 8, /*Size*/ 8.0, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        }
        wait(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(fighter.lua_state_agent, 21.0);
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.5, /*Angle*/ 85, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 8.2, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 13.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 2.5, /*Angle*/ 75, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 8.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 2.5, /*Angle*/ 90, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 8, /*Size*/ 8.0, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        }
        wait(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(fighter.lua_state_agent, 27.0);
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.5, /*Angle*/ 85, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 8.2, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 13.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 2.5, /*Angle*/ 75, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 8.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 2.5, /*Angle*/ 90, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 8, /*Size*/ 8.0, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        }
        wait(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        wait_loop_clear(fighter);
    }
}

//Attack100Sub
unsafe extern "C" fn game_demon_Attack100Sub(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.5, /*Angle*/ 85, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 8.2, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 13.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 2.5, /*Angle*/ 75, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 8.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 2.5, /*Angle*/ 90, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 8, /*Size*/ 8.0, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.7, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Attack100End
unsafe extern "C" fn game_demon_Attack100End(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 45, /*KBG*/ 116, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(15.0), /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new_raw(0x193bdcb0cc), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackDash
unsafe extern "C" fn game_demon_AttackDash(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.75);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("footl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("footr"), HitStatus(*HIT_STATUS_XLU), 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 17.0, 8.0, 0, 4.0, 8.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        JostleModule::set_push_speed_x_overlap_rate(fighter.module_accessor, 0.05);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {    
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 19.0, /*Angle*/ 44, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 44, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(2.5), /*Z2*/ Some(1.5), /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("footl"), /*Damage*/ 22.0, /*Angle*/ 44, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 5.5, /*X*/ 0.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("footl"), /*Damage*/ 22.0, /*Angle*/ 44, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 7.5, /*X*/ 0.5, /*Y*/ -1.0, /*Z*/ -1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 44, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 5.6, /*X*/ 1.0, /*Y*/ 2.6, /*Z*/ 1.8, /*X2*/ Some(-1.0), /*Y2*/ Some(2.6), /*Z2*/ Some(1.8), /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_down_only(fighter.module_accessor, 4, true);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 1, false);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 4, false);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 19.0, /*Angle*/ 44, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 63, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("footl"), /*Damage*/ 22.0, /*Angle*/ 44, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 63, /*Size*/ 5.5, /*X*/ 0.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("footl"), /*Damage*/ 22.0, /*Angle*/ 44, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 63, /*Size*/ 7.5, /*X*/ 0.5, /*Y*/ -1.0, /*Z*/ -1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        JostleModule::set_push_speed_x_overlap_rate(fighter.module_accessor, 0.0);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackStand1
unsafe extern "C" fn game_demon_AttackStand1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_DEMON_REFLECTOR_KIND_ATTACK_STAND1, *FIGHTER_DEMON_REFLECTOR_GROUP_ATTACK_STAND1);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legl"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 24.0, /*Angle*/ 84, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 6.8, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 10.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 30, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 24.0, /*Angle*/ 84, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 6.8, /*X*/ 0.0, /*Y*/ 11.5, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 30, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 24.0, /*Angle*/ 84, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 7.2, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 2.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 30, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 20.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 20.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 20.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.1);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 24.0, /*Angle*/ 84, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 6.8, /*X*/ 0.0, /*Y*/ 7.25, /*Z*/ 7.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 30, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 24.0, /*Angle*/ 84, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 6.8, /*X*/ 0.0, /*Y*/ 11.5, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 30, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 24.0, /*Angle*/ 84, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 7.2, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 2.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 30, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 20.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 20.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 20.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.1);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackS3Hi
unsafe extern "C" fn game_demon_AttackS3Hi(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 17.0, 8.0, 0, 4.0, 8.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("footl"), HitStatus(*HIT_STATUS_XLU), 0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("footl"), /*Damage*/ 16.0, /*Angle*/ 20, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 37, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("footl"), /*Damage*/ 16.0, /*Angle*/ 20, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 37, /*Size*/ 7.25, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 16.0, /*Angle*/ 20, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 37, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("legl"), /*Damage*/ 16.0, /*Angle*/ 20, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 37, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 3, /*ShieldstunMul*/ 1.5);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackS3
unsafe extern "C" fn game_demon_AttackS3(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 17.0, 8.0, 0, 4.0, 8.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("footl"), HitStatus(*HIT_STATUS_XLU), 0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("footl"), /*Damage*/ 16.0, /*Angle*/ 20, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 37, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("footl"), /*Damage*/ 16.0, /*Angle*/ 20, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 37, /*Size*/ 7.25, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 16.0, /*Angle*/ 20, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 37, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("legl"), /*Damage*/ 16.0, /*Angle*/ 20, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 37, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 3, /*ShieldstunMul*/ 1.5);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackS3Lw
unsafe extern "C" fn game_demon_AttackS3Lw(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 17.0, 8.0, 0, 4.0, 8.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("footl"), HitStatus(*HIT_STATUS_XLU), 0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("footl"), /*Damage*/ 16.0, /*Angle*/ 20, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 37, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("footl"), /*Damage*/ 16.0, /*Angle*/ 20, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 37, /*Size*/ 7.25, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 16.0, /*Angle*/ 20, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 37, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("legl"), /*Damage*/ 16.0, /*Angle*/ 20, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 37, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 3, /*ShieldstunMul*/ 1.5);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackStand21
unsafe extern "C" fn game_demon_AttackStand21(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 5.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 0, /*KBG*/ 50, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 10.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 0, /*KBG*/ 50, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 2.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.5), /*Z2*/ Some(7.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 10, /*KBG*/ 40, /*FKB*/ 100, /*BKB*/ 0, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(13.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 10.0, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_CHECK_STEP);
    }
}

//AttackStand22
unsafe extern "C" fn game_demon_AttackStand22(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 65, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 9.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 65, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.25, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(7.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 15, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(6.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 2.0);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 2.0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 65, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.4), /*Z2*/ Some(12.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 65, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.75, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.2), /*Z2*/ Some(11.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 15, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 2.0);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 2.0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_CHECK_STEP);
    }
}

//AttackStand23
unsafe extern "C" fn game_demon_AttackStand23(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 75, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 75, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.75, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.2), /*Z2*/ Some(11.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 7.75, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.2), /*Z2*/ Some(11.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 2.0);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 2.0);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_CHECK_STEP);
    }
}

//AttackStand24
unsafe extern "C" fn game_demon_AttackStand24(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 17.0, 8.0, 0, 4.0, 8.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 53, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 12.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 53, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.0), /*Z2*/ Some(12.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 53, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(13.5), /*Z2*/ Some(12.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 53, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackStand31
unsafe extern "C" fn game_demon_AttackStand31(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        JostleModule::set_push_speed_x_overlap_rate(fighter.module_accessor, 0.1);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 90, /*KBG*/ 120, /*FKB*/ 61, /*BKB*/ 0, /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 17.0, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 90, /*KBG*/ 120, /*FKB*/ 61, /*BKB*/ 0, /*Size*/ 7.2, /*X*/ 0.0, /*Y*/ 17.0, /*Z*/ 11.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.2), /*Z2*/ Some(3.5), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 90, /*KBG*/ 120, /*FKB*/ 61, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 70, /*KBG*/ 120, /*FKB*/ 21, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 90, /*KBG*/ 120, /*FKB*/ 21, /*BKB*/ 0, /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 17.0, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 90, /*KBG*/ 120, /*FKB*/ 21, /*BKB*/ 0, /*Size*/ 7.2, /*X*/ 0.0, /*Y*/ 17.0, /*Z*/ 11.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.2), /*Z2*/ Some(3.5), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, 12.0, false);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        JostleModule::set_push_speed_x_overlap_rate(fighter.module_accessor, 0.0);
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_STAND_3_FLAG_CHECK_STEP);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

//AttackStand32
unsafe extern "C" fn game_demon_AttackStand32(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.78);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 17.0, 8.0, 0, 4.0, 8.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 12.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_XLU), 0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.2, /*X*/ 0.0, /*Y*/ 20.5, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 40, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_ignore_ground_shield(fighter.module_accessor, 0, true);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 6.4, /*X*/ 0.0, /*Y*/ 17.0, /*Z*/ 3.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 40, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_ignore_ground_shield(fighter.module_accessor, 1, true);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 6.4, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 40, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_ignore_ground_shield(fighter.module_accessor, 2, true);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.2, /*X*/ 0.0, /*Y*/ 20.5, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(21.5), /*Z2*/ Some(3.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 40, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_ignore_ground_shield(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 3, false);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.2, /*X*/ 0.0, /*Y*/ 17.0, /*Z*/ 10.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 40, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 6.4, /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ 6.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 40, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 6.4, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 3.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 40, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.5);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.2, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 40, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 6.4, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 40, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 6.4, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 40, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.5);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackStand4
unsafe extern "C" fn game_demon_AttackStand4(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.76);
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_XLU), 0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.5, /*Angle*/ 0, /*KBG*/ 2, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 8.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bind"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.5, /*Angle*/ 0, /*KBG*/ 2, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 9.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bind"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.5, /*Angle*/ 0, /*KBG*/ 2, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.2, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 1.5, /*X2*/ Some(0.0), /*Y2*/ Some(7.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bind"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.7);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.7);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.7);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.5, /*Angle*/ 0, /*KBG*/ 2, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 5.5, /*X2*/ Some(0.0), /*Y2*/ Some(6.4), /*Z2*/ Some(5.5), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bind"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.5, /*Angle*/ 0, /*KBG*/ 2, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 6.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bind"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.5, /*Angle*/ 0, /*KBG*/ 2, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.2, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 1.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(6.5), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bind"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.7);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.7);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.7);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackStand5
unsafe extern "C" fn game_demon_AttackStand5(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, -15.0, 0, 4.0, 8.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("bust"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 23.0, /*Angle*/ 46, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ 10.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 50, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 23.0, /*Angle*/ 46, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 10.2, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 5.5, /*X2*/ Some(0.0), /*Y2*/ Some(13.72), /*Z2*/ Some(10.5), /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 50, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackStand6
unsafe extern "C" fn game_demon_AttackStand6(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, -15.0, 0, 4.0, 8.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_STAND_6_FLAG_KEEP_SITUATION_AIR);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_STAND_6_FLAG_IGNORE_CHANGE_FALL);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legl"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 270, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ 11.25, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 270, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 10.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 270, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 13.25, /*Z*/ 6.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 270, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 3.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 270, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 11.5, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(3.0), /*Hitlag*/ 0.35, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_STAND_6_FLAG_KEEP_SITUATION_AIR);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_STAND_6_FLAG_IGNORE_CHANGE_FALL);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

//AttackHi3
unsafe extern "C" fn game_demon_AttackHi3(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("bust"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 88, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 71, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 7.5, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 88, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 71, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 15.5, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 88, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 71, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 70, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 46, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 7.5, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 70, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 46, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 15.5, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 70, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 46, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 6.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 6.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 6.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 6.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 6.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, 6.0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 88, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ 5.5, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 88, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 17.5, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 88, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 70, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 46, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ 5.5, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 70, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 46, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 17.5, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 70, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 46, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(24.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, 8.0, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_HI_3_FLAG_CHECK_STEP);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

//AttackHi32
unsafe extern "C" fn game_demon_AttackHi32(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("bust"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 81, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 14.5, /*Z*/ 9.75, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.45, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 81, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 7.3, /*X*/ 0.0, /*Y*/ 16.25, /*Z*/ 10.75, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.45, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 81, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 7.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.45, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 81, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 14.5, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.45, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 81, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 19.6, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.45, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 81, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 7.3, /*X*/ 0.0, /*Y*/ 21.3, /*Z*/ 7.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.45, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 81, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 17.2, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.45, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 81, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 15.5, /*Z*/ 3.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.45, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 81, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 20.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.45, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 81, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 7.3, /*X*/ 0.0, /*Y*/ 22.1, /*Z*/ 5.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.45, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 81, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 17.6, /*Z*/ 5.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.45, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 81, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 2.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.45, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackLw3
unsafe extern "C" fn game_demon_AttackLw3(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.55);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_ATTACK_LW_3_FLAG_CHECK_STEP);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 0.9);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 85, /*KBG*/ 38, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 9.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_ATTACKLW3, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 85, /*KBG*/ 38, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_ATTACKLW3, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 85, /*KBG*/ 38, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 8.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_ATTACKLW3, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 85, /*KBG*/ 38, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_ATTACKLW3, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 25.0, /*Unk*/ false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackSquat1
unsafe extern "C" fn game_demon_AttackSquat1(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 15.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 24.0, /*Angle*/ 0, /*KBG*/ 84, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 5.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 24.0, /*Angle*/ 0, /*KBG*/ 84, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 5.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 24.0, /*Angle*/ 0, /*KBG*/ 84, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.5), /*Z2*/ Some(5.5), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 30.0, /*Unk*/ false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.1);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 24.0, /*Angle*/ 0, /*KBG*/ 84, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 24.0, /*Angle*/ 0, /*KBG*/ 84, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 24.0, /*Angle*/ 0, /*KBG*/ 84, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.0), /*Z2*/ Some(10.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 30.0, /*Unk*/ false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.1);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

//AttackSquat2
unsafe extern "C" fn game_demon_AttackSquat2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 15.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 77, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 13.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 77, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 6.4, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 12.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 77, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(12.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 270, /*KBG*/ 10, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(12.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(fighter.module_accessor, 0, true);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 50.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 50.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 50.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 50.0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_INPUT);
    }
}

//AttackSquat3
unsafe extern "C" fn game_demon_AttackSquat3(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 15.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 55, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 12.25, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_saving"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 55, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 13.25, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_saving"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 55, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(11.5), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_saving"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_2 as u8);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_2 as u8);
        AttackModule::set_attack_level(fighter.module_accessor, 2, *FIGHTER_RYU_SAVING_LV_2 as u8);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 25.0, /*Unk*/ false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.5);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackSquat4
unsafe extern "C" fn game_demon_AttackSquat4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 15.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("bust"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 361, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 11.25, /*Z*/ 9.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("0x19f2214801"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_ATTACKSQUAT4, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 361, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(9.0), /*Hitlag*/ 2.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("0x19f2214801"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_ATTACKSQUAT4, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 361, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("0x19f2214801"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_ATTACKSQUAT4, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 2, true, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackS4Transform
unsafe extern "C" fn game_demon_AttackS4Transform(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_DEMON_STATUS_ATTACK_S4_WORK_INT_CRITICAL_HIT_NO);
    }
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 15.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 10.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 11.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 13.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 14.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 21.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_DEMON_STATUS_ATTACK_S4_WORK_INT_CRITICAL_HIT_NO);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 38.0, /*Angle*/ 361, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(13.5), /*Z2*/ Some(4.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 30, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 40.0, /*Angle*/ 361, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 30, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.75);
    }
    wait(fighter.lua_state_agent, 2.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_S4_FLAG_HIT) == false {
        if is_excute(fighter) {
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(fighter.lua_state_agent, 51.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 57.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 58.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 59.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 61.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//AttackS4
unsafe extern "C" fn game_demon_AttackS4(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_DEMON_STATUS_ATTACK_S4_WORK_INT_CRITICAL_HIT_NO);
    }
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 15.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 10.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 11.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 13.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 14.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 21.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_DEMON_STATUS_ATTACK_S4_WORK_INT_CRITICAL_HIT_NO);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 38.0, /*Angle*/ 361, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(13.5), /*Z2*/ Some(4.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 40.0, /*Angle*/ 361, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.1);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.75);
    }
    wait(fighter.lua_state_agent, 2.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_S4_FLAG_HIT) == false { 
        if is_excute(fighter) {
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(fighter.lua_state_agent, 51.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 57.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 58.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 59.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 61.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//AttackHi4Transform
unsafe extern "C" fn game_demon_AttackHi4Transform(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {;
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 9.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 11.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
    frame(fighter.lua_state_agent, 12.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 16.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 28.0, /*Angle*/ 90, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 28.0, /*Angle*/ 90, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 12.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 28.0, /*Angle*/ 90, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 3.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(2.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 2, false);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 28.0, /*Angle*/ 90, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 28.0, /*Angle*/ 90, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 12.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 28.0, /*Angle*/ 90, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 28.0, /*Angle*/ 90, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 12.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_S, false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(fighter.lua_state_agent, 51.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 54.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 9.0);
    frame(fighter.lua_state_agent, 56.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//AttackHi4
unsafe extern "C" fn game_demon_AttackHi4(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 9.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 11.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
    frame(fighter.lua_state_agent, 12.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 16.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 28.0, /*Angle*/ 90, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 28.0, /*Angle*/ 90, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 12.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 28.0, /*Angle*/ 90, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 3.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(2.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 2, false);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 28.0, /*Angle*/ 90, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 28.0, /*Angle*/ 90, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 12.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 28.0, /*Angle*/ 90, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 28.0, /*Angle*/ 90, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 12.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_S, false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 51.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 54.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 9.0);
    frame(fighter.lua_state_agent, 56.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//AttackLw4Transform
unsafe extern "C" fn game_demon_AttackLw4Transform(fighter: &mut L2CAgentBase) {
    let rand_effect = [Hash40::new("collision_attr_normal"), Hash40::new("collision_attr_fire"), Hash40::new("collision_attr_aura"), Hash40::new("collision_attr_purple"), Hash40::new("collision_attr_bind"), Hash40::new("collision_attr_bury"), Hash40::new("collision_attr_elec"), Hash40::new("collision_attr_coin"), Hash40::new("collision_attr_ice"), Hash40::new("collision_attr_cutup"), Hash40::new("collision_attr_pierce"), Hash40::new("collision_attr_flower"), Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_magic"), Hash40::new("collision_attr_sleep"), Hash40::new("collision_attr_curse_poison"), Hash40::new("collision_attr_saving"), Hash40::new("collision_attr_death")];
    let rng = smash::app::sv_math::rand(hash40("demon"), rand_effect.len() as i32);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 12.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 13.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(fighter.lua_state_agent, 14.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 15.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 16.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 8.2, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 8.2, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 25.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ShieldstunMul*/ 1.2);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 2, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 3, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 6.3, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 6.3, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 6.3, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 6.3, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 25.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ShieldstunMul*/ 1.2);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 2, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 3, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ -5.0, /*Y*/ 2.5, /*Z*/ 11.5, /*X2*/ Some(5.0), /*Y2*/ Some(2.5), /*Z2*/ Some(11.5), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 25.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ShieldstunMul*/ 1.2);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 2, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 3, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(fighter.lua_state_agent, 46.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 47.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 48.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 50.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 52.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 9.0);
    frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//AttackLw4
unsafe extern "C" fn game_demon_AttackLw4(fighter: &mut L2CAgentBase) {
    let rand_effect = [Hash40::new("collision_attr_normal"), Hash40::new("collision_attr_fire"), Hash40::new("collision_attr_aura"), Hash40::new("collision_attr_purple"), Hash40::new("collision_attr_bind"), Hash40::new("collision_attr_bury"), Hash40::new("collision_attr_elec"), Hash40::new("collision_attr_coin"), Hash40::new("collision_attr_ice"), Hash40::new("collision_attr_cutup"), Hash40::new("collision_attr_pierce"), Hash40::new("collision_attr_flower"), Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_magic"), Hash40::new("collision_attr_sleep"), Hash40::new("collision_attr_curse_poison"), Hash40::new("collision_attr_saving"), Hash40::new("collision_attr_death")];
    let rng = smash::app::sv_math::rand(hash40("demon"), rand_effect.len() as i32);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 12.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 13.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(fighter.lua_state_agent, 14.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 15.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 16.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 18.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 8.2, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 8.2, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 25.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ShieldstunMul*/ 1.2);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 2, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 3, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 6.3, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 6.3, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 6.3, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 6.3, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 25.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ShieldstunMul*/ 1.2);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 2, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 3, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ -5.0, /*Y*/ 2.5, /*Z*/ 11.5, /*X2*/ Some(5.0), /*Y2*/ Some(2.5), /*Z2*/ Some(11.5), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 277, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 25.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 25.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ShieldstunMul*/ 1.2);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 2, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_attack_level(fighter.module_accessor, 3, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 2000, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(fighter.lua_state_agent, 46.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 47.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 48.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 50.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 52.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 9.0);
    frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//AttackStep2s [Spinning Demon]
unsafe extern "C" fn game_demon_AttackStep2s(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 16.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 65, /*KBG*/ 100, /*FKB*/ 55, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.25, /*Z*/ 0.5, /*X2*/ Some(0.0), /*Y2*/ Some(2.25), /*Z2*/ Some(0.5), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G_d, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 55, /*BKB*/ 0, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.25, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.25), /*Z2*/ Some(8.25), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 65, /*KBG*/ 100, /*FKB*/ 55, /*BKB*/ 0, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(7.25), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 16.0, /*Unk*/ false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 65, /*KBG*/ 100, /*FKB*/ 55, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.25, /*Z*/ 0.5, /*X2*/ Some(0.0), /*Y2*/ Some(2.25), /*Z2*/ Some(0.5), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G_d, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 55, /*BKB*/ 0, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.25, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.25), /*Z2*/ Some(12.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 16.0, /*Unk*/ false);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 65, /*KBG*/ 100, /*FKB*/ 55, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.25, /*Z*/ 0.5, /*X2*/ Some(0.0), /*Y2*/ Some(2.25), /*Z2*/ Some(0.5), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G_d, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 55, /*BKB*/ 0, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.25, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.25), /*Z2*/ Some(11.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 16.0, /*Unk*/ false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLAG_CRITICAL);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 44, /*KBG*/ 71, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 44, /*KBG*/ 71, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.5), /*Z2*/ Some(8.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 44, /*KBG*/ 71, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 3.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(3.5), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackStep2 [Wind God Fist]
unsafe extern "C" fn game_demon_AttackStep2(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 1.7);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 16.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("bust"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 150.0, /*Angle*/ 88, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 150.0, /*Angle*/ 88, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 150.0, /*Angle*/ 88, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ -1.0, /*Y*/ 9.0, /*Z*/ 3.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 150.0, /*Angle*/ 73, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 150.0, /*Angle*/ 73, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 150.0, /*Angle*/ 73, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ -1.0, /*Y*/ 9.0, /*Z*/ 3.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.2);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 100.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 100.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 100.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 100.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 100.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, 100.0, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 3, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 4, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 5, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 150.0, /*Angle*/ 88, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 150.0, /*Angle*/ 88, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 150.0, /*Angle*/ 88, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ -1.0, /*Y*/ 13.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 150.0, /*Angle*/ 73, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 150.0, /*Angle*/ 73, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 150.0, /*Angle*/ 73, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ -1.0, /*Y*/ 13.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.2);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 100.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 100.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 100.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 100.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 100.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, 100.0, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 3, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 4, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 5, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 150.0, /*Angle*/ 88, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 150.0, /*Angle*/ 88, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20,  /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 150.0, /*Angle*/ 88, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20,  /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 150.0, /*Angle*/ 73, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 150.0, /*Angle*/ 73, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 150.0, /*Angle*/ 73, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.2);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 100.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 100.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 100.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 100.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 100.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, 100.0, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 3, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 4, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 5, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.9);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackStep2f [Electric Wind God Fist]
unsafe extern "C" fn game_demon_AttackStep2f(fighter: &mut L2CAgentBase) {
    let rand_effect = [Hash40::new("collision_attr_saving"), Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_sleep"), Hash40::new("collision_attr_bind")];
    let rng = smash::app::sv_math::rand(hash40("demon"), rand_effect.len() as i32);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 16.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("bust"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(fighter.lua_state_agent, 9.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLAG_CRITICAL);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 200.0, /*Angle*/ 88, /*KBG*/ 1, /*FKB*/ 0, /*BKB*/ 3, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 200.0, /*Angle*/ 88, /*KBG*/ 1, /*FKB*/ 0, /*BKB*/ 3, /*Size*/ 15.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 200.0, /*Angle*/ 88, /*KBG*/ 1, /*FKB*/ 0, /*BKB*/ 3, /*Size*/ 6.0, /*X*/ -1.0, /*Y*/ 9.0, /*Z*/ 3.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, /*ID1*/ 0, /*ID2*/ 1.2, /*ShieldstunMul*/ 5.0);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.2);
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_attack_level(fighter.module_accessor, 2, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 200.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 200.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 200.0, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 0, false);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 1, false);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 2, false);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 50.0, /*Angle*/ 88, /*KBG*/ 22, /*FKB*/ 0, /*BKB*/ 18, /*Size*/ 12.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 50.0, /*Angle*/ 88, /*KBG*/ 22, /*FKB*/ 0, /*BKB*/ 18, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 50.0, /*Angle*/ 88, /*KBG*/ 22, /*FKB*/ 0, /*BKB*/ 18, /*Size*/ 12.0, /*X*/ -1.0, /*Y*/ 13.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_invalid_invincible(fighter.module_accessor, 0, true);
        AttackModule::set_invalid_invincible(fighter.module_accessor, 1, true);
        AttackModule::set_invalid_invincible(fighter.module_accessor, 2, true);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 1.2);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 19.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 19.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 19.0, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    FT_MOTION_RATE(fighter, /*FSM*/ 0.9);
}

//AttackStep2l [Dragon Uppercut]
unsafe extern "C" fn game_demon_AttackStep2l(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.9);
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 13.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("bust"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(fighter.lua_state_agent, 19.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 1.5);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLAG_CRITICAL);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 4000.0, /*Angle*/ 60, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4000.0, /*Angle*/ 60, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 8.25, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 4.75, /*X2*/ Some(0.0), /*Y2*/ Some(16.0), /*Z2*/ Some(7.0), /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4000.0, /*Angle*/ 60, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 8.25, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 2.5, /*X2*/ Some(0.0), /*Y2*/ Some(10.5), /*Z2*/ Some(4.75), /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 0.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 2, /*ShieldstunMul*/ 0.5);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 2, false);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 1.6);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 999.0, /*Angle*/ 70, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.1), /*Hitlag*/ 0.17, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 70, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(23.0), /*Z2*/ Some(3.0), /*Hitlag*/ 0.17, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 0.5);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLAG_CRITICAL);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 60.0, /*Angle*/ 70, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 18, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.17, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 60.0, /*Angle*/ 70, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 18, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(24.0), /*Z2*/ Some(3.0), /*Hitlag*/ 0.17, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 0.5);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 55.0, /*Angle*/ 70, /*KBG*/ 61, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 55.0, /*Angle*/ 70, /*KBG*/ 61, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 21.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(24.0), /*Z2*/ Some(3.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 50.0, /*Angle*/ 70, /*KBG*/ 62, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 21.5, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(24.5), /*Z2*/ Some(3.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 45.0, /*Angle*/ 70, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 22.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(25.0), /*Z2*/ Some(3.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.4);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackAirN
unsafe extern "C" fn game_demon_AttackAirN(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 22.0, /*Angle*/ 285, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 9.25, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 22.0, /*Angle*/ 300, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 9.25, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 280, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 9.25, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 270, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 6.25, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 0, false);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 1, false);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 290, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 9.25, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 290, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 6.25, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirF
unsafe extern "C" fn game_demon_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.0, 0, 10.0, 0.0, 0, 4.0, 13.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("toel"), /*Damage*/ 21.0, /*Angle*/ 20, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 20, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 20, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 20, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("toel"), /*Damage*/ 17.0, /*Angle*/ 362, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 362, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 362, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 362, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirB
unsafe extern "C" fn game_demon_AttackAirB(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 10.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("toer"), /*Damage*/ 26.0, /*Angle*/ 40, /*KBG*/ 71, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 7.0, /*X*/ -0.3, /*Y*/ -1.0, /*Z*/ 1.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 40, /*KBG*/ 71, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ -9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(-3.0), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 40, /*KBG*/ 71, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ -11.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(-3.0), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 2, false);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("toer"), /*Damage*/ 20.0, /*Angle*/ 40, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 7.0, /*X*/ -0.3, /*Y*/ -1.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 40, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ -11.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(-3.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}        

//AttackAirHi
unsafe extern "C" fn game_demon_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("toer"), /*Damage*/ 20.0, /*Angle*/ 77, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 77, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 77, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 3.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 77, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 77, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 3.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 77, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 77, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 3.5, /*X2*/ Some(0.0), /*Y2*/ Some(15.5), /*Z2*/ Some(4.0), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 77, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 77, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 3.5, /*X2*/ Some(0.0), /*Y2*/ Some(15.5), /*Z2*/ Some(4.0), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 70, /*KBG*/ 76, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(21.0), /*Z2*/ Some(2.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 70, /*KBG*/ 76, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 2.0, /*X2*/ Some(0.0), /*Y2*/ Some(15.5), /*Z2*/ Some(2.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 70, /*KBG*/ 76, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(21.0), /*Z2*/ Some(2.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 70, /*KBG*/ 76, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 2.0, /*X2*/ Some(0.0), /*Y2*/ Some(15.5), /*Z2*/ Some(2.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("toer"), /*Damage*/ 18.0, /*Angle*/ 77, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 70, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 21.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 70, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 2.0, /*X2*/ Some(0.0), /*Y2*/ Some(15.5), /*Z2*/ Some(2.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 70, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 21.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 70, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 2.0, /*X2*/ Some(0.0), /*Y2*/ Some(15.5), /*Z2*/ Some(2.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirLw
unsafe extern "C" fn game_demon_AttackAirLw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_KEEP_AIR);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        SET_SPEED_EX(fighter, 0.0, 2.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        SET_SPEED_EX(fighter, 0.0, 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        SET_SPEED_EX(fighter, 0.0, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        SET_SPEED_EX(fighter, 0.0, 1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        SET_SPEED_EX(fighter, 0.0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        SET_SPEED_EX(fighter, 0.0, -1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        SET_SPEED_EX(fighter, 0.0, -2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("legl"), /*Damage*/ 10.0, /*Angle*/ 300, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 10.0, /*Angle*/ 300, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("footl"), /*Damage*/ 10.0, /*Angle*/ 300, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("legl"), /*Damage*/ 10.0, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 55, /*BKB*/ 0, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 10.0, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 55, /*BKB*/ 0, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("footl"), /*Damage*/ 10.0, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 55, /*BKB*/ 0, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        SET_SPEED_EX(fighter, 0.0, -3.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_KEEP_AIR);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("legl"), /*Damage*/ 35.0, /*Angle*/ 270, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 35.0, /*Angle*/ 270, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 9.0, /*X*/ -0.5, /*Y*/ -2.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 1, /*Bone*/ Hash40::new("footl"), /*Damage*/ 35.0, /*Angle*/ 270, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 10.5, /*X*/ -1.0, /*Y*/ -0.5, /*Z*/ 3.5, /*X2*/ Some(-6.5), /*Y2*/ Some(-8.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("legl"), /*Damage*/ 35.0, /*Angle*/ 270, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 35.0, /*Angle*/ 270, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 1, /*Bone*/ Hash40::new("footl"), /*Damage*/ 35.0, /*Angle*/ 270, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 10.5, /*X*/ -0.2, /*Y*/ -0.5, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//LandingAirLw
unsafe extern "C" fn game_demon_LandingAirLw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 1, false);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 2, false);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 30.0, /*Angle*/ 50, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 22.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(-10.0), /*Hitlag*/ 0.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 50, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bind"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Catch
unsafe extern "C" fn game_demon_Catch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 7.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(14.2), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(15.2), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        game_CaptureCutCommon(fighter);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//CatchDash
unsafe extern "C" fn game_demon_CatchDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 6.6, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.6), /*Z2*/ Some(15.7), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 6.6, /*Z*/ 3.2, /*X2*/ Some(0.0), /*Y2*/ Some(6.6), /*Z2*/ Some(16.5), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        game_CaptureCutCommon(fighter);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//CatchTurn
unsafe extern "C" fn game_demon_CatchTurn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 6.3, /*X*/ 0.0, /*Y*/ 6.6, /*Z*/ -5.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.6), /*Z2*/ Some(-19.5), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 5.65, /*X*/ 0.0, /*Y*/ 6.6, /*Z*/ -3.35, /*X2*/ Some(0.0), /*Y2*/ Some(6.6), /*Z2*/ Some(-20.1), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        game_CaptureCutCommon(fighter);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//CatchCommand
unsafe extern "C" fn game_demon_CatchCommand(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.6);
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
    }
    FT_START_ADJUST_MOTION_FRAME_arg1(fighter, 1.0);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 6.6, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.6), /*Z2*/ Some(21.2), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 6.6, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.6), /*Z2*/ Some(22.2), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        game_CaptureCutCommon(fighter);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_CATCH_COMMAND_FLAG_CHANGE_THROW);
    }
}

//CatchAttack
unsafe extern "C" fn game_demon_CatchAttack(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.4);
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_CATCHATTACK, /*Type*/ *ATTACK_REGION_PUNCH);
        DamageModule::heal(fighter.module_accessor, -5.25, 0);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//ThrowF
unsafe extern "C" fn game_demon_ThrowF(fighter: &mut L2CAgentBase) {
    if !smash2::app::FighterCutInManager::is_vr_mode() {
        if smash2::app::FighterCutInManager::is_one_on_one_including_thrown(&*(fighter.module_accessor as *const BattleObjectModuleAccessor)) {
            if is_excute(fighter) {
                FighterSpecializer_Demon::check_disabled_motion_camera_of_scale(fighter.module_accessor);
                FighterSpecializer_Demon::check_disabled_motion_camera_of_stage(fighter.module_accessor);
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_THROW_MOTION_CAMERA) {
                if is_excute(fighter) {
                    CHECK_VALID_START_CAMERA(fighter, 0, 0, 0, 0, 0, 0, false);
                }
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_THROW_MOTION_CAMERA) {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
                        if is_excute(fighter) {
                            REQ_MOTION_CAMERA(fighter, Hash40::new("e01throwf.nuanmb"), false);
                        }
                    }
                }
                if is_excute(fighter) {
                    let scale = PostureModule::scale(fighter.module_accessor);
                    CAM_ZOOM_IN_arg5(fighter, 7.0, 0.0, scale * 1.5, 0.0, 0.0);
                }
            }
        }
    }
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 10.0, /*Angle*/ 40, /*KBG*/ 91, /*FKB*/ 0, /*BKB*/ 50, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 11.0);
    if PostureModule::scale(fighter.module_accessor) <= 1.4 {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("footr"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        }
        else {
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
                ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("footr"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
            }
        }
    }
    if is_excute(fighter) {
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 39.0);
    if PostureModule::scale(fighter.module_accessor) <= 1.4 {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 5.0, /*X*/ 1.0, /*Y*/ 14.0, /*Z*/ 5.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 5.0, /*X*/ 1.5, /*Y*/ 15.5, /*Z*/ 10.5, /*X2*/ Some(5.0), /*Y2*/ Some(15.5), /*Z2*/ Some(10.5), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        }
        else {
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 4.0, /*X*/ 1.0, /*Y*/ 14.0, /*Z*/ 5.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
                ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 4.0, /*X*/ 1.5, /*Y*/ 16.0, /*Z*/ 9.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
            }
        }
    }
    if is_excute(fighter) {
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 40.0);
    if PostureModule::scale(fighter.module_accessor) <= 1.4 {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("footr"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        }
        else {
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
                ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("footr"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 58, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
            }
        }
    }
    if is_excute(fighter) {
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        CHECK_FINISH_CAMERA(fighter, 9, 1);
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.3);
        FighterCutInManager::set_throw_finish_offset(FighterCutInManager(), Vector3f{x:5.0,y:2.0,z:0.0});
    }        
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    	CAM_ZOOM_OUT(fighter, );
    }
}  

//ThrowB
unsafe extern "C" fn game_demon_ThrowB(fighter: &mut L2CAgentBase) {
    if !smash2::app::FighterCutInManager::is_vr_mode() {
        if smash2::app::FighterCutInManager::is_one_on_one_including_thrown(&*(fighter.module_accessor as *const BattleObjectModuleAccessor)) {
            if is_excute(fighter) {
                FighterSpecializer_Demon::check_disabled_motion_camera_of_scale(fighter.module_accessor);
                FighterSpecializer_Demon::check_disabled_motion_camera_of_stage(fighter.module_accessor);
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_THROW_MOTION_CAMERA) {
                if is_excute(fighter) {
                    CHECK_VALID_START_CAMERA(fighter, 0, 7, 0, 35, 0, 0, false);
                }
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_THROW_MOTION_CAMERA) {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
                        if is_excute(fighter) {
                            REQ_MOTION_CAMERA(fighter, Hash40::new("e01throwb.nuanmb"), false);
                        }
                    }
                }
                if is_excute(fighter) {
                    let scale = PostureModule::scale(fighter.module_accessor);
                    CAM_ZOOM_IN_arg5(fighter, 7.0, 0.0, scale * 1.5, 0.0, 0.0);
                } 
            }
        }
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PostureModule::reverse_lr(fighter.module_accessor);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 1.0, /*Angle*/ 58, /*KBG*/ 300, /*FKB*/ 0, /*BKB*/ 30, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 30.0, /*Angle*/ 58, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(2.0), /*Z2*/ Some(-22.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_BODY);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 600, /*Rehit*/ 30, /* Damage*/ 1.7, /*Unk*/ false);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        CHECK_FINISH_CAMERA(fighter, 17, 0);
    }
    frame(fighter.lua_state_agent, 47.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    	CAM_ZOOM_OUT(fighter);
        AttackModule::clear_all(fighter.module_accessor);
    }
}  

//ThrowHi
unsafe extern "C" fn game_demon_ThrowHi(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 2.0, /*Angle*/ 76, /*KBG*/ 45, /*FKB*/ 10, /*BKB*/ 65, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(fighter.lua_state_agent, 38.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, false, 0);
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 43.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 48.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
    frame(fighter.lua_state_agent, 56.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 63.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 70.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 72.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 73.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 74.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 75.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}   

//ThrowLw
unsafe extern "C" fn game_demon_ThrowLw(fighter: &mut L2CAgentBase) {
    if !smash2::app::FighterCutInManager::is_vr_mode() {
        if smash2::app::FighterCutInManager::is_one_on_one_including_thrown(&*(fighter.module_accessor as *const BattleObjectModuleAccessor)) {
            if is_excute(fighter) {
                FighterSpecializer_Demon::check_disabled_motion_camera_of_scale(fighter.module_accessor);
                FighterSpecializer_Demon::check_disabled_motion_camera_of_stage(fighter.module_accessor);
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_THROW_MOTION_CAMERA) {
                if is_excute(fighter) {
                    CHECK_VALID_START_CAMERA(fighter, 0, 0, 0, 0, 0, 0, false);
                }
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_THROW_MOTION_CAMERA) {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
                        if is_excute(fighter) {
                            REQ_MOTION_CAMERA(fighter, Hash40::new("e01throwlw.nuanmb"), false);
                        }
                    }
                }
                if is_excute(fighter) {
                    let scale = PostureModule::scale(fighter.module_accessor);
                    CAM_ZOOM_IN_arg5(fighter, 7.0, 0.0, scale * 1.5, 0.0, 0.0);
                } 
            }
        }
    }
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 8.0, /*Angle*/ 72, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 30, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 50, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH02, /*Type*/ *ATTACK_REGION_HEAD);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        CHECK_FINISH_CAMERA(fighter, 9, 4);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(fighter.module_accessor);
        CAM_ZOOM_OUT(fighter);
    }
}  

//ThrowCommand
unsafe extern "C" fn game_demon_ThrowCommand(fighter: &mut L2CAgentBase) {
    if !smash2::app::FighterCutInManager::is_vr_mode() {
        if smash2::app::FighterCutInManager::is_one_on_one_including_thrown(&*(fighter.module_accessor as *const BattleObjectModuleAccessor)) {
            if is_excute(fighter) {
                FighterSpecializer_Demon::check_disabled_motion_camera_of_scale(fighter.module_accessor);
                FighterSpecializer_Demon::check_disabled_motion_camera_of_stage(fighter.module_accessor);
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_THROW_MOTION_CAMERA) {
                if is_excute(fighter) {
                    CHECK_VALID_START_CAMERA(fighter, 0, 7, 0, 50, 30, 0, false);
                }
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_THROW_MOTION_CAMERA) {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
                        if is_excute(fighter) {

                            REQ_MOTION_CAMERA(fighter, Hash40::new("e01throwcommand.nuanmb"), false);
                        }
                    }
                }
                if is_excute(fighter) {
                    let scale = PostureModule::scale(fighter.module_accessor);
                    CAM_ZOOM_IN_arg5(fighter, 7.0, 0.0, scale * 1.5, 0.0, 0.0);
                } 
            }
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PostureModule::reverse_lr(fighter.module_accessor);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 5.0, /*Angle*/ 165, /*KBG*/ 137, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 60, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ -3.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_THROWCOMMAND, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_THROW_COMMAND_FLAG_USE_OTHER_PARAM) == false {
        if is_excute(fighter) {
            CHECK_FINISH_CAMERA(fighter, 18, 2);
        }
        else {
            if is_excute(fighter) {
                CHECK_FINISH_CAMERA(fighter, 18, 15);
            }
        }
    }
    if is_excute(fighter) {
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.3);
        FighterCutInManager::set_throw_finish_offset(FighterCutInManager(), Vector3f{x:9.0,y:2.0,z:0.0});
    }
    frame(fighter.lua_state_agent, 80.0);
    let rng_hitbox = smash::app::sv_math::rand(hash40("demon"), 4);
    if rng_hitbox == 0 {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 19, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ -14.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(-14.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_THROWCOMMAND, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 19, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ -6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_THROWCOMMAND, /*Type*/ *ATTACK_REGION_KICK);
            AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        }
        else {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 30.0, /*Angle*/ 19, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ -14.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(-14.0), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_THROWCOMMAND, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 30.0, /*Angle*/ 19, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ -6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_THROWCOMMAND, /*Type*/ *ATTACK_REGION_KICK);
            AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        }
    }
    frame(fighter.lua_state_agent, 81.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    	CAM_ZOOM_OUT(fighter);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//CliffAttack
unsafe extern "C" fn game_demon_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 45, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SlipAttack
unsafe extern "C" fn game_demon_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(-3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 9.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//DownAttackD
unsafe extern "C" fn game_demon_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 48, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 48, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//DownAttackU
unsafe extern "C" fn game_demon_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 48, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 48, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialNStart
unsafe extern "C" fn game_demon_SpecialNStart(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    frame(fighter.lua_state_agent, 11.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 17.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 19.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 23.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
}

//SpecialAirNStart
unsafe extern "C" fn game_demon_SpecialAirNStart(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    frame(fighter.lua_state_agent, 11.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 17.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 19.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 23.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
}

//SpecialNHi
unsafe extern "C" fn game_demon_SpecialNHi(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, /*Rate*/ 2.0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, false, 0);
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_SCALING);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_SCALING);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 60.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 64.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 65.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 66.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 67.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//SpecialN
unsafe extern "C" fn game_demon_SpecialN(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, /*Rate*/ 2.0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, false, 0);
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_SCALING);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_SCALING);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 60.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 64.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 65.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 66.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 67.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//SpecialNLw
unsafe extern "C" fn game_demon_SpecialNLw(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, /*Rate*/ 2.0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, false, 0);
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_SCALING);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_SCALING);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 60.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 64.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 65.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 66.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 67.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//SpecialAirNHi
unsafe extern "C" fn game_demon_SpecialAirNHi(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, /*Rate*/ 2.0);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, false, 0);
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 43.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 46.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 50.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 54.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 56.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 58.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 60.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//SpecialAirN
unsafe extern "C" fn game_demon_SpecialAirN(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, /*Rate*/ 2.0);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, false, 0);
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 43.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 46.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 50.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 54.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 56.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 58.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 60.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//SpecialAirNLw
unsafe extern "C" fn game_demon_SpecialAirNLw(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, /*Rate*/ 2.0);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, false, 0);
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 43.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 46.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 50.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 54.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 56.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 58.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 60.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//SpecialS
unsafe extern "C" fn game_demon_SpecialS(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.5);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 16.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 5.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 14.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 35.0, /*Angle*/ 60, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(10.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 80, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("0x184c223f47"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 35.0, /*Angle*/ 60, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(10.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 80, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("0x184c223f47"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_S_FLAG_CHANGE_HIT);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 16.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 1.0);
    frame(fighter.lua_state_agent, 19.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
    frame(fighter.lua_state_agent, 24.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 41.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 58.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 60.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 61.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 62.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//SpecialAirS
unsafe extern "C" fn game_demon_SpecialAirS(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.5);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 16.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 5.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 14.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 35.0, /*Angle*/ 60, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(10.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 80, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("0x184c223f47"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 35.0, /*Angle*/ 60, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(10.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 80, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("0x184c223f47"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_S_FLAG_CHANGE_HIT);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 16.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 1.0);
    frame(fighter.lua_state_agent, 19.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
    frame(fighter.lua_state_agent, 24.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 41.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 58.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 60.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 61.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 62.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//SpecialSHit
unsafe extern "C" fn game_demon_SpecialSHit(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 1.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 16.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 29.0, /*Angle*/ 100, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(10.0), /*Hitlag*/ 0.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 70, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 29.0, /*Angle*/ 100, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(10.0), /*Hitlag*/ 0.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 70, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialAirSHit
unsafe extern "C" fn game_demon_SpecialAirSHit(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 1.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 16.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 29.0, /*Angle*/ 100, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(10.0), /*Hitlag*/ 0.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 70, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 29.0, /*Angle*/ 100, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(10.0), /*Hitlag*/ 0.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 70, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialHi
unsafe extern "C" fn game_demon_SpecialHi(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_HI_FLAG_AIR) == true {
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 76, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.25), /*Z2*/ Some(6.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 76, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 76, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -2.5, /*X2*/ Some(0.0), /*Y2*/ Some(10.25), /*Z2*/ Some(-2.5), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
        }
        frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 71, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.15, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
        }
        frame(fighter.lua_state_agent, 7.0);
        if is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 63, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.6, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
        }
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 60, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
        }
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 46.0);
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
        frame(fighter.lua_state_agent, 52.0);
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
        if is_excute(fighter) {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        }
        frame(fighter.lua_state_agent, 53.0);
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
        frame(fighter.lua_state_agent, 54.0);
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
    }
    else {
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 76, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.25), /*Z2*/ Some(6.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 76, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
            ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 76, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -2.5, /*X2*/ Some(0.0), /*Y2*/ Some(10.25), /*Z2*/ Some(-2.5), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
        }
        frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 71, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.15, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
        }
        frame(fighter.lua_state_agent, 7.0);
        if is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 63, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.6, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
        }
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 60, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
        }
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 46.0);
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
        frame(fighter.lua_state_agent, 52.0);
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
        if is_excute(fighter) {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        }
        frame(fighter.lua_state_agent, 53.0);
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
        frame(fighter.lua_state_agent, 54.0);
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
    }
}

//SpecialHiGround
unsafe extern "C" fn game_demon_SpecialHiGround(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 76, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.25), /*Z2*/ Some(6.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 76, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 76, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -2.5, /*X2*/ Some(0.0), /*Y2*/ Some(10.25), /*Z2*/ Some(-2.5), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 71, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.15, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 63, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.6, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 60, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 46.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 52.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 54.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//SpecialHiAir
unsafe extern "C" fn game_demon_SpecialHiAir(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 76, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.25), /*Z2*/ Some(6.0), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 76, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 76, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -2.5, /*X2*/ Some(0.0), /*Y2*/ Some(10.25), /*Z2*/ Some(-2.5), /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 71, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.15, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 63, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.6, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 66.6, /*Angle*/ 60, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 46.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 52.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 54.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//SpecialLw
unsafe extern "C" fn game_demon_SpecialLw(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 5.0, /*Angle*/ 70, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
    frame(fighter.lua_state_agent, 16.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.75, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 7.75, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.75), /*Z2*/ Some(10.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 8.75, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 7.75, /*Z*/ 12.5, /*X2*/ Some(0.0), /*Y2*/ Some(7.75), /*Z2*/ Some(9.5), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
        GrabModule::set_constraint(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.75), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.75), /*Z2*/ Some(10.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.75), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.75), /*Z2*/ Some(10.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
        GrabModule::set_constraint(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        SET_SPEED_EX(fighter, 0.1, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    FT_MOTION_RATE(fighter, /*FSM*/ 0.9);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 70, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 4.5, /*X2*/ Some(0.0), /*Y2*/ Some(23.0), /*Z2*/ Some(4.5), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 60, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(16.0), /*Z2*/ Some(6.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 64, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(5.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(fighter.module_accessor, 2, true);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 2, false);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 70, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 19.5, /*Z*/ 4.5, /*X2*/ Some(0.0), /*Y2*/ Some(23.5), /*Z2*/ Some(3.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 60, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(16.0), /*Z2*/ Some(6.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 70, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 20.0, /*Z*/ 4.5, /*X2*/ Some(0.0), /*Y2*/ Some(24.0), /*Z2*/ Some(3.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 1, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 25.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//SpecialAirLw
unsafe extern "C" fn game_demon_SpecialAirLw(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 5.0, /*Angle*/ 70, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
    frame(fighter.lua_state_agent, 16.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.75, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 7.75, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.75), /*Z2*/ Some(10.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 8.75, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 7.75, /*Z*/ 12.5, /*X2*/ Some(0.0), /*Y2*/ Some(7.75), /*Z2*/ Some(9.5), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
        GrabModule::set_constraint(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.75), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.75), /*Z2*/ Some(10.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.75), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.75), /*Z2*/ Some(10.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
        GrabModule::set_constraint(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        SET_SPEED_EX(fighter, 0.1, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    FT_MOTION_RATE(fighter, /*FSM*/ 0.9);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 70, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 4.5, /*X2*/ Some(0.0), /*Y2*/ Some(23.0), /*Z2*/ Some(4.5), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 60, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(16.0), /*Z2*/ Some(6.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 64, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(5.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(fighter.module_accessor, 2, true);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 2, false);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 70, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 19.5, /*Z*/ 4.5, /*X2*/ Some(0.0), /*Y2*/ Some(23.5), /*Z2*/ Some(3.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 60, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(16.0), /*Z2*/ Some(6.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 70, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 20.0, /*Z*/ 4.5, /*X2*/ Some(0.0), /*Y2*/ Some(24.0), /*Z2*/ Some(3.0), /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 1, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 25.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//SpecialLwGround
unsafe extern "C" fn game_demon_SpecialLwGround(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 36.0, /*Angle*/ 60, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 22, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_BODY);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 5.0, /*Angle*/ 70, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_BODY);
        ATTACK_IGNORE_THROW(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 50.0, /*Angle*/ 362, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 23, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ -7.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(11.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_PUNCH);
        fighter.clear_lua_stack();
        lua_args!(fighter, 10, 1);
        sv_animcmd::FT_CATCH_STOP(fighter.lua_state_agent);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 15, 3);
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.3);
        FighterCutInManager::set_throw_finish_offset(FighterCutInManager(), Vector3f{x:0.0,y:-2.0,z:0.0});
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_HIT);
        CAM_ZOOM_OUT(fighter);
    }
    frame(fighter.lua_state_agent, 35.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 38.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 39.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 40.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 41.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 42.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}

//SpecialLwFall
unsafe extern "C" fn game_demon_SpecialLwFall(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 5.0, /*Angle*/ 70, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_BODY);
        ATTACK_IGNORE_THROW(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 50.0, /*Angle*/ 270, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(4.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_PUNCH);
    }
}

/*//AttackRageDrive
unsafe extern "C" fn game_demon_AttackRageDrive(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ATTACK_RAGE_CAPTURE) == false {
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
        FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
        if is_excute(fighter) {
            ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 5.0, /*Angle*/ 70, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_BODY);
        }
        frame(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        frame(fighter.lua_state_agent, 10.0);
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
        }
        frame(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 5.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.5, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 5.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.5, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 5.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 12.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 5.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 12.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(12.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 21.0, /*Z*/ 8.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(12.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 22.0, /*Z*/ 7.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(10.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 24.0, /*Z*/ 6.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(10.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 16.0);
        if is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
        }
        frame(fighter.lua_state_agent, 38.0);
        FT_MOTION_RATE(fighter, /*FSM*/ 1.3);
        frame(fighter.lua_state_agent, 52.0);
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
    }
    else {
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
        FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
        if is_excute(fighter) {
            ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 5.0, /*Angle*/ 70, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_BODY);
        }
        frame(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        frame(fighter.lua_state_agent, 10.0);
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
        }
        frame(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 5.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.5, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 5.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.5, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 5.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 12.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 5.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 12.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(12.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 21.0, /*Z*/ 8.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(12.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 22.0, /*Z*/ 7.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(10.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 24.0, /*Z*/ 6.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(10.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 16.0);
        if is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
        }
        frame(fighter.lua_state_agent, 38.0);
        FT_MOTION_RATE(fighter, /*FSM*/ 1.3);
        frame(fighter.lua_state_agent, 52.0);
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
    }
}

//AttackAirRageDrive
unsafe extern "C" fn game_demon_AttackAirRageDrive(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ATTACK_RAGE_CAPTURE) == false {
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
        FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
        if is_excute(fighter) {
            ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 5.0, /*Angle*/ 70, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_BODY);
        }
        frame(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        frame(fighter.lua_state_agent, 10.0);
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
        }
        frame(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 5.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.5, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 5.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.5, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 5.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 12.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 5.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 12.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(12.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 21.0, /*Z*/ 8.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(12.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 22.0, /*Z*/ 7.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(10.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 24.0, /*Z*/ 6.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(10.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 16.0);
        if is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
        }
        frame(fighter.lua_state_agent, 38.0);
        FT_MOTION_RATE(fighter, /*FSM*/ 1.3);
        frame(fighter.lua_state_agent, 52.0);
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
    }
    else {
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
        FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
        if is_excute(fighter) {
            ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 5.0, /*Angle*/ 70, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_BODY);
        }
        frame(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        frame(fighter.lua_state_agent, 10.0);
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
        }
        frame(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 5.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.5, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 5.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.5, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 5.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 12.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 5.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 12.0, None, None, None, /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(12.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 21.0, /*Z*/ 8.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(12.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 22.0, /*Z*/ 7.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(10.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
            CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(5.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            CATCH(fighter, /*ID*/ 3, /*Bone*/ Hash40::new("top"), /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 24.0, /*Z*/ 6.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(10.0), /*Status*/ *FIGHTER_STATUS_KIND_DEMON_DIVED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(fighter.module_accessor, 0, true);
            GrabModule::set_constraint(fighter.module_accessor, 1, true);
            GrabModule::set_constraint(fighter.module_accessor, 2, true);
            GrabModule::set_constraint(fighter.module_accessor, 3, true);
        }
        frame(fighter.lua_state_agent, 16.0);
        if is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
        }
        frame(fighter.lua_state_agent, 38.0);
        FT_MOTION_RATE(fighter, /*FSM*/ 1.3);
        frame(fighter.lua_state_agent, 52.0);
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
    }
}*/

//AttackRageDriveFall
unsafe extern "C" fn game_demon_AttackRageDriveFall(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ATTACK_RAGE_CAPTURE) == false {
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
        if is_excute(fighter) {
            WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
            ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 5.0, /*Angle*/ 70, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_BODY);
            ATTACK_IGNORE_THROW(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 300.0, /*Angle*/ 270, /*KBG*/ 3, /*FKB*/ 0, /*BKB*/ 3, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(-1.0), /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
    else {
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
        if is_excute(fighter) {
            WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
            ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 5.0, /*Angle*/ 70, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_BODY);
            ATTACK_IGNORE_THROW(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 150.0, /*Angle*/ 361, /*KBG*/ 11, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(-1.0), /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
}

/*//AttackRageDriveGround
unsafe extern "C" fn game_demon_AttackRageDriveGround(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ATTACK_RAGE_CAPTURE) == false {
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
        if is_excute(fighter) {
            WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
        }
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            WorkModule::set_int(fighter.module_accessor, *HIT_STATUS_NORMAL as i32, *FIGHTER_DEMON_STATUS_ATTACK_RAGE_DRIVE_INT_TARGET_HIT_STATUS);
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 30.0, /*Angle*/ 100, /*KBG*/ 100, /*FKB*/ 100, /*BKB*/ 50, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 5.0, /*X2*/ Some(10.0), /*Y2*/ Some(1.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_PUNCH);
            AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
            AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
            ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 70.0, /*Angle*/ 60, /*KBG*/ 38, /*FKB*/ 0, /*BKB*/ 5, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_NONE, true);
            AttackModule::set_no_finish_camera(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
            ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 5.0, /*Angle*/ 70, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK_IGNORE_THROW(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 150.0, /*Angle*/ 361, /*KBG*/ 11, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 12.5, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(14.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
        }
        else {
            FighterSpecializer_Demon::clear_log_past_throw(fighter.module_accessor);
            frame(fighter.lua_state_agent, 2.0);
            if is_excute(fighter) {
                WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_CHECK_DAMAGE);
                AttackModule::clear_all(fighter.module_accessor);
                WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_HIT);
                CAM_ZOOM_OUT(fighter, );
            }
            frame(fighter.lua_state_agent, 4.0);
            if is_excute(fighter) {
                WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
            }
            frame(fighter.lua_state_agent, 35.0);
            FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
            frame(fighter.lua_state_agent, 38.0);
            FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
            frame(fighter.lua_state_agent, 40.0);
            FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
            frame(fighter.lua_state_agent, 41.0);
            FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
            frame(fighter.lua_state_agent, 42.0);
            FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
        }
    }
    else {
        FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
        if is_excute(fighter) {
            WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
        }
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            WorkModule::set_int(fighter.module_accessor, *HIT_STATUS_NORMAL as i32, *FIGHTER_DEMON_STATUS_ATTACK_RAGE_DRIVE_INT_TARGET_HIT_STATUS);
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 30.0, /*Angle*/ 100, /*KBG*/ 100, /*FKB*/ 100, /*BKB*/ 50, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 5.0, /*X2*/ Some(10.0), /*Y2*/ Some(1.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_PUNCH);
            AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
            AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
            ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 70.0, /*Angle*/ 60, /*KBG*/ 38, /*FKB*/ 0, /*BKB*/ 5, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 0, *CAMERA_QUAKE_KIND_NONE, true);
            AttackModule::set_no_finish_camera(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
            ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 5.0, /*Angle*/ 70, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_PUNCH);
            ATTACK_IGNORE_THROW(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 150.0, /*Angle*/ 361, /*KBG*/ 11, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 12.5, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(14.0), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
        }
        else {
            FighterSpecializer_Demon::clear_log_past_throw(fighter.module_accessor);
            frame(fighter.lua_state_agent, 2.0);
            if is_excute(fighter) {
                WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_CHECK_DAMAGE);
                AttackModule::clear_all(fighter.module_accessor);
                WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_HIT);
                CAM_ZOOM_OUT(fighter);
            }
            frame(fighter.lua_state_agent, 4.0);
            if is_excute(fighter) {
                WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
            }
            frame(fighter.lua_state_agent, 35.0);
            FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
            frame(fighter.lua_state_agent, 38.0);
            FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
            frame(fighter.lua_state_agent, 40.0);
            FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
            frame(fighter.lua_state_agent, 41.0);
            FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
            frame(fighter.lua_state_agent, 42.0);
            FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
        }
    }
}*/

//AppealHiR
unsafe extern "C" fn game_demon_AppealHiR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 16.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_DEATHSCYTHE), 0, 0, false, false);
    }
}

//AppealHiL
unsafe extern "C" fn game_demon_AppealHiL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 16.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_DEATHSCYTHE), 0, 0, false, false);
    }
}

//AppealSR
unsafe extern "C" fn game_demon_AppealSR(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.444);
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 180, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 16.5, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 180, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 6.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 180, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 3.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 8.0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 180, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 8.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 180, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 20.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 20.0, false);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 47.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 2, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 180, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 10.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 6, /*Part*/ 2, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 180, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, 20.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 6, 20.0, false);
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 62.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 7, /*Part*/ 3, /*Bone*/ Hash40::new("top"), /*Damage*/ 666.0, /*Angle*/ 40, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 8.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_APPEAL, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 8, /*Part*/ 3, /*Bone*/ Hash40::new("top"), /*Damage*/ 666.0, /*Angle*/ 40, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_APPEAL, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 7, 20.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 8, 20.0, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 7, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 8, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(fighter.lua_state_agent, 63.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AppealSL
unsafe extern "C" fn game_demon_AppealSL(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.444);
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 180, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 16.5, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 180, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 6.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 180, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 3.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 8.0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 180, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 8.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 180, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 20.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 20.0, false);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 47.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 2, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 180, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 10.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 6, /*Part*/ 2, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 180, /*KBG*/ 5, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, 20.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 6, 20.0, false);
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 62.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 7, /*Part*/ 3, /*Bone*/ Hash40::new("top"), /*Damage*/ 666.0, /*Angle*/ 40, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 10.5, /*Z*/ 8.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_APPEAL, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 8, /*Part*/ 3, /*Bone*/ Hash40::new("top"), /*Damage*/ 666.0, /*Angle*/ 40, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DEMON_APPEAL, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 7, 20.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 8, 20.0, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 7, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(fighter.module_accessor, 8, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(fighter.lua_state_agent, 63.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AppealLwR
unsafe extern "C" fn game_demon_AppealLwR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 16.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        DamageModule::heal(fighter.module_accessor, -30.0, 0);
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BUMPER), 0, 0, false, false);
    }
}

//AppealLwL
unsafe extern "C" fn game_demon_AppealLwL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.8, 0, 10.0, 0.0, 0, 4.0, 16.0, 1.4, 1.6, 3000, false, 7.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        DamageModule::heal(fighter.module_accessor, -30.0, 0);
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BUMPER), 0, 0, false, false);
    }
}

pub fn install() {
    Agent::new("demon")
    .game_acmd("game_attack11", game_demon_Attack11, Low)
    .game_acmd("game_attack12", game_demon_Attack12, Low)
    .game_acmd("game_attack13", game_demon_Attack13, Low)
    .game_acmd("game_flashpunch", game_demon_FlashPunch, Low)
    .game_acmd("game_attack14", game_demon_Attack14, Low)
    .game_acmd("game_attack15", game_demon_Attack15, Low)
    .game_acmd("game_attack16", game_demon_Attack16, Low)
    .game_acmd("game_attack17", game_demon_Attack17, Low)
    .game_acmd("game_attack18", game_demon_Attack18, Low)
    .game_acmd("game_attack19", game_demon_Attack19, Low)
    .game_acmd("game_attack110", game_demon_Attack110, Low)
    .game_acmd("game_attack100", game_demon_Attack100, Low)
    .game_acmd("game_attack100sub", game_demon_Attack100Sub, Low)
    .game_acmd("game_attack100end", game_demon_Attack100End, Low)
    .game_acmd("game_attackdash", game_demon_AttackDash, Low)
    .game_acmd("game_attackstand1", game_demon_AttackStand1, Low)
    .game_acmd("game_attacks3hi", game_demon_AttackS3Hi, Low)
    .game_acmd("game_attacks3", game_demon_AttackS3, Low)
    .game_acmd("game_attacks3lw", game_demon_AttackS3Lw, Low)
    .game_acmd("game_attackstand21", game_demon_AttackStand21, Low)
    .game_acmd("game_attackstand22", game_demon_AttackStand22, Low)
    .game_acmd("game_attackstand23", game_demon_AttackStand23, Low)
    .game_acmd("game_attackstand24", game_demon_AttackStand24, Low)
    .game_acmd("game_attackstand31", game_demon_AttackStand31, Low)
    .game_acmd("game_attackstand32", game_demon_AttackStand32, Low)
    .game_acmd("game_attackstand5", game_demon_AttackStand5, Low)
    .game_acmd("game_attackstand4", game_demon_AttackStand4, Low)
    .game_acmd("game_attackstand6", game_demon_AttackStand6, Low)
    .game_acmd("game_attackhi3", game_demon_AttackHi3, Low)
    .game_acmd("game_attackhi32", game_demon_AttackHi32, Low)
    .game_acmd("game_attacklw3", game_demon_AttackLw3, Low)
    .game_acmd("game_attacksquat1", game_demon_AttackSquat1, Low)
    .game_acmd("game_attacksquat2", game_demon_AttackSquat2, Low)
    .game_acmd("game_attacksquat3", game_demon_AttackSquat3, Low)
    .game_acmd("game_attacksquat4", game_demon_AttackSquat4, Low)
    .game_acmd("game_attacks4transform", game_demon_AttackS4Transform, Low)
    .game_acmd("game_attacks4", game_demon_AttackS4, Low)
    .game_acmd("game_attackhi4transform", game_demon_AttackHi4Transform, Low)
    .game_acmd("game_attackhi4", game_demon_AttackHi4, Low)
    .game_acmd("game_attacklw4transform", game_demon_AttackLw4Transform, Low)
    .game_acmd("game_attacklw4", game_demon_AttackLw4, Low)
    .game_acmd("game_attackstep2s", game_demon_AttackStep2s, Low)
    .game_acmd("game_attackstep2", game_demon_AttackStep2, Low)
    .game_acmd("game_attackstep2f", game_demon_AttackStep2f, Low)
    .game_acmd("game_attackstep2l", game_demon_AttackStep2l, Low)
    .game_acmd("game_attackairn", game_demon_AttackAirN, Low)
    .game_acmd("game_attackairf", game_demon_AttackAirF, Low)    
    .game_acmd("game_attackairb", game_demon_AttackAirB, Low)
    .game_acmd("game_attackairhi", game_demon_AttackAirHi, Low)
    .game_acmd("game_attackairlw", game_demon_AttackAirLw, Low)
    .game_acmd("game_landingairlw", game_demon_LandingAirLw, Low)
    .game_acmd("game_catch", game_demon_Catch, Low)
    .game_acmd("game_catchdash", game_demon_CatchDash, Low)
    .game_acmd("game_catchturn", game_demon_CatchTurn, Low)
    .game_acmd("game_catchcommand", game_demon_CatchCommand, Low)
    .game_acmd("game_catchattack", game_demon_CatchAttack, Low)
    .game_acmd("game_throwf", game_demon_ThrowF, Low)
    .game_acmd("game_throwb", game_demon_ThrowB, Low)
    .game_acmd("game_throwhi", game_demon_ThrowHi, Low)
    .game_acmd("game_throwlw", game_demon_ThrowLw, Low)
    .game_acmd("game_throwcommand", game_demon_ThrowCommand, Low)
    .game_acmd("game_downattackd", game_demon_DownAttackD, Low)
    .game_acmd("game_downattacku", game_demon_DownAttackU, Low)
    .game_acmd("game_cliffattack", game_demon_CliffAttack, Low)
    .game_acmd("game_slipattack", game_demon_SlipAttack, Low)
    .game_acmd("game_specialnstart", game_demon_SpecialNStart, Low)
    .game_acmd("game_specialairnstart", game_demon_SpecialAirNStart, Low)
    .game_acmd("game_specialnhi", game_demon_SpecialNHi, Low)
    .game_acmd("game_specialn", game_demon_SpecialN, Low)
    .game_acmd("game_specialnlw", game_demon_SpecialNLw, Low)
    .game_acmd("game_specialairnhi", game_demon_SpecialAirNHi, Low)
    .game_acmd("game_specialairn", game_demon_SpecialAirN, Low)
    .game_acmd("game_specialairnlw", game_demon_SpecialAirNLw, Low)
    .game_acmd("game_specials", game_demon_SpecialS, Low)
    .game_acmd("game_specialshit", game_demon_SpecialSHit, Low)
    .game_acmd("game_specialairs", game_demon_SpecialAirS, Low)
    .game_acmd("game_specialairshit", game_demon_SpecialAirSHit, Low)
    .game_acmd("game_specialhi", game_demon_SpecialHi, Low)
    .game_acmd("game_specialhiground", game_demon_SpecialHiGround, Low)
    .game_acmd("game_specialhiair", game_demon_SpecialHiAir, Low)
    .game_acmd("game_speciallw", game_demon_SpecialLw, Low)
    .game_acmd("game_specialairlw", game_demon_SpecialAirLw, Low)
    .game_acmd("game_speciallwground", game_demon_SpecialLwGround, Low)
    .game_acmd("game_speciallwfall", game_demon_SpecialLwFall, Low)
    //.game_acmd("game_attackragedrive", game_demon_AttackRageDrive, Low)
    //.game_acmd("game_attackairragedrive", game_demon_AttackAirRageDrive, Low)
    //.game_acmd("game_attackragedriveground", game_demon_AttackRageDriveGround, Low)
    .game_acmd("game_attackragedrivefall", game_demon_AttackRageDriveFall, Low)
    .game_acmd("game_appealsr", game_demon_AppealSR, Low)
    .game_acmd("game_appealsl", game_demon_AppealSL, Low)
    .game_acmd("game_appealhir", game_demon_AppealHiR, Low)
    .game_acmd("game_appealhil", game_demon_AppealHiL, Low)
    .game_acmd("game_appeallwr", game_demon_AppealLwR, Low)
    .game_acmd("game_appeallwl", game_demon_AppealLwL, Low)
    .install();
}