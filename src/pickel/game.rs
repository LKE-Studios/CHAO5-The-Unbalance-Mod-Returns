use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;
use crate::utils::FIGHTER_CUTIN_MANAGER;

#[acmd_script(//AttackDash
    agent = "pickel", 
    script = "game_attackdash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_dashattack(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(fighter.lua_state_agent, 8.0);
    let material = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if [
        *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, *FIGHTER_PICKEL_MATERIAL_KIND_GOLD, *FIGHTER_PICKEL_MATERIAL_KIND_IRON,
        *FIGHTER_PICKEL_MATERIAL_KIND_STONE, *FIGHTER_PICKEL_MATERIAL_KIND_WOOD
    ].contains(&material) {
        let damage = if material == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            21.0
        }
        else if material == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            16.0
        }
        else if material == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            18.2
        }
        else if material == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            14.5
        }
        else {//DIAMOND
            24.0
        };
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), damage, 60, 93, 0, 40, 5.8, 0.0, 8.8, 4.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), damage, 60, 93, 0, 40, 5.8, 0.0, 8.8, 9.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(fighter.module_accessor, 2.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), damage * 0.8, 60, 96, 0, 42, 5.2, 0.0, 8.8, 3.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), damage * 0.8, 60, 96, 0, 42, 5.2, 0.0, 8.8, 9.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 50, 79, 0, 70, 9.8, 0.0, 8.8, 4.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 50, 85, 0, 70, 9.2, 0.0, 8.8, 3.8, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.2);
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.2);
        }
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
}

#[acmd_script(//AttackS3
    agent = "pickel", 
    script = "game_attacks3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_sidetilt(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(fighter.lua_state_agent, 2.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
        }
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    let material = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if [
        *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, *FIGHTER_PICKEL_MATERIAL_KIND_GOLD, *FIGHTER_PICKEL_MATERIAL_KIND_IRON,
        *FIGHTER_PICKEL_MATERIAL_KIND_STONE, *FIGHTER_PICKEL_MATERIAL_KIND_WOOD
    ].contains(&material) {
        if macros::is_excute(fighter) {
            let damage = if material == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
                12.4
            }
            else if material == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
                3.0
            }
            else if material == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
                11.0
            }
            else if material == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
                8.5
            }
            else {//DIAMOND
                14.0
            };
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), damage, 45, 100, 0, 25, 6.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), damage, 45, 100, 0, 25, 6.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), damage, 45, 100, 0, 25, 6.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), damage, 45, 100, 0, 25, 6.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), damage, 57, 100, 0, 20, 6.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 5, 0, Hash40::new("haver"), damage, 57, 100, 0, 20, 6.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 6, 0, Hash40::new("haver"), damage, 57, 100, 0, 20, 6.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 7, 0, Hash40::new("top"), damage, 57, 134, 0, 20, 6.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 1.25);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 1.25);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 3.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, 3.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 6, 3.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 7, 3.0, false);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 7.0, 60, 99, 0, 50, 5.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 7.0, 60, 99, 0, 50, 5.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.25);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 5.0, false);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 1.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 2.25);
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.25);
        }
    }
    else {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 2.0);
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
}

#[acmd_script(//AttackHi3
    agent = "pickel", 
    script = "game_attackhi3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_uptilt(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_AXE, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(fighter.lua_state_agent, 2.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
    }
    let material = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if [
        *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, *FIGHTER_PICKEL_MATERIAL_KIND_GOLD, *FIGHTER_PICKEL_MATERIAL_KIND_IRON,
        *FIGHTER_PICKEL_MATERIAL_KIND_STONE, *FIGHTER_PICKEL_MATERIAL_KIND_WOOD
    ].contains(&material) {
        let damage = if material == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            15.5
        }
        else if material == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            12.2
        }
        else if material == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            14.0
        }
        else if material == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            11.0
        }
        else {
            17.0
        };
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), damage, 86, 97, 0, 38, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), damage, 86, 97, 0, 38, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(fighter.module_accessor, 5.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.85);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.85);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), damage, 98, 97, 0, 41, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), damage, 98, 97, 0, 41, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.85);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.85);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 9.0, 86, 110, 0, 40, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 9.0, 98, 110, 0, 40, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) != *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 0.5);
        }
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        }
    }
}

