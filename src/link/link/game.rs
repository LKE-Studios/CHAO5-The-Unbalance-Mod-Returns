use crate::imports::BuildImports::*;

//AscendJump
unsafe extern "C" fn game_link_AscendJump(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, false, ArticleOperationTarget(0));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("ascend_jump"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        SA_SET(fighter, *SITUATION_KIND_AIR);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, ArticleOperationTarget(0));
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 60, 100, 0, 50, 6.0, 0.0, 4.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 60, 100, 0, 50, 6.0, 0.0, 4.0, 5.0, None, None, None, 1.0, 0.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AscendAirJump
unsafe extern "C" fn game_link_AscendAirJump(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, false, ArticleOperationTarget(0));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("ascend_jump"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        SA_SET(fighter, *SITUATION_KIND_AIR);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, ArticleOperationTarget(0));
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 60, 100, 0, 50, 6.0, 0.0, 4.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 60, 100, 0, 50, 6.0, 0.0, 4.0, 5.0, None, None, None, 1.0, 0.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AscendStart
unsafe extern "C" fn game_link_AscendStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {       
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
}

//RevaliGlideDrop
unsafe extern "C" fn game_link_RevaliGlideDrop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//RevaliGlideLanding
unsafe extern "C" fn game_link_RevaliGlideLanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Attack11 
unsafe extern "C" fn game_link_Attack11(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    }
    frame(fighter.lua_state_agent, 7.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 25, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 25, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 14.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 180, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 18.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 22.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
}

//Attack12 
unsafe extern "C" fn game_link_Attack12(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 4.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 25, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 9.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(8.5), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 25, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 14.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(8.5), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 180, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 18.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(8.5), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 23.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(8.5), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
}

//Attack13 
unsafe extern "C" fn game_link_Attack13(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    }
    frame(fighter.lua_state_agent, 5.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 30, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 30, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 15.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword1"), /*Damage*/ 7.5, /*Angle*/ 30, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 6.8, /*X*/ 11.0, /*Y*/ 0.0, /*Z*/ -1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackDash
unsafe extern "C" fn game_link_AttackDash(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 0.58);
    }
    frame(fighter.lua_state_agent, 17.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 16.0, /*Angle*/ 45, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 145, /*Size*/ 7.2, /*X*/ 8.5, /*Y*/ 0.0, /*Z*/ -2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 16.0, /*Angle*/ 50, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 145, /*Size*/ 7.5, /*X*/ 3.0, /*Y*/ 0.0, /*Z*/ -2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 16.0, /*Angle*/ 45, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 145, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, /*ID1*/ 0, /*ID2*/ 1, /*ID3*/ 2, /*ShieldstunMul*/ 1.4);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackS3
