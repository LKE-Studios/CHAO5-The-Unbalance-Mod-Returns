use crate::imports::BuildImports::*;

//End
unsafe extern "C" fn game_packun_bosspackun_End(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attack_critical"), 0, false, 0);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 40, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 40.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 20.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(35.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_BODY);
        ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 999.0, /*Angle*/ 40, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 40.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -20.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(-35.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_BODY);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 2, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 3, true, true, -1.0, false);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_PACKUN_BOSSPACKUN_STATUS_WORK_FLAG_END_CAPTURE);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("packun_bosspackun")
    .game_acmd("game_end", game_packun_bosspackun_End, Low)
    .install();
}