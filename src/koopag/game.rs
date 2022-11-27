use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;
//use crate::utils::FIGHTER_CUTIN_MANAGER;

#[acmd_script(//JumpSquat
    agent = "koopag", 
    script = "game_jumpsquat", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_jumpsquat(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
}

#[acmd_script(//Attack11 
    agent = "koopag", 
    script = "game_attack11", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_jab1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 19.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 70, /*BKB*/ 0, /*Size*/ 15.0, /*X*/ 6.0, /*Y*/ -0.1, /*Z*/ 0.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 19.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 70, /*BKB*/ 0, /*Size*/ 15.0, /*X*/ 8.0, /*Y*/ -0.7, /*Z*/ -0.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderl"), /*Damage*/ 19.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 70, /*BKB*/ 0, /*Size*/ 15.0, /*X*/ 4.0, /*Y*/ 0.1, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

#[acmd_script(//Attack12 
    agent = "koopag", 
    script = "game_attack12", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_jab2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 21.0, /*Angle*/ 45, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 16.0, /*X*/ 6.0, /*Y*/ -0.1, /*Z*/ 0.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 21.0, /*Angle*/ 45, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 16.0, /*X*/ 8.0, /*Y*/ -0.7, /*Z*/ -0.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderr"), /*Damage*/ 21.0, /*Angle*/ 45, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 16.0, /*X*/ 4.0, /*Y*/ 0.1, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//AttackDash
    agent = "koopag", 
    script = "game_attackdash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_dashattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 3.0);
	}
	frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
		macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 26.0, 80, 60, 0, 60, 16.0, 0.0, 6.0, 15.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 1, 0, Hash40::new("trans"), 26.0, 80, 60, 0, 60, 16.0, 0.0, 6.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 3, 0, Hash40::new("trans"), 26.0, 80, 60, 0, 60, 16.0, 0.0, 6.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 4, 0, Hash40::new("trans"), 26.0, 80, 60, 0, 60, 16.0, 0.0, 6.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
	}
	wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 25.0, 80, 60, 0, 50, 16.0, 0.0, 6.0, 15.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 1, 0, Hash40::new("trans"), 25.0, 80, 60, 0, 50, 16.0, 0.0, 6.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 2, 0, Hash40::new("trans"), 25.0, 80, 60, 0, 50, 16.0, 0.0, 6.0, 15.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 3, 0, Hash40::new("trans"), 25.0, 80, 60, 0, 50, 16.0, 0.0, 6.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
	}
	wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 24.0, 74, 80, 0, 30, 18.4, 0.0, 6.0, 16.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 1, 0, Hash40::new("trans"), 24.0, 74, 80, 0, 30, 18.4, 0.0, 6.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 2, 0, Hash40::new("trans"), 24.0, 74, 80, 0, 30, 18.4, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 3, 0, Hash40::new("trans"), 24.0, 74, 80, 0, 30, 18.4, 0.0, 6.0, -8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
	}
	wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(//AttackS3Hi
    agent = "koopag", 
    script = "game_attacks3hi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_sidetiltup(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 25.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 23.0, /*Angle*/ 361, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 20.0, /*X*/ 6.5, /*Y*/ -0.2, /*Z*/ 0.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 23.0, /*Angle*/ 361, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 15.0, /*X*/ -2.0, /*Y*/ -0.8, /*Z*/ -0.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
}

#[acmd_script(//AttackS3
    agent = "koopag", 
    script = "game_attacks3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_sidetilt(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 25.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 23.0, /*Angle*/ 361, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 20.0, /*X*/ 6.5, /*Y*/ -0.2, /*Z*/ 0.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 23.0, /*Angle*/ 361, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 15.0, /*X*/ -2.0, /*Y*/ -0.8, /*Z*/ -0.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
}

#[acmd_script(//AttackS3Lw
    agent = "koopag", 
    script = "game_attacks3lw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_sidetiltdown(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 25.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 23.0, /*Angle*/ 361, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 20.0, /*X*/ 6.5, /*Y*/ -0.2, /*Z*/ 0.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 23.0, /*Angle*/ 361, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 15.0, /*X*/ -2.0, /*Y*/ -0.8, /*Z*/ -0.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
}