unsafe extern "C" fn game_link_AttackS3(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 0.4);
    }
    frame(fighter.lua_state_agent, 13.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 14.0, /*Angle*/ 30, /*KBG*/ 82, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 6.0, /*X*/ 7.8, /*Y*/ 0.0, /*Z*/ -2.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 14.0, /*Angle*/ 30, /*KBG*/ 82, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.9, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("colonells"), /*Damage*/ 14.0, /*Angle*/ 30, /*KBG*/ 82, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.1, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 82, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 7.5, /*X*/ 7.8, /*Y*/ 0.0, /*Z*/ -2.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackHi3
unsafe extern "C" fn game_link_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 13.0, /*Angle*/ 95, /*KBG*/ 108, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.4, /*X*/ 8.4, /*Y*/ 0.0, /*Z*/ -2.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 13.0, /*Angle*/ 85, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.3, /*X*/ 2.0, /*Y*/ 0.0, /*Z*/ -3.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 13.0, /*Angle*/ 85, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.7, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("colonells"), /*Damage*/ 13.0, /*Angle*/ 85, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.16, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackLw3
unsafe extern "C" fn game_link_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.41);
    frame(fighter.lua_state_agent, 13.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 80, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 1.3, /*Z*/ 25.5, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackS4
unsafe extern "C" fn game_link_AttackS4(fighter: &mut L2CAgentBase) {
    let damage = DamageModule::damage(fighter.module_accessor, 0);
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.75);
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 10.0, /*Angle*/ 69, /*KBG*/ 12, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 6.3, /*X*/ 2.7, /*Y*/ -2.0, /*Z*/ -2.3, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.5, /*X*/ 8.5, /*Y*/ -2.0, /*Z*/ -2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 60, /*KBG*/ 12, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 6.2, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        if damage == 0.0 || damage >= 100.0 {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_SWORD_BEAM, false, -1);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

//AttackS4S2
unsafe extern "C" fn game_link_AttackS4S2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 19.0, /*Angle*/ 48, /*KBG*/ 89, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 6.5, /*X*/ 8.0, /*Y*/ 2.3, /*Z*/ -2.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_flower"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 19.0, /*Angle*/ 48, /*KBG*/ 89, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 6.5, /*X*/ 1.5, /*Y*/ 2.3, /*Z*/ -2.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_flower"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 18.0, /*Angle*/ 48, /*KBG*/ 89, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 5.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_flower"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 48, /*KBG*/ 89, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_flower"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackHi4
unsafe extern "C" fn game_link_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("colonells"), /*Damage*/ 6.0, /*Angle*/ 105, /*KBG*/ 100, /*FKB*/ 55, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 6.0, /*Angle*/ 115, /*KBG*/ 100, /*FKB*/ 38, /*BKB*/ 0, /*Size*/ 6.8, /*X*/ 2.5, /*Y*/ 0.0, /*Z*/ -3.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 6.0, /*Angle*/ 107, /*KBG*/ 100, /*FKB*/ 28, /*BKB*/ 0, /*Size*/ 6.8, /*X*/ 2.5, /*Y*/ 0.0, /*Z*/ -3.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 6.0, /*Angle*/ 115, /*KBG*/ 100, /*FKB*/ 28, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 8.5, /*Y*/ 0.0, /*Z*/ -2.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 6.0, /*Angle*/ 107, /*KBG*/ 100, /*FKB*/ 28, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 8.5, /*Y*/ 0.0, /*Z*/ -2.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 3.0, false);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 4.0, /*Angle*/ 135, /*KBG*/ 100, /*FKB*/ 45, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 8.5, /*Y*/ 0.0, /*Z*/ -2.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("colonells"), /*Damage*/ 4.0, /*Angle*/ 105, /*KBG*/ 100, /*FKB*/ 55, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 4.0, /*Angle*/ 115, /*KBG*/ 100, /*FKB*/ 38, /*BKB*/ 0, /*Size*/ 6.8, /*X*/ 2.5, /*Y*/ 0.0, /*Z*/ -3.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 4.0, /*Angle*/ 107, /*KBG*/ 100, /*FKB*/ 28, /*BKB*/ 0, /*Size*/ 6.8, /*X*/ 2.5, /*Y*/ 0.0, /*Z*/ -3.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 4.0, /*Angle*/ 115, /*KBG*/ 100, /*FKB*/ 28, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 8.5, /*Y*/ 0.0, /*Z*/ -2.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 4.0, /*Angle*/ 107, /*KBG*/ 100, /*FKB*/ 28, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 8.5, /*Y*/ 0.0, /*Z*/ -2.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 3.0, false);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 3.0, /*Angle*/ 135, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 8.5, /*Y*/ 0.0, /*Z*/ -2.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 40.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 14.0, /*Angle*/ 80, /*KBG*/ 96, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 7.3, /*X*/ 7.0, /*Y*/ 0.0, /*Z*/ -3.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 14.0, /*Angle*/ 90, /*KBG*/ 96, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 7.3, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ -3.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 14.0, /*Angle*/ 90, /*KBG*/ 96, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.0, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 90, /*KBG*/ 96, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 7.2, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ -0.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackLw4
unsafe extern "C" fn game_link_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.75);
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 19.0, /*Angle*/ 78, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.5, /*X*/ 1.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.3, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 19.0, /*Angle*/ 78, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.6, /*X*/ 8.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.3, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 19.0, /*Angle*/ 78, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.4, /*X*/ 1.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.3, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("colonells"), /*Damage*/ 19.0, /*Angle*/ 78, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.3, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 21.0, /*Angle*/ 30, /*KBG*/ 63, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 6.5, /*X*/ 1.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.3, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 21.0, /*Angle*/ 30, /*KBG*/ 63, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 6.6, /*X*/ 8.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.3, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 21.0, /*Angle*/ 30, /*KBG*/ 63, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 5.4, /*X*/ 1.0, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.3, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("colonells"), /*Damage*/ 21.0, /*Angle*/ 30, /*KBG*/ 63, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.3, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackAirN
unsafe extern "C" fn game_link_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.712);
    HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_INVINCIBLE);
    HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_INVINCIBLE);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 26, /*Size*/ 7.0, /*X*/ 4.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 26, /*Size*/ 7.0, /*X*/ -2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 26, /*Size*/ 5.0, /*X*/ 1.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 130, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 7.5, /*X*/ 4.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 130, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 7.5, /*X*/ -2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 130, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 1.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirF 