#[acmd_script(//AttackLw3
    agent = "pickel", 
    script = "game_attacklw3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_downtilt(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FIRE, false, 0);
    }
}

#[acmd_script(//AttackLw3
    agent = "pickel_fire", 
    script = "game_attacklw3", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_fire(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::disable_tip(fighter.module_accessor);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.4, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 9.2, /*X*/ 0.0, /*Y*/ 2.8, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.4, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 9.2, /*X*/ 0.0, /*Y*/ 2.8, /*Z*/ -2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 14.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 14.0, false);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 280, /*KBG*/ 116, /*FKB*/ 0, /*BKB*/ 42, /*Size*/ 11.5, /*X*/ 0.0, /*Y*/ 2.8, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 280, /*KBG*/ 116, /*FKB*/ 0, /*BKB*/ 42, /*Size*/ 11.5, /*X*/ 0.0, /*Y*/ 2.8, /*Z*/ -2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_OBJECT);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
}

#[acmd_script(//AttackS4
    agent = "pickel", 
    script = "game_attacks4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_sidesmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    MotionModule::set_rate(fighter.module_accessor, 0.714286);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
        MotionModule::set_rate(fighter.module_accessor, 0.5);
    }
    frame(fighter.lua_state_agent, 11.0);
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 30.0, /*Angle*/ 45, /*KBG*/ 89, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.8, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ice"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 30.0, /*Angle*/ 45, /*KBG*/ 89, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.4, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ice"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 30.0, /*Angle*/ 45, /*KBG*/ 89, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.6, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 14.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_ice"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 14.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 27.3, /*Angle*/ 42, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.8, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 27.3, /*Angle*/ 42, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.4, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 27.3, /*Angle*/ 42, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.6, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 14.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 18.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 42, /*KBG*/ 91, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 8.8, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 42, /*KBG*/ 91, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.4, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 25.0, /*Angle*/ 42, /*KBG*/ 91, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.6, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 14.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 14.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 23.0, /*Angle*/ 42, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 8.8, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 23.0, /*Angle*/ 42, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.4, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 23.0, /*Angle*/ 42, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.6, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 14.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 14.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.5, /*Angle*/ 42, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 8.8, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.5, /*Angle*/ 42, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.4, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.5, /*Angle*/ 42, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.6, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 14.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 9.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 42, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.8, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 42, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 8.4, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
        MotionModule::set_rate(fighter.module_accessor, 0.75);
    }
}

#[acmd_script(//AttackHi4
    agent = "pickel", 
    script = "game_attackhi4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_upsmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, false, -1);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 105, /*KBG*/ 100, /*FKB*/ 90, /*BKB*/ 0, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(1.5), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 13.0);
    MotionModule::set_rate(fighter.module_accessor, 0.1);
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(fighter.lua_state_agent, 16.0);
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 80, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 14.0, /*X*/ 0.0, /*Y*/ 27.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 22.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 19.0, /*Angle*/ 52, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 7.5, /*X*/ 4.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 18.0, /*Angle*/ 52, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.5, /*X*/ 4.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 17.0, /*Angle*/ 52, /*KBG*/ 106, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 7.5, /*X*/ 4.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 16.0, /*Angle*/ 52, /*KBG*/ 114, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 7.5, /*X*/ 4.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 15.0, /*Angle*/ 52, /*KBG*/ 122, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 7.5, /*X*/ 4.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
			CancelModule::enable_cancel(fighter.module_accessor);
			WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
    }
}

