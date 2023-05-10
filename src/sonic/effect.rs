use smash::app::sv_animcmd::*;
use smash::app::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::phx::Vector3f;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;
use crate::sonic::game::THROWHI_FRAME_LAND;

#[acmd_script(//Run 
    agent = "sonic", 
    script = "effect_run", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_run(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
	    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {
            if macros::is_excute(fighter) {
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            }
            wait(fighter.lua_state_agent, 3.0);
            if macros::is_excute(fighter) {
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
            }
            wait(fighter.lua_state_agent, 4.0);
            if macros::is_excute(fighter) {
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
            wait(fighter.lua_state_agent, 4.0);
        }
    } else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_DISABLE_RUN_TRACE) {
            if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
                if macros::is_excute(fighter) {
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_runtrace"), Hash40::new("top"), 0, 0, 0, 180, 0, 0, -1, true);
                    macros::LAST_EFFECT_SET_RATE(fighter, 1.09);
                }
            }
            else if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_runtrace"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
                macros::LAST_EFFECT_SET_RATE(fighter, 1.09);
            }
        }
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 4.0);
    }
}

#[acmd_script(//Attack11 
    agent = "sonic", 
    script = "effect_attack11", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attack11(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7.8, -2, 0, 0, 0, 0.9, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
	    }
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 13.5, 7.8, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, false);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        }
    } else { //Sonic
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7.8, -2, 0, 0, 0, 0.9, true, *EF_FLIP_YZ);
        }
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 13, 7.5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, false);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        }
    }
}

#[acmd_script(//Attack12 
    agent = "sonic", 
    script = "effect_attack12", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attack12(fighter: &mut L2CAgentBase) { 
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 3, 3.8, 3, -6, -15, 30, 0.97, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
        }
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 13.5, 7.8, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, false);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7.5, 0, 0, 0, 0, 0.9, true, *EF_FLIP_YZ);
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 13, 7.5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, false);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        }
    }    
}

#[acmd_script(//Attack13
    agent = "sonic", 
    script = "effect_attack13", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attack13(fighter: &mut L2CAgentBase) {
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 8, 6.3, 6.6, -7, 0, -100, 0.8, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
        }
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 13.5, 7.8, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, false);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -1.7, 6.5, -2, -10, 7, 0, 1, true, *EF_FLIP_YZ);
        }
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 13, 8.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, false);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        }
    } 
}

#[acmd_script(//AttackDash 
    agent = "sonic", 
    script = "effect_attackdash", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attackdash(fighter: &mut L2CAgentBase) {
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7.8, -2, -24, 25, 360, 0.9, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 14.8, 11.8, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, false);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        }
    } else { //Sonic
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7, -9, -7, 0, 0, 1.2, true, *EF_FLIP_YZ);
        }
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 14, 9.5, 0, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 360, false);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

#[acmd_script(//AttackS3Hi
    agent = "sonic", 
    script = "effect_attacks3hi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attacks3hi(fighter: &mut L2CAgentBase) { 
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, -3, -42, 0, 0, 1.1, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9, 15.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 1, -3, -22, 0, 0, 1.1, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 3, 0.7, 0.2);
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9, 15.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
        }
    }
}

#[acmd_script(//AttackS3
    agent = "sonic", 
    script = "effect_attacks3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attacks3(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 3, -3, -22, 0, 0, 1.1, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9, 15.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
        }
    } else { //Sonic
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 1, -3, -22, 0, 0, 1.1, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 3, 0.7, 0.2);
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9, 15.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
        }
    }
}

#[acmd_script(//AttackS3Lw
    agent = "sonic", 
    script = "effect_attacks3lw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attacks3lw(fighter: &mut L2CAgentBase) {
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3.5, -4, -5, 0, 0, 1.1, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9, 15.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 1, -3, -22, 0, 0, 1.1, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 3, 0.7, 0.2);
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9, 15.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
        }
    }
}

