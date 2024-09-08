use crate::imports::BuildImports::*;

//Attack11 
unsafe extern "C" fn effect_basyaamo_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("koopa_scratch"), Hash40::new("koopa_scratch"), Hash40::new("top"), -2, 14, 0, 4, 20, 180, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 14, 12, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 360, true, 0.5);
    }
}

//Attack12 
unsafe extern "C" fn effect_basyaamo_Attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("koopa_scratch"), Hash40::new("koopa_scratch"), Hash40::new("top"), 3, 10, 0, -12, 20, -4, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 14, 12, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 360, true, 0.5);
    }
}

//Attack13 
unsafe extern "C" fn effect_basyaamo_Attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 2, 6, -1, -35, -10, 0, 0.8, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("kneel"), -2, -1, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 360, true, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

//AttackDash
unsafe extern "C" fn effect_basyaamo_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.4);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 10, 12, -5, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.8);

        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 12, 12, -5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.4);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 2, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
}

//AttackS3Hi
unsafe extern "C" fn effect_basyaamo_AttackS3Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 8, 8, -70, -40, 50, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 8, 8, 0, -30, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
}

//AttackS3
unsafe extern "C" fn effect_basyaamo_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 8, 8, 100, -100, -112, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 8, 8, 0, -10, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
}

//AttackS3Lw
unsafe extern "C" fn effect_basyaamo_AttackS3Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 8, 12, 70, -100, 300, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 8, 8, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
}

//AttackHi3
unsafe extern "C" fn effect_basyaamo_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.4);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 10, 2, -5, -60, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 8, 2, -5, -60, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 23, 5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true, 0.6);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 2, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
}

//AttackLw3
unsafe extern "C" fn effect_basyaamo_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), -2, 5.5, 4, -9.2, -36, 175, 1.1, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
    } 
    else {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), -2, 5.5, 3, -11, -36, 168, 1.1, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
    }
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackS4
unsafe extern "C" fn effect_basyaamo_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("koopa_scratch"), Hash40::new("koopa_scratch"), Hash40::new("top"), -2, 12, 4, 4, 20, 220, 0.9, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 2, 10, -3, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("koopa_scratch"), Hash40::new("koopa_scratch"), Hash40::new("top"), 2, 10, 6, -12, 20, 40, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 2, 10, -3, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
    }
}

//AttackHi4
unsafe extern "C" fn effect_basyaamo_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("gaogaen_smash_arc"), Hash40::new("gaogaen_smash_arc"), Hash40::new("top"), 1, 16, 3, 0, 0, 100, 1, true, *EF_FLIP_YZ, 1);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackLw4
unsafe extern "C" fn effect_basyaamo_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.3, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.3, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("gaogaen_smash_arc"), Hash40::new("gaogaen_smash_arc"), Hash40::new("top"), 4, 12, 4, 0, 170, 100, 1, true, *EF_FLIP_YZ, 1);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("gaogaen_smash_arc"), Hash40::new("gaogaen_smash_arc"), Hash40::new("top"), 4, 12, 4, 180, 170, 100, 1, true, *EF_FLIP_YZ, 1);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("captain_fn_flash"), true, true);
    }
}

//AttackAirN
unsafe extern "C" fn effect_basyaamo_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.3, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("gaogaen_smash_arc"), Hash40::new("gaogaen_smash_arc"), Hash40::new("top"), 1, 12, 4, -90, 90, 100, 1, true, *EF_FLIP_YZ, 1);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("gaogaen_smash_arc"), Hash40::new("gaogaen_smash_arc"), Hash40::new("top"), 1, 12, 4, -90, 90, 100, 1, true, *EF_FLIP_YZ, 1);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("captain_fn_flash"), true, true);
    }
}

//AttackAirF 
unsafe extern "C" fn effect_basyaamo_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.3, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("gaogaen_smash_arc"), Hash40::new("gaogaen_smash_arc"), Hash40::new("top"), 1, 12, 4, 0, 0, -100, 1, true, *EF_FLIP_YZ, 1);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("captain_fn_flash"), true, true);
    }
}