#[acmd_script(//AttackHi4
    agent = "pickel_meltobject", 
    script = "game_attackhi4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_magma(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 9.75, /*Z*/ 3.25, /*X2*/ Some(0.0), /*Y2*/ Some(9.75), /*Z2*/ Some(-3.25), /*Hitlag*/ 0.2, /*SDI*/ 0.2, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 113, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 3.25, /*Z*/ 3.25, /*X2*/ Some(0.0), /*Y2*/ Some(3.25), /*Z2*/ Some(-3.25), /*Hitlag*/ 0.0, /*SDI*/ 0.2, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_OBJECT);
    }
}

#[acmd_script(//AttackLw4
    agent = "pickel", 
    script = "game_attacklw4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_downsmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_PICKEL_STATUS_ATTACK_FLOAT_ATTACK_LW4_MELT_LR);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_MELT, false, 0);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        WorkModule::set_float(fighter.module_accessor, -1.0, *FIGHTER_PICKEL_STATUS_ATTACK_FLOAT_ATTACK_LW4_MELT_LR);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_MELT, false, 0);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script(//AttackLw4
    agent = "pickel_melt", 
    script = "game_attacklw4", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_lava(fighter: &mut L2CAgentBase) {
    let rand_angle = [27, 361, 0, 30, 45, 90, 135, 180, 362, 270, 310, 15, 345];
    let rng = smash::app::sv_math::rand(hash40("pickel"), rand_angle.len() as i32);
    if macros::is_excute(fighter) {
        AttackModule::enable_safe_pos(fighter.module_accessor);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.6, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 55, /*BKB*/ 0, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 2, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.6, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 55, /*BKB*/ 0, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 2, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_OBJECT);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ rand_angle[rng as usize], /*KBG*/ 99, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 7.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(-9.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ rand_angle[rng as usize], /*KBG*/ 99, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 1.5, /*Z*/ 14.5, /*X2*/ Some(0.0), /*Y2*/ Some(1.5), /*Z2*/ Some(-10.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ rand_angle[rng as usize], /*KBG*/ 99, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(-9.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ rand_angle[rng as usize], /*KBG*/ 99, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 1.5, /*Z*/ 17.5, /*X2*/ Some(0.0), /*Y2*/ Some(1.5), /*Z2*/ Some(-10.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_OBJECT);
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
}

#[acmd_script(//AttackAirN
    agent = "pickel", 
    script = "game_attackairn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_nair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(fighter.lua_state_agent, 2.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
        }
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    let material = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if [
        *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, *FIGHTER_PICKEL_MATERIAL_KIND_GOLD, *FIGHTER_PICKEL_MATERIAL_KIND_IRON,
        *FIGHTER_PICKEL_MATERIAL_KIND_STONE, *FIGHTER_PICKEL_MATERIAL_KIND_WOOD
    ].contains(&material) {
        if macros::is_excute(fighter) {
            let damage = if material == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
                12.4
            }
            else if material == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
                3.0
            }
            else if material == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
                11.0
            }
            else if material == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
                8.5
            }
            else {//DIAMOND
                14.0
            };
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), damage, 45, 100, 0, 25, 6.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), damage, 45, 100, 0, 25, 6.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), damage, 45, 100, 0, 25, 6.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), damage, 45, 100, 0, 25, 6.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), damage, 57, 100, 0, 20, 6.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 5, 0, Hash40::new("haver"), damage, 57, 100, 0, 20, 6.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 6, 0, Hash40::new("haver"), damage, 57, 100, 0, 20, 6.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 7, 0, Hash40::new("top"), damage, 57, 134, 0, 20, 6.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 1.25);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 1.25);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 3.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, 3.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 6, 3.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 7, 3.0, false);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 7.0, 60, 99, 0, 50, 5.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 7.0, 60, 99, 0, 50, 5.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.25);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 5.0, false);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 1.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 2.25);
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.25);
        }
    }
    else {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 2.0);
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
}

