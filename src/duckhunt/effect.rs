use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;
use crate::helper;

#[acmd_script(//JumpAerialFront 
    agent = "duckhunt", 
    script = "effect_jumpaerialfront", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_jumpaerialfront(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//JumpAerialBack
    agent = "duckhunt", 
    script = "effect_jumpaerialback", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_jumpaerialback(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//Attack100End
    agent = "duckhunt", 
    script = "effect_attack100end", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_attack100end(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack100"), false, false);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 7, 5, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 1, 1, 1);
        macros::LAST_EFFECT_SET_RATE(fighter, 2);
    }
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("duckhunt_atk_smoke_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("duckhunt_atk_smoke_l"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
    }
    if macros::is_excute(fighter) {
        macros::EFFECT_FLIP(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 7, 18, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 360, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), 0, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//AttackS3Hi
    agent = "duckhunt", 
    script = "effect_attacks3hi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_attacks3hi(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 9.5, 10, -20, 0, 0, 0.6, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 6.8, 1.2, -20, 0, 0, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line_b"), true, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack_line"), true, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//AttackS3
    agent = "duckhunt", 
    script = "effect_attacks3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_attacks3(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 7, 11, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 7, 1.7, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line_b"), true, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack_line"), true, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//AttackS3Lw
    agent = "duckhunt", 
    script = "effect_attacks3lw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_attacks3lw(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 5, 10, 25, 0, 0, 0.6, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 8, 3, 25, 0, 0, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line_b"), true, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack_line"), true, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//AttackHi3 
    agent = "duckhunt", 
    script = "effect_attackhi3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_attackhi3(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 1, 8, -0.5, -90, 0, 0, 0.85, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//AttackLw3
    agent = "duckhunt", 
    script = "effect_attacklw3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_attacklw3(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 0, 7, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 2, 1.5, 0, 0, 0, 0.8, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//AttackAirF
    agent = "duckhunt",
    script = "effect_attackairf", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_attackairf(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 7.5, 9, 0, 0, 0, 0.9, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 7.5, 3, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
        macros::LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack_line"), false, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line_b"), false, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//AttackAirB
    agent = "duckhunt", 
    script = "effect_attackairb", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_attackairb(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), -1.5, 6.8, -2, 0, 180, 0, 0.9, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), -1.5, 6.8, 2, 0, 180, 0, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
        macros::LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack_line"), false, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line_b"), false, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//AttackAirLw
    agent = "duckhunt", 
    script = "effect_attackairlw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_attackairlw(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -4, 0, -55, 0, 270, 0.75, true, 0.9);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -4, 0, 45, 0, 270, 0.75, true, 0.9);
        }
    }
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, -4.3, 2, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 360, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckrot"), 0, 0, 0, 90, 0, 0, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 19.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 5.5, 4.5, 7.5, 114, 8.6, -17.7, 1, true);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -5.8, 4.5, 7.5, 114, -7.8, 18, 1, true);
        }
    }
    frame(fighter.lua_state_agent, 20.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -1.35, -11, 0, 0, 0, 0, 1.2, true);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 1.35, -11, 0, 0, 0, 0, 1.2, true);
        }
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc"), true, true);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckrot"), 0, 0, 0, 90, 0, 0, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//SpecialHi
    agent = "duckhunt", 
    script = "effect_specialhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_specialhi(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather_long"), Hash40::new("duckrot"), 0, 4, 0, 20, 0, 0, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//DownBoundU
    agent = "duckhunt", 
    script = "effect_downboundu", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_downboundu(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        helper::DOWN_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -2, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -2, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//DownBoundD
    agent = "duckhunt", 
    script = "effect_downboundd", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_downboundd(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        helper::DOWN_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -2, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -2, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//AppealHiL
    agent = "duckhunt", 
    script = "effect_appealhil", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_appealhil(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 93.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script(//AppealHiR
    agent = "duckhunt", 
    script = "effect_appealhir", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_appealhir(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 93.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script(//AppealSL
    agent = "duckhunt", 
    script = "effect_appealsl", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_appealsl(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 72.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 86.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//AppealSR
    agent = "duckhunt", 
    script = "effect_appealsr", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_appealsr(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 72.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 86.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//FinalAirEnd
    agent = "duckhunt", 
    script = "effect_finalairend",
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_finalairend(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_can"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit2"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit3"), true, true);
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather_long"), Hash40::new("duckrot"), 0, 4, 0, 20, 0, 0, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 72.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//FinalEnd
    agent = "duckhunt", 
    script = "effect_finalend", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_finalend(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_can"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit2"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit3"), true, true);
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather_long"), Hash40::new("duckrot"), 0, 4, 0, 20, 0, 0, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 72.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//Win1
    agent = "duckhunt", 
    script = "effect_win1", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_win1(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    frame(fighter.lua_state_agent, 55.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("duckhunt_win1_grass"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, 89, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    wait(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    wait(fighter.lua_state_agent, 25.0);
}

#[acmd_script(//MagicPotAirStartAerial
    agent = "duckhunt", 
    script = "effect_magicpotairstartaerial", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_magicpotairstartaerial(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//MagicPotAirEndAerial
    agent = "duckhunt", 
    script = "effect_magicpotairendaerial", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_magicpotairendaerial(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//FFlowerShootAerial
    agent = "duckhunt", 
    script = "effect_fflowershootaerial", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_fflowershootaerial(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script(//GenesisAerialLegs
    agent = "duckhunt", 
    script = "effect_genesisaeriallegs", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_duckhunt_genesisaeriallegs(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 % 9 % 10 % 11 % 12 % 13;

    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_duckhunt_jumpaerialfront,
        effect_duckhunt_jumpaerialback,
        effect_duckhunt_attack100end,
        effect_duckhunt_attacks3hi,
        effect_duckhunt_attacks3,
        effect_duckhunt_attacks3lw,
        effect_duckhunt_attackhi3,
        effect_duckhunt_attacklw3,
        effect_duckhunt_attackairf,
        effect_duckhunt_attackairb,
        effect_duckhunt_attackairlw,
        effect_duckhunt_specialhi,
        effect_duckhunt_downboundu,
        effect_duckhunt_downboundd,
        effect_duckhunt_appealhil,
        effect_duckhunt_appealhir,
        effect_duckhunt_appealsl,
        effect_duckhunt_appealsr,
        effect_duckhunt_finalairend,
        effect_duckhunt_finalend,
        effect_duckhunt_win1,
        effect_duckhunt_magicpotairstartaerial,
        effect_duckhunt_magicpotairendaerial,
        effect_duckhunt_fflowershootaerial,
        effect_duckhunt_genesisaeriallegs
    );
}