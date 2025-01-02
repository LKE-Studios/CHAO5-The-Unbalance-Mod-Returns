use crate::imports::BuildImports::*;
use crate::kamek::kamek::frame::*;

//JumpAerialFront
unsafe extern "C" fn game_kamek_JumpAerialFront(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 250, 0, 40, 8.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//JumpAerialBack
unsafe extern "C" fn game_kamek_JumpAerialBack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 250, 0, 40, 8.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Attack11 
unsafe extern "C" fn game_kamek_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 45, 134, 0, 27, 6.5, 0.0, 7.0, 11.0, Some(0.0), Some(9.0), Some(9.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.5, 45, 134, 0, 27, 7.0, 0.0, 7.0, 5.0, Some(0.0), Some(9.0), Some(9.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_MIDDLE), false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackDash
unsafe extern "C" fn game_kamek_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        VisibilityModule::set_whole(fighter.module_accessor, false);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        VisibilityModule::set_whole(fighter.module_accessor, true);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 15.0, 55, 99, 0, 80, 8.0, 0.5, 0.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 15.0, 55, 99, 0, 80, 9.0, 0.5, 0.0, 5.2, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackS3
unsafe extern "C" fn game_kamek_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 30, 70, 0, 70, 9.0, 0.0, 5.0, 18.5, Some(0.0), Some(6.0), Some(8.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 30, 70, 0, 73, 7.5, 0.0, 5.0, 9.0, Some(0.0), Some(6.0), Some(8.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackHi3
unsafe extern "C" fn game_kamek_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 15.0, 100, 90, 0, 62, 10.0, 2.5, 0.0, 0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 15.0, 100, 90, 0, 62, 10.0, 2.5, 0.0, 7.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackLw3
unsafe extern "C" fn game_kamek_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5, 36, 100, 0, 50, 5.7, 0.0, 3.5, 13.0, Some(0.0), Some(4.1), Some(11.2), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, FIGHTER_KAMEK_GENERATE_ARTICLE_FIRE, false, -1);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackS4
unsafe extern "C" fn game_kamek_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kamek_broom_a"), 24.0, 40, 91, 0, 30, 9.0, 2.0, 1.0, -8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_THROW);
        ATTACK(fighter, 1, 0, Hash40::new("kamek_broom_a"), 24.0, 40, 91, 0, 30, 8.0, 2.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_THROW);
        ATTACK(fighter, 2, 0, Hash40::new("kamek_broom_a"), 22.0, 45, 93, 0, 30, 6.0, 2.0, 0.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
        ATTACK(fighter, 3, 0, Hash40::new("kamek_broom_a"), 22.0, 45, 93, 0, 30, 6.0, 2.0, 0.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackHi4
unsafe extern "C" fn game_kamek_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 19.0);
    for _ in 0..3 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("kamek_box"), 2.5, 367, 100, 50, 0, 15.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
        }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 3.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kamek_box"), 20.0, 70, 83, 0, 50, 12.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackLw4
unsafe extern "C" fn game_kamek_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderl"), /*Damage*/ 21.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 21.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.0, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderr"), /*Damage*/ 21.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_PUNCH);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 21.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.0, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackAirN
unsafe extern "C" fn game_kamek_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 361, 80, 0, 45, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 361, 80, 0, 45, 6.0, 0.0, 0.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 15.0, 361, 80, 0, 45, 6.0, 0.0, 0.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirF 
unsafe extern "C" fn game_kamek_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE, false, -1);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirB
unsafe extern "C" fn game_kamek_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 361, 106, 0, 36, 4.2, 0.0, 8.0, -7.0, Some(0.0), Some(8.0), Some(-4.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 17.0, 361, 106, 0, 36, 4.0, 2.5, 0.0, 0.4, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 17.0, 361, 106, 0, 36, 4.0, 2.5, 0.0, 7.2, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirHi
unsafe extern "C" fn game_kamek_AttackAirHi(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, 0.7);
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 13.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 25.0, 90, 73, 0, 50, 12.5, 0.0, 25.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 21.0, 90, 70, 0, 50, 12.5, 0.0, 25.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirLw
unsafe extern "C" fn game_kamek_AttackAirLw(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, 0.6);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, FIGHTER_KAMEK_GENERATE_ARTICLE_BOOK, false, -1);
    }
}