#[acmd_script(//AttackAirF 
    agent = "pickel", 
    script = "game_attackairf", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_fair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 8.0);
    let material = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if [
        *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, *FIGHTER_PICKEL_MATERIAL_KIND_GOLD, *FIGHTER_PICKEL_MATERIAL_KIND_IRON,
        *FIGHTER_PICKEL_MATERIAL_KIND_STONE, *FIGHTER_PICKEL_MATERIAL_KIND_WOOD
    ].contains(&material) {
        let damage = if material == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            25.5
        }
        else if material == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            22.0
        }
        else if material == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            24.0
        }
        else if  material == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            20.0
        }
        else {
            28.0
        };
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), damage, 280, 89, 0, 40, 6.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), damage, 280, 89, 0, 40, 6.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), damage, 280, 89, 0, 40, 8.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), damage, 280, 89, 0, 40, 8.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(fighter.module_accessor, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), damage, 275, 100, 0, 30, 7.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, 2, false);
            macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), damage, 275, 100, 0, 30, 7.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 16.0, 325, 85, 0, 30, 6.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 16.0, 325, 85, 0, 30, 6.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        wait(fighter.lua_state_agent, 2.0);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        macros::FT_MOTION_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}                               

#[acmd_script(//AttackAirB
    agent = "pickel", 
    script = "game_attackairb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_bair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 12.0);
    let material = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if [
        *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, *FIGHTER_PICKEL_MATERIAL_KIND_GOLD, *FIGHTER_PICKEL_MATERIAL_KIND_IRON,
        *FIGHTER_PICKEL_MATERIAL_KIND_STONE, *FIGHTER_PICKEL_MATERIAL_KIND_WOOD
    ].contains(&material) {
        if macros::is_excute(fighter) {
            let damage = if material == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
                25.0
            }
            else if material == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
                21.0
            }
            else if material == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
                23.0
            }
            else if material == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
                18.5
            }
            else {
                27.0
            };
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), damage, 50, 84, 0, 40, 8.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), damage, 50, 84, 0, 40, 8.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), damage, 361, 84, 0, 40, 9.2, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), damage, 361, 84, 0, 40, 9.2, 0.0, 4.4, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(fighter.module_accessor, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 15.0, 30, 97, 0, 30, 8.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 15.0, 30, 97, 0, 30, 8.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        macros::FT_MOTION_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script(//AttackAirHi
    agent = "pickel", 
    script = "game_attackairhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_uair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_AXE, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_HI_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 3.0);
    let material = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if [
        *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, *FIGHTER_PICKEL_MATERIAL_KIND_GOLD, *FIGHTER_PICKEL_MATERIAL_KIND_IRON,
        *FIGHTER_PICKEL_MATERIAL_KIND_STONE, *FIGHTER_PICKEL_MATERIAL_KIND_WOOD
    ].contains(&material) {
        let damage = if material == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            11.5
        }
        else if material == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            9.4
        }
        else if material == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            10.3
        }
        else if material == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            8.5
        }
        else {
            13.0
        };
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), damage, 79, 100, 0, 35, 8.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), damage, 79, 100, 0, 35, 9.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(fighter.module_accessor, 5.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), damage, 79, 100, 0, 35, 8.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), damage, 79, 100, 0, 35, 9.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.0, 71, 115, 0, 30, 6.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.0, 71, 115, 0, 30, 6.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_HI_ENABLE_LANDING);
    }
}

#[acmd_script(//AttackAirLw
    agent = "pickel", 
    script = "game_attackairlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_dair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_STATUS_ATTACK_FLAG_FORGE_GENERATE_ENABLE);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script(//AttackAirLw2
    agent = "pickel", 
    script = "game_attackairlw2", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_dair2(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script(//FallAttack
    agent = "pickel_forge", 
    script = "game_fallattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_anvil1(fighter: &mut L2CAgentBase) {
    let rand_angle = [270, 90];
    let rng = smash::app::sv_math::rand(hash40("pickel"), rand_angle.len() as i32);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 50.0, /*Angle*/ rand_angle[rng as usize], /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 9.4, /*X*/ 0.0, /*Y*/ 4.6, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.8), /*Z2*/ Some(0.0), /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 50.0, /*Angle*/ rand_angle[rng as usize], /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 9.4, /*X*/ 0.0, /*Y*/ 4.6, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.8), /*Z2*/ Some(0.0), /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_saving"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        AttackModule::set_attack_level(fighter.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
    }
}