#[acmd_script(//AttackHi3
    agent = "koopag", 
    script = "game_attackhi3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_uptilt(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.75);
    frame(fighter.lua_state_agent, 24.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 22.0, /*Angle*/ 80, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 20.0, /*X*/ 7.0, /*Y*/ -0.2, /*Z*/ 0.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 22.0, /*Angle*/ 80, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 17.0, /*X*/ 8.0, /*Y*/ -0.8, /*Z*/ -0.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderl"), /*Damage*/ 20.0, /*Angle*/ 80, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 17.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 80, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 15.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 45.0, /*X2*/ Some(0.0), /*Y2*/ Some(20.0), /*Z2*/ Some(45.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, /*ID*/ 3, false);
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    }
    frame(fighter.lua_state_agent, 60.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.85);
}

#[acmd_script(//AttackLw3
    agent = "koopag", 
    script = "game_attacklw3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_downtilt(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 12.0, 361, 100, 30, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 20.0, false);
	}
	wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("tail2"), 17.0, 80, 60, 0, 60, 12.6, -4.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
		macros::ATTACK(fighter, 1, 0, Hash40::new("tail2"), 17.0, 80, 60, 0, 60, 12.5, 1.9, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
		macros::ATTACK(fighter, 2, 0, Hash40::new("tail2"), 17.0, 80, 60, 0, 60, 12.3, 7.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
		macros::ATTACK(fighter, 3, 0, Hash40::new("tail2"), 17.0, 80, 60, 0, 60, 12.3, 9.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 480, /*Rehit*/ 30, /* Damage*/ 2.3, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 480, /*Rehit*/ 30, /* Damage*/ 2.3, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 480, /*Rehit*/ 30, /* Damage*/ 2.3, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 480, /*Rehit*/ 30, /* Damage*/ 2.3, /*Unk*/ false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 12.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 12.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 12.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 12.0, false);
	}
	wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(//AttackS4
    agent = "koopag", 
    script = "game_attacks4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_sidesmash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.4);
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.0);
    frame(fighter.lua_state_agent, 22.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 44.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("head"), /*Damage*/ 100.0, /*Angle*/ 45, /*KBG*/ 18, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 16.0, /*X*/ 7.0, /*Y*/ 3.0, /*Z*/ 9.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 49.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("head"), /*Damage*/ 100.0, /*Angle*/ 45, /*KBG*/ 18, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 16.0, /*X*/ 7.0, /*Y*/ 3.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("head"), /*Damage*/ 100.0, /*Angle*/ 45, /*KBG*/ 18, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 25.0, /*X*/ 18.0, /*Y*/ -6.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 52.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("head"), /*Damage*/ 100.0, /*Angle*/ 45, /*KBG*/ 18, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 30.0, /*X*/ 18.0, /*Y*/ -8.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("head"), /*Damage*/ 100.0, /*Angle*/ 45, /*KBG*/ 18, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 15.0, /*X*/ -5.0, /*Y*/ 3.0, /*Z*/ 5.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 108.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
}

#[acmd_script(//AttackHi4
    agent = "koopag", 
    script = "game_attackhi4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_upsmash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 2);
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 3);
    frame(fighter.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_INVINCIBLE);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("bust"), /*Damage*/ 42.0, /*Angle*/ 90, /*KBG*/ 49, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 27.0, /*X*/ -9.0, /*Y*/ 5.7, /*Z*/ -13.5, /*X2*/ Some(6.0), /*Y2*/ Some(-3.7), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("bust"), /*Damage*/ 42.0, /*Angle*/ 90, /*KBG*/ 49, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 30.0, /*X*/ -3.0, /*Y*/ 1.8, /*Z*/ -4.5, /*X2*/ Some(6.0), /*Y2*/ Some(-3.7), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("rot"), /*Damage*/ 30.0, /*Angle*/ 270, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 20.0, /*X*/ 0.0, /*Y*/ -28.0, /*Z*/ -14.0, /*X2*/ Some(0.0), /*Y2*/ Some(-28.0), /*Z2*/ Some(22.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 40.0);
}