//Catch
unsafe extern "C" fn game_kamek_Catch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 8.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 5.0, 0.0, 5.2, 2.0, Some(0.0), Some(5.2), Some(16.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//CatchDash
unsafe extern "C" fn game_kamek_CatchDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 8.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 5.6, 0.0, 5.2, 2.4, Some(0.0), Some(5.2), Some(17.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//CatchTurn
unsafe extern "C" fn game_kamek_CatchTurn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 8.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-10.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 6.0, 0.0, 5.2, -2.0, Some(0.0), Some(5.2), Some(-22.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//CatchAttack
unsafe extern "C" fn game_kamek_CatchAttack(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 3.5, 361, 100, 30, 0, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//ThrowF
unsafe extern "C" fn game_kamek_ThrowF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 15.0, 45, 76, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 9.0);
    FT_MOTION_RATE(fighter, 1.5);
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 18, 8);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(fighter.lua_state_agent, 19.0);
    FT_MOTION_RATE(fighter, 1.0);
}

//ThrowB
unsafe extern "C" fn game_kamek_ThrowB(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 16.0, 42, 68, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 2.0);
    FT_MOTION_RATE(fighter, 1.5);
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
    }
    frame(fighter.lua_state_agent, 9.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 17, 5);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(fighter.lua_state_agent, 20.0);
    FT_MOTION_RATE(fighter, 1.0);
}

//ThrowHi
unsafe extern "C" fn game_kamek_ThrowHi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 15.0, 80, 86, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 30, 3);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}      

//ThrowLw
unsafe extern "C" fn game_kamek_ThrowLw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 17.5, 68, 70, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 13, 3);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

