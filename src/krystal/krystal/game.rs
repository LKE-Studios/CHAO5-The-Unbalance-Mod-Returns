use crate::imports::BuildImports::*;

//JumpAerialF1
unsafe extern "C" fn game_krystal_JumpAerialF1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_JUMP_FLY_NEXT);
    }
}

//JumpAerialF2
unsafe extern "C" fn game_krystal_JumpAerialF2(fighter: &mut L2CAgentBase) {
    let speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("speed_y_table"), 1);
    if is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.3, z: 0.0});
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_JUMP_FLY_NEXT);
    }
}

//JumpAerialF3
unsafe extern "C" fn game_krystal_JumpAerialF3(fighter: &mut L2CAgentBase) {
    let speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("speed_y_table"), 1);
    if is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_JUMP_FLY_NEXT);
    }
}

//Attack11 
unsafe extern "C" fn game_krystal_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 3.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 4.0, 361, 20, 10, 25, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 4.0, 361, 20, 10, 25, 5.0, 0.0, 4.6, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 4.0, 361, 20, 10, 25, 5.0, 0.0, 9.2, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 8.0, false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Attack12
unsafe extern "C" fn game_krystal_Attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 20, 10, 25, 7.5, 0.0, 7.0, 5.6, Some(0.0), Some(7.3), Some(17.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 9.0, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Attack13
unsafe extern "C" fn game_krystal_Attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 7.0, 36, 108, 0, 60, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.0, 36, 108, 0, 60, 4.5, 0.0, 4.8, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 7.0, 36, 108, 0, 60, 4.8, 0.0, 9.8, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackDash
unsafe extern "C" fn game_krystal_AttackDash(fighter: &mut L2CAgentBase) {
    let rand_effect = [Hash40::new("collision_attr_cutup"), Hash40::new("collision_attr_fire"), Hash40::new("collision_attr_aura"), Hash40::new("collision_attr_purple"), Hash40::new("collision_attr_bind"), Hash40::new("collision_attr_bury"), Hash40::new("collision_attr_elec"), Hash40::new("collision_attr_coin"), Hash40::new("collision_attr_ice"), Hash40::new("collision_attr_pierce"), Hash40::new("collision_attr_flower"), Hash40::new("collision_attr_paralyze"), Hash40::new("collision_attr_magic"), Hash40::new("collision_attr_water"), Hash40::new("collision_attr_sleep"), Hash40::new("collision_attr_curse_poison"), Hash40::new("collision_attr_ink_hit"), Hash40::new("collision_attr_saving")];
    let rng = sv_math::rand(hash40("pitb"), rand_effect.len() as i32);
    frame(fighter.lua_state_agent, 7.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.3, 20, 100, 36, 0, 9.5, 0.0, 9.8, 0.5, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 33.0);
	if is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 34.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 40, 125, 0, 70, 5.0, 0.0, 7.0, 13.0, Some(0.0), Some(6.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, rand_effect[rng as usize], *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 40, 125, 0, 70, 13.5, 0.0, 8.8, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, rand_effect[rng as usize], *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1080, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 0, /*Ink*/ 100.0);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 1080, /*Rehit*/ 40, /* Damage*/ 2.0, /*Unk*/ false);
        AttackModule::set_ink_value(fighter.module_accessor, /*ID*/ 1, /*Ink*/ 100.0);
    }
	wait(fighter.lua_state_agent, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

//AttackS3
unsafe extern "C" fn game_krystal_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("footr"), 14.0, 361, 84, 0, 48, 7.5, 0.0, -4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 14.0, 361, 84, 0, 48, 7.5, 0.0, 1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    	ATTACK(fighter, 2, 0, Hash40::new("top"), 14.0, 361, 84, 0, 48, 8.0, 4.0, 9.0, 3.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 14.0, 361, 84, 0, 48, 8.0, 4.0, 9.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(fighter.lua_state_agent, 1.0);
	if is_excute(fighter) {
		AttackModule::clear(fighter.module_accessor, 3, false);
	}
	wait(fighter.lua_state_agent, 9.0);
	if is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
}

//AttackHi3
unsafe extern "C" fn game_krystal_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 365, /*KBG*/ 100, /*FKB*/ 85, /*BKB*/ 0, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 24.0, /*Z*/ 2.3, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(8.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 3.8, y: 23.5} as *const Vector2f, 10 as u32, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 83, /*KBG*/ 108, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 24.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(22.0), /*Z2*/ Some(3.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    FT_MOTION_RATE(fighter, /*FSM*/ 0.85);
}

//AttackLw3
unsafe extern "C" fn game_krystal_AttackLw3(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.4);
    frame(fighter.lua_state_agent, 6.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 68, 60, 0, 60, 5.8, 0.0, 1.0, -3.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 68, 60, 0, 60, 5.8, 0.0, 1.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 11.0, 68, 60, 0, 60, 5.8, 0.0, 1.0, 5.1, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 12.0, 104, 55, 0, 60, 8.5, 0.0, 1.0, 11.0, Some(0.0), Some(0.0), Some(20.0), 0.3, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.9, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackS4
unsafe extern "C" fn game_krystal_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 23.4, 35, 96, 0, 30, 8.0, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(19.8), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 24.7, 35, 94, 0, 30, 9.5, 0.0, 8.5, 28.2, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 40.0);
	if is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

//AttackHi4
unsafe extern "C" fn game_krystal_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 22.0, 80, 99, 0, 30, 6.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 27.0, 150, 90, 0, 40, 6.1, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackLw4Charge 
unsafe extern "C" fn game_krystal_AttackLw4Charge(fighter: &mut L2CAgentBase) {}

//AttackLw4
unsafe extern "C" fn game_krystal_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 25.0, 55, 88, 0, 40, 10.0, 0.0, 2.0, -6.1, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 25.0, 55, 88, 0, 40, 10.0, 0.0, 2.0, 3.4, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 22.5, 55, 85, 0, 35, 8.5, 0.0, 2.0, -13.3, Some(0.0), Some(2.0), Some(13.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 20.0, 55, 82, 0, 35, 7.0, 0.0, 2.0, -20.8, Some(0.0), Some(2.0), Some(20.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 18.5, 55, 80, 0, 35, 6.0, 0.0, 2.0, -29.6, Some(0.0), Some(2.0), Some(29.6), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 15.0, 0, 100, 0, 45, 50.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_MAGIC);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        JostleModule::set_status(fighter.module_accessor, true);
    }
}

