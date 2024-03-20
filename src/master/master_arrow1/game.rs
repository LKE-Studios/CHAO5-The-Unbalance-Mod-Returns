use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn game_master_arrow1_Fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 35, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 39, /*Size*/ 4.6, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting_bowarrow"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 2.0);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("master_arrow1")
    .game_acmd("game_fly", game_master_arrow1_Fly)
    .install();
}