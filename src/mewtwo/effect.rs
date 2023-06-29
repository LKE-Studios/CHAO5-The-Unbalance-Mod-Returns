use crate::imports::BuildImports::*;

#[acmd_script(//AttackS3Hi
    agent = "mewtwo", 
    script = "effect_attacks3hi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_mewtwo_attacks3hi(fighter: &mut L2CAgentBase) {
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
    } else {
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

#[acmd_script(//AttackS3
    agent = "mewtwo", 
    script = "effect_attacks3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_mewtwo_attacks3(fighter: &mut L2CAgentBase) {
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
    } else {
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

#[acmd_script(//AttackS3Lw
    agent = "mewtwo", 
    script = "effect_attacks3lw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_mewtwo_attacks3lw(fighter: &mut L2CAgentBase) {
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
    } else {
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

#[acmd_script(//AttackHi3 
    agent = "mewtwo", 
    script = "effect_attackhi3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_mewtwo_attackhi3(fighter: &mut L2CAgentBase) {
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
    } else {
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

#[acmd_script(//AttackLw3
    agent = "mewtwo", 
    script = "effect_attacklw3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_mewtwo_attacklw3(fighter: &mut L2CAgentBase) {
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
    } else {
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

#[acmd_script(//AttackAirB
    agent = "mewtwo", 
    script = "effect_attackairb", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_mewtwo_attackairb(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if slot_wrapped >= 8 {
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0.0, 13.5, -6.0, 180.0, 45.0, 90.0, 1.1, true, *EF_FLIP_YZ, 0);
            LAST_EFFECT_SET_RATE(fighter, 1.05);
        }
    } else {
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0.0, 13.5, -6.0, 180.0, 45.0, 90.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
            LAST_EFFECT_SET_RATE(fighter, 1.05);
        }
    }
}

#[acmd_script(//AttackAirHi
    agent = "mewtwo", 
    script = "effect_attackairhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_mewtwo_attackairhi(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if slot_wrapped >= 8 {
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0.0, 9.5, -4.0, 0.0, 30.0, 90.0, 1.15, true, *EF_FLIP_YZ, 0);
        }
    } else {
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0.0, 9.5, -4.0, 0.0, 30.0, 90.0, 1.15, true, *EF_FLIP_YZ, slot_wrapped);
        }
    }
}

#[acmd_script(//ThrowLw
    agent = "mewtwo", 
    script = "effect_throwlw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_mewtwo_throwlw(fighter: &mut L2CAgentBase) {
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
    } else {
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

#[acmd_script(//CliffAttack
    agent = "mewtwo", 
    script = "effect_cliffattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_mewtwo_cliffattack(fighter: &mut L2CAgentBase) {
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
    } else {
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

#[acmd_script(//DownAttackU
    agent = "mewtwo", 
    script = "effect_downattacku", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_mewtwo_downattacku(fighter: &mut L2CAgentBase) {
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
    } else {
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

#[acmd_script(//DownAttackD
    agent = "mewtwo", 
    script = "effect_downattackd", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_mewtwo_downattackd(fighter: &mut L2CAgentBase) {
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
    } else {
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
    smashline::install_acmd_scripts!(
        effect_mewtwo_attacks3hi,
        effect_mewtwo_attacks3,
        effect_mewtwo_attacks3lw,
        effect_mewtwo_attackhi3,
        effect_mewtwo_attacklw3,
        effect_mewtwo_attackairb,
        effect_mewtwo_attackairhi,
        effect_mewtwo_throwlw,
        effect_mewtwo_cliffattack,
        effect_mewtwo_downattacku,
        effect_mewtwo_downattackd
    );
}