//AttackAirB
unsafe extern "C" fn effect_basyaamo_AttackAirB(fighter: &mut L2CAgentBase) {
    let lr = PostureModule::lr(fighter.module_accessor);
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 10.0 * -lr, 7, 2, 180, 0, 0, 0.9, true, 0.75);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 12.0 * -lr, 7, 2, 180, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line"), true, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 10.0 * -lr, 12, 2, 180, 0, 0, 0.9, true, 0.75);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 12.0 * -lr, 12, 2, 180, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line"), true, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 10.0 * -lr, 9, 2, 170, 0, 0, 0.9, true, 0.75);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 12.0 * -lr, 9, 2, 170, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
}

//AttackAirHi
unsafe extern "C" fn effect_basyaamo_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("gaogaen_smash_arc"), Hash40::new("gaogaen_smash_arc"), Hash40::new("top"), 4, 13, 2, 0, 60, 100, 1.2, true, *EF_FLIP_YZ, 1);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
}

//AttackAirLw
unsafe extern "C" fn effect_basyaamo_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 15, 2, 60, 0, 0, 1.1, true, 0.75);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 2, 15, 4, 60, 0, 0, 1.1, true, 0.75);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -4, 0, -120, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line"), true, true);
    }
}

//CatchDash
unsafe extern "C" fn effect_basyaamo_CatchDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//CatchTurn
unsafe extern "C" fn effect_basyaamo_CatchTurn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//ThrowF
unsafe extern "C" fn effect_basyaamo_ThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 0.4, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 8, 16, -3, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 8, 10, -5, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("toel"), 2, 5, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("captain_fn_flash"), true, true);
    }
}

//ThrowB
unsafe extern "C" fn effect_basyaamo_ThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), 5, 8, -5, 0, 100, 30, 0.7, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 5, 0, 0, 0, 180, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 0.4, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -3, 16, 2, -160, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), -5, 10, 5, -160, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("toel"), -2, 5, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("captain_fn_flash"), true, true);
    }
}

//ThrowHi
unsafe extern "C" fn effect_basyaamo_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.4, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), -5, 7, 2, -70, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 5, 10, 2, -70, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("havel"), 2, 4, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("captain_fn_flash"), true, true);
    }
}      

//ThrowLw
unsafe extern "C" fn effect_basyaamo_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -10, 16, 5, 80, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 11, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 0.4, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("gaogaen_smash_arc"), Hash40::new("gaogaen_smash_arc"), Hash40::new("top"), 0, 12, 4, 180, 200, 100, 0.9, true, *EF_FLIP_YZ, 1);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("captain_fn_flash"), true, true);
    }
}

//CliffAttack
unsafe extern "C" fn effect_basyaamo_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("koopa_scratch"), Hash40::new("koopa_scratch"), Hash40::new("top"), 3, 8, 0, 0, -70, -110, 1.1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//CliffClimb
unsafe extern "C" fn effect_basyaamo_CliffClimb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//CliffEscape
unsafe extern "C" fn effect_basyaamo_CliffEscape(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//SlipAttack
unsafe extern "C" fn effect_basyaamo_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -1, 8, -3, 7, 0, 0, 1, true, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_c"), Hash40::new("top"), 1, 5, -1, -17, -103, 37, 1, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 6, 14, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

//DownAttackD
unsafe extern "C" fn effect_basyaamo_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 0.4, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), 5, 8, 0, 0, -80, -100, 0.9, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("captain_fn_flash"), true, true);
    }
}

//DownForwardD
unsafe extern "C" fn effect_basyaamo_DownForwardD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//DownBackD
unsafe extern "C" fn effect_basyaamo_DownBackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 4, 0, 0, 0, 180, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//DownAttackU
unsafe extern "C" fn effect_basyaamo_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 0.4, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), 12, 3, -6, 0, 80, 160, 0.7, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), -12, 3, 0, 0, -80, -160, 0.8, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("captain_fn_flash"), true, true);
    }
}