//AttackAirN
unsafe extern "C" fn game_krystal_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 18.0, 361, 100, 0, 25, 9.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 18.0, 361, 100, 0, 25, 9.8, 2.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    FT_MOTION_RATE(fighter, 0.85);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 15.0, 361, 100, 0, 10, 9.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 15.0, 361, 100, 0, 10, 10.4, 2.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    FT_MOTION_RATE(fighter, 1.0);
    wait(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirF 
unsafe extern "C" fn game_krystal_AttackAirF(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, 0.4);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 7.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 25, 0, 70, 9.0, 0.0, 11.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 367, 25, 0, 70, 9.0, 0.0, 10.5, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, 8.0, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 11.0, 40, 100, 0, 50, 9.3, 4.4, -1.7, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("legl"), 11.0, 40, 100, 0, 50, 9.3, 3.2, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirB
unsafe extern "C" fn game_krystal_AttackAirB(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 4.0);
	if is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(fighter.lua_state_agent, 7.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneer"), 18.0, 30, 92, 0, 30, 8.8, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 18.0, 30, 92, 0, 30, 8.2, -3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(fighter.lua_state_agent, 2.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("kneer"), 16.0, 30, 91, 0, 25, 8.6, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 16.0, 30, 91, 0, 25, 8.0, -2.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(fighter.lua_state_agent, 17.0);
	if is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	wait(fighter.lua_state_agent, 10.0);
	if is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

//AttackAirHi
unsafe extern "C" fn game_krystal_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.4, 367, 100, 30, 0, 8.1, 0.0, 12.0, 0.8, Some(0.0), Some(21.5), Some(0.8), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 78, 120, 0, 50, 9.2, 0.0, 12.0, 0.8, Some(0.0), Some(21.5), Some(0.8), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirLw
unsafe extern "C" fn game_krystal_AttackAirLw(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 16.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 77, 91, 0, 30, 8.0, 0.0, 0.0, 0.4, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 20.0, 77, 91, 0, 30, 8.5, 0.0, -7.0, 0.4, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 22.0, 270, 88, 0, 30, 8.3, 0.0, -15.0, 0.4, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Catch
unsafe extern "C" fn game_krystal_Catch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 5.1, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(12.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 4.55, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 2.45, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(14.45), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        game_CaptureCutCommon(fighter);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//CatchDash
unsafe extern "C" fn game_krystal_CatchDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(15.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 4.25, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 2.75, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(16.75), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        game_CaptureCutCommon(fighter);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//CatchTurn
unsafe extern "C" fn game_krystal_CatchTurn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 5.1, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ -4.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(-21.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 4.55, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ -2.45, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(-22.95), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        game_CaptureCutCommon(fighter);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//CatchAttack
unsafe extern "C" fn game_krystal_CatchAttack(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KNEE);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//ThrowF
unsafe extern "C" fn game_krystal_ThrowF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 40, 100, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 30, 100, 100, 20, 5.7, 5.0, 5.0, 9.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        CHECK_FINISH_CAMERA(fighter, 15, 4);
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.3);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//ThrowB
unsafe extern "C" fn game_krystal_ThrowB(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 40, 80, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 130, 0, 30, 11.0, 0.0, 12.0, -14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 12.0, false);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        REVERSE_LR(fighter);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 18, 9);
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.5);
        FighterCutInManager::set_throw_finish_offset(FighterCutInManager(), Vector3f{x: 7.0, y: 3.0, z: 0.0});
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 145, 80, 0, 60, 11.0, 0.0, 12.0, 15.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
}

//ThrowHi
unsafe extern "C" fn game_krystal_ThrowHi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 90, 71, 60, 71, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 30, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        CHECK_FINISH_CAMERA(fighter, 15, 14);
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.3);
        FighterCutInManager::set_throw_finish_offset(FighterCutInManager(), Vector3f{x: 9.0, y: 3.0, z: 0.0});
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no); 
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 50.0, false);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 10.0, 83, 90, 0, 60, 18.0, 0.0, 0.0, 5.0, None, None, None, 0.3, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 1, 0, Hash40::new("throw"), 10.0, 83, 90, 0, 60, 18.0, 0.0, 0.0, -5.0, None, None, None, 0.3, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 3.0, false);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 25.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 40.0);
    FT_MOTION_RATE(fighter, 1.0);
}      

