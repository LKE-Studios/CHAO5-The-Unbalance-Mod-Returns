use crate::imports::BuildImports::*;

//DHSpecialNEnd1
unsafe extern "C" fn game_kamui_dragonhand_DHSpecialNEnd1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLAG_IS_KAMUI) {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
            attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
            let hold_rate = WorkModule::get_float(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
            ATK_LERP_RATIO(fighter, hold_rate);
        }
    }
    else {
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 8.8, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.8), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
                ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 8.8, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.8), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
            }
            else {
                if is_excute(fighter) {
                    ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 8.1, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.1), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
                    ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 8.1, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.1), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
                }
            }
            if is_excute(fighter) {
                attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
                let hold_rate = WorkModule::get_float(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
                ATK_LERP_RATIO(fighter, hold_rate);
            }
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//DHSpecialNEnd2
unsafe extern "C" fn game_kamui_dragonhand_DHSpecialNEnd2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLAG_IS_KAMUI) == true {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
            attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
            let hold_rate = WorkModule::get_float(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
            ATK_LERP_RATIO(fighter, hold_rate);
        }
    }
    else {
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 8.8, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.8), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
                ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 8.8, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.8), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
            }
            else {
                if is_excute(fighter) {
                    ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 8.1, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.1), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
                    ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 8.1, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.1), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
                }
            }
            if is_excute(fighter) {
                attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
                let hold_rate = WorkModule::get_float(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
                ATK_LERP_RATIO(fighter, hold_rate);
            }
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//DHSpecialNEndMax
unsafe extern "C" fn game_kamui_dragonhand_DHSpecialNEndMax(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLAG_IS_KAMUI) {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 300.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 28, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 22.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(14.0), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
        }
    }
    else {
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 300.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 28, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 8.9, /*Z*/ 22.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.9), /*Z2*/ Some(14.0), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
            }
            else {
                if is_excute(fighter) {
                    ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 300.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 28, /*Size*/ 9.0, /*X*/ 7.0, /*Y*/ 12.0, /*Z*/ 22.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.0), /*Z2*/ Some(14.0), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
                }
            }
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//DHSpecialAirNEnd1
unsafe extern "C" fn game_kamui_dragonhand_DHSpecialAirNEnd1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLAG_IS_KAMUI) {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
            attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
            let hold_rate = WorkModule::get_float(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
            ATK_LERP_RATIO(fighter, hold_rate);
        }
    }
    else {
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 8.8, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.8), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
                ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 8.8, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.8), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
            }
            else {
                if is_excute(fighter) {
                    ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 8.1, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.1), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
                    ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 8.1, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.1), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
                }
            }
            if is_excute(fighter) {
                attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
                let hold_rate = WorkModule::get_float(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
                ATK_LERP_RATIO(fighter, hold_rate);
            }
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//DHSpecialAirNEnd2
unsafe extern "C" fn game_kamui_dragonhand_DHSpecialAirNEnd2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLAG_IS_KAMUI) {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
            attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
            let hold_rate = WorkModule::get_float(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
            ATK_LERP_RATIO(fighter, hold_rate);
        }
    }
    else {
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 8.8, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.8), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
                ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 8.8, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.8), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
            }
            else {
                if is_excute(fighter) {
                    ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 8.1, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.1), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
                    ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 8.1, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.1), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
                }
            }
            if is_excute(fighter) {
                attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
                let hold_rate = WorkModule::get_float(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
                ATK_LERP_RATIO(fighter, hold_rate);
            }
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//DHSpecialAirNEndMax
unsafe extern "C" fn game_kamui_dragonhand_DHSpecialAirNEndMax(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLAG_IS_KAMUI) {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 300.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 28, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 22.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(14.0), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
        }
    }
    else {
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 300.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 28, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 8.9, /*Z*/ 22.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.9), /*Z2*/ Some(14.0), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
            }
            else {
                if is_excute(fighter) {
                    ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 300.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 28, /*Size*/ 9.0, /*X*/ 7.0, /*Y*/ 12.0, /*Z*/ 22.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.0), /*Z2*/ Some(14.0), /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BITE);
                }
            }
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("kamui_dragonhand")
    .game_acmd("game_dhspecialnend1", game_kamui_dragonhand_DHSpecialAirNEnd1)
    .game_acmd("game_dhspecialnend2", game_kamui_dragonhand_DHSpecialAirNEnd2)
    .game_acmd("game_dhspecialnendmax", game_kamui_dragonhand_DHSpecialNEndMax)
    .game_acmd("game_dhspecialairnend1", game_kamui_dragonhand_DHSpecialAirNEnd1)
    .game_acmd("game_dhspecialairnend2", game_kamui_dragonhand_DHSpecialAirNEnd2)
    .game_acmd("game_dhspecialairnendmax", game_kamui_dragonhand_DHSpecialAirNEndMax)
    .install();
}