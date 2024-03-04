use crate::imports::BuildImports::*;

//Attack
unsafe extern "C" fn game_koopa_koopag_Attack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("rot"), /*Damage*/ 600.0, /*Angle*/ 361, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 20.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 66.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_no_squat_damage_reaction_mul(fighter.module_accessor, 0, true, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_KOOPA_KOOPAG_INSTANCE_WORK_ID_FLAG_REQUEST_RUMBLE);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("koopa_koopag")
    .game_acmd("game_attack", game_koopa_koopag_Attack)
    .install();
}