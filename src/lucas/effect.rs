use crate::imports::BuildImports::*;

#[acmd_script(//Attack11
    agent = "lucas", 
    script = "effect_attack11" , 
    category = ACMD_EFFECT , 
    low_priority)]
unsafe fn effect_lucas_attack11(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            EFFECT_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -4.0, 5.1, 0, 0, 0, 0, 0.7, 0, 1, 0, 0, 0, 0, false, *EF_FLIP_YZ);
            LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.9, /*B*/ 0.5);
        }
        frame(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("handr"), 3.0, 0.1, 0.0, 0, 0, 0, 0.35, true);
        }
        frame(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
        }
    } else { //Lucas
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 5.3, 2.5, -12.0, -3.05, 20.0, 0.85, true, *EF_FLIP_YZ);
        }
    }
}

#[acmd_script(//Attack12
    agent = "lucas", 
    script = "effect_attack12" , 
    category = ACMD_EFFECT , 
    low_priority)]
unsafe fn effect_lucas_attack12(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
        frame(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_hold"), Hash40::new("haver"), -0.2, 0, 0, 0, 0, 0, 0.8, true);
        }
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3.0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -1.5, 5.3, 0.0, -10.0, -20.0, 185.0, 0.7, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.9, /*B*/ 0.5);
        }
        frame(fighter.lua_state_agent, 7.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("handr"), 3.1, 0.2, 0.0, 0, 0, 0, 0.36, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_hold"), false, false);
        }
    } else {
        frame(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -1.5, 5.3, 4.5, 10, -20, 185, 0.75, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(fighter, 2.0);
        }
    }
}

#[acmd_script(//Attack13
    agent = "lucas", 
    script = "effect_attack13", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_lucas_attack13(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0.0, 5.8, 7.0, -28.0, -45.0, 30.0, 0.9, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.9, /*B*/ 0.5);
        }
    } else {//Lucas
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0.0, 5.8, 7.0, -28.0, -45.0, 30.0, 0.9, true, *EF_FLIP_YZ);
        }
    }
}

#[acmd_script(//AttackHi3
    agent = "lucas", 
    script = "effect_attackhi3" , 
    category = ACMD_EFFECT , 
    low_priority)]
unsafe fn effect_lucas_attackhi3(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 13.5, 2.0, 0, 45, 90, 0.85, true);
            LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.9, /*B*/ 0.5);
            LAST_EFFECT_SET_RATE(fighter, 1.0);
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("kneer"), 4.5, 0.2, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_RATE(fighter, 2.0);
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
        }
        frame(fighter.lua_state_agent, 28.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
    } else { //Lucas
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 7.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("kneer"), 4, 0, 0, 0, 90, 0, 0.9, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
        }
        frame(fighter.lua_state_agent, 28.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

#[acmd_script(//AttackLw3
    agent = "lucas", 
    script = "effect_attacklw3" , 
    category = ACMD_EFFECT , 
    low_priority)]
unsafe fn effect_lucas_attacklw3(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
        frame(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 2.0, 7.0, 0, 20, 0, 0.85, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
            LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.9, /*B*/ 0.5);
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("kneer"), 4.0, 0.1, -1.0, 0, 0, 0, 0.35, true);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
        }
    } else { //Lucas
        frame(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 2, 2.7, 0, 20, 0, 0.85, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

#[acmd_script(//AttackLw4
    agent = "lucas", 
    script = "effect_attacklw4", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn effect_lucas_attacklw4(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
        frame(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_psi_hold"), Hash40::new("lucas_psi_hold"), Hash40::new("havel"), 0.5, 0.5, 1.3, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
        }
        frame(fighter.lua_state_agent, 20.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_hold"), false, false);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_psi_atk_lw"), Hash40::new("lucas_psi_atk_lw"), Hash40::new("top"), 0, 2.5, 8, 0, 0, 0, 2.1, true, *EF_FLIP_YZ);
        }
        frame(fighter.lua_state_agent, 21.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.7, 2, 2, 2, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 1.3);
        }
    } else {
        frame(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_psi_hold"), Hash40::new("lucas_psi_hold"), Hash40::new("havel"), 0.5, 0.5, 1.3, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        }
        frame(fighter.lua_state_agent, 20.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_hold"), false, false);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_psi_atk_lw"), Hash40::new("lucas_psi_atk_lw"), Hash40::new("top"), 0, 2.5, 8, 0, 0, 0, 1.25, true, *EF_FLIP_YZ);
        }
        frame(fighter.lua_state_agent, 21.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.7, 2, 2, 2, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 1.3);
        }
        frame(fighter.lua_state_agent, 29.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_psi_atk_lw"), Hash40::new("lucas_psi_atk_lw"), Hash40::new("top"), 0, 2.5, 8, 0, 0, 0, 1.3, true, *EF_FLIP_YZ);
        }
        frame(fighter.lua_state_agent, 30.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.7, 2, 2, 2, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 1.3);
        }
        frame(fighter.lua_state_agent, 39.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_psi_atk_lw"), Hash40::new("lucas_psi_atk_lw"), Hash40::new("top"), 0, 2.5, 8, 0, 0, 0, 1.45, true, *EF_FLIP_YZ);
        }
        frame(fighter.lua_state_agent, 40.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, false);
        }
    }
}

