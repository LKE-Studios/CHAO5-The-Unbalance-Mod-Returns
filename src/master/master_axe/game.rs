use crate::imports::BuildImports::*;

//SpecialLw
unsafe extern "C" fn game_master_axe_SpecialLw(fighter: &mut L2CAgentBase) {
    let rand_effect = [Hash40::new("collision_attr_bury"), Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_sleep"), Hash40::new("collision_attr_bind"), Hash40::new("collision_attr_saving")];
    let rng = smash::app::sv_math::rand(hash40("master"), rand_effect.len() as i32);
    if is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, 0, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_INT_CRITICAL_ATTACK_ID);
    }
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 999.0, /*Angle*/ 0, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 50.0, /*Angle*/ 362, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 15.6, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(35.0), /*Z2*/ Some(0.0), /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 50, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 20.0, /*Angle*/ 270, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 90.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_SLAP, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
        EFFECT(fighter, Hash40::new("sys_wind_wave"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 20.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 7.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("haver"), 8.0, 0.0, 0.0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.09, /*B*/ 0.08);
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
    frame(fighter.lua_state_agent, 70.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialAirLw
unsafe extern "C" fn game_master_axe_SpecialAirLw(fighter: &mut L2CAgentBase) {
    let rand_effect = [Hash40::new("collision_attr_bury"), Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_sleep"), Hash40::new("collision_attr_bind"), Hash40::new("collision_attr_saving")];
    let rng = smash::app::sv_math::rand(hash40("master"), rand_effect.len() as i32);
    if is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, 0, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_INT_CRITICAL_ATTACK_ID);
    }
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 999.0, /*Angle*/ 0, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 50.0, /*Angle*/ 362, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 15.6, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(35.0), /*Z2*/ Some(0.0), /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 50, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ rand_effect[rng as usize], /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ *ATTACK_REGION_OBJECT);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 20.0, /*Angle*/ 270, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 90.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_SLAP, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
        EFFECT(fighter, Hash40::new("sys_wind_wave"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 20.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 7.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("haver"), 8.0, 0.0, 0.0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.09, /*B*/ 0.08);
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
    frame(fighter.lua_state_agent, 70.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialLwHit
unsafe extern "C" fn game_master_axe_SpecialLwHit(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 30.0, /*Angle*/ 270, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 85.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_SLAP, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 0, false);
        EFFECT(fighter, Hash40::new("sys_wind_wave"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 20.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 7.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.09, /*B*/ 0.08);
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialAirLwHit
unsafe extern "C" fn game_master_axe_SpecialAirLwHit(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 30.0, /*Angle*/ 270, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 85.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_SLAP, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 0, false);
        EFFECT(fighter, Hash40::new("sys_wind_wave"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 20.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 7.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.09, /*B*/ 0.08);
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("master_axe")
    .game_acmd("game_speciallw", game_master_axe_SpecialLw)
    .game_acmd("game_specialairlw", game_master_axe_SpecialAirLw)
    .game_acmd("game_speciallwhit", game_master_axe_SpecialLwHit)
    .game_acmd("game_specialairlwhit", game_master_axe_SpecialAirLwHit)
    .install();
}