#[acmd_script(//FallAttackRide
    agent = "pickel_forge", 
    script = "game_fallattackride", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_anvil2(fighter: &mut L2CAgentBase) {
    let rand_angle = [270, 90];
    let rng = smash::app::sv_math::rand(hash40("pickel"), rand_angle.len() as i32);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 50.0, /*Angle*/ rand_angle[rng as usize], /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 18, /*Size*/ 10.4, /*X*/ 0.0, /*Y*/ 4.6, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.8), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 35, /*Trip*/ 0.0, /*Rehit*/ 80, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 50.0, /*Angle*/ rand_angle[rng as usize], /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 18, /*Size*/ 10.4, /*X*/ 0.0, /*Y*/ 4.6, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.8), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 35, /*Trip*/ 0.0, /*Rehit*/ 80, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_saving"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
}

#[acmd_script(//Wait
    agent = "pickel_forge", 
    script = "game_wait", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_anvil3(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 40.0, /*Angle*/ 270, /*KBG*/ 72, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 10.4, /*X*/ 0.0, /*Y*/ 4.6, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.8), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_saving"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.0, /*Angle*/ 361, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 10, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_search"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_77_kanatoko_fall"), 0, false, 0);
    }
}

#[acmd_script(//Catch
    agent = "pickel", 
    script = "game_catch", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_grab(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("catch"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_STATUS_CATCH_FLAG_SHOOT);
        GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("rod"), Hash40::new("rod_cast"), ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("throw"), /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ -4.0, /*Z*/ -2.5, /*X2*/ Some(0.0), /*Y2*/ Some(-4.8), /*Z2*/ Some(-2.5), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 3.2, /*X2*/ Some(0.0), /*Y2*/ Some(7.0), /*Z2*/ Some(13.8), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.0), /*Z2*/ Some(12.0), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        macros::game_CaptureCutCommon(fighter);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("throw"), /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ -5.0, /*Z*/ -2.5, /*X2*/ Some(0.0), /*Y2*/ Some(-4.0), /*Z2*/ Some(-2.5), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("throw"), /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ -2.5, /*X2*/ Some(0.0), /*Y2*/ Some(-0.5), /*Z2*/ Some(-2.5), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("throw"), /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ -2.5, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-2.5), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
    }
    frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    }
}

#[acmd_script(//CatchDash
    agent = "pickel", 
    script = "game_catchdash", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_dashgrab(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("catch_dash"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_STATUS_CATCH_FLAG_SHOOT);
        GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("rod"), Hash40::new("rod_cast"), ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("throw"), /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ -5.0, /*Z*/ -2.5, /*X2*/ Some(0.0), /*Y2*/ Some(-4.8), /*Z2*/ Some(-2.5), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 3.2, /*X2*/ Some(0.0), /*Y2*/ Some(7.0), /*Z2*/ Some(15.8), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.0), /*Z2*/ Some(14.0), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        macros::game_CaptureCutCommon(fighter);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("throw"), /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ -5.0, /*Z*/ -2.5, /*X2*/ Some(0.0), /*Y2*/ Some(-4.0), /*Z2*/ Some(-2.5), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("throw"), /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ -2.5, /*X2*/ Some(0.0), /*Y2*/ Some(-0.5), /*Z2*/ Some(-2.5), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("throw"), /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ -2.5, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-2.5), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    }
}

