use crate::imports::BuildImports::*;

//BlackHole
unsafe extern "C" fn game_palutena_blackhole_BlackHole(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 500.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 1, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
        AREA_WIND_2ND_RAD_arg9(fighter, 0, 4, -0.1, 1000, 1, 0, 0, 110, 80);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
    }
    wait(fighter.lua_state_agent, 156.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        AreaModule::erase_wind(fighter.module_accessor, 0);
    }
}

pub fn install() {
    Agent::new("palutena_blackhole")
    .game_acmd("game_explode", game_palutena_blackhole_BlackHole)
    .install();
}