#[acmd_script(//AttackLw4
    agent = "koopag", 
    script = "game_attacklw4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_downsmash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.0);
    frame(fighter.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("toel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("tail3"), *HIT_STATUS_XLU);
    }
    for _ in 0..5 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 120, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 19.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -25.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 3.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 120, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 19.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 25.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 3.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 160, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 18.0, /*X*/ 0.0, /*Y*/ 24.0, /*Z*/ -14.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 3.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 160, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 18.0, /*X*/ 0.0, /*Y*/ 24.0, /*Z*/ 14.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 3.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 367, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 17.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 3.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 100, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 19.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -20.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 3.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ice"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 100, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 19.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 20.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 3.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ice"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 100, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 18.0, /*X*/ 0.0, /*Y*/ 24.0, /*Z*/ -14.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 3.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ice"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 100, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 18.0, /*X*/ 0.0, /*Y*/ 24.0, /*Z*/ 14.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 3.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ice"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 100, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 17.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 3.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ice"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script(//AttackAirN
    agent = "koopag", 
    script = "game_attackairn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_nair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 0.6);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 2.0);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 27.0, 361, 80, 0, 20, 24.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 24.0, 60, 95, 0, 20, 20.0, 0.0, 15.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(//AttackAirF 
    agent = "koopag", 
    script = "game_attackairf", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_fair(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.0);
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 35.0, /*Angle*/ 362, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 22.0, /*X*/ 7.0, /*Y*/ -0.2, /*Z*/ 0.7, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 35.0, /*Angle*/ 362, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 20.0, /*X*/ 8.0, /*Y*/ -0.8, /*Z*/ -0.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderl"), /*Damage*/ 35.0, /*Angle*/ 362, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 20.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script(//AttackAirB
    agent = "koopag", 
    script = "game_attackairb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_bair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.5);
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("waist"), /*Damage*/ 40.0, /*Angle*/ 24, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 35.0, /*X*/ 3.5, /*Y*/ -3.7, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script(//AttackAirHi
    agent = "koopag", 
    script = "game_attackairhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_uair(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.75);
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 22.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("head"), /*Damage*/ 25.0, /*Angle*/ 85, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 14.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("head"), /*Damage*/ 25.0, /*Angle*/ 85, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 14.0, /*X*/ 0.0, /*Y*/ -7.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script(//AttackAirLw
    agent = "koopag", 
    script = "game_attackairlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_dair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(fighter.lua_state_agent, 15.0);
	for _ in 0..7 {
        if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 367, 40, 0, 35, 18.3, 0.0, 0.0, -10.0, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 367, 40, 0, 35, 18.3, 0.0, 0.0, 10.0, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.5, 367, 40, 0, 35, 16.1, 0.0, -5.0, -6.5, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 5.5, 367, 40, 0, 35, 16.1, 0.0, -5.0, 6.5, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 5.5, 367, 40, 0, 35, 20.4, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		}
		wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		wait(fighter.lua_state_agent, 1.0);
	}
	frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 30.0, 270, 100, 0, 50, 9.3, 0.0, 0.0, -10.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 30.0, 270, 100, 0, 50, 9.3, 0.0, 0.0, 10.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 30.0, 270, 100, 0, 50, 8.1, 0.0, -5.0, -6.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 30.0, 270, 100, 0, 50, 8.1, 0.0, -5.0, 6.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 30.0, 270, 100, 0, 50, 10.4, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(//LandingAirB
    agent = "koopag", 
    script = "game_landingairb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_bairland(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        KineticModule::set_consider_ground_friction(fighter.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 30.0, /*Angle*/ 80, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 25.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_saving"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_attack_level(fighter.module_accessor, /*ID*/ 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//LandingAirLw
    agent = "koopag", 
    script = "game_landingairlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_dairland(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 25.0, 361, 70, 0, 30, 30.6, 0.0, 4.0, 16.0, Some(0.0), Some(4.0), Some(-16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(//Catch
    agent = "koopag", 
    script = "game_catch", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_grab(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        //macros::game_CaptureCutCommon(fighter);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 12.0, 0.0, 9.0, 18.8, Some(0.0), Some(9.0), Some(26.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

#[acmd_script(//CatchAttack
    agent = "koopag", 
    script = "game_catchattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_pummel(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 15.0, 361, 0, 0, 0, 15.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
	}
	wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(//ThrowF
    agent = "koopag", 
    script = "game_throwf", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_throwf(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 30.0, 361, 71, 0, 20, 10.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(//ThrowB
    agent = "koopag", 
    script = "game_throwb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_throwb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 33.0, 361, 73, 0, 20, 10.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(//ThrowHi
    agent = "koopag", 
    script = "game_throwhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_throwup(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.7);
	}
	frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 31.0, 90, 68, 0, 22, 10.0, 0.0, 9.0, 8.8, Some(0.0), Some(9.0), Some(28.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}      

#[acmd_script(//ThrowLw
    agent = "koopag", 
    script = "game_throwlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_throwdown(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 100.0, 270, 20, 0, 0, 10.0, 0.0, 9.0, 8.8, Some(0.0), Some(9.0), Some(28.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(//CliffAttack
    agent = "koopag", 
    script = "game_cliffattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_cliffattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("bust"), /*Damage*/ 30.0, /*Angle*/ 361, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 22.0, /*X*/ 2.5, /*Y*/ -3.7, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 30.0, /*Angle*/ 361, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 20.0, /*X*/ -5.4, /*Y*/ -1.8, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//DownAttackD
    agent = "koopag", 
    script = "game_downattackd", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_downattackd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 28.0, /*Angle*/ 48, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 25.5, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 58.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.5), /*Z2*/ Some(5.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 28.0, /*Angle*/ 48, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 25.5, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ -53.5, /*X2*/ Some(0.0), /*Y2*/ Some(5.5), /*Z2*/ Some(-5.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//DownAttackU
    agent = "koopag", 
    script = "game_downattacku", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_downattacku(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 28.0, /*Angle*/ 48, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 25.5, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 58.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.5), /*Z2*/ Some(5.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 28.0, /*Angle*/ 48, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 25.5, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ -53.5, /*X2*/ Some(0.0), /*Y2*/ Some(5.5), /*Z2*/ Some(-5.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//SpecialNBreath
    agent = "koopag_breath", 
    script = "game_move", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_fire(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 30.0, /*Angle*/ 45, /*KBG*/ 89, /*FKB*/ 0, /*BKB*/ 22, /*Size*/ 20.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_BOMB, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 29.4, /*Angle*/ 45, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 21, /*Size*/ 23.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_BOMB, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 28.8, /*Angle*/ 45, /*KBG*/ 91, /*FKB*/ 0, /*BKB*/ 18, /*Size*/ 27.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_BOMB, /*Type*/ *ATTACK_REGION_NONE);
    }
}

#[acmd_script(//SpecialSCatch
    agent = "koopag", 
    script = "game_specialscatch", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_specialscatch(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	frame(fighter.lua_state_agent, 37.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
		macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 42.0, 361, 70, 0, 50, 16.0, 6.0, -0.100000001, 0.600000024, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 42.0, 361, 70, 0, 50, 16.0, 8.0, -0.699999988, -0.200000003, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 42.0, 361, 70, 0, 50, 16.0, 4.0, 0.100000001, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		MotionModule::set_rate(fighter.module_accessor, 1.5);
	}
}

#[acmd_script(//SpecialSAirCatch
    agent = "koopag", 
    script = "game_specialsaircatch", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_specialairscatch(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
		KineticModule::add_speed(fighter.module_accessor, &Vector3f{x:3.0,y:0.0,z:0.0});
    }
	frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
		macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 43.0, 60, 55, 0, 50, 17.0, 6.0, -0.100000001, 0.600000024, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 43.0, 60, 55, 0, 50, 17.0, 8.0, -0.699999988, -0.200000003, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 43.0, 60, 55, 0, 50, 17.0, 4.0, 0.100000001, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
	wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}	
}

#[acmd_script(//SpecialSFall
    agent = "koopag", 
    script = "game_specialsfall", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_sideb1(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 50.0, /*Angle*/ 361, /*KBG*/ 48, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 20.0, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 6.5, /*X2*/ Some(0.0), /*Y2*/ Some(2.0), /*Z2*/ Some(-4.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_flower"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_BODY);
    }
}

#[acmd_script(//SpecialSLanding
    agent = "koopag", 
    script = "game_specialslanding", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_sideb2(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 50.0, /*Angle*/ 60, /*KBG*/ 46, /*FKB*/ 0, /*BKB*/ 4, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 50.0, /*Angle*/ 70, /*KBG*/ 46, /*FKB*/ 0, /*BKB*/ 4, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_HIT);
    }
}

#[acmd_script(//SpecialHi
    agent = "koopag", 
    script = "game_specialhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_upb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::clear_speed_all(fighter.module_accessor);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 8.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 20.0, 20.0);
    }
    frame(fighter.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 20.0);
    frame(fighter.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.2);
    if macros::is_excute(fighter) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 180, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 24.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ -8.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 180, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 24.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 12.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG3);
    }
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("toel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("tail3"), *HIT_STATUS_XLU);
    }
    if macros::is_excute(fighter) {
        KineticModule::set_consider_ground_friction(fighter.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG2);
    }
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("toel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("tail3"), *HIT_STATUS_XLU);
    }
    if macros::is_excute(fighter) {
        KineticModule::set_consider_ground_friction(fighter.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
		AttackModule::clear(fighter.module_accessor, 1, false);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 70, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 27.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ -8.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ice"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 70, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 27.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 12.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ice"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG3);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        KineticModule::set_consider_ground_friction(fighter.module_accessor, true, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1);
    }
}