//CliffAttack
unsafe extern "C" fn game_kamek_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 45, 50, 0, 90, 8.0, 0.0, 5.0, 7.5, Some(0.0), Some(5.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SlipAttack
unsafe extern "C" fn game_kamek_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -6.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(-2.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(3.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//DownAttackU
unsafe extern "C" fn game_kamek_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 48, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -13.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 48, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//DownAttackD
unsafe extern "C" fn game_kamek_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 48, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -13.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 48, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialNStart
unsafe extern "C" fn game_kamek_SpecialNStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
}

//SpecialAirNStart
unsafe extern "C" fn game_kamek_SpecialAirNStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
}

//SpecialNFire
unsafe extern "C" fn game_kamek_SpecialNFire(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        SET_SPEED_EX(fighter, -1.0, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ArticleModule::generate_article(fighter.module_accessor, FIGHTER_KAMEK_GENERATE_ARTICLE_PINKMAGIC, false, -1);
    }
}

//SpecialAirNFire
unsafe extern "C" fn game_kamek_SpecialAirNFire(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        SET_SPEED_EX(fighter, -1.0, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ArticleModule::generate_article(fighter.module_accessor, FIGHTER_KAMEK_GENERATE_ARTICLE_PINKMAGIC, false, -1);
    }
}

//SpecialS
unsafe extern "C" fn game_kamek_SpecialS(fighter: &mut L2CAgentBase) {
    let rand_num = sv_math::rand(hash40("ness"), 100);
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        if rand_num >= 0 && rand_num <= 30 { //Magic 30%
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 50.0, 361, 0, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("kamek_spike"), 50.0, 361, 0, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        }
        else if rand_num >= 31 && rand_num <= 45 { //Heal Self + Poison Enemy 15%
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 25.0, 88, 72, 0, 20, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
            ATTACK(fighter, 1, 0, Hash40::new("kamek_spike"), 25.0, 88, 72, 0, 20, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
            AttackModule::set_poison_param(fighter.module_accessor, 0, 900, 60, 9.5, false);
            AttackModule::set_poison_param(fighter.module_accessor, 1, 900, 60, 9.5, false);
            DamageModule::heal(fighter.module_accessor, -40.0, 0);
            WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
            PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
            EFFECT_FOLLOW(fighter, Hash40::new("sys_recovery"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        else if rand_num >= 46 && rand_num <= 55 { //Cape 10%
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 30.0, 361, 100, 80, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("kamek_spike"), 30.0, 361, 100, 80, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_OBJECT);
        }
        else if rand_num >= 56 && rand_num <= 65 { //Sleep 10%
            ATTACK(fighter, 1, 0, Hash40::new("kamek_book"), 0.0, 361, 100, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PSI);
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 40.0, 361, 100, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PSI);
            ATTACK(fighter, 3, 0, Hash40::new("kamek_spike"), 0.0, 361, 100, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PSI);
            ATTACK(fighter, 2, 0, Hash40::new("kamek_spike"), 40.0, 361, 100, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PSI);
        }
        else if rand_num >= 66 && rand_num <= 80 { //Ice 15%
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 35.0, 25, 65, 0, 50, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("kamek_spike"), 35.0, 25, 65, 0, 50, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_OBJECT);
        }
        else if rand_num >= 81 && rand_num <= 85 { //Death 5%
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 999.0, 45, 66, 0, 60, 22.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            ATTACK(fighter, 1, 0, Hash40::new("kamek_spike"), 999.0, 45, 66, 0, 60, 22.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        }
        else if rand_num >= 86 && rand_num <= 90 { //Grow Enemy 5%
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 5.0, 361, 0, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("kamek_spike"), 5.0, 361, 0, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            WorkModule::set_int(fighter.module_accessor, 6, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
        }
        else if rand_num >= 91 && rand_num <= 95 { //Shrink Enemy 5%
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 50.0, 361, 0, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("kamek_spike"), 50.0, 361, 0, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            WorkModule::set_int(fighter.module_accessor, 7, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE); 
        }
        else { //Slow Enemy
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 10.0, 361, 0, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("kamek_spike"), 10.0, 361, 0, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            WorkModule::set_int(fighter.module_accessor, 8, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
        }
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        if rand_num >= 0 && rand_num <= 30 { //Magic 30%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 50.0, 361, 0, 0, 0, 18.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        }
        else if rand_num >= 31 && rand_num <= 45 { //Heal Self + Poison Enemy 15%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 25.0, 88, 72, 0, 20, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
            AttackModule::set_poison_param(fighter.module_accessor, 0, 900, 60, 9.5, false);
            WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
        }
        else if rand_num >= 46 && rand_num <= 55 { //Cape Turn 10%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 30.0, 361, 100, 80, 0, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_OBJECT);
        }
        else if rand_num >= 56 && rand_num <= 65 { //Sleep 10%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 40.0, 361, 100, 0, 0, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PSI);
        }
        else if rand_num >= 66 && rand_num <= 80 { //Ice 15%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 35.0, 25, 65, 0, 50, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_OBJECT);
        }
        else if rand_num >= 81 && rand_num <= 85 { //Death 5%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 999.0, 45, 66, 0, 60, 26.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        }
        else if rand_num >= 86 && rand_num <= 90 { //Grow Enemy 5%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 5.0, 361, 0, 0, 0, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            WorkModule::set_int(fighter.module_accessor, 6, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
        }
        else if rand_num >= 91 && rand_num <= 95 { //Shrink Enemy 5%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 50.0, 361, 0, 0, 0, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            WorkModule::set_int(fighter.module_accessor, 7, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
        }
        else { //Slow Enemy
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 10.0, 361, 0, 0, 0, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            WorkModule::set_int(fighter.module_accessor, 8, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
        }
    }
    frame(fighter.lua_state_agent, 70.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialAirS
unsafe extern "C" fn game_kamek_SpecialAirS(fighter: &mut L2CAgentBase) {
    let rand_num = sv_math::rand(hash40("ness"), 100);
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        if rand_num >= 0 && rand_num <= 30 { //Magic 30%
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 50.0, 361, 0, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("kamek_spike"), 50.0, 361, 0, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        }
        else if rand_num >= 31 && rand_num <= 45 { //Heal Self + Poison Enemy 15%
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 25.0, 88, 72, 0, 20, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
            ATTACK(fighter, 1, 0, Hash40::new("kamek_spike"), 25.0, 88, 72, 0, 20, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
            AttackModule::set_poison_param(fighter.module_accessor, 0, 900, 60, 9.5, false);
            AttackModule::set_poison_param(fighter.module_accessor, 1, 900, 60, 9.5, false);
            DamageModule::heal(fighter.module_accessor, -40.0, 0);
            WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
            PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
            EFFECT_FOLLOW(fighter, Hash40::new("sys_recovery"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        else if rand_num >= 46 && rand_num <= 55 { //Cape 10%
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 30.0, 361, 100, 80, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("kamek_spike"), 30.0, 361, 100, 80, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_OBJECT);
        }
        else if rand_num >= 56 && rand_num <= 65 { //Sleep 10%
            ATTACK(fighter, 1, 0, Hash40::new("kamek_book"), 0.0, 361, 100, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PSI);
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 40.0, 361, 100, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PSI);
            ATTACK(fighter, 3, 0, Hash40::new("kamek_spike"), 0.0, 361, 100, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PSI);
            ATTACK(fighter, 2, 0, Hash40::new("kamek_spike"), 40.0, 361, 100, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PSI);
        }
        else if rand_num >= 66 && rand_num <= 80 { //Ice 15%
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 35.0, 25, 65, 0, 50, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("kamek_spike"), 35.0, 25, 65, 0, 50, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_OBJECT);
        }
        else if rand_num >= 81 && rand_num <= 85 { //Death 5%
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 999.0, 45, 66, 0, 60, 22.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            ATTACK(fighter, 1, 0, Hash40::new("kamek_spike"), 999.0, 45, 66, 0, 60, 22.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        }
        else if rand_num >= 86 && rand_num <= 90 { //Grow Enemy 5%
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 5.0, 361, 0, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("kamek_spike"), 5.0, 361, 0, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            WorkModule::set_int(fighter.module_accessor, 6, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
        }
        else if rand_num >= 91 && rand_num <= 95 { //Shrink Enemy 5%
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 50.0, 361, 0, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("kamek_spike"), 50.0, 361, 0, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            WorkModule::set_int(fighter.module_accessor, 7, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
        }
        else { //Slow Enemy 5%
            ATTACK(fighter, 0, 0, Hash40::new("kamek_book"), 10.0, 361, 0, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("kamek_spike"), 10.0, 361, 0, 0, 0, 22.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            WorkModule::set_int(fighter.module_accessor, 8, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
        }
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        if rand_num >= 0 && rand_num <= 30 { //Magic 30%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 50.0, 361, 0, 0, 0, 18.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        }
        else if rand_num >= 31 && rand_num <= 45 { //Heal Self + Poison Enemy 15%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 25.0, 88, 72, 0, 20, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
            AttackModule::set_poison_param(fighter.module_accessor, 0, 900, 60, 9.5, false);
            WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
        }
        else if rand_num >= 46 && rand_num <= 55 { //Cape Turn 10%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 30.0, 361, 100, 80, 0, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_OBJECT);
        }
        else if rand_num >= 56 && rand_num <= 65 { //Sleep 10%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 40.0, 361, 100, 0, 0, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PSI);
        }
        else if rand_num >= 66 && rand_num <= 80 { //Ice 15%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 35.0, 25, 65, 0, 50, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_OBJECT);
        }
        else if rand_num >= 81 && rand_num <= 85 { //Death 5%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 999.0, 45, 66, 0, 60, 26.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        }
        else if rand_num >= 86 && rand_num <= 90 { //Grow Enemy 5%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 5.0, 361, 0, 0, 0, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            WorkModule::set_int(fighter.module_accessor, 6, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
        }
        else if rand_num >= 91 && rand_num <= 95 { //Shrink Enemy 5%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_book"), 50.0, 361, 0, 0, 0, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            WorkModule::set_int(fighter.module_accessor, 7, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
        }
        else { //Slow Enemy 5%
            ATTACK(fighter, 2, 0, Hash40::new("kamek_box"), 10.0, 361, 0, 0, 0, 26.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            WorkModule::set_int(fighter.module_accessor, 8, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
        }
    }
    frame(fighter.lua_state_agent, 70.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialHiStart
unsafe extern "C" fn game_kamek_SpecialHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 30.0, 362, 75, 0, 20, 8.0, 0.0, 5.0, 0.0, Some(0.0), Some(5.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }
}

//SpecialAirHiStart
unsafe extern "C" fn game_kamek_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 30.0, 362, 75, 0, 20, 8.0, 0.0, 5.0, 0.0, Some(0.0), Some(5.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }
}

//SpecialHiEnd
unsafe extern "C" fn game_kamek_SpecialHiEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 78, 0, 30, 8.0, 0.0, 5.0, 0.0, Some(0.0), Some(5.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialAirHiEnd
unsafe extern "C" fn game_kamek_SpecialAirHiEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 78, 0, 30, 8.0, 0.0, 5.0, 0.0, Some(0.0), Some(5.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialLwStart
unsafe extern "C" fn game_kamek_SpecialLwStart(fighter: &mut L2CAgentBase) {
    let itemlist = [*ITEM_KIND_GREENSHELL, *ITEM_KIND_BOMBHEI, *ITEM_KIND_PITFALL, *ITEM_KIND_DEKU, *ITEM_KIND_SMARTBOMB, *ITEM_KIND_BUMPER];
    let rng = sv_math::rand(hash40("ness"), itemlist.len() as i32);
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 8.8, 0.0, 5.0, 4.8, 0.0, 0.0, 0.0, 1.0, 1.0, 700.0, false, 2.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT); 
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(itemlist[rng as usize]), 0, 0, false, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ItemModule::throw_item(fighter.module_accessor, 5.0, 2.2, 1.0, 0, true, 0.0);
    }
}


//SpecialAirLwStart
unsafe extern "C" fn game_kamek_SpecialAirLwStart(fighter: &mut L2CAgentBase) {
    let itemlist = [*ITEM_KIND_GREENSHELL, *ITEM_KIND_BOMBHEI, *ITEM_KIND_PITFALL, *ITEM_KIND_DEKU, *ITEM_KIND_SMARTBOMB, *ITEM_KIND_BUMPER];
    let rng = sv_math::rand(hash40("ness"), itemlist.len() as i32);
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 8.8, 0.0, 5.0, 4.8, 0.0, 0.0, 0.0, 1.0, 1.0, 700.0, false, 2.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT); 
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(itemlist[rng as usize]), 0, 0, false, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ItemModule::throw_item(fighter.module_accessor, 5.0, 2.2, 1.0, 0, true, 0.0);
    }
}

//AppealHiR
unsafe extern "C" fn game_kamek_AppealHiR(fighter: &mut L2CAgentBase) {}

//AppealHiL
unsafe extern "C" fn game_kamek_AppealHiL(fighter: &mut L2CAgentBase) {}

//AppealSR
unsafe extern "C" fn game_kamek_AppealSR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 25, 0, 10, 4.6, 0.0, 5.0, 7.0, Some(0.0), Some(5.0), Some(14.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_NONE);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1200, /*Rehit*/ 30, /* Damage*/ 5.0, /*Unk*/ false);
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_DEATHSCYTHE), 0, 0, false, false);
    }
}

//AppealSL
unsafe extern "C" fn game_kamek_AppealSL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 25, 0, 10, 4.6, 0.0, 5.0, 7.0, Some(0.0), Some(5.0), Some(14.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_NONE);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1200, /*Rehit*/ 30, /* Damage*/ 5.0, /*Unk*/ false);
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_DEATHSCYTHE), 0, 0, false, false);
    }
}

//AppealLwR
unsafe extern "C" fn game_kamek_AppealLwR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_TEAMHEALFIELD), 0, 0, false, false);
    }
}

//AppealLwL
unsafe extern "C" fn game_kamek_AppealLwL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_TEAMHEALFIELD), 0, 0, false, false);
    }
}

