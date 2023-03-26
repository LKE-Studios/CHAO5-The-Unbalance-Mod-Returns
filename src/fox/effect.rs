use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script(//AttackLw3 
    agent = "fox", 
    script = "effect_attacklw3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_fox_attacklw3(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Melee Fox
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("fox_tail_attack_01"), Hash40::new("top"), -1, 4, 2, 7, -40, 170, 1, true);
        }
    } else { 
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 6.0);
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("fox_tail_attack_01"), Hash40::new("top"), -1, 4, 2, 7, -40, 170, 1, true, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
            }
        }
        else {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("fox_tail_attack_01"), Hash40::new("top"), -2, 4, 2, 7, -40, 183, 1, true, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
            }
        }
    }
}

#[acmd_script(//AttackAirHi
    agent = "fox", 
    script = "effect_attackairhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_fox_attackairhi(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Melee Fox
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("fox_tail_attack_01"), Hash40::new("top"), 0, 13.3, -0.5, 100, -30, 150, 0.9, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.3);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("fox_tail_attack_01"), false, false);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -3, 13, 0, 100, -30, 150, 0.8, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
    } else {
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("fox_tail_attack_01"), Hash40::new("top"), 0, 13.3, -0.5, 100, -30, 150, 0.9, true, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.3);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("fox_tail_attack_01"), false, false);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -3, 13, 0, 100, -30, 150, 0.8, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_fox_attacklw3,
        effect_fox_attackairhi
    );
}