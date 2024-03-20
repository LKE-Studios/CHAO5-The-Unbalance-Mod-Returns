use crate::imports::BuildImports::*;

//AttackS3Hi
unsafe extern "C" fn effect_mewtwo_AttackS3Hi(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if slot_wrapped >= 8 {
        frame(fighter.lua_state_agent, 9.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 3.0, 10.0, 10.0, 0.0, -85.0, 15.0, 1.1, true, *EF_FLIP_YZ, 0);
            LAST_EFFECT_SET_RATE(fighter, 1.1);
        }
        frame(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
    } 
    else {
        frame(fighter.lua_state_agent, 9.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 3.0, 10.0, 10.0, 0.0, -85.0, 15.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
            LAST_EFFECT_SET_RATE(fighter, 1.1);
        }
        frame(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

//AttackS3
unsafe extern "C" fn effect_mewtwo_AttackS3(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if slot_wrapped >= 8 {
        frame(fighter.lua_state_agent, 9.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 3.0, 7.0, 10.0, 0.0, -85.0, -15.0, 1.1, true, *EF_FLIP_YZ, 0);
            LAST_EFFECT_SET_RATE(fighter, 1.1);
        }
        frame(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
    } 
    else {
        frame(fighter.lua_state_agent, 9.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 3.0, 7.0, 10.0, 0.0, -85.0, -15.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
            LAST_EFFECT_SET_RATE(fighter, 1.1);
        }
        frame(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

//AttackS3Lw
unsafe extern "C" fn effect_mewtwo_AttackS3Lw(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if slot_wrapped >= 8 {
        frame(fighter.lua_state_agent, 9.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 3.0, 3.5, 10.0, 0.0, -85.0, -13.0, 1.1, true, *EF_FLIP_YZ, 0);
            LAST_EFFECT_SET_RATE(fighter, 1.1);
        }
        frame(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
    } 
    else {
        frame(fighter.lua_state_agent, 9.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 3.0, 3.5, 10.0, 0.0, -85.0, -13.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
            LAST_EFFECT_SET_RATE(fighter, 1.1);
        }
        frame(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

//AttackHi3 
unsafe extern "C" fn effect_mewtwo_AttackHi3(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if slot_wrapped >= 8 {
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 9.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0.0, 14.0, 1.5, 0.0, 10.0, 90.0, 1.05, true, *EF_FLIP_YZ, 0);
        }
    } 
    else {
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 9.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0.0, 14.0, 1.5, 0.0, 10.0, 90.0, 1.05, true, *EF_FLIP_YZ, slot_wrapped);
        }
    }
}

//AttackLw3
unsafe extern "C" fn effect_mewtwo_AttackLw3(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if slot_wrapped >= 8 {
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), -5.5, 5.0, 4.5, 0.0, -70.0, 190.0, 1.05, true, *EF_FLIP_YZ, 0);
        }
        frame(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        }
    } 
    else {
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), -5.5, 5.0, 4.5, 0.0, -70.0, 190.0, 1.05, true, *EF_FLIP_YZ, slot_wrapped);
        }
        frame(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        }
    }
}

//AttackAirB
unsafe extern "C" fn effect_mewtwo_AttackAirB(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if slot_wrapped >= 8 {
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0.0, 13.5, -6.0, 180.0, 45.0, 90.0, 1.1, true, *EF_FLIP_YZ, 0);
            LAST_EFFECT_SET_RATE(fighter, 1.05);
        }
    } 
    else {
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0.0, 13.5, -6.0, 180.0, 45.0, 90.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
            LAST_EFFECT_SET_RATE(fighter, 1.05);
        }
    }
}

//AttackAirHi
unsafe extern "C" fn effect_mewtwo_AttackAirHi(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if slot_wrapped >= 8 {
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0.0, 9.5, -4.0, 0.0, 30.0, 90.0, 1.15, true, *EF_FLIP_YZ, 0);
        }
    } 
    else {
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0.0, 9.5, -4.0, 0.0, 30.0, 90.0, 1.15, true, *EF_FLIP_YZ, slot_wrapped);
        }
    }
}

//ThrowLw
unsafe extern "C" fn effect_mewtwo_ThrowLw(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if slot_wrapped >= 8 {
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 1.0, 12.0, 4.0, 180.0, -130.0, 85.0, 1.0, true, *EF_FLIP_YZ, 0);
        }
        frame(fighter.lua_state_agent, 16.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 17.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_tail_attack_a_01"), false, false);
        }
    } 
    else {
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 1.0, 12.0, 4.0, 180.0, -130.0, 8.0, 1.0, true, *EF_FLIP_YZ, slot_wrapped);
        }
        frame(fighter.lua_state_agent, 16.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 17.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_tail_attack_a_01"), false, false);
        }
    }
}