#[acmd_script(//AttackHi3 
    agent = "sonic", 
    script = "effect_attackhi3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attackhi3(fighter: &mut L2CAgentBase) { 
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -1.5, 9.0, -0.2, 0, 50, 100, 0.92, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.82);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 13, 2.5, 95, -28.5, 131, 1.05, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 2);
        }
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 17.5, 0.5, 80, 24, 180, 0.95, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 2);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
    } 
}

#[acmd_script(//AttackLw3
    agent = "sonic", 
    script = "effect_attacklw3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attacklw3(fighter: &mut L2CAgentBase) { 
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -2, 2, 2, 1, 5, 5, 0.925, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.86);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
        }
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -2, 2, 2, 1, 5, 5, 0.925, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

#[acmd_script(//AttackS4Charge
    agent = "sonic", 
    script = "effect_attacks4charge", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attacks4charge(fighter: &mut L2CAgentBase) {
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        for _ in 0..i32::MAX {
            frame(fighter.lua_state_agent, 1.0);
            if macros::is_excute(fighter) {
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 5, 0, 4, 0, 0, 0, false);
            }
            wait(fighter.lua_state_agent, 7.0);
            if macros::is_excute(fighter) {
                macros::EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -3, 11.5, -8, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, true, *EF_FLIP_YZ);
            }
            wait(fighter.lua_state_agent, 3.0);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sonic_punchtrace"), Hash40::new("sonic_punchtrace"), Hash40::new("top"), -6.25, 10, -5, -34.857, 18.764, 94.68, 1, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.05);
        }
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 5, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -3, 11.5, -8, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, true, *EF_FLIP_YZ);
        }
    }
}

#[acmd_script(//AttackS4
    agent = "sonic", 
    script = "effect_attacks4", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attacks4(fighter: &mut L2CAgentBase) {
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 2.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 23.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 2, 6.5, -1.2, 0, 0, 0, 1.3, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 3, 0.7, 0.2);
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 25.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 18, 6.5, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, false);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 2.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 2, 6.5, -4, 0, 0, 0, 1.3, true, *EF_FLIP_YZ);
            macros::LAST_PARTICLE_SET_COLOR(fighter, 3, 0.7, 0.2);
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 18, 6.5, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, false);
        }
    }
}

#[acmd_script(//AttackHi4
    agent = "sonic", 
    script = "effect_attackhi4", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attackhi4(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 6.0);
            if macros::is_excute(fighter) { 
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 13.5, 2.0, 0, 0, 90, 1.0, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.4, /*G*/ 0.14, /*B*/ 0.05);
        }
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sphere"), 0, 0, 0, 0, -160, 270, 0.75, true, 1);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_spintrace"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinwind"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinblur_plain"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 32.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sphere"), -0.5, 3, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 360, false);
        }
        frame(fighter.lua_state_agent, 33.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spinwind"), true, false);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spinblur_plain"), true, false);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spintrace"), false, true);
        }
        frame(fighter.lua_state_agent, 49.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

#[acmd_script(//AttackLw4Charge
    agent = "sonic", 
    script = "effect_attacklw4charge", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attacklw4charge(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 12, 0, 4, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_soil_landing"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 0.0, 2.0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("sys_merikomi"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_soil_landing"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_soil_landing"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_soil_landing"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 12, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, true);
        }
    }
}