#[acmd_script(//CatchTurn
    agent = "pickel", 
    script = "game_catchturn", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_pivotgrab(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("catch_turn"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_STATUS_CATCH_FLAG_SHOOT);
        GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("rod"), Hash40::new("rod_cast"), ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("throw"), /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ -5.0, /*Z*/ 2.5, /*X2*/ Some(0.0), /*Y2*/ Some(-4.8), /*Z2*/ Some(2.5), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ -3.2, /*X2*/ Some(0.0), /*Y2*/ Some(7.0), /*Z2*/ Some(-10.8), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, /*ID*/ 2, /*Bone*/ Hash40::new("top"), /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ -5.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.0), /*Z2*/ Some(-9.0), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
        macros::game_CaptureCutCommon(fighter);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("throw"), /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ -5.0, /*Z*/ 2.5, /*X2*/ Some(0.0), /*Y2*/ Some(-4.0), /*Z2*/ Some(2.5), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("throw"), /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ 2.5, /*X2*/ Some(0.0), /*Y2*/ Some(-0.5), /*Z2*/ Some(2.5), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("throw"), /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ 2.5, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(2.5), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    }
}

#[acmd_script(//CatchAttack
    agent = "pickel", 
    script = "game_catchattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_pummel(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.1);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.2, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//ThrowF
    agent = "pickel", 
    script = "game_throwf", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_throwf(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 14.4, /*Angle*/ 10, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 80, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_PUSHOBJECT, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_PUSHOBJECT, Hash40::new("throw_f"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 17, 4.8);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.7);
        lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, Vector3f{x:6.0,y:4.0,z:0.0});
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_PUSHOBJECT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script(//ThrowB
    agent = "pickel", 
    script = "game_throwb", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_throwb(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("throw_b"), false, -1.0);
        macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 2, /*Damage*/ 18.0, /*Angle*/ 43, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 70, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter, );
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 21, 11.8);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.8);
        lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, Vector3f{x:10.0,y:3.0,z:0.0});
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

#[acmd_script(//ThrowHi
    agent = "pickel", 
    script = "game_throwhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_throwup(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 15.5, /*Angle*/ 90, /*KBG*/ 102, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_PUSHOBJECT, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_PUSHOBJECT, Hash40::new("throw_hi"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 12.2, 12.6);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.5);
        lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, Vector3f{x:7.0,y:2.0,z:0.0});
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(fighter.lua_state_agent, 47.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_PUSHOBJECT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}      

#[acmd_script(//ThrowLw
    agent = "pickel", 
    script = "game_throwlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_throwdown(fighter: &mut L2CAgentBase) {
    let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
    let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
    let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 16.0, /*Angle*/ 280, /*KBG*/ 91, /*FKB*/ 0, /*BKB*/ 45, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_STATUS_THROW_FLAG_FORGE_GENERATE_ENABLE);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 9, 2);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.5);
        lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, Vector3f{x:7.0,y:0.0,z:0.0});
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_THROW_FLAG_IS_GENERATE_FORGE) {
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
	    }
        frame(fighter.lua_state_agent, 23.0);
        if macros::is_excute(fighter) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        }
        frame(fighter.lua_state_agent, 28.0);
        if macros::is_excute(fighter) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 38.0);
        if macros::is_excute(fighter) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    else {
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 12.0, /*Angle*/ 280, /*KBG*/ 91, /*FKB*/ 0, /*BKB*/ 45, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
	    }
        sv_animcmd::frame(fighter.lua_state_agent, 28.0);
        if macros::is_excute(fighter) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

#[acmd_script(//ThrowLw
    agent = "pickel_forge", 
    script = "game_throwlw", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_anvil4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("trans"), /*Damage*/ 0.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 9.8, /*X*/ 0.0, /*Y*/ 3.8, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 10, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_search"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_OBJECT);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("trans"), /*Damage*/ 40.0, /*Angle*/ 285, /*KBG*/ 67, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 9.8, /*X*/ 0.0, /*Y*/ 3.8, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_ignore_capture_cut(fighter.module_accessor, 0, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("trans"), /*Damage*/ 0.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 9.8, /*X*/ 0.0, /*Y*/ 3.8, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 10, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_search"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_OBJECT);
    }
}