#[acmd_script(//AttackAirN
    agent = "lucas", 
    script = "effect_attackairn" , 
    category = ACMD_EFFECT , 
    low_priority)]
unsafe fn effect_lucas_attackairn(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_hold"), Hash40::new("havel"), 0, 0, 1, 0, 0, 0, 0.9, true);
        }
        frame(fighter.lua_state_agent, 7.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_hold"), false, false);
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 1, 4.5, 0, -90, 0, 0, 1.1, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 1, 3.5, 0, -90, 0, 0, 1.1, true);
        }
        frame(fighter.lua_state_agent, 26.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 1.5, 3.5, 0, -90, 0, 0, 1.41, true);
            LAST_EFFECT_SET_RATE(fighter, 1.3);
        }
        frame(fighter.lua_state_agent, 30.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
        }
    } else { //Lucas
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_hold"), Hash40::new("havel"), 0, 0, 1, 0, 0, 0, 0.8, true);
        }
        frame(fighter.lua_state_agent, 7.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_hold"), false, false);
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 1, 4.5, 0, -90, 0, 0, 0.95, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 1, 3.5, 0, -90, 0, 0, 0.95, true);
        }
        frame(fighter.lua_state_agent, 26.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 1.5, 3.5, 0, -90, 0, 0, 1.3, true);
            LAST_EFFECT_SET_RATE(fighter, 1.3);
        }
        frame(fighter.lua_state_agent, 30.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
        }
    }
}

#[acmd_script(//AttackAirLw
    agent = "lucas", 
    script = "effect_attackairlw" , 
    category = ACMD_EFFECT , 
    low_priority)]
unsafe fn effect_lucas_attackairlw(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0.0, -4, 0.0, 0, 0, 0, 0.81, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(fighter, 2.0);
        }
        frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0.0, -4.0, 0.3, 0, 90, 0, 0.65, true);
        }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0.0, -4.0, 0.3, 0, 90, 0, 0.7, true);
        }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0.0, -4.0, 0.3, 0, 90, 0, 0.84, true);
            LAST_EFFECT_SET_RATE(fighter, 1.3);
        }
    } else { //Lucas
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -4, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("kneer"), 3, 0, 0, 0, 90, 0, 0.6, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
        }
        frame(fighter.lua_state_agent, 18.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("kneel"), 3, 0, 0, 0, 90, 0, 0.65, true);
        }
        frame(fighter.lua_state_agent, 23.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
        }
        frame(fighter.lua_state_agent, 26.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("kneer"), 3, 0, 0, 0, 90, 0, 0.7, true);
        }
        frame(fighter.lua_state_agent, 29.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
        }
        frame(fighter.lua_state_agent, 34.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("kneel"), 3, 0, 0, 0, 90, 0, 0.85, true);
            LAST_EFFECT_SET_RATE(fighter, 1.3);
        }
    }
}