#[acmd_script(//SpecialAirHi
    agent = "koopag", 
    script = "game_specialairhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_upbair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 60, 0, 80, 12.0, 0.0, 18.0, -5.0, Some(0.0), Some(18.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 367, 60, 0, 80, 12.0, 0.0, 18.0, -5.0, Some(0.0), Some(18.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	frame(fighter.lua_state_agent, 7.0);
	for _ in 0..5 {
        if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 84, 20, 0, 50, 12.0, 0.0, 18.0, -5.0, Some(0.0), Some(18.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
			macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 367, 20, 0, 50, 12.0, 0.0, 18.0, -5.0, Some(0.0), Some(18.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		}
		wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		}
		wait(fighter.lua_state_agent, 1.0);
	}
	frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 22.0, 65, 78, 0, 35, 12.0, 0.0, 18.0, -5.0, Some(0.0), Some(18.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG3);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1);
	}
	wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(//SpecialLw
    agent = "koopag", 
    script = "game_speciallw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_downb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(fighter.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.0);
    frame(fighter.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 4.0);
    frame(fighter.lua_state_agent, 22.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.5);
    frame(fighter.lua_state_agent, 23.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 85, /*KBG*/ 100, /*FKB*/ 110, /*BKB*/ 0, /*Size*/ 19.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 29.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1);
    }
    frame(fighter.lua_state_agent, 52.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 76, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 22.0, /*X*/ 0.0, /*Y*/ 20.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_NONE);
    }
}

