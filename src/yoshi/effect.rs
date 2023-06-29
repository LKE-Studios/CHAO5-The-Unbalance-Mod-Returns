use crate::imports::BuildImports::*;

#[acmd_script(//EntryR
    agent = "yoshi", 
    script = "effect_entryr", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_yoshi_entryr(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if slot_wrapped >= 8 {
        frame(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("yoshi_entry_01"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT(fighter, Hash40::new("yoshi_tamagoumi_smoke"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 49.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 71.0);
        if is_excute(fighter) {
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_down_smoke"), Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 2.3, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_ZX);
        }
    } else {    
        frame(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_arg11(fighter, Hash40::new("yoshi_entry_01"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
            EFFECT(fighter, Hash40::new("yoshi_tamagoumi_smoke"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 49.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 71.0);
        if is_excute(fighter) {
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_down_smoke"), Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 2.3, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_ZX);
        }
    }
}

#[acmd_script(//EntryL
    agent = "yoshi", 
    script = "effect_entryl", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_yoshi_entryl(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if slot_wrapped >= 8 {
        frame(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("yoshi_entry_01"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT(fighter, Hash40::new("yoshi_tamagoumi_smoke"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 49.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 71.0);
        if is_excute(fighter) {
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_down_smoke"), Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 2.3, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_ZX);
        }
    } else {
        frame(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_arg11(fighter, Hash40::new("yoshi_entry_01"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
            EFFECT(fighter, Hash40::new("yoshi_tamagoumi_smoke"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 49.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 71.0);
        if is_excute(fighter) {
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_down_smoke"), Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 2.3, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_ZX);
        }
    }
}

#[acmd_script(//AttackAirLw
    agent = "yoshi", 
    script = "effect_attackairlw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_yoshi_attackairlw(fighter: &mut L2CAgentBase) {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 0 {
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("yoshi_air_trace_01"), Hash40::new("top"), 0, -2.0, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 40.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("yoshi_air_trace_01"), true, true);
        }
    }
    if color == 1 {
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("yoshi_air_trace_02"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 40.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("yoshi_air_trace_02"), true, true);
        }
    }
    if color == 2 {
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("yoshi_air_trace_03"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 40.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("yoshi_air_trace_03"), true, true);
        }
    }
    if color == 3 {
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("yoshi_air_trace_04"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 40.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("yoshi_air_trace_04"), true, true);
        }
    }
    if color == 4 {
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("yoshi_air_trace_05"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 40.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("yoshi_air_trace_05"), true, true);
        }
    }
    if color == 5 {
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("yoshi_air_trace_06"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 40.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("yoshi_air_trace_06"), true, true);
        }
    }
    if color == 6 {
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("yoshi_air_trace_07"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 40.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("yoshi_air_trace_07"), true, true);
        }
    }
    if color == 7 {
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("yoshi_air_trace_08"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 40.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("yoshi_air_trace_08"), true, true);
        }
    }
    if color == 8 {
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("yoshi_air_trace_01"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 40.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("yoshi_air_trace_01"), true, true);
        }
    }
    if color == 9 {
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("yoshi_air_trace_03"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 40.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("yoshi_air_trace_03"), true, true);
        }
    }
    if color == 10 {
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("yoshi_air_trace_05"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 40.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("yoshi_air_trace_05"), true, true);
        }
    }
    if color == 11 {
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("yoshi_air_trace_01"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 40.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("yoshi_air_trace_01"), true, true);
        }
    }
    if color == 12 {
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("yoshi_air_trace_06"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 40.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("yoshi_air_trace_06"), true, true);
        }
    }
    if color == 13 {
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("yoshi_air_trace_01"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 40.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("yoshi_air_trace_01"), true, true);
        }
    }
}

#[acmd_script(//SpecialSLoop
    agent = "yoshi", 
    script = "effect_specialsloop", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_yoshi_specialsloop(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if slot_wrapped >= 8 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_METAL) {
            if is_excute(fighter){
                EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_metal"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
            } 
        }
        else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GOLD) {
            if is_excute(fighter){
                EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_gold"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        }
        else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPYCLOAK) {
            if is_excute(fighter){
                EFFECT_FOLLOW(fighter, Hash40::new("null"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        }
        else if FT_IS_SAME_FIGHTER_CATEGORY(fighter, *METAMON) {
            if is_excute(fighter){
                EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_metamon"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        }
        else if FT_IS_SAME_FIGHTER_CATEGORY(fighter, *LIGHT) {
            if is_excute(fighter){
                EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_light"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        }
        else if FT_IS_SAME_FIGHTER_CATEGORY(fighter, *DARK) { 
            if is_excute(fighter){
                EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_dark"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        }
        else {
            if is_excute(fighter){
                EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_01"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        }
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 5.0);
    } else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_METAL) {
            if is_excute(fighter){
                EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_metal"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
            } 
        }
        else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GOLD) {
            if is_excute(fighter){
                EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_gold"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        }
        else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPYCLOAK) {
            if is_excute(fighter){
                EFFECT_FOLLOW(fighter, Hash40::new("null"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        }
        else if FT_IS_SAME_FIGHTER_CATEGORY(fighter, *METAMON) {
            if is_excute(fighter){
                EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_metamon"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        }
        else if FT_IS_SAME_FIGHTER_CATEGORY(fighter, *LIGHT) {
            if is_excute(fighter){
                EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_light"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        }
        else if FT_IS_SAME_FIGHTER_CATEGORY(fighter, *DARK) { 
            if is_excute(fighter){
                EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_dark"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        }
        else {
            if is_excute(fighter){
                EFFECT_FOLLOW_arg11(fighter, Hash40::new("yoshi_gorogorotamago_01"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
            }
        }
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 5.0);
    }
}


pub fn install() {
    smashline::install_acmd_scripts!(
        effect_yoshi_entryr,
        effect_yoshi_entryl,
        effect_yoshi_attackairlw,
        effect_yoshi_specialsloop,
    );
}