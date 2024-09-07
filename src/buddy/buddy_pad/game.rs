use crate::imports::BuildImports::*;

//Fall
unsafe extern "C" fn game_buddy_pad_Fall(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 50.0, /*Angle*/ 270, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 18, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 2.2, /*Z*/ 3.2, /*X2*/ Some(0.0), /*Y2*/ Some(2.2), /*Z2*/ Some(-3.2), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_BUDDY_PAD_STATUS_WORK_FLAG_FALL_LEANING_TO_THE_LEFT);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *WEAPON_BUDDY_PAD_STATUS_WORK_FLAG_FALL_LEANING_TO_THE_LEFT);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_BUDDY_PAD_STATUS_WORK_FLAG_FALL_LEANING_TO_THE_RIGHT);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *WEAPON_BUDDY_PAD_STATUS_WORK_FLAG_FALL_LEANING_TO_THE_RIGHT);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_BUDDY_PAD_STATUS_WORK_FLAG_FALL_LEANING_TO_THE_LEFT);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *WEAPON_BUDDY_PAD_STATUS_WORK_FLAG_FALL_LEANING_TO_THE_LEFT);
    }
}

pub fn install() {
    Agent::new("buddy_pad")
    .game_acmd("game_fall", game_buddy_pad_Fall, Low)
    .install();
}