#[acmd_script(//SpecialAirLw
    agent = "koopag", 
    script = "game_specialairlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_downbair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 76, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 24.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_NONE);
    }
}

#[acmd_script(//SpecialLwLanding
    agent = "koopag", 
    script = "game_speciallwlanding", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn koopag_downbairland(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 30.0, /*Angle*/ 60, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 36.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -5.5, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
}

pub fn install() {
    smashline::install_acmd_scripts!(
        koopag_jumpsquat,
        koopag_jab1,
        koopag_jab2,
        koopag_dashattack,
        koopag_sidetiltup,
        koopag_sidetilt,
        koopag_sidetiltdown,
        koopag_uptilt,
        koopag_downtilt,
        koopag_sidesmash,
        koopag_upsmash,
        koopag_downsmash,
        koopag_nair,
        koopag_fair,
        koopag_bair,
        koopag_uair,
        koopag_dair,
        koopag_bairland,
        koopag_dairland,
        koopag_grab,
        koopag_pummel,
        koopag_throwf,
        koopag_throwb,
        koopag_throwup,
        koopag_throwdown,
        koopag_cliffattack,
        koopag_downattackd,
        koopag_downattacku,
        koopag_fire,
        koopag_sideb1,
        koopag_specialscatch,
        koopag_specialairscatch,
        koopag_sideb2,
        koopag_upb,
        koopag_upbair,
        koopag_downb,
        koopag_downbair,
        koopag_downbairland
    );
}