//ThrowLw
unsafe extern "C" fn game_krystal_ThrowLw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FT_LEAVE_NEAR_OTTOTTO(fighter, -2.5, 2.5);
        FT_LEAVE_NEAR_OTTOTTO(fighter, 1.5, 1.5);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 330, 0, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_THROW);
        AttackModule::set_ice_frame_mul(fighter.module_accessor, 0, 2.0, true);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 0, 0, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 5.0, 340, 50, 100, 50, 5.3, 2.0, 3.8, 0.0, Some(-4.0), Some(0.8), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 28.0, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 2.2, 361, 0, 0, 0, 4.3, 4.0, 3.8, 0.0, Some(-4.0), Some(3.8), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 02.2, 361, 0, 0, 0, 4.3, 4.0, 3.8, 0.0, Some(-4.0), Some(3.8), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 2.2, 361, 0, 0, 0, 4.3, 4.0, 3.8, 0.0, Some(-4.0), Some(3.8), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 2.2, 361, 0, 0, 0, 4.3, 4.0, 3.8, 0.0, Some(-4.0), Some(3.8), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 2.2, 300, 0, 0, 0, 6.0, 4.0, 4.2, 0.0, Some(-4.0), Some(4.2), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        CHECK_FINISH_CAMERA(fighter, 2, 0);
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.5);
        FighterCutInManager::set_throw_finish_offset(FighterCutInManager(), Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//CliffAttack
unsafe extern "C" fn game_krystal_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 45, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 11.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SlipAttack
unsafe extern "C" fn game_krystal_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 361, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-4.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PALUTENA);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 361, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 14.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(4.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PALUTENA);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//DownAttackD
unsafe extern "C" fn game_krystal_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 48, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(4.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PALUTENA);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 48, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -16.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-4.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PALUTENA);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//DownAttackU
unsafe extern "C" fn game_krystal_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 48, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 15.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(4.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PALUTENA);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 48, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -16.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-4.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PALUTENA);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialNFireS
unsafe extern "C" fn game_krystal_SpecialNFireS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PITB_GENERATE_ARTICLE_BOWARROW, false, -1);
    }
        frame(fighter.lua_state_agent, 21.0);
        if is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_PITB_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    wait(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialAirNFireS
unsafe extern "C" fn game_krystal_SpecialAirNFireS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PITB_GENERATE_ARTICLE_BOWARROW, false, -1);
    }
        frame(fighter.lua_state_agent, 21.0);
        if is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_PITB_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    wait(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialSStart
unsafe extern "C" fn game_krystal_SpecialSStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 55, 10, 0, 46, 10.3, 0.0, 7.0, 12.0, Some(0.0), Some(7.0), Some(80.6), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.6, 45, 10, 0, 46, 16.6, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(80.6), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        AttackModule::set_ice_frame_mul(fighter.module_accessor, 1, 2.0, true);
    }
    wait(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 55, 100, 0, 70, 10.3, 0.0, 7.0, 12.0, Some(0.0), Some(7.0), Some(80.6), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 40, 100, 0, 70, 20.0, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(80.6), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        AttackModule::set_ice_frame_mul(fighter.module_accessor, 1, 2.5, true);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialSEnd
unsafe extern "C" fn game_krystal_SpecialSEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 0, 100, 60, 0, 16.0, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(80.6), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 0, 100, 60, 0, 16.0, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(80.6), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialAirSStart
unsafe extern "C" fn game_krystal_SpecialAirSStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 55, 10, 0, 46, 5.3, 0.0, 7.0, 12.0, Some(0.0), Some(7.0), Some(40.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.6, 45, 10, 0, 46, 8.6, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(80.6), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        AttackModule::set_ice_frame_mul(fighter.module_accessor, 1, 2.0, true);
    }
    wait(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 55, 100, 0, 70, 5.3, 0.0, 7.0, 12.0, Some(0.0), Some(7.0), Some(40.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 40, 100, 0, 70, 9.6, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(80.6), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        AttackModule::set_ice_frame_mul(fighter.module_accessor, 1, 2.5, true);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialAirSEnd
unsafe extern "C" fn game_krystal_SpecialAirSEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 0, 100, 60, 0, 16.0, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(80.6), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 0, 100, 60, 0, 16.0, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(80.6), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialHi
unsafe extern "C" fn game_krystal_SpecialHi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false)
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 30.0, 270, 63, 0, 0, 10.8, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 29.0, 90, 90, 0, 60, 9.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 29.0, 90, 90, 0, 60, 9.6, 0.0, 4.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 29.5, 90, 90, 0, 60, 9.6, 0.0, 9.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 24.5, 90, 100, 0, 60, 9.6, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 24.5, 90, 100, 0, 60, 9.6, 0.0, 4.6, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 24.5, 90, 100, 0, 60, 9.6, 0.0, 9.2, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//SpecialLwHold
unsafe extern "C" fn game_krystal_SpecialLwHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 0, *FIGHTER_PIT_SHIELD_GROUP_KIND_SPECIAL_LW);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 1, *FIGHTER_PIT_SHIELD_GROUP_KIND_SPECIAL_LW);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 1, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 50, 120, 0, 50, 12.9, 0.0, -0.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        DamageModule::set_damage_mul(fighter.module_accessor, 0.0);
	}
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 50, 100, 30, 0, 12.9, 0.0, -0.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 100.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 50, 100, 30, 0, 12.9, 0.0, -0.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

