use crate::imports::BuildImports::*;

//Attack11
unsafe extern "C" fn game_cloud_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.2, 361, 25, 30, 35, 5.0, 0.0, 10.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.2, 361, 25, 30, 35, 5.0, 0.0, 10.0, 10.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.2, 180, 15, 20, 20, 5.3, 0.0, 10.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 4.2, 361, 15, 20, 20, 5.3, 0.0, 10.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
       
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

//Attack12
unsafe extern "C" fn game_cloud_Attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 25, 30, 35, 4.0, 0.0, 9.5, 3.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 361, 25, 30, 35, 4.0, 0.0, 9.5, 7.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 361, 20, 20, 25, 4.5, 0.0, 9.5, 11.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

//Attack13
unsafe extern "C" fn game_cloud_Attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.5, 30, 108, 0, 75, 5.1, 0.0, 9.0, 18.0, Some(0.0), Some(9.0), Some(8.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

//AttackDash
unsafe extern "C" fn game_cloud_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 19.0, 60, 79, 0, 100, 5.0, 0.0, 14.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 19.0, 65, 79, 0, 100, 4.4, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 19.0, 55, 79, 0, 100, 3.0, 0.0, 8.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 15.0, 35, 66, 0, 100, 4.5, 0.0, 14.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 15.0, 30, 66, 0, 100, 3.8, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 15.0, 30, 66, 0, 100, 3.0, 0.0, 8.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

//AttackS3
unsafe extern "C" fn game_cloud_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 110, 0, 20, 9.0, 0.0, 9.0, 15.8, Some(0.0), Some(9.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackHi3
unsafe extern "C" fn game_cloud_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 14.3, 95, 85, 0, 38, 9.6, 0.0, 12.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 14.3, 95, 85, 0, 38, 9.0, 0.0, 8.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 14.3, 95, 85, 0, 38, 10.0, -1.0, 3.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 14.3, 95, 85, 0, 38, 9.5, 0.0, 21.0, 0.0, Some(0.0), Some(16.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 3, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

//AttackLw3
unsafe extern "C" fn game_cloud_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 11.5, 78, 74, 0, 65, 8.9, 5.0, -1.0, 1.5, Some(-1.5), Some(-1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 10, 75, 0, 65, 7.5, 0.0, 2.5, -2.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legl"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
        MotionModule::set_rate(fighter.module_accessor, 1.1);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

//AttackS4
unsafe extern "C" fn game_cloud_AttackS4(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 30, 25, 30, 8.5, 0.0, 9.0, 10.0, Some(0.0), Some(9.0), Some(6.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_SMASH_01, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 170, 100, 10, 0, 8.5, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(6.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_SMASH_01, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 0, 100, 10, 0, 8.5, 0.0, 9.0, 10.0, Some(0.0), Some(9.0), Some(6.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_SMASH_01, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 180, 100, 10, 0, 8.5, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(6.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_SMASH_01, *ATTACK_REGION_SWORD);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10, 0, 6.0, 0.0, 0, 6.0, 16.0, 1.2, 1.25, 600, false, 2, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 15, 100, 80, 0, 8.5, 0.0, 9.0, 5.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_SMASH_02, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 15, 100, 50, 0, 8.2, 0.0, 9.0, 10.0, Some(0.0), Some(9.0), Some(6.5), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_SMASH_02, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 170, 100, 10, 0, 8.2, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(6.5), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_SMASH_02, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 27.0);
    FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 115, 0, 45, 9.5, 0.0, 9.0, 19.0, Some(0.0), Some(9.0), Some(6.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_SMASH_03, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackHi4
unsafe extern "C" fn game_cloud_AttackHi4(fighter: &mut L2CAgentBase) {
    MotionModule::set_rate(fighter.module_accessor, 1.2048192);
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,  *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        MotionModule::set_rate(fighter.module_accessor, 1.3550135);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 24.0, 83, 100, 0, 40, 4.4, 0.0, 9.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 24.0, 83, 100, 0, 40, 4.4, 0.0, 13.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 23.0, 83, 96, 0, 40, 3.3, 0.0, 2.5, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 23.0, 83, 90, 0, 40, 3.5, 0.0, 16.0, 5.0, Some(0.0), Some(8.0), Some(5.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 19.5, 30, 100, 0, 50, 4.0, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 19.5, 30, 100, 0, 50, 4.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 19.5, 30, 100, 0, 50, 4.0, 0.0, 13.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        AttackModule::clear(fighter.module_accessor, 3, false);
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackLw4
unsafe extern "C" fn game_cloud_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 170, 90, 90, 0, 7.5, 0.0, 4.0, 8.0, Some(0.0), Some(4.0), Some(5.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 155, 90, 85, 0, 7.5, 0.0, 4.0, 13.5, Some(0.0), Some(4.0), Some(5.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 160, 90, 90, 0, 7.5, 0.0, 4.0, 8.0, Some(0.0), Some(4.0), Some(5.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, 150, 90, 85, 0, 7.5, 0.0, 4.0, 13.5, Some(0.0), Some(4.0), Some(5.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 0.9);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 22.0, 35, 90, 0, 35, 8.4, 0.0, 4.8, -14.0, Some(0.0), Some(4.8), Some(-1.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

//AttackAirN
unsafe extern "C" fn game_cloud_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 16.0, 44, 90, 0, 25, 10.8, 3.0, 14.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 16.0, 42, 90, 0, 25, 10.8, 3.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 16.0, 40, 90, 0, 25, 10.8, 3.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 12, 0, 0.0, 0.0, 0, 0.0, 0.0, 1.2, 1.25, 200, false, 2, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirF
unsafe extern "C" fn game_cloud_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 21.0, 270, 100, 0, 40, 8.3, 3.0, 4.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 21.0, 278, 100, 0, 40, 9.3, 3.0, 10.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 21.0, 290, 100, 0, 40, 10.3, 3.0, 15.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);    
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
        AttackModule::clear(fighter.module_accessor, 2, false);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 19.0, 64, 83, 0, 30, 10.0, 0.0, 15.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//AttackAirB
unsafe extern "C" fn game_cloud_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 22.0, 361, 110, 0, 30, 7.0, 0.0, 8.5, -15.0, Some(0.0), Some(8.5), Some(-7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 22.0, 361, 110, 0, 30, 7.0, 0.0, 16.0, -15.0, Some(0.0), Some(16.0), Some(-7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirHi
unsafe extern "C" fn game_cloud_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, /*FSM*/ 0.9);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 19.0, 85, 110, 0, 30, 9.0, 0.0, 4.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 19.0, 85, 110, 0, 30, 9.0, 0.0, 9.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 19.0, 85, 110, 0, 30, 9.0, 0.0, 14.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 480, /*Rehit*/ 30, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 480, /*Rehit*/ 30, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 480, /*Rehit*/ 30, /* Damage*/ 1.5, /*Unk*/ false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 18.0, 80, 100, 0, 32, 8.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 18.0, 80, 100, 0, 32, 8.5, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 18.0, 80, 100, 0, 32, 8.5, 0.0, 14.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 480, /*Rehit*/ 30, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 480, /*Rehit*/ 30, /* Damage*/ 1.5, /*Unk*/ false);
        AttackModule::set_poison_param(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 480, /*Rehit*/ 30, /* Damage*/ 1.5, /*Unk*/ false);
    }
    wait(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//AttackAirLw
unsafe extern "C" fn game_cloud_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 23.0, 270, 100, 0, 10, 11.5, 0.0, 11.0, 0.0, Some(0.0), Some(12.5), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 23.0, 270, 105, 0, 30, 11.5, 0.0, 7.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 21.0, 262, 55, 0, 66, 11.5, 0.0, 9.0, 0.0, Some(0.0), Some(11.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        AttackModule::clear(fighter.module_accessor, 0, false);
        MotionModule::set_rate(fighter.module_accessor, 1.1);
    }
    wait(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Catch
unsafe extern "C" fn game_cloud_Catch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 5.2, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(14.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

//CatchDash
unsafe extern "C" fn game_cloud_CatchDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 5.4, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(17.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

//CatchTurn
unsafe extern "C" fn game_cloud_CatchTurn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 5.3, 0.0, 6.6, -4.0, Some(0.0), Some(6.6), Some(-23.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

//CatchAttack
unsafe extern "C" fn game_cloud_CatchAttack(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.61);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.8, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 8.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KNEE);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//ThrowF
unsafe extern "C" fn game_cloud_ThrowF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.2, 33, 106, 0, 48, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("footl"), 8.2, 45, 120, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        CHECK_FINISH_CAMERA(fighter, 15, 4);
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.3);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.1);
    }
}

//ThrowB
unsafe extern "C" fn game_cloud_ThrowB(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 38, 100, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("footr"), 10.0, 45, 90, 0, 48, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        CHECK_FINISH_CAMERA(fighter, 13, 8);
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.3);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        PostureModule::reverse_lr(fighter.module_accessor);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

//ThrowHi
unsafe extern "C" fn game_cloud_ThrowHi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 80, 93, 0, 78, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("footr"), 3.0, 50, 150, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 50, 150, 0, 80, 4.0, 0.0, 21.5, 7.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        CHECK_FINISH_CAMERA(fighter, 9, 18);
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.3);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

//ThrowLw
unsafe extern "C" fn game_cloud_ThrowLw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 16.0, 100, 78, 0, 35, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        FT_CATCH_STOP(fighter, 10.0, 1.0);
        CHECK_FINISH_CAMERA(fighter, 0, 0);
        FighterCutInManager::set_throw_finish_zoom_rate(FighterCutInManager(), 1.3);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

//CliffAttack
unsafe extern "C" fn game_cloud_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 45, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 17.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-1.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SlipAttack
unsafe extern "C" fn game_cloud_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -18.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(-5.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 19.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(6.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//DownAttackD
unsafe extern "C" fn game_cloud_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -18.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(-6.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 19.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(6.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//DownAttackU
unsafe extern "C" fn game_cloud_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -18.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(-6.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 19.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(6.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialN
unsafe extern "C" fn game_cloud_SpecialN(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 5.5, /*X2*/ Some(0.0), /*Y2*/ Some(6.5), /*Z2*/ Some(4.5), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE, false, 0);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
}

//SpecialAirN
unsafe extern "C" fn game_cloud_SpecialAirN(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 5.5, /*X2*/ Some(0.0), /*Y2*/ Some(6.5), /*Z2*/ Some(4.5), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE, false, 0);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
}

//SpecialN_LB
unsafe extern "C" fn game_cloud_SpecialN_LB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE, false, 0);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x25813802b6));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
}

//SpecialAirN_LB
unsafe extern "C" fn game_cloud_SpecialAirN_LB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE, false, 0);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x25813802b6));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
}

//SpecialS1
unsafe extern "C" fn game_cloud_SpecialS1(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_DETACH_EFFECT);
    }
    if PostureModule::lr(fighter.module_accessor) > 0.0 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 75, 35, 0, 30, 3.5, 0.0, 9.0, 12.5, Some(0.0), Some(6.5), Some(13.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 112, 35, 0, 32, 3.5, 0.0, 17.0, 18.0, Some(0.0), Some(6.5), Some(20.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 4.5, 366, 40, 0, 30, 7.0, 0.0, 19.0, 14.0, Some(0.0), Some(9.0), Some(16.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 75, 35, 0, 30, 3.5, 0.0, 9.0, 10.5, Some(0.0), Some(6.5), Some(10.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 112, 35, 0, 32, 3.5, 0.0, 17.0, 19.2, Some(0.0), Some(6.5), Some(17.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 4.5, 366, 40, 0, 30, 7.0, 0.0, 19.0, 16.0, Some(0.0), Some(9.0), Some(14.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
}

//SpecialAirS1
unsafe extern "C" fn game_cloud_SpecialAirS1(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_DETACH_EFFECT);
    }
    //get_value_float(*SO_VAR_FLOAT_LR);
    if PostureModule::lr(fighter.module_accessor) > 0.0 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 75, 35, 0, 30, 3.5, 0.0, 9.0, 12.5, Some(0.0), Some(6.5), Some(13.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 112, 35, 0, 32, 3.5, 0.0, 17.0, 18.0, Some(0.0), Some(6.5), Some(20.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 4.5, 366, 40, 0, 30, 7.0, 0.0, 19.0, 14.0, Some(0.0), Some(9.0), Some(16.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 75, 35, 0, 30, 3.5, 0.0, 9.0, 10.5, Some(0.0), Some(6.5), Some(10.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 112, 35, 0, 32, 3.5, 0.0, 17.0, 19.2, Some(0.0), Some(6.5), Some(17.0), 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 4.5, 366, 40, 0, 30, 7.0, 0.0, 19.0, 16.0, Some(0.0), Some(9.0), Some(14.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
}

//SpecialS1_LB
unsafe extern "C" fn game_cloud_SpecialS1_LB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if PostureModule::lr(fighter.module_accessor) > 0.0 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 367, 100, 80, 0, 4.025, 0.0, 9.0, 16.0, Some(0.0), Some(6.5), Some(16.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.5, 367, 100, 80, 0, 8.05, 0.0, 19.0, 14.0, Some(0.0), Some(9.0), Some(16.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 367, 100, 80, 0, 4.025, 0.0, 9.0, 10.5, Some(0.0), Some(6.5), Some(10.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.5, 367, 100, 80, 0, 8.05, 0.0, 19.0, 16.0, Some(0.0), Some(9.0), Some(14.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    MotionModule::set_rate(fighter.module_accessor, 1.33333333);
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
}

//SpecialAirS1_LB
unsafe extern "C" fn game_cloud_SpecialAirS1_LB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if PostureModule::lr(fighter.module_accessor) > 0.0 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 367, 100, 80, 0, 4.025, 0.0, 9.0, 16.0, Some(0.0), Some(6.5), Some(16.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.5, 367, 100, 80, 0, 8.05, 0.0, 19.0, 14.0, Some(0.0), Some(9.0), Some(16.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 367, 100, 80, 0, 4.025, 0.0, 9.0, 10.5, Some(0.0), Some(6.5), Some(10.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.5, 367, 100, 80, 0, 8.05, 0.0, 19.0, 16.0, Some(0.0), Some(9.0), Some(14.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    MotionModule::set_rate(fighter.module_accessor, 1.33333333);
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
}

//SpecialS2
unsafe extern "C" fn game_cloud_SpecialS2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_DETACH_EFFECT);
    }
    frame(fighter.lua_state_agent, 2.0);
    if PostureModule::lr(fighter.module_accessor) >0.0 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 95, 25, 0, 33, 7.3, 0.0, 13.5, 17.7, Some(0.0), Some(9.0), Some(15.7), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 60, 25, 0, 43, 3.0, 0.0, 8.5, 9.5, Some(0.0), Some(8.5), Some(9.5), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 95, 25, 0, 33, 7.3, 0.0, 13.5, 15.7, Some(0.0), Some(9.0), Some(17.7), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 60, 25, 0, 43, 3.0, 0.0, 8.5, 9.5, Some(0.0), Some(8.5), Some(9.5), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

//SpecialAirS2
unsafe extern "C" fn game_cloud_SpecialAirS2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_DETACH_EFFECT);
    }
    frame(fighter.lua_state_agent, 2.0);
    if PostureModule::lr(fighter.module_accessor) >0.0 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 95, 25, 0, 33, 7.3, 0.0, 13.5, 17.7, Some(0.0), Some(9.0), Some(15.7), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 60, 25, 0, 43, 3.0, 0.0, 8.5, 9.5, Some(0.0), Some(8.5), Some(9.5), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 95, 25, 0, 33, 7.3, 0.0, 13.5, 15.7, Some(0.0), Some(9.0), Some(17.7), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 60, 25, 0, 43, 3.0, 0.0, 8.5, 9.5, Some(0.0), Some(8.5), Some(9.5), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

//SpecialS2_LB
unsafe extern "C" fn game_cloud_SpecialS2_LB(fighter: &mut L2CAgentBase) {
    MotionModule::set_rate(fighter.module_accessor, 1.3333333);
    frame(fighter.lua_state_agent, 2.0);
    if PostureModule::lr(fighter.module_accessor) > 0.0 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 367, 100, 80, 0, 8.395, 0.0, 13.5, 17.7, Some(0.0), Some(9.0), Some(15.7), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 367, 100, 80, 0, 8.395, 0.0, 13.5, 15.7, Some(0.0), Some(9.0), Some(17.7), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
}

//SpecialAirS2_LB
unsafe extern "C" fn game_cloud_SpecialAirS2_LB(fighter: &mut L2CAgentBase) {
    MotionModule::set_rate(fighter.module_accessor, 1.3333333);
    frame(fighter.lua_state_agent, 2.0);
    if PostureModule::lr(fighter.module_accessor) > 0.0 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 367, 100, 80, 0, 8.395, 0.0, 13.5, 17.7, Some(0.0), Some(9.0), Some(15.7), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 367, 100, 80, 0, 8.395, 0.0, 13.5, 15.7, Some(0.0), Some(9.0), Some(17.7), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
}

//SpecialS3
unsafe extern "C" fn game_cloud_SpecialS3(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,  *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(fighter.lua_state_agent, 2.0);
    if PostureModule::lr(fighter.module_accessor) >0.0 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 280, 100, 20, 0, 8.8, 0.0, 13.5, 16.8, Some(0.0), Some(8.5), Some(16.8), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 70, 100, 20, 0, 8.8, 0.0, 8.5, 16.8, Some(0.0), Some(8.5), Some(16.8), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 95, 100, 37, 0, 7.0, 0.0, 9.0, 11.7, Some(0.0), Some(9.0), Some(11.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 110, 100, 37, 0, 8.0, 0.0, 9.0, 12.7, Some(0.0), Some(9.0), Some(18.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 50, 140, 0, 30, 9.0, 0.0, 17.0, 12.7, Some(0.0), Some(9.0), Some(12.7), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 280, 100, 20, 0, 8.8, 0.0, 13.5, 12.7, Some(0.0), Some(8.5), Some(12.7), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 70, 100, 20, 0, 8.8, 0.0, 8.5, 12.7, Some(0.0), Some(8.5), Some(12.7), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 95, 100, 37, 0, 7.0, 0.0, 9.0, 11.7, Some(0.0), Some(9.0), Some(11.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 110, 100, 37, 0, 8.0, 0.0, 9.0, 18.7, Some(0.0), Some(9.0), Some(12.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 50, 140, 0, 30, 9.0, 0.0, 17.0, 16.8, Some(0.0), Some(9.0), Some(16.8), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialAirS3
unsafe extern "C" fn game_cloud_SpecialAirS3(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,  *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(fighter.lua_state_agent, 2.0);
    if PostureModule::lr(fighter.module_accessor) > 0.0 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 280, 100, 20, 0, 8.8, 0.0, 13.5, 16.8, Some(0.0), Some(8.5), Some(16.8), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 70, 100, 20, 0, 8.8, 0.0, 8.5, 16.8, Some(0.0), Some(8.5), Some(16.8), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 95, 100, 37, 0, 7.0, 0.0, 9.0, 11.7, Some(0.0), Some(9.0), Some(11.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 110, 100, 37, 0, 8.0, 0.0, 9.0, 12.7, Some(0.0), Some(9.0), Some(18.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 50, 140, 0, 30, 9.0, 0.0, 17.0, 12.7, Some(0.0), Some(9.0), Some(12.7), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 280, 100, 20, 0, 8.8, 0.0, 13.5, 12.7, Some(0.0), Some(8.5), Some(12.7), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 70, 100, 20, 0, 8.8, 0.0, 8.5, 12.7, Some(0.0), Some(8.5), Some(12.7), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 95, 100, 37, 0, 7.0, 0.0, 9.0, 11.7, Some(0.0), Some(9.0), Some(11.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 110, 100, 37, 0, 8.0, 0.0, 9.0, 18.7, Some(0.0), Some(9.0), Some(12.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 50, 140, 0, 30, 9.0, 0.0, 17.0, 16.8, Some(0.0), Some(9.0), Some(16.8), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialS3_LB
unsafe extern "C" fn game_cloud_SpecialS3_LB(fighter: &mut L2CAgentBase) {
    MotionModule::set_rate(fighter.module_accessor, 1.3333333);
    frame(fighter.lua_state_agent, 2.0);
    if PostureModule::lr(fighter.module_accessor) >0.0 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 280, 100, 14, 0, 10.12, 0.0, 13.5, 16.8, Some(0.0), Some(8.5), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 70, 100, 21, 0, 10.12, 0.0, 8.5, 16.8, Some(0.0), Some(8.5), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 95, 100, 26, 0, 8.05, 0.0, 9.0, 11.7, Some(0.0), Some(9.0), Some(11.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 110, 100, 26, 0, 9.2, 0.0, 9.0, 12.7, Some(0.0), Some(9.0), Some(18.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 100.0, 50, 147, 0, 30, 10.35, 0.0, 18.0, 12.7, Some(0.0), Some(9.0), Some(12.7), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 280, 100, 14, 0, 10.12, 0.0, 13.5, 12.7, Some(0.0), Some(8.5), Some(12.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 70, 100, 21, 0, 10.12, 0.0, 8.5, 12.7, Some(0.0), Some(8.5), Some(12.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 95, 100, 26, 0, 8.05, 0.0, 9.0, 11.7, Some(0.0), Some(9.0), Some(11.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 110, 100, 26, 0, 9.2, 0.0, 9.0, 18.7, Some(0.0), Some(9.0), Some(12.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 100.0, 50, 147, 0, 30, 10.35, 0.0, 18.0, 16.8, Some(0.0), Some(9.0), Some(16.8), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_LB_SCENE);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x25813802b6));
    }
}

//SpecialAirS3_LB
unsafe extern "C" fn game_cloud_SpecialAirS3_LB(fighter: &mut L2CAgentBase) {
    MotionModule::set_rate(fighter.module_accessor, 1.3333333);
    frame(fighter.lua_state_agent, 2.0);
    if PostureModule::lr(fighter.module_accessor) > 0.0 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 280, 100, 14, 0, 10.12, 0.0, 13.5, 16.8, Some(0.0), Some(8.5), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 70, 100, 21, 0, 10.12, 0.0, 8.5, 16.8, Some(0.0), Some(8.5), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 95, 100, 26, 0, 8.05, 0.0, 9.0, 11.7, Some(0.0), Some(9.0), Some(11.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 110, 100, 26, 0, 9.2, 0.0, 9.0, 12.7, Some(0.0), Some(9.0), Some(18.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 100.0, 50, 147, 0, 30, 10.35, 0.0, 18.0, 12.7, Some(0.0), Some(9.0), Some(12.7), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 280, 100, 14, 0, 10.12, 0.0, 13.5, 12.7, Some(0.0), Some(8.5), Some(12.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 70, 100, 21, 0, 10.12, 0.0, 8.5, 12.7, Some(0.0), Some(8.5), Some(12.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 95, 100, 26, 0, 8.05, 0.0, 9.0, 11.7, Some(0.0), Some(9.0), Some(11.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 110, 100, 26, 0, 9.2, 0.0, 9.0, 18.7, Some(0.0), Some(9.0), Some(12.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 100.0, 50, 147, 0, 30, 10.35, 0.0, 18.0, 16.8, Some(0.0), Some(9.0), Some(16.8), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_LB_SCENE);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x25813802b6));
    }
}

//SpecialHi
unsafe extern "C" fn game_cloud_SpecialHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 40, 100, 80, 0, 8.0, 0.0, 7.0, 3.0, Some(0.0), Some(7.0), Some(3.0), 2.5, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 78, 100, 80, 0, 9.0, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(6.5), 2.5, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 102, 100, 80, 0, 8.5, 0.0, 7.0, 15.0, Some(0.0), Some(7.0), Some(6.5), 2.5, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        SA_SET(fighter, *SITUATION_KIND_AIR);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("haver"), 8.0, 94, 100, 121, 0, 10.0, 0.0, 6.0, 0.0, Some(0.0), Some(6.0), Some(0.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 1, Hash40::new("haver"), 8.0, 96, 100, 121, 0, 10.0, 0.0, 14.5, 0.0, Some(0.0), Some(6.0), Some(0.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 1, Hash40::new("haver"), 8.0, 90, 40, 0, 98, 10.5, 0.0, 14.5, -10.0, Some(0.0), Some(6.0), Some(-10.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("haver"), 8.0, 85, 50, 0, 50, 10.0, 0.0, 6.0, 0.0, Some(0.0), Some(6.0), Some(0.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 1, Hash40::new("haver"), 8.0, 96, 50, 0, 50, 10.5, 0.0, 14.5, 0.0, Some(0.0), Some(6.0), Some(0.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 1, Hash40::new("haver"), 8.0, 90, 40, 0, 70, 10.5, 0.0, 14.5, -12.0, Some(0.0), Some(6.0), Some(-12.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT_RESERVE);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.25);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT);
    }
}

//SpecialAirHi
unsafe extern "C" fn game_cloud_SpecialAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.2, 40, 100, 80, 0, 2.0, 0.0, 7.0, 3.0, Some(0.0), Some(7.0), Some(3.0), 2.5, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.2, 78, 100, 80, 0, 5.0, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(6.5), 2.5, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 11.2, 102, 100, 80, 0, 3.5, 0.0, 7.0, 15.0, Some(0.0), Some(7.0), Some(6.5), 2.5, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        SA_SET(fighter, *SITUATION_KIND_AIR);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("haver"), 8.0, 94, 100, 121, 0, 10.0, 0.0, 6.0, 0.0, Some(0.0), Some(6.0), Some(0.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 1, Hash40::new("haver"), 8.0, 96, 100, 121, 0, 10.0, 0.0, 14.5, 0.0, Some(0.0), Some(6.0), Some(0.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 1, Hash40::new("haver"), 8.0, 90, 40, 0, 98, 10.5, 0.0, 14.5, -10.0, Some(0.0), Some(6.0), Some(-10.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("haver"), 8.0, 85, 50, 0, 50, 10.0, 0.0, 6.0, 0.0, Some(0.0), Some(6.0), Some(0.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 1, Hash40::new("haver"), 8.0, 96, 50, 0, 50, 10.5, 0.0, 14.5, 0.0, Some(0.0), Some(6.0), Some(0.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 1, Hash40::new("haver"), 8.0, 90, 40, 0, 70, 10.5, 0.0, 14.5, -12.0, Some(0.0), Some(6.0), Some(-12.0), 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT_RESERVE);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.25);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT);
    }
}

//SpecialHi_LB
unsafe extern "C" fn game_cloud_SpecialHi_LB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 30.0, 40, 100, 80, 0, 8.0, 0.0, 7.0, 3.0, Some(0.0), Some(7.0), Some(3.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 30.0, 78, 100, 80, 0, 8.0, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(6.5), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 30.0, 102, 100, 80, 0, 9.5, 0.0, 7.0, 15.0, Some(0.0), Some(7.0), Some(6.5), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_LB_SCENE);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        SA_SET(fighter, *SITUATION_KIND_AIR);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_LB_SCENE);
        ATTACK(fighter, 0, 1, Hash40::new("haver"), 16.0, 88, 142, 0, 50, 9.0, 0.0, 6.0, 0.0, Some(0.0), Some(6.0), Some(0.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 1, Hash40::new("haver"), 16.0, 88, 142, 0, 50, 9.0, 0.0, 14.5, 0.0, Some(0.0), Some(6.0), Some(0.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 1, Hash40::new("top"), 16.0, 88, 142, 0, 50, 8.5, 0.0, 7.0, 15.0, Some(0.0), Some(7.0), Some(6.5), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL);
        ATTACK(fighter, 0, 1, Hash40::new("haver"), 16.0, 88, 110, 0, 50, 9.0, 0.0, 6.0, 0.0, Some(0.0), Some(6.0), Some(0.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 1, Hash40::new("haver"), 16.0, 88, 110, 0, 50, 9.0, 0.0, 14.5, 0.0, Some(0.0), Some(6.0), Some(0.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 1, Hash40::new("haver"), 16.0, 90, 40, 0, 98, 8.5, 0.0, 14.5, -10.0, Some(0.0), Some(6.0), Some(-10.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    MotionModule::set_rate(fighter.module_accessor, 0.5);
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT_RESERVE);
    }
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x25813802b6));
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT);
    }
}

//SpecialAirHi_LB
unsafe extern "C" fn game_cloud_SpecialAirHi_LB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 30.0, 40, 100, 80, 0, 8.0, 0.0, 7.0, 3.0, Some(0.0), Some(7.0), Some(3.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 30.0, 78, 100, 80, 0, 8.0, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(6.5), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 30.0, 102, 100, 80, 0, 7.5, 0.0, 7.0, 15.0, Some(0.0), Some(7.0), Some(6.5), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_LB_SCENE);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        SA_SET(fighter, *SITUATION_KIND_AIR);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_LB_SCENE);
        ATTACK(fighter, 0, 1, Hash40::new("haver"), 16.0, 88, 142, 0, 50, 9.0, 0.0, 6.0, 0.0, Some(0.0), Some(6.0), Some(0.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 1, Hash40::new("haver"), 16.0, 88, 142, 0, 50, 9.0, 0.0, 14.5, 0.0, Some(0.0), Some(6.0), Some(0.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 1, Hash40::new("top"), 16.0, 88, 142, 0, 50, 8.5, 0.0, 7.0, 15.0, Some(0.0), Some(7.0), Some(6.5), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL);
        ATTACK(fighter, 0, 1, Hash40::new("haver"), 16.0, 88, 110, 0, 50, 9.0, 0.0, 6.0, 0.0, Some(0.0), Some(6.0), Some(0.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 1, Hash40::new("haver"), 16.0, 88, 110, 0, 50, 9.0, 0.0, 14.5, 0.0, Some(0.0), Some(6.0), Some(0.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 1, Hash40::new("haver"), 16.0, 90, 40, 0, 98, 8.5, 0.0, 14.5, -10.0, Some(0.0), Some(6.0), Some(-10.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    MotionModule::set_rate(fighter.module_accessor, 0.5);
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT_RESERVE);
    }
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x25813802b6));
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT);
    }
}


//SpecialHi2
unsafe extern "C" fn game_cloud_SpecialHi2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 28.0, 262, 100, 0, 20, 11.0, 0.0, 4.6, 9.4, Some(0.0), Some(4.2), Some(-1.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 35, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

//SpecialHi2Landing
unsafe extern "C" fn game_cloud_SpecialHi2Landing(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 19.0, 54, 125, 0, 55, 13.4, 0.0, 8.0, 9.0, Some(0.0), Some(8.0), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialLwLoop
unsafe extern "C" fn game_cloud_SpecialLwLoop(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            DamageModule::heal(fighter.module_accessor, -8.0, 0);
        }
        wait(fighter.lua_state_agent, 12.0);
        wait_loop_clear(fighter);
    } 
}

//SpecialAirLwLoop
unsafe extern "C" fn game_cloud_SpecialAirLwLoop(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            DamageModule::heal(fighter.module_accessor, -8.0, 0);
        }
        wait(fighter.lua_state_agent, 12.0);
        wait_loop_clear(fighter);
    } 
}

//SpecialLw
unsafe extern "C" fn game_cloud_SpecialLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_FLAG_LB_SCENE);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 999.0, 80, 50, 0, 60, 10.6, 0.0, 10.0, 13.0, Some(0.0), Some(10.0), Some(9.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10, 0, 6.5, 0.0, 0, 6.5, 8.0, 1.2, 1.25, 600, false, 2, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 999.0, 80, 50, 0, 60, 8.6, 0.0, 20.0, 9.0, Some(0.0), Some(11.0), Some(10.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 999.0, 80, 50, 0, 60, 7.6, 0.0, 22.0, -2.0, Some(0.0), Some(22.0), Some(-8.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 999.0, 80, 50, 0, 60, 8.6, 0.0, 16.0, -11.5, Some(0.0), Some(10.0), Some(-11.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(fighter.module_accessor, 0, 0.1);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x25813802b6));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_FLAG_LB_SCENE);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 200, 0, 70, 13.8, 0.0, 15.0, 19.0, Some(0.0), Some(15.0), Some(-19.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialAirLw
unsafe extern "C" fn game_cloud_SpecialAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_FLAG_LB_SCENE);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 999.0, 80, 50, 0, 60, 10.6, 0.0, 10.0, 13.0, Some(0.0), Some(10.0), Some(9.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10, 0, 6.5, 0.0, 0, 6.5, 8.0, 1.2, 1.25, 600, false, 2, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 999.0, 80, 50, 0, 60, 8.6, 0.0, 20.0, 9.0, Some(0.0), Some(11.0), Some(10.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 999.0, 80, 50, 0, 60, 7.6, 0.0, 22.0, -2.0, Some(0.0), Some(22.0), Some(-8.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 999.0, 80, 50, 0, 60, 8.6, 0.0, 16.0, -11.5, Some(0.0), Some(10.0), Some(-11.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x25813802b6));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_FLAG_LB_SCENE);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 200, 0, 70, 20.0, 0.0, 15.0, 19.0, Some(0.0), Some(15.0), Some(-19.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AppealHiL
unsafe extern "C" fn game_cloud_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 50.0, /*Angle*/ 45, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bind"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ *ATTACK_REGION_SWORD);
    }
}

//AppealHiR
unsafe extern "C" fn game_cloud_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 50.0, /*Angle*/ 45, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bind"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ *ATTACK_REGION_SWORD);
    }
}

//AppealSL
unsafe extern "C" fn game_cloud_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_MAXIMTOMATO), 0, 0, false, false);
    }
}

//AppealSR
unsafe extern "C" fn game_cloud_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_MAXIMTOMATO), 0, 0, false, false);
    }
}

//AppealLwL
unsafe extern "C" fn game_cloud_AppealLwL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_PASARAN), 0, 0, false, false);
    }
}

//AppealLwR
unsafe extern "C" fn game_cloud_AppealLwR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_PASARAN), 0, 0, false, false);
    }
}

//FinalEnd
unsafe extern "C" fn game_cloud_FinalEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter, );
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        AttackModule::clear_all(fighter.module_accessor);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 250.0, /*Angle*/ 65, /*KBG*/ 113, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 34.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 250.0, /*Angle*/ 65, /*KBG*/ 113, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 36.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 18.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 250.0, /*Angle*/ 65, /*KBG*/ 113, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 38.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 36.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, false, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 1, true, false, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 2, true, false, -1.0, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
} 

//Final2End
unsafe extern "C" fn game_cloud_Final2End(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter, );
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        AttackModule::clear_all(fighter.module_accessor);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 250.0, /*Angle*/ 65, /*KBG*/ 113, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 34.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
        ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 250.0, /*Angle*/ 65, /*KBG*/ 113, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 36.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 18.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 250.0, /*Angle*/ 65, /*KBG*/ 113, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 38.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 36.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, false, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 1, true, false, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 2, true, false, -1.0, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
} 

pub fn install() {
    Agent::new("cloud")
    .game_acmd("game_attack11", game_cloud_Attack11)
    .game_acmd("game_attack12", game_cloud_Attack12)
    .game_acmd("game_attack13", game_cloud_Attack13)
    .game_acmd("game_attackdash", game_cloud_AttackDash)
    .game_acmd("game_attacks3", game_cloud_AttackS3)
    .game_acmd("game_attackhi3", game_cloud_AttackHi3)
    .game_acmd("game_attacklw3", game_cloud_AttackLw3)
    .game_acmd("game_attacks4", game_cloud_AttackS4)
    .game_acmd("game_attackhi4", game_cloud_AttackHi4)
    .game_acmd("game_attacklw4", game_cloud_AttackLw4)
    .game_acmd("game_attackairn", game_cloud_AttackAirN)
    .game_acmd("game_attackairf", game_cloud_AttackAirF)    
    .game_acmd("game_attackairb", game_cloud_AttackAirB)
    .game_acmd("game_attackairhi", game_cloud_AttackAirHi)
    .game_acmd("game_attackairlw", game_cloud_AttackAirLw)
    .game_acmd("game_catch", game_cloud_Catch)
    .game_acmd("game_catchdash", game_cloud_CatchDash)
    .game_acmd("game_catchturn", game_cloud_CatchTurn)
    .game_acmd("game_catchattack", game_cloud_CatchAttack)
    .game_acmd("game_throwf", game_cloud_ThrowF)
    .game_acmd("game_throwb", game_cloud_ThrowB)
    .game_acmd("game_throwhi", game_cloud_ThrowHi)
    .game_acmd("game_throwlw", game_cloud_ThrowLw)
    .game_acmd("game_downattackd", game_cloud_DownAttackD)
    .game_acmd("game_downattacku", game_cloud_DownAttackU)
    .game_acmd("game_cliffattack", game_cloud_CliffAttack)
    .game_acmd("game_slipattack", game_cloud_SlipAttack)
    .game_acmd("game_specialn", game_cloud_SpecialN)
    .game_acmd("game_specialairn", game_cloud_SpecialAirN)
    .game_acmd("game_specialn_lb", game_cloud_SpecialN_LB)
    .game_acmd("game_specialairn_lb", game_cloud_SpecialAirN_LB)
    .game_acmd("game_specials1", game_cloud_SpecialS1)
    .game_acmd("game_specialairs1", game_cloud_SpecialAirS1)
    .game_acmd("game_specials1_lb", game_cloud_SpecialS1_LB)
    .game_acmd("game_specialairs1_lb", game_cloud_SpecialAirS1_LB)
    .game_acmd("game_specials2", game_cloud_SpecialS2)
    .game_acmd("game_specialairs2", game_cloud_SpecialAirS2)
    .game_acmd("game_specials2_lb", game_cloud_SpecialS2_LB)
    .game_acmd("game_specialairs2_lb", game_cloud_SpecialAirS2_LB)
    .game_acmd("game_specials3", game_cloud_SpecialS3)
    .game_acmd("game_specialairs3", game_cloud_SpecialAirS3)
    .game_acmd("game_specials3_lb", game_cloud_SpecialS3_LB)
    .game_acmd("game_specialairs3_lb", game_cloud_SpecialAirS3_LB)
    .game_acmd("game_specialhi", game_cloud_SpecialHi)
    .game_acmd("game_specialairhi", game_cloud_SpecialAirHi)
    .game_acmd("game_specialhi_lb", game_cloud_SpecialHi_LB)
    .game_acmd("game_specialairhi_lb", game_cloud_SpecialAirHi_LB)
    .game_acmd("game_specialhi2", game_cloud_SpecialHi2)
    .game_acmd("game_specialhi2landing", game_cloud_SpecialHi2Landing)
    .game_acmd("game_speciallwloop", game_cloud_SpecialLwLoop)
    .game_acmd("game_specialairlwloop", game_cloud_SpecialAirLwLoop)
    .game_acmd("game_speciallw", game_cloud_SpecialLw)
    .game_acmd("game_specialairlw", game_cloud_SpecialAirLw)
    .game_acmd("game_appealsr", game_cloud_AppealSR)
    .game_acmd("game_appealsl", game_cloud_AppealSL)
    .game_acmd("game_appealhir", game_cloud_AppealHiR)
    .game_acmd("game_appealhil", game_cloud_AppealHiL)
    .game_acmd("game_appeallwr", game_cloud_AppealLwR)
    .game_acmd("game_appeallwl", game_cloud_AppealLwL)
    .game_acmd("game_finalend", game_cloud_FinalEnd)
    .game_acmd("game_final2end", game_cloud_Final2End)
    .install();
}
