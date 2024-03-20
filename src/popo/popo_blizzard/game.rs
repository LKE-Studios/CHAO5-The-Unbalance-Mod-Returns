use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn game_popo_blizzard_Fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.8, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 15.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(20.0), /*Hitlag*/ 0.5, /*SDI*/ 1.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ice"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FREEZE, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 14.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(18.0), /*Hitlag*/ 0.5, /*SDI*/ 1.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ -2.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ice"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FREEZE, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 12.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(16.0), /*Hitlag*/ 0.5, /*SDI*/ 1.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ -3.2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ice"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FREEZE, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
}

pub fn install() {
    Agent::new("popo_blizzard")
    .game_acmd("game_fly", game_popo_blizzard_Fly)
    .install();
}