//CliffAttack
unsafe extern "C" fn effect_mewtwo_CliffAttack(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if slot_wrapped >= 8 {
        frame(fighter.lua_state_agent, 7.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 23, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 20.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), -3.0, 5.7, 4.5, 190.0, -80.0, 0.0, 0.95, true, *EF_FLIP_YZ, 0);
        }
        frame(fighter.lua_state_agent, 22.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    } 
    else {
        frame(fighter.lua_state_agent, 7.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 23, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 20.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), -3.0, 5.7, 4.5, 190.0, -80.0, 0.0, 0.95, true, *EF_FLIP_YZ, slot_wrapped);
        }
        frame(fighter.lua_state_agent, 22.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

//DownAttackU
unsafe extern "C" fn effect_mewtwo_DownAttackU(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if slot_wrapped >= 8 {
        frame(fighter.lua_state_agent, 18.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), -0.5, 4.6, 5.0, 0.0, -60.0, -170.0, 1.1, true, *EF_FLIP_YZ, 0);
        }
        frame(fighter.lua_state_agent, 19.0);
        if is_excute(fighter) {
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
            LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        }
        frame(fighter.lua_state_agent, 22.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_tail_attack_a_01"), false, true);
        }
        frame(fighter.lua_state_agent, 23.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 2.0, 3.3, -4.0, 0.0, 110.0, -195.0, 1.1, true, *EF_FLIP_YZ, 0);
        }
        frame(fighter.lua_state_agent, 27.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_tail_attack_a_01"), false, true);
        }
    } 
    else {
        frame(fighter.lua_state_agent, 18.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), -0.5, 4.6, 5.0, 0.0, -60.0, -170.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
        }
        frame(fighter.lua_state_agent, 19.0);
        if is_excute(fighter) {
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
            LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        }
        frame(fighter.lua_state_agent, 22.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_tail_attack_a_01"), false, true);
        }
        frame(fighter.lua_state_agent, 23.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 2.0, 3.3, -4.0, 0.0, 110.0, -195.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
        }
        frame(fighter.lua_state_agent, 27.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_tail_attack_a_01"), false, true);
        }
    }
}

//DownAttackD
unsafe extern "C" fn effect_mewtwo_DownAttackD(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if slot_wrapped >= 8 {
        frame(fighter.lua_state_agent, 18.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), -0.5, 4.6, 5.0, 0.0, -60.0, -170.0, 1.1, true, *EF_FLIP_YZ, 0);
        }
        frame(fighter.lua_state_agent, 19.0);
        if is_excute(fighter) {
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
            LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        }
        frame(fighter.lua_state_agent, 22.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_tail_attack_a_01"), false, true);
        }
        frame(fighter.lua_state_agent, 23.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 2.0, 3.3, -4.0, 0.0, 110.0, -195.0, 1.1, true, *EF_FLIP_YZ, 0);
        }
        frame(fighter.lua_state_agent, 27.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_tail_attack_a_01"), false, true);
        }
    } 
    else {
        frame(fighter.lua_state_agent, 18.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), -0.5, 4.6, 5.0, 0.0, -60.0, -170.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
        }
        frame(fighter.lua_state_agent, 19.0);
        if is_excute(fighter) {
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
            LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        }
        frame(fighter.lua_state_agent, 22.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_tail_attack_a_01"), false, true);
        }
        frame(fighter.lua_state_agent, 23.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 2.0, 3.3, -4.0, 0.0, 110.0, -195.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
        }
        frame(fighter.lua_state_agent, 27.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_tail_attack_a_01"), false, true);
        }
    }
}

pub fn install() {
    Agent::new("mewtwo")
    .effect_acmd("effect_attacks3hi", effect_mewtwo_AttackS3Hi)
    .effect_acmd("effect_attacks3", effect_mewtwo_AttackS3)
    .effect_acmd("effect_attacks3lw", effect_mewtwo_AttackS3Lw)
    .effect_acmd("effect_attackhi3", effect_mewtwo_AttackHi3)
    .effect_acmd("effect_attacklw3", effect_mewtwo_AttackLw3)   
    .effect_acmd("effect_attackairb", effect_mewtwo_AttackAirB)
    .effect_acmd("effect_attackairhi", effect_mewtwo_AttackAirHi)
    .effect_acmd("effect_throwlw", effect_mewtwo_ThrowLw)
    .effect_acmd("effect_downattackd", effect_mewtwo_DownAttackD)
    .effect_acmd("effect_downattacku", effect_mewtwo_DownAttackU)
    .effect_acmd("effect_cliffattack", effect_mewtwo_CliffAttack)
    .install();
}