//DownForwardU
unsafe extern "C" fn effect_basyaamo_DownForwardU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//DownBackU
unsafe extern "C" fn effect_basyaamo_DownBackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 5, 0, 0, 0, 180, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialN
unsafe extern "C" fn effect_basyaamo_SpecialN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fk"), Hash40::new("top"), 0, 12, 12, -90, 0, 160, 0.7, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

//SpecialNOverheat
unsafe extern "C" fn effect_basyaamo_SpecialNOverheat(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fk"), Hash40::new("top"), 0, 12, 12, -90, 0, 160, 0.7, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("plizardon_flare_blitz"), Hash40::new("plizardon_flare_blitz"), Hash40::new("havel"), 0, 12, 15, 90, 0, 0, 1.0, true, *EF_FLIP_NONE);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.3, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        EFFECT_OFF_KIND(fighter, Hash40::new("captain_fk"), true, true);
    }
}

//SpecialAirN
unsafe extern "C" fn effect_basyaamo_SpecialAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fk"), Hash40::new("top"), 0, 12, 12, -90, 0, 160, 0.7, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

//SpecialS
unsafe extern "C" fn effect_basyaamo_SpecialS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("plizardon_flare_blitz"), Hash40::new("plizardon_flare_blitz"), Hash40::new("hip"), 0, 12, 0, 0, 90, 0, 1.0, true, *EF_FLIP_NONE);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

//SpecialAirS
unsafe extern "C" fn effect_basyaamo_SpecialAirS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("plizardon_flare_blitz"), Hash40::new("plizardon_flare_blitz"), Hash40::new("hip"), 0, 12, 0, 0, 90, 0, 1.0, true, *EF_FLIP_NONE);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

//SpecialHi
unsafe extern "C" fn effect_basyaamo_SpecialHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 4, -4, -4, -80, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
}

//SpecialAirHi
unsafe extern "C" fn effect_basyaamo_SpecialAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 4, -4, -4, -80, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
}

//SpecialHiOverheat
unsafe extern "C" fn effect_basyaamo_SpecialHiOverheat(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 4, -4, -4, -80, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("captain_fn_flash"), true, true);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("eflame_promrevolt_firepillar"), Hash40::new("top"), 0, -52, 0, 0, 0, 0, 2.4, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("eflame_promrevolt_firepillar_ground"), Hash40::new("top"), 0, -52, 0, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
}

//SpecialLw
unsafe extern "C" fn effect_basyaamo_SpecialLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 8, 6, -3, -60, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 5, -2, -5, -60, 0, 0, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.7, 0.4, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
}

//SpecialLw2
unsafe extern "C" fn effect_basyaamo_SpecialLw2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//SpecialLwLoop
unsafe extern "C" fn effect_basyaamo_SpecialLwLoop(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        let lr = PostureModule::lr(fighter.module_accessor);
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fk"), Hash40::new("top"), 6.0 * lr, 9, 7, -130, 0, 160, 0.7, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

//SpecialLwLanding
unsafe extern "C" fn effect_basyaamo_SpecialLwLanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bomb_c"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("eflame_promrevolt_firepillar"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("eflame_promrevolt_firepillar_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

//AppealHiR
unsafe extern "C" fn effect_basyaamo_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.2, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("captain_fn_flash"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.2, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("captain_fn_flash"), true, true);
    }
}

//AppealHiL
unsafe extern "C" fn effect_basyaamo_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.2, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("captain_fn_flash"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("captain_fn_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.2, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("captain_fn_flash"), true, true);
    }
}