#[acmd_script(//AttackLw4
    agent = "sonic", 
    script = "effect_attacklw4", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attacklw4(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sphere"), 0, 0, 0, 0, -160, 270, 0.75, true, 1);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_spintrace"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.4, /*G*/ 0.14, /*B*/ 0.05);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinblur_plain"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.4, /*G*/ 0.14, /*B*/ 0.05);
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sonic_homing_hold"), Hash40::new("sonic_homing_hold"), Hash40::new("sphere"), 0, 0, 0, 0, 90, 0, 0.44, true, *EF_FLIP_XY);
            macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.4, /*G*/ 0.14, /*B*/ 0.05);
        }
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_soil_landing"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinwind"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 0.84, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.4, /*G*/ 0.14, /*B*/ 0.05);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinblur_plain"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.4, /*G*/ 0.14, /*B*/ 0.05);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_merikomi"), true, true);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 2.5, -2, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1);
            macros::EFFECT(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), -1, 2.5, 4, 180, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1);
        }
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 2.5, 17, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 360, true);
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 2.5, -15, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 360, true);
            macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2.5, -2, 0, 0, 0, 1.1, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 3, 0.7, 0.2);
            macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2.5, 4, 0, 180, 0, 1.1, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 3, 0.7, 0.2);
        }
    }
}

#[acmd_script(//AttackAirN
    agent = "sonic", 
    script = "effect_attackairn", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attackairn(fighter: &mut L2CAgentBase) {
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 8.5, -4, 35, 0, 0, 1.1, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
        }
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 0, 13.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinwind"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 0.84, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinblur_plain"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        for _ in 0..3 {
            if macros::is_excute(fighter) {
                macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 0.8, 12, 15, 15, 0, 0, 0, true);
            }
            wait(fighter.lua_state_agent, 4.0);
            if macros::is_excute(fighter) {
                macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1, 12, 15, 15, 0, 0, 0, true);
            }
            wait(fighter.lua_state_agent, 5.0);
        }
        frame(fighter.lua_state_agent, 36.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spinblur_plain"), false, false);
        }
        frame(fighter.lua_state_agent, 39.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spinwind"), true, false);
        }
    }
}

#[acmd_script(//AttackAirF
    agent = "sonic", 
    script = "effect_attackairf", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attackairf(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 7, -1, -3, -11, -113, 1.1, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 14.5, 8, 0, 0, 0, 0, 1.35, 0, 0, 0, 0, 0, 360, false);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8, 13, -90, -8, 0, 0.75, true, *EF_FLIP_YZ, 0.85);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
        }
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8, 8.5, -90, -8, 90, 0.55, true, *EF_FLIP_YZ, 0.6);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8, 4, -90, -8, 180, 0.45, true, *EF_FLIP_YZ, 0.4);
        }
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8, 0, -90, -8, 270, 0.4, true, *EF_FLIP_YZ, 0.3);
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 14.5, 8, 0, 0, 0, 0, 1.35, 0, 0, 0, 0, 0, 360, false);
        }
    }
}

#[acmd_script(//AttackAirB
    agent = "sonic", 
    script = "effect_attackairb", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attackairb(fighter: &mut L2CAgentBase) { 
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("armr"), 0, 0, 0.0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 7.5, -6.5, 160, 60, 15, 1, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 2);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc"), true, true);
        }
    }
}

#[acmd_script(//AttackAirHi 
    agent = "sonic", 
    script = "effect_attackairhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attackairhi(fighter: &mut L2CAgentBase) {
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles 
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1.5, 7.5, 0, 0, -80, -105, 0.7, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
        }
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("head"), 0, 0, 6.0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7.5, 2.5, -10, 0, 0, 0.5, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, 3, 0.7, 0.2);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -1, 7.5, -2.5, 190, 0, 0, 0.5, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, 3, 0.7, 0.2);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("sonic_atk_hi"), Hash40::new("sonic_atk_hi"), Hash40::new("top"), 0.8, 8.7, 0, 59, 74, 154, 1, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 3, 0.7, 0.2);
            macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("sonic_atk_hi"), Hash40::new("sonic_atk_hi"), Hash40::new("top"), -0.9, 8.7, 0, 245, -77, 22, 1, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, 3, 0.7, 0.2);
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 19.5, 0, 0, 0, 0, 1.55, 0, 0, 0, 0, 0, 360, false);
        }
    }
}

