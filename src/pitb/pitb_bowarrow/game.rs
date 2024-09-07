use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn game_pitb_bowarrow_Fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 40, /*KBG*/ 87, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.3, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ -10.0, /*Trip*/ 0.0, /*Rehit*/ 30, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_curse_poison"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PALUTENA);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 540, /*Rehit*/ 30, /* Damage*/ 2.0, /*Unk*/ false);        
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("pitb_bowarrow")
    .game_acmd("game_fly", game_pitb_bowarrow_Fly, Low)
    .install();
}