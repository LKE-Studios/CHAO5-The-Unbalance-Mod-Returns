use crate::imports::BuildImports::*;

//Break
unsafe extern "C" fn game_pacman_trampoline_Break(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 300.0, /*Angle*/ 270, /*KBG*/ 1, /*FKB*/ 1, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.3, /*Z*/ 0.5, /*X2*/ Some(0.0), /*Y2*/ Some(0.3), /*Z2*/ Some(-0.5), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_PACMAN_TRAMPOLINE_INSTANCE_WORK_FLAG_ON_JUMP);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("pacman_trampoline")
    .game_acmd("game_break", game_pacman_trampoline_Break, Low)
    .install();
}