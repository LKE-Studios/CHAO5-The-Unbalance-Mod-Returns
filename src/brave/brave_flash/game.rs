use crate::imports::BuildImports::*;

//FlashHit1 [Sizz]
unsafe extern "C" fn game_brave_flash_FlashHit1(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::disable_tip(fighter.module_accessor);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 45, /*KBG*/ 84, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 12.5, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ -6.0, /*X2*/ Some(0.0), /*Y2*/ Some(-3.0), /*Z2*/ Some(8.0), /*Hitlag*/ 1.2, /*SDI*/ 0.4, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_MAGIC);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 4, true, 0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, true, 0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 26.0, /*Angle*/ 45, /*KBG*/ 84, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 12.5, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ -14.0, /*X2*/ Some(0.0), /*Y2*/ Some(-3.0), /*Z2*/ Some(14.0), /*Hitlag*/ 1.2, /*SDI*/ 0.4, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_MAGIC);
    }
}

//FlashHit2 [Sizzle]
unsafe extern "C" fn game_brave_flash_FlashHit2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::disable_tip(fighter.module_accessor);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 40.0, /*Angle*/ 42, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 24, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ -6.0, /*X2*/ Some(0.0), /*Y2*/ Some(-3.0), /*Z2*/ Some(8.0), /*Hitlag*/ 1.3, /*SDI*/ 0.4, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -11, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_MAGIC);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_loopblast"), 14, true, 0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 40.0, /*Angle*/ 42, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 24, /*Size*/ 12.5, /*X*/ 0.0, /*Y*/ -2.5, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.5), /*Z2*/ Some(12.0), /*Hitlag*/ 1.3, /*SDI*/ 0.4, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -11, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_MAGIC);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 40.0, /*Angle*/ 42, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 24, /*Size*/ 13.0, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ -18.0, /*X2*/ Some(0.0), /*Y2*/ Some(-2.0), /*Z2*/ Some(18.0), /*Hitlag*/ 1.3, /*SDI*/ 0.4, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -11, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_MAGIC);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, true, 0);
    }
}

pub fn install() {
    Agent::new("brave_flash")
    .game_acmd("game_flashhit1", game_brave_flash_FlashHit1, Low)
    .game_acmd("game_flashhit2", game_brave_flash_FlashHit2, Low)
    .install();
}