#[acmd_script(//CliffAttack
    agent = "pickel", 
    script = "game_cliffattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_cliffattack(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(fighter.lua_state_agent, 24.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 22.0, /*Angle*/ 45, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(2.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 45, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(2.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.5, /*Angle*/ 45, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(2.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 45, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(2.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 45, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(2.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 45, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(2.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//SlipAttack
    agent = "pickel", 
    script = "game_slipattack", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_slipattack(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(fighter.lua_state_agent, 19.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(-3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(-3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(-3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(-3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(-3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -6.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(-3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 9.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 9.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 9.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 9.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 9.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 6.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//DownAttackU
    agent = "pickel", 
    script = "game_downattacku", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_downattacku(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(fighter.lua_state_agent, 16.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 48, /*KBG*/ 48, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -7.5, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 7.5, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//DownAttackD
    agent = "pickel", 
    script = "game_downattackd", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_downattackd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(fighter.lua_state_agent, 16.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 48, /*KBG*/ 48, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -7.5, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 21.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 19.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 17.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
            WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 48, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 7.5, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(//SpecialSRide, SpecialSDrive
    agent = "pickel", 
    scripts = ["game_specialsride", "game_specialsdrive"],
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_sideb(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.0, 0, 10.0, 0.0, 0, 4.0, 8.0, 1.2, 1.2, 2000, false, 4.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
		macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.0, /*Angle*/ 350, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 10, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(4.5), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA_d, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
	}
}

#[acmd_script(//SpecialAirSRide
    agent = "pickel", 
    script = "game_specialairsride", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_sidebair(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.0, 0, 10.0, 0.0, 0, 4.0, 8.0, 1.2, 1.2, 2000, false, 4.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
	}
}

#[acmd_script(//SpecialSDriveEmptyPartial
    agent = "pickel_trolley", 
    script = "game_specialsdriveemptypartial", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_minecart1(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_NO_ATTACK_HIT_MOTION);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 60, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(6.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 30, /*Trip*/ 0.0, /*Rehit*/ 60, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_ignore_ground_shield(fighter.module_accessor, 0, true);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 60, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -6.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-6.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 30, /*Trip*/ 0.0, /*Rehit*/ 60, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_ignore_ground_shield(fighter.module_accessor, 1, true);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 60, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(-6.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 30, /*Trip*/ 0.0, /*Rehit*/ 60, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_ignore_ground_shield(fighter.module_accessor, 2, true);
        macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 60, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 3.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 30, /*Trip*/ 0.0, /*Rehit*/ 40, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_IIE, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 60, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 3.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 40, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        AttackModule::set_down_only(fighter.module_accessor, 4, true);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 6.5, 0.0, 3.0, 1.5, Some(0.0), Some(3.0), Some(-1.5), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 60, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 40, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        macros::SEARCH(fighter, 1, 0, Hash40::new("top"), 5.0, 0.0, 5.0, 0.0, Some(0.0), Some(5.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NI, 1, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_NO_ATTACK_HIT_MOTION);
    }
}

#[acmd_script(//SpecialSDrivePartial
    agent = "pickel_trolley", 
    script = "game_specialsdrivepartial", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_minecart2(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 22.0, /*Angle*/ 43, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 30, /*Trip*/ 0.0, /*Rehit*/ 40, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
        macros::SEARCH(fighter, 1, 0, Hash40::new("top"), 5.0, 0.0, 5.0, 0.0, Some(0.0), Some(5.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NI, 1, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_NO_ATTACK_HIT_MOTION);
    }
}

#[acmd_script(//SpecialHiStart
    agent = "pickel", 
    script = "game_specialhistart", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_upbstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_STATUS_SPECIAL_HI_FLAG_WAIT_START);
    }
}

#[acmd_script(//SpecialAirHiStart
    agent = "pickel", 
    script = "game_specialairhistart", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_upbairstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
}

#[acmd_script(//SpecialAirHi
    agent = "pickel", 
    script = "game_specialairhi", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_upbair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("head"), /*Damage*/ 25.0, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_HEAD);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("head"), /*Damage*/ 25.0, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.7, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_HEAD);
    }
    frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_STATUS_SPECIAL_HI_FLAG_ENABLE_CANCEL);
    }
}

#[acmd_script(//AppealHiR
    agent = "pickel", 
    script = "game_appealhir", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_uptauntr(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BEETLE), 0, 0, false, false);
    }
}

