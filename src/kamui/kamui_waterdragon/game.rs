use crate::imports::BuildImports::*;

//SpecialLwHit
unsafe extern "C" fn game_kamui_waterdragon_SpecialLwHit(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 80, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 100.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(-100.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 80, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -100.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(100.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
        if is_excute(fighter) {
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 0, Hash40::new("se_kamui_criticalhit"));
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 1, Hash40::new("se_kamui_criticalhit"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 90, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 100.0, /*X2*/ Some(0.0), /*Y2*/ Some(21.0), /*Z2*/ Some(-100.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 90, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -100.0, /*X2*/ Some(0.0), /*Y2*/ Some(21.0), /*Z2*/ Some(100.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 3, true, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
        if is_excute(fighter) {
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 2, Hash40::new("se_kamui_criticalhit"));
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 3, Hash40::new("se_kamui_criticalhit"));
        }
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialLwHitTurn
unsafe extern "C" fn game_kamui_waterdragon_SpecialLwHitTurn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 80, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 100.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(-100.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 80, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -100.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(100.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
        if is_excute(fighter) {
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 0, Hash40::new("se_kamui_criticalhit"));
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 1, Hash40::new("se_kamui_criticalhit"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 90, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 100.0, /*X2*/ Some(0.0), /*Y2*/ Some(21.0), /*Z2*/ Some(-100.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 90, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -100.0, /*X2*/ Some(0.0), /*Y2*/ Some(21.0), /*Z2*/ Some(100.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 3, true, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
        if is_excute(fighter) {
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 2, Hash40::new("se_kamui_criticalhit"));
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 3, Hash40::new("se_kamui_criticalhit"));
        }
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialAirLwHit
unsafe extern "C" fn game_kamui_waterdragon_SpecialAirLwHit(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 80, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 100.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(-100.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 80, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -100.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(100.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
        if is_excute(fighter) {
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 0, Hash40::new("se_kamui_criticalhit"));
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 1, Hash40::new("se_kamui_criticalhit"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 90, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 100.0, /*X2*/ Some(0.0), /*Y2*/ Some(21.0), /*Z2*/ Some(-100.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 90, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -100.0, /*X2*/ Some(0.0), /*Y2*/ Some(21.0), /*Z2*/ Some(100.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 3, true, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
        if is_excute(fighter) {
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 2, Hash40::new("se_kamui_criticalhit"));
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 3, Hash40::new("se_kamui_criticalhit"));
        }
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialAirLwHitTurn
unsafe extern "C" fn game_kamui_waterdragon_SpecialAirLwHitTurn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 80, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 100.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(-100.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 80, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -100.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(100.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
        if is_excute(fighter) {
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 0, Hash40::new("se_kamui_criticalhit"));
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 1, Hash40::new("se_kamui_criticalhit"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 90, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 100.0, /*X2*/ Some(0.0), /*Y2*/ Some(21.0), /*Z2*/ Some(-100.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 90, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -100.0, /*X2*/ Some(0.0), /*Y2*/ Some(21.0), /*Z2*/ Some(100.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_WATER, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 3, true, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
        if is_excute(fighter) {
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 2, Hash40::new("se_kamui_criticalhit"));
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 3, Hash40::new("se_kamui_criticalhit"));
        }
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("kamui_waterdragon")
    .game_acmd("game_speciallwhit", game_kamui_waterdragon_SpecialLwHit)
    .game_acmd("game_speciallwhitturn", game_kamui_waterdragon_SpecialLwHitTurn)
    .game_acmd("game_specialairlwhit", game_kamui_waterdragon_SpecialAirLwHit)
    .game_acmd("game_specialairlwhitturn", game_kamui_waterdragon_SpecialAirLwHitTurn)
    .install();
}