//Final
unsafe extern "C" fn game_kamek_Final(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 5.0, 100.0);
        FT_START_CUTIN(fighter);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        FT_SET_FINAL_FEAR_FACE(fighter, 60);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
    }   
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        VisibilityModule::set_whole(fighter.module_accessor, false);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 50, -50, 50, -10);
        let pos = Vector3f { x: 0.0, y: 110.0, z: 0.0 };
        PostureModule::set_pos(fighter.module_accessor, &pos);
		PostureModule::init_pos(fighter.module_accessor, &pos, true, true);
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        VisibilityModule::set_whole(fighter.module_accessor, true);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 110.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        ArticleModule::generate_article(fighter.module_accessor, FIGHTER_KAMEK_GENERATE_ARTICLE_FINALMAGIC, false, -1);
    }
    frame(fighter.lua_state_agent, 150.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

//FinalAir
unsafe extern "C" fn game_kamek_FinalAir(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 5.0, 100.0);
        FT_START_CUTIN(fighter);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        FT_SET_FINAL_FEAR_FACE(fighter, 60);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
    }   
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        VisibilityModule::set_whole(fighter.module_accessor, false);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 50, -50, 50, -10);
        let pos = Vector3f { x: 0.0, y: 110.0, z: 0.0 };
        PostureModule::set_pos(fighter.module_accessor, &pos);
		PostureModule::init_pos(fighter.module_accessor, &pos, true, true);
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        VisibilityModule::set_whole(fighter.module_accessor, true);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 110.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        ArticleModule::generate_article(fighter.module_accessor, FIGHTER_KAMEK_GENERATE_ARTICLE_FINALMAGIC, false, -1);
    }
    frame(fighter.lua_state_agent, 150.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

pub fn install() {
    Agent::new("ness")
    .game_acmd("game_jumpaerialfront_kamek", game_kamek_JumpAerialFront, Low)
    .game_acmd("game_jumpaerialback_kamek", game_kamek_JumpAerialBack, Low)
    .game_acmd("game_attack11_kamek", game_kamek_Attack11, Low)
    .game_acmd("game_attackdash_kamek", game_kamek_AttackDash, Low)
    .game_acmd("game_attacks3hi_kamek", game_kamek_AttackS3, Low)
    .game_acmd("game_attacks3_kamek", game_kamek_AttackS3, Low)
    .game_acmd("game_attacks3lw_kamek", game_kamek_AttackS3, Low)
    .game_acmd("game_attackhi3_kamek", game_kamek_AttackHi3, Low)
    .game_acmd("game_attacklw3_kamek", game_kamek_AttackLw3, Low)
    .game_acmd("game_attacks4_kamek", game_kamek_AttackS4, Low)
    .game_acmd("game_attackhi4_kamek", game_kamek_AttackHi4, Low)
    .game_acmd("game_attacklw4_kamek", game_kamek_AttackLw4, Low)
    .game_acmd("game_attackairn_kamek", game_kamek_AttackAirN, Low)
    .game_acmd("game_attackairf_kamek", game_kamek_AttackAirF, Low)    
    .game_acmd("game_attackairb_kamek", game_kamek_AttackAirB, Low)
    .game_acmd("game_attackairhi_kamek", game_kamek_AttackAirHi, Low)
    .game_acmd("game_attackairlw_kamek", game_kamek_AttackAirLw, Low)
    .game_acmd("game_catch_kamek", game_kamek_Catch, Low)
    .game_acmd("game_catchdash_kamek", game_kamek_CatchDash, Low)
    .game_acmd("game_catchturn_kamek", game_kamek_CatchTurn, Low)
    .game_acmd("game_catchattack_kamek", game_kamek_CatchAttack, Low)
    .game_acmd("game_throwf_kamek", game_kamek_ThrowF, Low)
    .game_acmd("game_throwb_kamek", game_kamek_ThrowB, Low)
    .game_acmd("game_throwhi_kamek", game_kamek_ThrowHi, Low)
    .game_acmd("game_throwlw_kamek", game_kamek_ThrowLw, Low)
    .game_acmd("game_downattackd_kamek", game_kamek_DownAttackD, Low)
    .game_acmd("game_downattacku_kamek", game_kamek_DownAttackU, Low)
    .game_acmd("game_cliffattack_kamek", game_kamek_CliffAttack, Low)
    .game_acmd("game_slipattack_kamek", game_kamek_SlipAttack, Low)
    .game_acmd("game_specialnstart_kamek", game_kamek_SpecialNStart, Low)  
    .game_acmd("game_specialairnstart_kamek", game_kamek_SpecialAirNStart, Low)  
    .game_acmd("game_specialnfire_kamek", game_kamek_SpecialNFire, Low)  
    .game_acmd("game_specialairnfire_kamek", game_kamek_SpecialAirNFire, Low)  
    .game_acmd("game_specials_kamek", game_kamek_SpecialS, Low)  
    .game_acmd("game_specialairs_kamek", game_kamek_SpecialAirS, Low) 
    .game_acmd("game_specialhistart_kamek", game_kamek_SpecialHiStart, Low)
    .game_acmd("game_specialairhistart_kamek", game_kamek_SpecialAirHiStart, Low)
    .game_acmd("game_specialhiend_kamek", game_kamek_SpecialHiEnd, Low)
    .game_acmd("game_specialairhiend_kamek", game_kamek_SpecialAirHiEnd, Low)
    .game_acmd("game_speciallwstart_kamek", game_kamek_SpecialLwStart, Low)
    .game_acmd("game_specialairlwstart_kamek", game_kamek_SpecialAirLwStart, Low)
    .game_acmd("game_appealsr_kamek", game_kamek_AppealSR, Low)
    .game_acmd("game_appealsl_kamek", game_kamek_AppealSL, Low)
    .game_acmd("game_appealhir_kamek", game_kamek_AppealHiR, Low)
    .game_acmd("game_appealhil_kamek", game_kamek_AppealHiL, Low)
    .game_acmd("game_appeallwr_kamek", game_kamek_AppealLwR, Low)
    .game_acmd("game_appeallwl_kamek", game_kamek_AppealLwL, Low)
    .game_acmd("game_final_kamek", game_kamek_Final, Low)
    .game_acmd("game_finalair_kamek", game_kamek_FinalAir, Low)
    .install();
}
