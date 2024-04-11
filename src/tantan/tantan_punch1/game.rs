use crate::imports::BuildImports::*;

//AttackShort (Covers F-Tilt, Hold, F-Air, B-Air);
unsafe extern "C" fn game_tantan_punch1_AttackShort(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.75);
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 89, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 16.7, /*X*/ 3.1, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
            AttackModule::enable_safe_pos(fighter.module_accessor);
            AttackModule::disable_tip(fighter.module_accessor);
            AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
        }
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 89, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 24.9, /*X*/ 10.1, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(1.0), /*Y2*/ Some(0.5), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 9, false, 0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 16.7, /*X*/ 3.1, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::disable_tip(fighter.module_accessor);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 89, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 24.9, /*X*/ 10.1, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(1.0), /*Y2*/ Some(0.5), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 9, false, 0);
    }
}

//AttackDragonShootShort 
unsafe extern "C" fn game_tantan_punch1_AttackDragonShootShort(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) {
        if is_excute(fighter) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 17.5, /*Angle*/ 30, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 16.7, /*X*/ 5.4, /*Y*/ 0.5, /*Z*/ 0.3, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -2.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
            AttackModule::enable_safe_pos(fighter.module_accessor);
            AttackModule::disable_tip(fighter.module_accessor);
            AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
        }
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 17.5, /*Angle*/ 30, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 24.2, /*X*/ 10.4, /*Y*/ 0.5, /*Z*/ 0.3, /*X2*/ Some(1.0), /*Y2*/ Some(0.5), /*Z2*/ Some(0.3), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -2.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 9, false, 0);
    }
    frame(fighter.lua_state_agent, 8.857);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 17.5, /*Angle*/ 30, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 16.7, /*X*/ 5.4, /*Y*/ 0.5, /*Z*/ 0.3, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -2.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::disable_tip(fighter.module_accessor);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 17.5, /*Angle*/ 30, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 24.2, /*X*/ 10.4, /*Y*/ 0.5, /*Z*/ 0.3, /*X2*/ Some(1.0), /*Y2*/ Some(0.5), /*Z2*/ Some(0.3), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -2.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 9, false, 0);
    }
}

//AttackLong
unsafe extern "C" fn game_tantan_punch1_AttackLong(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_KIRBY) {
        if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) { 
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 20.0, /*Angle*/ 361, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 16.7, /*X*/ 3.1, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -3.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
                AttackModule::enable_safe_pos(fighter.module_accessor);
                AttackModule::disable_tip(fighter.module_accessor);
                AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
            }
            frame(fighter.lua_state_agent, 1.0);
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 20.0, /*Angle*/ 361, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 24.9, /*X*/ 10.1, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(1.0), /*Y2*/ Some(0.5), /*Z2*/ Some(0.0), /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -3.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
                ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 18, false, 0);
            }
            frame(fighter.lua_state_agent, 3.0);
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 24.0, /*Angle*/ 361, /*KBG*/ 91, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 24.9, /*X*/ 10.1, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(1.0), /*Y2*/ Some(0.5), /*Z2*/ Some(0.0), /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -3.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
                WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
            }
            frame(fighter.lua_state_agent, 7.0);
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 24.0, /*Angle*/ 361, /*KBG*/ 91, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 24.9, /*X*/ 10.1, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(1.0), /*Y2*/ Some(0.5), /*Z2*/ Some(0.0), /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -3.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
                WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
            }
            frame(fighter.lua_state_agent, 11.0);
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT) {
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 20.0, /*Angle*/ 361, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 16.7, /*X*/ 3.1, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -3.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
                AttackModule::enable_safe_pos(fighter.module_accessor);
                AttackModule::disable_tip(fighter.module_accessor);
                AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
            }
            frame(fighter.lua_state_agent, 1.0);
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 20.0, /*Angle*/ 361, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 24.9, /*X*/ 16.1, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(1.0), /*Y2*/ Some(0.5), /*Z2*/ Some(0.0), /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -3.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
                ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 18, false, 0);
            }
            frame(fighter.lua_state_agent, 4.0);
            if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND) {
                //Nothing lol
            }
        }
        else {
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 20.0, /*Angle*/ 361, /*KBG*/ 91, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 16.7, /*X*/ 3.1, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -3.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
                AttackModule::enable_safe_pos(fighter.module_accessor);
                AttackModule::disable_tip(fighter.module_accessor);
                AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
            }
            frame(fighter.lua_state_agent, 1.0);
            if is_excute(fighter) {
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 20.0, /*Angle*/ 361, /*KBG*/ 91, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 24.9, /*X*/ 10.1, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(1.0), /*Y2*/ Some(0.5), /*Z2*/ Some(0.0), /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -3.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
                ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 6, false, 0);
            }
            frame(fighter.lua_state_agent, 4.0);
            if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND) {
                //Nothing lol
            }
            else {
                if is_excute(fighter) {
                    ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 24.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 24.9, /*X*/ 10.1, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(1.0), /*Y2*/ Some(0.5), /*Z2*/ Some(0.0), /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -3.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
                    WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
                }
                frame(fighter.lua_state_agent, 8.0);
                if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND) {
                    if is_excute(fighter) {
                        AttackModule::clear_all(fighter.module_accessor);
                    }
                }
                else {
                    if is_excute(fighter) {
                        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 24.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 24.9, /*X*/ 10.1, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(1.0), /*Y2*/ Some(0.5), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -3.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
                        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
                    }
                    frame(fighter.lua_state_agent, 9.0);
                    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND) {
                        //Nothing lol
                    }
                    else {
                        if is_excute(fighter) {
                            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 22.0, /*Angle*/ 361, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 24.9, /*X*/ 10.1, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ Some(1.0), /*Y2*/ Some(0.5), /*Z2*/ Some(0.0), /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -3.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
                            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_CHANGE_HIT_EFFECT);
                        }
                    }
                }
            }
        }
    }
}

//SpecialNShootK
unsafe extern "C" fn game_tantan_punch1_SpecialNShootK(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 15.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.7, /*X*/ 3.1, /*Y*/ 0.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::disable_tip(fighter.module_accessor);
    }
}

//SpecialAirHiAttackDragon
unsafe extern "C" fn game_tantan_punch1_SpecialAirHiAttackDragon(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 36.0, /*Angle*/ 45, /*KBG*/ 71, /*FKB*/ 0, /*BKB*/ 22, /*Size*/ 11.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.4, /*X2*/ Some(-6.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.4), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 0, /*Ink*/ 100.0);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 36.0, /*Angle*/ 45, /*KBG*/ 71, /*FKB*/ 0, /*BKB*/ 22, /*Size*/ 11.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.4, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.4), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 0, /*Ink*/ 100.0);
    }
}

pub fn install() {
    Agent::new("tantan_punch1")
    .game_acmd("game_attackshort", game_tantan_punch1_AttackShort) 
    .game_acmd("game_attackdragonshootshort", game_tantan_punch1_AttackDragonShootShort)    
    .game_acmd("game_attacklong", game_tantan_punch1_AttackLong) 
    .game_acmd("game_specialnshootk", game_tantan_punch1_SpecialNShootK) 
    .game_acmd("game_specialairhiattackdragon", game_tantan_punch1_SpecialAirHiAttackDragon) 
    .install();
}