unsafe extern "C" fn game_link_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 43, /*KBG*/ 28, /*FKB*/ 0, /*BKB*/ 31, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 11.5, /*Z*/ 18.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 367, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 11.5, /*Z*/ 18.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 47, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 33, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 11.5, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 367, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 11.5, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 47, /*KBG*/ 28, /*FKB*/ 0, /*BKB*/ 33, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 11.5, /*Z*/ 6.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 38, /*KBG*/ 26, /*FKB*/ 0, /*BKB*/ 26, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 11.5, /*Z*/ 6.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 44, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 17.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 44, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 44, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 8.8, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 6.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 51.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirB
unsafe extern "C" fn game_link_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.384);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 5.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.9);
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 12.0, /*Angle*/ 73, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 5.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 12.0, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 36, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 5.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 12.0, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 7.5, /*X*/ -0.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 12.0, /*Angle*/ 66, /*KBG*/ 100, /*FKB*/ 36, /*BKB*/ 0, /*Size*/ 7.5, /*X*/ -0.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 12.0, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 12.0, /*Angle*/ 66, /*KBG*/ 100, /*FKB*/ 36, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 20.0, /*Angle*/ 361, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 7.5, /*X*/ 5.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 20.0, /*Angle*/ 361, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 7.0, /*X*/ -0.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 20.0, /*Angle*/ 361, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}        


//AttackAirHi
unsafe extern "C" fn game_link_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 10.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 21.0, /*Angle*/ 85, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 9.5, /*X*/ 6.9, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-1.5), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 20.0, /*Angle*/ 85, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 11.5, /*X*/ 6.9, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-1.5), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATK_POWER(fighter, 0, 20);
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirLw
unsafe extern "C" fn game_link_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 3.0, 3.0, 7.0, 3.0);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 270, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 9.0, /*X*/ 1.5, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 270, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 9.0, /*X*/ 1.5, /*Y*/ 1.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 22.0, /*Angle*/ 270, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 9.2, /*X*/ 1.0, /*Y*/ 1.5, /*Z*/ 0.0, /*X2*/ Some(1.0), /*Y2*/ Some(4.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 1, false);
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirLw2Bound
unsafe extern "C" fn game_link_AttackAirLw2Bound(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 2.2, z: 0.0} as *const Vector3f);
    }
}   

//AttackAirLw2Attack
unsafe extern "C" fn game_link_AttackAirLw2Attack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 270, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 11.3, /*X*/ 1.0, /*Y*/ 1.0, /*Z*/ 0.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
}
        
//Catch
unsafe extern "C" fn game_link_Catch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 5.3, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(7.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 1.65, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 2.35, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(9.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        game_CaptureCutCommon(fighter);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//CatchDash
unsafe extern "C" fn game_link_CatchDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 5.6, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.0), /*Z2*/ Some(9.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 1.3, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 2.7, /*X2*/ Some(0.0), /*Y2*/ Some(7.0), /*Z2*/ Some(10.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        game_CaptureCutCommon(fighter);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//CatchTurn
unsafe extern "C" fn game_link_CatchTurn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 5.3, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ -4.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.0), /*Z2*/ Some(-13.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 1.65, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ -2.35, /*X2*/ Some(0.0), /*Y2*/ Some(7.0), /*Z2*/ Some(-14.85), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        game_CaptureCutCommon(fighter);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//CatchAttack
unsafe extern "C" fn game_link_CatchAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.3, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_flower"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//ThrowF
unsafe extern "C" fn game_link_ThrowF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 2.5, /*Angle*/ 33, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 75, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.8, /*X*/ 3.5, /*Y*/ -0.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        CHECK_FINISH_CAMERA(fighter, 12, 4);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}    

//ThrowB
unsafe extern "C" fn game_link_ThrowB(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 2.5, /*Angle*/ 130, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 67, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 7.5, /*X*/ 3.5, /*Y*/ -0.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        CHECK_FINISH_CAMERA(fighter, -18, 12);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(fighter.module_accessor);
    }
}  

//ThrowHi
unsafe extern "C" fn game_link_ThrowHi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 2.0, /*Angle*/ 90, /*KBG*/ 230, /*FKB*/ 0, /*BKB*/ 24, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 11.0, /*Angle*/ 20, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.8, /*X*/ 6.8, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 11.0, /*Angle*/ 20, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.8, /*X*/ 2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        CHECK_FINISH_CAMERA(fighter, 6, 28);
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.8);
        FighterCutInManager::set_throw_finish_offset(FighterCutInManager(), Vector3f{x: 0.0, y: 12.0, z: 0.0});
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}   

