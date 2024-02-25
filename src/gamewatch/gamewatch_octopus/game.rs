use crate::imports::BuildImports::*;

//FinalStart
unsafe extern "C" fn game_gamewatch_octopus_FinalStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("roty"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 30.0, 60, 90, 0, 55, 11.0, 0.0, 22.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//FinalAirStart
unsafe extern "C" fn game_gamewatch_octopus_FinalAirStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("roty"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 30.0, 60, 90, 0, 55, 11.0, 0.0, 22.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//FinalAttack
unsafe extern "C" fn game_gamewatch_octopus_FinalAttack(fighter: &mut L2CAgentBase) {
    FT_MOTION_RATE(fighter, 2.5);
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("roty"), 100.0, 15, 100, 150, 10, 15.0, 0.0, 7.0, 6.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("roty"), 100.0, 40, 30, 0, 90, 17.0, 0.0, 6.0, -17.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 1, true, true, -1.0, false);
        CATCH(fighter, 0, Hash40::new("lega"), 8.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_OCTOPUS, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("legb"), 8.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_OCTOPUS, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 2, Hash40::new("legc"), 8.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_OCTOPUS, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 3, Hash40::new("legd"), 8.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_OCTOPUS, *COLLISION_SITUATION_MASK_GA);
        ATTACK_ABS(fighter, *WEAPON_GAMEWATCH_OCTOPUS_ATTACK_ABSOLUTE_KIND_LEG, 0, 30.0, 0, 40, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_LEFT, 1.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, true);
        WorkModule::set_int(fighter.module_accessor, *WEAPON_GAMEWATCH_OCTOPUS_LEG_A_PATTERN_1, *WEAPON_GAMEWATCH_OCTOPUS_STATUS_WORK_INT_CATCH_PATTERN_LEG_A);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 69.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 75.0);
    if is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *WEAPON_GAMEWATCH_OCTOPUS_LEG_A_PATTERN_2, *WEAPON_GAMEWATCH_OCTOPUS_STATUS_WORK_INT_CATCH_PATTERN_LEG_A);
    }
    frame(fighter.lua_state_agent, 90.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 110.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//FinalHold
unsafe extern "C" fn game_gamewatch_octopus_FinalHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("roty"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("roty"), 100.0, 150, 100, 100, 0, 15.0, 0.0, 7.0, 6.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 35, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("roty"), 100.0, 150, 100, 100, 0, 17.0, 0.0, 6.0, -17.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 35, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(fighter.module_accessor, 1, true, true, -1.0, false);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//FinalWait
unsafe extern "C" fn game_gamewatch_octopus_FinalWait(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_GAMEWATCH_OCTOPUS_STATUS_WORK_FLAG_START_ATTACK);
    }
}

pub fn install() {
    Agent::new("gamewatch_octopus")
    .game_acmd("game_finalstart", game_gamewatch_octopus_FinalStart)
    .game_acmd("game_finalairstart", game_gamewatch_octopus_FinalAirStart)
    .game_acmd("game_finalattack", game_gamewatch_octopus_FinalAttack)
    .game_acmd("game_finalhold", game_gamewatch_octopus_FinalHold)
    .game_acmd("game_finalwait", game_gamewatch_octopus_FinalWait)
    .install();
}