#[acmd_script(//AttackAirLw
    agent = "sonic", 
    script = "effect_attackairlw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_attackairlw(fighter: &mut L2CAgentBase) {
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, -1, 0.7, 90, 0, 0, 0.5, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, 3, 0.7, 0.2);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 1.5, -7, 0, 0, 0, 0, 1.35, 0, 0, 0, 0, 0, 360, false);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sonic_atk_lw"), Hash40::new("sonic_atk_lw"), Hash40::new("top"), 0, -1, 2, 74.8, 0, 0, 1, true, *EF_FLIP_YZ);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_atk_lw_kick"), Hash40::new("top"), 0, -0.5, 4, -25, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 35.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sonic_atk_lw_kick"), true, true);
        }
    }
}

#[acmd_script(//ThrowHi
    agent = "sonic", 
    script = "effect_throwhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_throwhi(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, THROWHI_FRAME_LAND);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("sonic_throw_hi"), Hash40::new("sonic_throw_hi"), Hash40::new("top"), 0, 5, 4, -90, 0, 0, 0.62, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
            macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("sonic_throw_hi"), Hash40::new("sonic_throw_hi"), Hash40::new("top"), 0, 5, 6.5, -75, 0, 0, 0.62, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
            macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("sonic_throw_hi"), Hash40::new("sonic_throw_hi"), Hash40::new("top"), 2, 5, 4, -110, 0, -10, 0.62, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
            macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("sonic_throw_hi"), Hash40::new("sonic_throw_hi"), Hash40::new("top"), -2, 5, 2.5, -110, 0, 10, 0.62, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
        }
        frame(fighter.lua_state_agent, 45.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 3, 0, 1.5, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 21.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("sonic_throw_hi"), Hash40::new("sonic_throw_hi"), Hash40::new("top"), 0, 5, 4, -90, 0, 0, 0.62, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
            macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("sonic_throw_hi"), Hash40::new("sonic_throw_hi"), Hash40::new("top"), 0, 5, 6.5, -75, 0, 0, 0.62, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
            macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("sonic_throw_hi"), Hash40::new("sonic_throw_hi"), Hash40::new("top"), 2, 5, 4, -110, 0, -10, 0.62, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
            macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("sonic_throw_hi"), Hash40::new("sonic_throw_hi"), Hash40::new("top"), -2, 5, 2.5, -110, 0, 10, 0.62, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
        }
        frame(fighter.lua_state_agent, 25.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 3, 0, 1.5, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        }
    }
}

#[acmd_script(//EntryL
    agent = "sonic", 
    script = "effect_entryl", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_entryl(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {
        frame(fighter.lua_state_agent, 35.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    } else {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinwind"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinblur_plain"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_spintrace_homing"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 40.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spinwind"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spinblur_plain"), false, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spintrace_homing"), false, true);
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

#[acmd_script(//EntryR
    agent = "sonic", 
    script = "effect_entryr", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_sonic_entryr(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {
        frame(fighter.lua_state_agent, 35.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    } else {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinwind"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinblur_plain"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sonic_spintrace_homing"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 40.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spinwind"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spinblur_plain"), false, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spintrace_homing"), false, true);
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_sonic_run,
        effect_sonic_attack11,
        effect_sonic_attack12,
        effect_sonic_attack13,
        effect_sonic_attackdash,
        effect_sonic_attacks3hi,
        effect_sonic_attacks3,
        effect_sonic_attacks3lw,
        effect_sonic_attackhi3,
        effect_sonic_attacklw3,
        effect_sonic_attacks4charge,
        effect_sonic_attacks4,
        effect_sonic_attackhi4,
        effect_sonic_attacklw4charge,
        effect_sonic_attacklw4,
        effect_sonic_attackairn,
        effect_sonic_attackairf,
        effect_sonic_attackairb,
        effect_sonic_attackairhi,
        effect_sonic_attackairlw,
        effect_sonic_throwhi,
        effect_sonic_entryl,
        effect_sonic_entryr
    );
}