//SpecialAirLwHold
unsafe extern "C" fn game_krystal_SpecialAirLwHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 0, *FIGHTER_PIT_SHIELD_GROUP_KIND_SPECIAL_LW);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 1, *FIGHTER_PIT_SHIELD_GROUP_KIND_SPECIAL_LW);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 1, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 50, 120, 0, 50, 12.9, 0.0, -0.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        DamageModule::set_damage_mul(fighter.module_accessor, 0.0);
	}
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 50, 100, 30, 0, 12.9, 0.0, -0.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 100.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 50, 100, 30, 0, 12.9, 0.0, -0.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

//AppealHiR
unsafe extern "C" fn game_krystal_AppealHiR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_KILLSWORD), 0, 0, false, false);
    }
}

//AppealHiL 
unsafe extern "C" fn game_krystal_AppealHiL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_KILLSWORD), 0, 0, false, false);
    }
}

//AppealSR
unsafe extern "C" fn game_krystal_AppealSR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_CROSSBOMB), 0, 0, false, false);
    }
}

//AppealSL
unsafe extern "C" fn game_krystal_AppealSL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_CROSSBOMB), 0, 0, false, false);
    }
}

//AppealLwR
unsafe extern "C" fn game_krystal_AppealLwR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_PASARAN), 0, 0, false, false);
    }
}