//AppealSR
unsafe extern "C" fn effect_basyaamo_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("koopa_scratch"), Hash40::new("koopa_scratch"), Hash40::new("top"), 3, 2, 8, 180, 90, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.2);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("koopa_scratch"), true, true);
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 120.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//AppealSL
unsafe extern "C" fn effect_basyaamo_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("koopa_scratch"), Hash40::new("koopa_scratch"), Hash40::new("top"), 3, 2, 8, 180, 90, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.2);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("koopa_scratch"), true, true);
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 120.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//AppealLwR
unsafe extern "C" fn effect_basyaamo_AppealLwR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("captain_fp_flash"), Hash40::new("top"), 0, 14, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_appeal_hi"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 0.9, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("captain_fp_flash"), Hash40::new("top"), 0, 14, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        FLASH(fighter, 0.921, 0.087, 0, 0.5);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_appeal_hi"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 0.9, true);
        COL_NORMAL(fighter);
        BURN_COLOR(fighter, 2, 0.301, 0.02, 0.8);
    }
    wait(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        BURN_COLOR_FRAME(fighter, 10, 1, 1, 1, 0);
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        BURN_COLOR_NORMAL(fighter);
    }
}

//AppealLwL
unsafe extern "C" fn effect_basyaamo_AppealLwL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("captain_fp_flash"), Hash40::new("top"), 0, 14, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_appeal_hi"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 0.9, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("captain_fp_flash"), Hash40::new("top"), 0, 14, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        FLASH(fighter, 0.921, 0.087, 0, 0.5);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("captain_appeal_hi"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 0.9, true);
        COL_NORMAL(fighter);
        BURN_COLOR(fighter, 2, 0.301, 0.02, 0.8);
    }
    wait(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        BURN_COLOR_FRAME(fighter, 10, 1, 1, 1, 0);
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        BURN_COLOR_NORMAL(fighter);
    }
}

//EntryR
unsafe extern "C" fn effect_basyaamo_EntryR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_mball"), Hash40::new("throw"), 0, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.9, 1, 1, 0.9);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//EntryL
unsafe extern "C" fn effect_basyaamo_EntryL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_mball"), Hash40::new("throw"), 0, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.9, 1, 1, 0.9);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//PassiveStandF
unsafe extern "C" fn effect_basyaamo_PassiveStandF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