#[acmd_script(//AppealHiL
    agent = "pickel", 
    script = "game_appealhil", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_uptauntl(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BEETLE), 0, 0, false, false);
    }
}

#[acmd_script(//AppealSR
    agent = "pickel", 
    script = "game_appealsr", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_sidetauntr(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        DamageModule::heal(fighter.module_accessor, -20.0, 0);
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BOMBER), 0, 0, false, false);
    }
}

#[acmd_script(//AppealSL
    agent = "pickel", 
    script = "game_appealsl", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_sidetauntl(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        DamageModule::heal(fighter.module_accessor, -20.0, 0);
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BOMBER), 0, 0, false, false);
    }
}

#[acmd_script(//AppealLwR
    agent = "pickel", 
    script = "game_appeallwr", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_downtauntr(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_MAGICBALL), 0, 0, false, false);
    }
}

#[acmd_script(//AppealLwL
    agent = "pickel", 
    script = "game_appeallwl", 
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_downtauntl(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_MAGICBALL), 0, 0, false, false);
    }
}

#[acmd_script(//Final
    agent = "pickel", 
    scripts = ["game_finalend", "game_finalairend"],
    category = ACMD_GAME, 
    low_priority )]
unsafe fn pickel_final(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_PICKEL_ATTACK_ABSOLUTE_KIND_FINAL, /*ID*/ 0, /*Damage*/ 2009.0, /*Angle*/ 35, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_BOMB, /*Type*/ *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2009.0, /*Angle*/ 0, /*KBG*/ 50, /*FKB*/ 5, /*BKB*/ 0, /*Size*/ 5000.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ f32::NAN, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_death"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_LL, /*SFXType*/ *COLLISION_SOUND_ATTR_BOMB, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, false, -1.0, true);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_VISUAL_SCENE_FLAG_ABS_SET);
        macros::CAM_ZOOM_OUT(fighter, );
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_PICKEL_STATUS_FINAL_FLAG_END_FINAL);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_VISUAL_SCENE_FLAG_END_EXIT);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        pickel_dashattack,
        pickel_sidetilt,
        pickel_uptilt,
        pickel_downtilt,
        pickel_fire,
        pickel_sidesmash,
        pickel_upsmash,
        pickel_downsmash,
        pickel_lava,
        pickel_magma,
        pickel_nair,
        pickel_fair,
        pickel_bair,
        pickel_uair,
        pickel_dair,
        pickel_dair2,
        pickel_anvil1,
        pickel_anvil2,
        pickel_anvil3,
        pickel_anvil4,
        pickel_grab,
        pickel_dashgrab,
        pickel_pivotgrab,
        pickel_pummel,
        pickel_throwf,
        pickel_throwb,
        pickel_throwup,
        pickel_throwdown,
        pickel_cliffattack,
        pickel_slipattack,
        pickel_downattacku,
        pickel_downattackd,
        pickel_sideb,
        pickel_sidebair,
        pickel_minecart1,
        pickel_minecart2,
        pickel_upbstart,
        pickel_upbairstart,
        pickel_upbair,
        pickel_uptauntr,
        pickel_uptauntl,
        pickel_sidetauntr,
        pickel_sidetauntl,
        pickel_downtauntr,
        pickel_downtauntl,
        pickel_final
    );
}
