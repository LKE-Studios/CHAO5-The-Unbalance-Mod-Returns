use crate::imports::BuildImports::*;

//Hit
unsafe extern "C" fn game_mewtwo_psychobreak_Hit(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 62.0);
    if is_excute(fighter) {
        SlowModule::set_whole(fighter.module_accessor, 4, 0);
    }
    frame(fighter.lua_state_agent, 67.0);
    if is_excute(fighter) {
        SlowModule::clear_whole(fighter.module_accessor);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 68.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 70.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 800.0, /*Angle*/ 361, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_invalid_invincible(fighter.module_accessor, 0, true);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, false, -1.0, false);
    }
    frame(fighter.lua_state_agent, 71.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_MEWTWO_PSYCHOBREAK_INSTANCE_WORK_ID_FLAG_HIT_ABS);
        AttackModule::clear_all(fighter.module_accessor);
        SlowModule::set_whole(fighter.module_accessor, 3, 0);
    }
    frame(fighter.lua_state_agent, 72.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
        SlowModule::clear_whole(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("mewtwo_psychobreak")
    .game_acmd("game_hit", game_mewtwo_psychobreak_Hit, Low)
    .install();
}