//PassiveStandB
unsafe extern "C" fn effect_basyaamo_PassiveStandB(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    Agent::new("captain")
    .effect_acmd("effect_attack11_basyaamo", effect_basyaamo_Attack11, Low)
    .effect_acmd("effect_attack12_basyaamo", effect_basyaamo_Attack12, Low)
    .effect_acmd("effect_attack13_basyaamo", effect_basyaamo_Attack13, Low)
    .effect_acmd("effect_attackdash_basyaamo", effect_basyaamo_AttackDash, Low)
    .effect_acmd("effect_attacks3hi_basyaamo", effect_basyaamo_AttackS3Hi, Low)
    .effect_acmd("effect_attacks3_basyaamo", effect_basyaamo_AttackS3, Low)
    .effect_acmd("effect_attacks3lw_basyaamo", effect_basyaamo_AttackS3Lw, Low)
    .effect_acmd("effect_attackhi3_basyaamo", effect_basyaamo_AttackHi3, Low)
    .effect_acmd("effect_attacklw3_basyaamo", effect_basyaamo_AttackLw3, Low)
    .effect_acmd("effect_attacks4_basyaamo", effect_basyaamo_AttackS4, Low)
    .effect_acmd("effect_attackhi4_basyaamo", effect_basyaamo_AttackHi4, Low)
    .effect_acmd("effect_attacklw4_basyaamo", effect_basyaamo_AttackLw4, Low)
    .effect_acmd("effect_attackairn_basyaamo", effect_basyaamo_AttackAirN, Low)
    .effect_acmd("effect_attackairf_basyaamo", effect_basyaamo_AttackAirF, Low)    
    .effect_acmd("effect_attackairb_basyaamo", effect_basyaamo_AttackAirB, Low)
    .effect_acmd("effect_attackairhi_basyaamo", effect_basyaamo_AttackAirHi, Low)
    .effect_acmd("effect_attackairlw_basyaamo", effect_basyaamo_AttackAirLw, Low)
    .effect_acmd("effect_catchdash_basyaamo", effect_basyaamo_CatchDash, Low)
    .effect_acmd("effect_catchturn_basyaamo", effect_basyaamo_CatchTurn, Low)
    .effect_acmd("effect_throwf_basyaamo", effect_basyaamo_ThrowF, Low)
    .effect_acmd("effect_throwb_basyaamo", effect_basyaamo_ThrowB, Low)
    .effect_acmd("effect_throwhi_basyaamo", effect_basyaamo_ThrowHi, Low)
    .effect_acmd("effect_throwlw_basyaamo", effect_basyaamo_ThrowLw, Low)
    .effect_acmd("effect_downattackd_basyaamo", effect_basyaamo_DownAttackD, Low)
    .effect_acmd("effect_downforwardd_basyaamo", effect_basyaamo_DownForwardD, Low)
    .effect_acmd("effect_downbackd_basyaamo", effect_basyaamo_DownBackD, Low)
    .effect_acmd("effect_downattacku_basyaamo", effect_basyaamo_DownAttackU, Low)
    .effect_acmd("effect_downforwardu_basyaamo", effect_basyaamo_DownForwardU, Low)
    .effect_acmd("effect_downbacku_basyaamo", effect_basyaamo_DownBackU, Low)
    .effect_acmd("effect_cliffattack_basyaamo", effect_basyaamo_CliffAttack, Low)
    .effect_acmd("effect_cliffclimb_basyaamo", effect_basyaamo_CliffClimb, Low)
    .effect_acmd("effect_cliffescape_basyaamo", effect_basyaamo_CliffEscape, Low)
    .effect_acmd("effect_slipattack_basyaamo", effect_basyaamo_SlipAttack, Low)
    .effect_acmd("effect_specialn_basyaamo", effect_basyaamo_SpecialN, Low)
    .effect_acmd("effect_specialairn_basyaamo", effect_basyaamo_SpecialAirN, Low)
    .effect_acmd("effect_specialnoverheat_basyaamo", effect_basyaamo_SpecialNOverheat, Low)
    .effect_acmd("effect_specials_basyaamo", effect_basyaamo_SpecialS, Low)
    .effect_acmd("effect_specialairs_basyaamo", effect_basyaamo_SpecialAirS, Low)
    .effect_acmd("effect_specialhi_basyaamo", effect_basyaamo_SpecialHi, Low)
    .effect_acmd("effect_specialairhi_basyaamo", effect_basyaamo_SpecialAirHi, Low)
    .effect_acmd("effect_specialhioverheat_basyaamo", effect_basyaamo_SpecialHiOverheat, Low)
    .effect_acmd("effect_speciallw_basyaamo", effect_basyaamo_SpecialLw, Low)
    .effect_acmd("effect_speciallw2_basyaamo", effect_basyaamo_SpecialLw2, Low)
    .effect_acmd("effect_speciallwloop_basyaamo", effect_basyaamo_SpecialLwLoop, Low)
    .effect_acmd("effect_speciallwlanding_basyaamo", effect_basyaamo_SpecialLwLanding, Low)
    .effect_acmd("effect_appealsr_basyaamo", effect_basyaamo_AppealSR, Low)
    .effect_acmd("effect_appealsl_basyaamo", effect_basyaamo_AppealSL, Low)
    .effect_acmd("effect_appealhir_basyaamo", effect_basyaamo_AppealHiR, Low)
    .effect_acmd("effect_appealhil_basyaamo", effect_basyaamo_AppealHiL, Low)
    .effect_acmd("effect_appeallwr_basyaamo", effect_basyaamo_AppealLwR, Low)
    .effect_acmd("effect_appeallwl_basyaamo", effect_basyaamo_AppealLwL, Low)
    .effect_acmd("effect_entryr_basyaamo", effect_basyaamo_EntryR, Low)
    .effect_acmd("effect_entryl_basyaamo", effect_basyaamo_EntryL, Low)
    .effect_acmd("effect_passivestandf_basyaamo", effect_basyaamo_PassiveStandF, Low)
    .effect_acmd("effect_passivestandb_basyaamo", effect_basyaamo_PassiveStandB, Low)
    .install();
}