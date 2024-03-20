use crate::imports::BuildImports::*;

//Burst
unsafe extern "C" fn game_murabito_clayrocket_Burst(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 50.0, /*Angle*/ 68, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 18.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BOMB);
        AREA_WIND_2ND_RAD_arg9(fighter, 0, 1, 0.02, 1000, 1, 0, 0, 25, 30);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
}

//Fall
unsafe extern "C" fn game_murabito_clayrocket_Fall(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 55, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -2.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_OBJECT);
    }
}

//Fly
unsafe extern "C" fn game_murabito_clayrocket_Fly(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 31.0, /*Angle*/ 60, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -3.5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
    }
}

//LodgedBurst
unsafe extern "C" fn game_murabito_clayrocket_LodgedBurst(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 38.0, /*Angle*/ 68, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 36, /*Size*/ 18.0, /*X*/ 0.0, /*Y*/ 3.1, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BOMB);
        AREA_WIND_2ND_RAD_arg9(fighter, 0, 1, 0.02, 1000, 1, 0, 0, 25, 30);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
}

pub fn install() {
    Agent::new("murabito_clayrocket")
    .game_acmd("game_burst", game_murabito_clayrocket_Burst)
    .game_acmd("game_fall", game_murabito_clayrocket_Fall)
    .game_acmd("game_fly", game_murabito_clayrocket_Fly)
    .game_acmd("game_lodgedburst", game_murabito_clayrocket_LodgedBurst)
    .install();
}