/*#[acmd_script(//SpecialS
    agent = "lucas", 
    script = "effect_specials" , 
    category = ACMD_EFFECT , 
    low_priority)]
unsafe fn lucas_sidebgfx(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
        CLAUS_PK_BEAM[ENTRY_ID] = true;
        frame(fighter.lua_state_agent, 7.0);
        if is_excute(fighter) {
            if CLAUS_PK_BEAM[ENTRY_ID] {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_pkfi_start"), Hash40::new("lucas_pkfi_start"), Hash40::new("havel"), -0.5, 0, 0, 0, 0, 0, 1.6, true, *EF_FLIP_YZ);
            }
        }
        frame(fighter.lua_state_agent, 21.0);
        if is_excute(fighter) {
            if CLAUS_PK_BEAM[ENTRY_ID] {
                FOOT_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, false);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_sting"), Hash40::new("top"), 0, 5, 26, 0, 0, 0, 1.4, true);
                LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.1, /*G*/ 0.86, /*B*/ 1.3);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("top"), 0, 5, 10, 0, 0, 0, 1.7, true);                    
                LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.1, /*G*/ 1.6, /*B*/ 1.2);
            }
        }
        frame(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            CLAUS_PK_BEAM[ENTRY_ID] = false;
        }
        else if CLAUS_PK_BEAM[ENTRY_ID] == false {
            frame(fighter.lua_state_agent, 7.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_pkfi_start"), Hash40::new("lucas_pkfi_start"), Hash40::new("havel"), -0.5, 0, 0, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
            }
            frame(fighter.lua_state_agent, 21.0);
            if is_excute(fighter) {
                FOOT_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            }
        }
    } else { //Lucas
        frame(fighter.lua_state_agent, 7.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_pkfi_start"), Hash40::new("lucas_pkfi_start"), Hash40::new("havel"), -0.5, 0, 0, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
        }
        frame(fighter.lua_state_agent, 21.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

#[acmd_script(//SpecialAirS
    agent = "lucas", 
    script = "effect_specialairs" , 
    category = ACMD_EFFECT , 
    low_priority)]
unsafe fn lucas_sidebairgfx(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
        CLAUS_PK_BEAM[ENTRY_ID] = true;
        frame(fighter.lua_state_agent, 7.0);
        if is_excute(fighter) {
            if CLAUS_PK_BEAM[ENTRY_ID] {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_pkfi_start"), Hash40::new("lucas_pkfi_start"), Hash40::new("havel"), -0.5, 0, 0, 0, 0, 0, 1.6, true, *EF_FLIP_YZ);
            }
        }
        frame(fighter.lua_state_agent, 21.0);
        if is_excute(fighter) {
            if CLAUS_PK_BEAM[ENTRY_ID] {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_sting"), Hash40::new("top"), 0, 5, 26, 0, 0, 0, 1.4, true);
                LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.1, /*G*/ 0.86, /*B*/ 1.3);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("top"), 0, 5, 10, 0, 0, 0, 1.7, true);                    
                LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.1, /*G*/ 1.6, /*B*/ 1.2);
            }
        } 
        frame(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            CLAUS_PK_BEAM[ENTRY_ID] = false;
        }
        else if CLAUS_PK_BEAM[ENTRY_ID] == false {
            frame(fighter.lua_state_agent, 7.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_pkfi_start"), Hash40::new("lucas_pkfi_start"), Hash40::new("havel"), -0.5, 0, 0, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
            }
        }
    } else { //Lucas
        frame(fighter.lua_state_agent, 7.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_pkfi_start"), Hash40::new("lucas_pkfi_start"), Hash40::new("havel"), -0.5, 0, 0, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
        }
    }
}*/

#[acmd_script(//WessDance
    agent = "lucas", 
    script = "effect_wessdance", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_lucas_wessdance(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1370.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_kusudama"), Hash40::new("top"), 0, 22.0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_kusudama"), Hash40::new("top"), 0, 22.0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_kusudama"), Hash40::new("top"), 0, 22.0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_lucas_attack11,
        effect_lucas_attack12,
        effect_lucas_attack13,
        effect_lucas_attackhi3,
        effect_lucas_attacklw3,
        effect_lucas_attacklw4,
        effect_lucas_attackairn,
        effect_lucas_attackairlw,
        effect_lucas_wessdance
    );
}