//AppealLwL
unsafe extern "C" fn game_krystal_AppealLwL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_PASARAN), 0, 0, false, false);
    }
}

//Final
unsafe extern "C" fn game_krystal_Final(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 3.0, 83.0);
        JostleModule::set_status(fighter.module_accessor, false);
        FT_SET_FINAL_FEAR_FACE(fighter, 30);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
        FT_START_CUTIN(fighter);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        SlowModule::set_whole(fighter.module_accessor, 5, 0);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 78, 25, 60, 20.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 52.0, false);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        SlowModule::clear_whole(fighter.module_accessor);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 38, 45, 20, 21.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 48.0, false);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 78, 25, 60, 22.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 48.0, false);
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 43.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 38, 45, 20, 23.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 40.0, false);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 47.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 78, 25, 60, 24.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 38.0, false);
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 51.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 38, 45, 20, 25.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 38.0, false);
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 78, 25, 60, 26.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 30.0, false);
    }
    frame(fighter.lua_state_agent, 57.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 59.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 38, 45, 20, 27.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 28.0, false);
    }
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 63.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 78, 25, 60, 28.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 28.0, false);
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 67.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 38, 45, 20, 29.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 18.0, false);
    }
    frame(fighter.lua_state_agent, 69.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 71.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 78, 25, 60, 30.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 18.0, false);
    }
    frame(fighter.lua_state_agent, 73.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 75.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 38, 45, 20, 31.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 10.0, false);
    }
    frame(fighter.lua_state_agent, 77.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 79.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 38, 105, 20, 34.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 8.0, false);
    }
    frame(fighter.lua_state_agent, 81.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 83.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 333.0, 67, 80, 0, 64, 38.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 87.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 103.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 200.0);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_NORMAL); 
    }
}

//FinalAir
unsafe extern "C" fn game_krystal_FinalAir(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 3.0, 83.0);
        JostleModule::set_status(fighter.module_accessor, false);
        FT_SET_FINAL_FEAR_FACE(fighter, 30);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
        FT_START_CUTIN(fighter);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        SlowModule::set_whole(fighter.module_accessor, 5, 0);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 78, 25, 60, 20.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 48.0, false);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        SlowModule::clear_whole(fighter.module_accessor);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 38, 45, 20, 21.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 48.0, false);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 78, 25, 60, 22.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 48.0, false);
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 43.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 38, 45, 20, 23.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 38.0, false);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 47.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 78, 25, 60, 24.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 38.0, false);
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 51.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 38, 45, 20, 25.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 38.0, false);
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 78, 25, 60, 26.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 28.0, false);
    }
    frame(fighter.lua_state_agent, 57.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 59.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 38, 45, 20, 27.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 28.0, false);
    }
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 63.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 78, 25, 60, 28.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 28.0, false);
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 67.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 38, 45, 20, 29.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 18.0, false);
    }
    frame(fighter.lua_state_agent, 69.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 71.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 78, 25, 60, 30.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 18.0, false);
    }
    frame(fighter.lua_state_agent, 73.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 75.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 38, 45, 20, 31.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 8.0, false);
    }
    frame(fighter.lua_state_agent, 77.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 79.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.5, 10, 38, 105, 20, 34.8, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 8.0, false);
    }
    frame(fighter.lua_state_agent, 81.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 83.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 333.0, 67, 80, 0, 64, 38.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 87.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 103.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 200.0);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_NORMAL); 
    }
}

//FinalCom
unsafe extern "C" fn game_krystal_FinalCom(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 3.0, 83.0);
        JostleModule::set_status(fighter.module_accessor, false);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
        FT_SET_FINAL_FEAR_FACE(fighter, 30);
        FT_START_CUTIN(fighter);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        SlowModule::set_whole(fighter.module_accessor, 5, 0);
    }
}

