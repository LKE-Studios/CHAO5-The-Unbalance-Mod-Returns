use crate::imports::BuildImports::*;

//Shoot
unsafe extern "C" fn game_lucario_qigong_Shoot(fighter: &mut L2CAgentBase) {
	if is_excute(fighter) {
		ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 43, /*KBG*/ 58, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 10.6, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(8.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.2, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_LUCARIO, /*Type*/ *ATTACK_REGION_NONE);
		ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 43, /*KBG*/ 58, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 2.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(35.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.2, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_LUCARIO, /*Type*/ *ATTACK_REGION_NONE);
		ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 43, /*KBG*/ 58, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ -1.0, /*Z*/ -4.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.2, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_aura"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_LUCARIO, /*Type*/ *ATTACK_REGION_NONE);
	}
	wait(fighter.lua_state_agent, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	wait(fighter.lua_state_agent, 8.0);
	if is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
	}
}

pub fn install() {
    Agent::new("lucario_qigong")    
    .game_acmd("game_shoot", game_lucario_qigong_Shoot, Low)
    .install();
}