//ThrowLw
unsafe extern "C" fn game_link_ThrowLw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FT_LEAVE_NEAR_OTTOTTO(fighter, -2.5, 2.5);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 83, /*KBG*/ 125, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.5, /*Angle*/ 361, /*KBG*/ 150, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.3, /*X*/ 0.0, /*Y*/ 2.4, /*Z*/ -8.4, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ice"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_ELBOW);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        CHECK_FINISH_CAMERA(fighter, -2, 0);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(fighter.module_accessor);
    }
}  

//CliffAttack
unsafe extern "C" fn game_link_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 45, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 15.5, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-4.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SlipAttack
unsafe extern "C" fn game_link_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 24.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(7.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bind"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -19.5, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bind"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//DownAttackD
unsafe extern "C" fn game_link_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 48, /*KBG*/ 48, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -15.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-4.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sleep"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 48, /*KBG*/ 48, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 17.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(4.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sleep"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//DownAttackU
unsafe extern "C" fn game_link_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 48, /*KBG*/ 48, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -15.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-4.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sleep"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 48, /*KBG*/ 48, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 17.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(4.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sleep"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialNStart
unsafe extern "C" fn game_link_SpecialNStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 0.1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, false, 0);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
    }
}

//SpecialAirNStart
unsafe extern "C" fn game_link_SpecialAirNStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 0.1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, false, 0);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
    }
}  

//SpecialHi
unsafe extern "C" fn game_link_SpecialHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 20.0, /*Angle*/ 45, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.0, /*X*/ -2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 20.0, /*Angle*/ 45, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 7.5, /*X*/ 2.5, /*Y*/ 0.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 20.0, /*Angle*/ 45, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 6.8, /*X*/ 8.0, /*Y*/ 0.0, /*Z*/ 1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 18.0, /*Angle*/ 43, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 8.0, /*X*/ -2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 18.0, /*Angle*/ 43, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 7.5, /*X*/ 2.5, /*Y*/ 0.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 18.0, /*Angle*/ 43, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 6.8, /*X*/ 8.0, /*Y*/ 0.0, /*Z*/ 1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 14.0, /*Angle*/ 43, /*KBG*/ 84, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 8.0, /*X*/ -2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 14.0, /*Angle*/ 43, /*KBG*/ 84, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 7.5, /*X*/ 2.5, /*Y*/ 0.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 14.0, /*Angle*/ 43, /*KBG*/ 84, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 6.8, /*X*/ 8.0, /*Y*/ 0.0, /*Z*/ 1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 82, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.0, /*X*/ -2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 82, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.5, /*X*/ 2.5, /*Y*/ 0.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 82, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.8, /*X*/ 8.0, /*Y*/ 0.0, /*Z*/ 1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X);
    }
}

//SpecialAirHi
unsafe extern "C" fn game_link_SpecialAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 46, /*KBG*/ 145, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 14.5, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(8.0), /*Hitlag*/ 1.0, /*SDI*/ 0.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 145, /*KBG*/ 200, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 11.5, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(-9.0), /*Hitlag*/ 1.0, /*SDI*/ 0.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 10.0, y: 20.0} as *const Vector2f, 10 as u32, false);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 107, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(8.0), /*Hitlag*/ 1.0, /*SDI*/ 0.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 145, /*KBG*/ 100, /*FKB*/ 95, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 11.5, /*Z*/ -12.5, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(-9.0), /*Hitlag*/ 1.0, /*SDI*/ 0.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 10.0, y: 20.0} as *const Vector2f, 10 as u32, false);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 83, /*KBG*/ 200, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.5), /*Z2*/ Some(8.0), /*Hitlag*/ 1.0, /*SDI*/ 0.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 11.0, /*Angle*/ 150, /*KBG*/ 200, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 11.0, /*X*/ 2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(5.5), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 0.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 10.0, y: 20.0} as *const Vector2f, 10 as u32, false);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 69, /*KBG*/ 200, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 12.5, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(8.0), /*Hitlag*/ 1.0, /*SDI*/ 0.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("sword2"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 200, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 11.0, /*X*/ 2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(5.5), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 0.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 10.0, y: 20.0} as *const Vector2f, 10 as u32, false);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 47.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 200, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 11.5, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 14.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.5), /*Z2*/ Some(7.5), /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_flower"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 200, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 11.5, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 15.5, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(7.5), /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_flower"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X);
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 200, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 11.5, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 12.5, /*X2*/ Some(0.0), /*Y2*/ Some(14.0), /*Z2*/ Some(7.5), /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_flower"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        JostleModule::set_status(fighter.module_accessor, true);
    }
}

