use crate::imports::BuildImports::*;

//JumpAerialFront 
unsafe extern "C" fn effect_duckhunt_JumpAerialFront(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//JumpAerialBack
unsafe extern "C" fn effect_duckhunt_JumpAerialBack(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//Attack100End
unsafe extern "C" fn effect_duckhunt_Attack100End(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack100"), false, false);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 7, 5, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 1, 1, 1);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("duckhunt_atk_smoke_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else {
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("duckhunt_atk_smoke_l"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
    }
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 7, 18, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 360, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), 0, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//AttackS3Hi
unsafe extern "C" fn effect_duckhunt_AttackS3Hi(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 9.5, 10, -20, 0, 0, 0.6, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 6.8, 1.2, -20, 0, 0, 1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line_b"), true, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack_line"), true, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//AttackS3
unsafe extern "C" fn effect_duckhunt_AttackS3(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 7, 11, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 7, 1.7, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line_b"), true, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack_line"), true, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//AttackS3Lw
unsafe extern "C" fn effect_duckhunt_AttackS3Lw(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 5, 10, 25, 0, 0, 0.6, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 8, 3, 25, 0, 0, 1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line_b"), true, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack_line"), true, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//AttackHi3 
unsafe extern "C" fn effect_duckhunt_AttackHi3(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 1, 8, -0.5, -90, 0, 0, 0.85, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//AttackLw3
unsafe extern "C" fn effect_duckhunt_AttackLw3(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 0, 7, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 2, 1.5, 0, 0, 0, 0.8, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//AttackAirF
unsafe extern "C" fn effect_duckhunt_AttackAirF(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 7.5, 9, 0, 0, 0, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 7.5, 3, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
        LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack_line"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line_b"), false, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//AttackAirB
unsafe extern "C" fn effect_duckhunt_AttackAirB(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), -1.5, 6.8, -2, 0, 180, 0, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), -1.5, 6.8, 2, 0, 180, 0, 1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
        LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack_line"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line_b"), false, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//AttackAirLw
unsafe extern "C" fn effect_duckhunt_AttackAirLw(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -4, 0, -55, 0, 270, 0.75, true, 0.9);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -4, 0, 45, 0, 270, 0.75, true, 0.9);
        }
    }
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, -4.3, 2, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 360, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckrot"), 0, 0, 0, 90, 0, 0, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 19.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 5.5, 4.5, 7.5, 114, 8.6, -17.7, 1, true);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -5.8, 4.5, 7.5, 114, -7.8, 18, 1, true);
        }
    }
    frame(fighter.lua_state_agent, 20.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -1.35, -11, 0, 0, 0, 0, 1.2, true);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 1.35, -11, 0, 0, 0, 0, 1.2, true);
        }
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc"), true, true);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckrot"), 0, 0, 0, 90, 0, 0, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//SpecialHi
unsafe extern "C" fn effect_duckhunt_SpecialHi(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather_long"), Hash40::new("duckrot"), 0, 4, 0, 20, 0, 0, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//DownBoundU
unsafe extern "C" fn effect_duckhunt_DownBoundU(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        DOWN_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -2, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -2, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//DownBoundD
unsafe extern "C" fn effect_duckhunt_DownBoundD(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        DOWN_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -2, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -2, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//AppealHiL
unsafe extern "C" fn effect_duckhunt_AppealHiL(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 93.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//AppealHiR
unsafe extern "C" fn effect_duckhunt_AppealHiR(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 93.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//AppealSL
unsafe extern "C" fn effect_duckhunt_AppealSL(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 72.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 86.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//AppealSR
unsafe extern "C" fn effect_duckhunt_AppealSR(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 72.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 86.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//FinalAirEnd
unsafe extern "C" fn effect_duckhunt_FinalAirEnd(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_can"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit2"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit3"), true, true);
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather_long"), Hash40::new("duckrot"), 0, 4, 0, 20, 0, 0, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 72.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//FinalEnd
unsafe extern "C" fn effect_duckhunt_FinalEnd(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_can"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit2"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit3"), true, true);
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather_long"), Hash40::new("duckrot"), 0, 4, 0, 20, 0, 0, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 72.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//Win1
unsafe extern "C" fn effect_duckhunt_Win1(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("duckhunt_win1_grass"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, 89, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    wait(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    wait(fighter.lua_state_agent, 25.0);
}

//MagicPotAirStartAerial
unsafe extern "C" fn effect_duckhunt_MagicPotAirStartAerial(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//MagicPotAirEndAerial
unsafe extern "C" fn effect_duckhunt_MagicPotAirEndAerial(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//FFlowerShootAerial
unsafe extern "C" fn effect_duckhunt_FFlowerShootAerial(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

//GenesisAerialLegs
unsafe extern "C" fn effect_duckhunt_GenesisAerialLegs(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

pub fn install() {
    Agent::new("duckhunt")
    .effect_acmd("effect_jumpaerialfront", effect_duckhunt_JumpAerialFront)
    .effect_acmd("effect_jumpaerialback", effect_duckhunt_JumpAerialBack)
    .effect_acmd("effect_attack100end", effect_duckhunt_Attack100End)
    .effect_acmd("effect_attacks3hi", effect_duckhunt_AttackS3Hi)
    .effect_acmd("effect_attacks3", effect_duckhunt_AttackS3)
    .effect_acmd("effect_attacks3lw", effect_duckhunt_AttackS3Lw)
    .effect_acmd("effect_attackhi3", effect_duckhunt_AttackHi3)
    .effect_acmd("effect_attacklw3", effect_duckhunt_AttackLw3)
    .effect_acmd("effect_attackairf", effect_duckhunt_AttackAirF)
    .effect_acmd("effect_attackairb", effect_duckhunt_AttackAirB)
    .effect_acmd("effect_attackairlw", effect_duckhunt_AttackAirLw)
    .effect_acmd("effect_specialhi", effect_duckhunt_SpecialHi)
    .effect_acmd("effect_downboundd", effect_duckhunt_DownBoundD)
    .effect_acmd("effect_downboundu", effect_duckhunt_DownBoundU)
    .effect_acmd("effect_appealhil", effect_duckhunt_AppealHiL)
    .effect_acmd("effect_appealhir", effect_duckhunt_AppealHiR)
    .effect_acmd("effect_appealsl", effect_duckhunt_AppealSL)
    .effect_acmd("effect_appealsr", effect_duckhunt_AppealSR)
    .effect_acmd("effect_finalairend", effect_duckhunt_FinalAirEnd)
    .effect_acmd("effect_finalend", effect_duckhunt_FinalEnd)
    .effect_acmd("effect_win1", effect_duckhunt_Win1)
    .effect_acmd("effect_magicpotairstartaerial", effect_duckhunt_MagicPotAirStartAerial)
    .effect_acmd("effect_magicpotairendaerial", effect_duckhunt_MagicPotAirEndAerial)
    .effect_acmd("effect_fflowershootaerial", effect_duckhunt_GenesisAerialLegs)
    .effect_acmd("effect_genesisaeriallegs", effect_duckhunt_GenesisAerialLegs)
    .install();
}