pub fn install() {
    Agent::new("pitb")
    .game_acmd("game_jumpaerialf1_krystal", game_krystal_JumpAerialF1, Default)
    .game_acmd("game_jumpaerialf2_krystal", game_krystal_JumpAerialF2, Default)
    .game_acmd("game_jumpaerialf3_krystal", game_krystal_JumpAerialF3, Default)
    .game_acmd("game_attack11_krystal", game_krystal_Attack11, Low)
    .game_acmd("game_attack12_krystal", game_krystal_Attack12, Low)
    .game_acmd("game_attack13_krystal", game_krystal_Attack13, Low)
    .game_acmd("game_attackdash_krystal", game_krystal_AttackDash, Low)
    .game_acmd("game_attacks3_krystal", game_krystal_AttackS3, Low)
    .game_acmd("game_attackhi3_krystal", game_krystal_AttackHi3, Low)
    .game_acmd("game_attacklw3_krystal", game_krystal_AttackLw3, Low)
    .game_acmd("game_attacks4_krystal", game_krystal_AttackS4, Low)
    .game_acmd("game_attackhi4_krystal", game_krystal_AttackHi4, Low)
    .game_acmd("game_attacklw4charge_krystal", game_krystal_AttackLw4Charge, Low)
    .game_acmd("game_attacklw4_krystal", game_krystal_AttackLw4, Low)
    .game_acmd("game_attackairn_krystal", game_krystal_AttackAirN, Low)
    .game_acmd("game_attackairf_krystal", game_krystal_AttackAirF, Low)    
    .game_acmd("game_attackairb_krystal", game_krystal_AttackAirB, Low)
    .game_acmd("game_attackairhi_krystal", game_krystal_AttackAirHi, Low)
    .game_acmd("game_attackairlw_krystal", game_krystal_AttackAirLw, Low)
    .game_acmd("game_catch_krystal", game_krystal_Catch, Low)
    .game_acmd("game_catchdash_krystal", game_krystal_CatchDash, Low)
    .game_acmd("game_catchturn_krystal", game_krystal_CatchTurn, Low)
    .game_acmd("game_catchattack_krystal", game_krystal_CatchAttack, Low)
    .game_acmd("game_throwf_krystal", game_krystal_ThrowF, Low)
    .game_acmd("game_throwb_krystal", game_krystal_ThrowB, Low)
    .game_acmd("game_throwhi_krystal", game_krystal_ThrowHi, Low)
    .game_acmd("game_throwlw_krystal", game_krystal_ThrowLw, Low)
    .game_acmd("game_downattackd_krystal", game_krystal_DownAttackD, Low)
    .game_acmd("game_downattacku_krystal", game_krystal_DownAttackU, Low)
    .game_acmd("game_cliffattack_krystal", game_krystal_CliffAttack, Low)
    .game_acmd("game_slipattack_krystal", game_krystal_SlipAttack, Low)
    .game_acmd("game_specialnfires_krystal", game_krystal_SpecialNFireS, Low)  
    .game_acmd("game_specialairnfires_krystal", game_krystal_SpecialAirNFireS, Low)  
    .game_acmd("game_specialsstart_krystal", game_krystal_SpecialSStart, Low)  
    .game_acmd("game_specialsend_krystal", game_krystal_SpecialSEnd, Low)
    .game_acmd("game_specialairsstart_krystal", game_krystal_SpecialAirSStart, Low)  
    .game_acmd("game_specialairsend_krystal", game_krystal_SpecialAirSEnd, Low)
    .game_acmd("game_specialhi_krystal", game_krystal_SpecialHi, Low)
    .game_acmd("game_speciallwhold_krystal", game_krystal_SpecialLwHold, Low)
    .game_acmd("game_specialairlwhold_krystal", game_krystal_SpecialAirLwHold, Low)
    .game_acmd("game_appealsr_krystal", game_krystal_AppealSR, Low)
    .game_acmd("game_appealsl_krystal", game_krystal_AppealSL, Low)
    .game_acmd("game_appealhir_krystal", game_krystal_AppealHiR, Low)
    .game_acmd("game_appealhil_krystal", game_krystal_AppealHiL, Low)
    .game_acmd("game_appeallwr_krystal", game_krystal_AppealLwR, Low)
    .game_acmd("game_appeallwl_krystal", game_krystal_AppealLwL, Low)
    .game_acmd("game_final_krystal", game_krystal_Final, Low)
    .game_acmd("game_finalair_krystal", game_krystal_FinalAir, Low)
    .game_acmd("game_finalcom_krystal", game_krystal_FinalCom, Low)
    .install();
}