//AppealHiR
unsafe extern "C" fn game_link_AppealHiR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_MAGICPOT), 0, 0, false, false);
    }
}

//AppealHiL
unsafe extern "C" fn game_link_AppealHiL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_MAGICPOT), 0, 0, false, false);
    }
}

//AppealSR
unsafe extern "C" fn game_link_AppealSR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_FAIRYBOTTLE), 0, 0, false, false);
    }
}

//AppealSL
unsafe extern "C" fn game_link_AppealSL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_FAIRYBOTTLE), 0, 0, false, false);
    }
}

//AppealLwR
unsafe extern "C" fn game_link_AppealLwR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_CHICKEN), 0, 0, false, false);
    }
}

//AppealLwL
unsafe extern "C" fn game_link_AppealLwL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_CHICKEN), 0, 0, false, false);
    }
}        

pub fn install() {
    Agent::new("link")
    .game_acmd("game_ascendjump", game_link_AscendJump)
    .game_acmd("game_ascendairjump", game_link_AscendAirJump)
    .game_acmd("game_ascendstart", game_link_AscendStart)
    .game_acmd("game_revaliglidedrop", game_link_RevaliGlideDrop)
    .game_acmd("game_revaliglidelanding", game_link_RevaliGlideLanding)
    .game_acmd("game_attack11", game_link_Attack11)
    .game_acmd("game_attack12", game_link_Attack12)
    .game_acmd("game_attack13", game_link_Attack13)
    .game_acmd("game_attackdash", game_link_AttackDash)
    .game_acmd("game_attacks3", game_link_AttackS3)
    .game_acmd("game_attackhi3", game_link_AttackHi3)
    .game_acmd("game_attacklw3", game_link_AttackLw3)
    .game_acmd("game_attacks4", game_link_AttackS4)
    .game_acmd("game_attacks4s2", game_link_AttackS4S2)
    .game_acmd("game_attackhi4", game_link_AttackHi4)
    .game_acmd("game_attacklw4", game_link_AttackLw4)
    .game_acmd("game_attackairn", game_link_AttackAirN)
    .game_acmd("game_attackairf", game_link_AttackAirF)    
    .game_acmd("game_attackairb", game_link_AttackAirB)
    .game_acmd("game_attackairhi", game_link_AttackAirHi)
    .game_acmd("game_attackairlw", game_link_AttackAirLw)
    .game_acmd("game_attackairlw2attack", game_link_AttackAirLw2Attack)
    .game_acmd("game_attackairlw2bound", game_link_AttackAirLw2Bound)
    .game_acmd("game_catch", game_link_Catch)
    .game_acmd("game_catchdash", game_link_CatchDash)
    .game_acmd("game_catchturn", game_link_CatchTurn)
    .game_acmd("game_catchattack", game_link_CatchAttack)
    .game_acmd("game_throwf", game_link_ThrowF)
    .game_acmd("game_throwb", game_link_ThrowB)
    .game_acmd("game_throwhi", game_link_ThrowHi)
    .game_acmd("game_throwlw", game_link_ThrowLw)
    .game_acmd("game_downattackd", game_link_DownAttackD)
    .game_acmd("game_downattacku", game_link_DownAttackU)
    .game_acmd("game_cliffattack", game_link_CliffAttack)
    .game_acmd("game_slipattack", game_link_SlipAttack)    
    .game_acmd("game_specialnstart", game_link_SpecialNStart)
    .game_acmd("game_specialairnstart", game_link_SpecialAirNStart)
    .game_acmd("game_specialhi", game_link_SpecialHi)
    .game_acmd("game_specialairhi", game_link_SpecialAirHi)
    .game_acmd("game_appealsr", game_link_AppealSR)
    .game_acmd("game_appealsl", game_link_AppealSL)
    .game_acmd("game_appealhir", game_link_AppealHiR)
    .game_acmd("game_appealhil", game_link_AppealHiL)
    .game_acmd("game_appeallwr", game_link_AppealLwR)
    .game_acmd("game_appeallwl", game_link_AppealLwL)
    .install();
}
