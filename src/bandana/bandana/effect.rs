use crate::imports::BuildImports::*;

//Attack11
unsafe extern "C" fn effect_bandana_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 3, 7.5, 5, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 71.0 / 255.0, 31.0 / 255.0, 0.0);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 2.25, 17.5, 0, 0, 0, 0.6, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
}

//Attack12
unsafe extern "C" fn effect_bandana_Attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2.5, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 5, 6.5, -15, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 71.0 / 255.0, 31.0 / 255.0, 0.0);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9.2, 17, 0, 0, 0, 0.6, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
}

//Attack13
unsafe extern "C" fn effect_bandana_Attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2.5, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 4, 10, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 71.0 / 255.0, 31.0 / 255.0, 0.0);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4, 18, 0, 0, 0, 0.68, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
}

//Attack100 
unsafe extern "C" fn effect_bandana_Attack100(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("buddy_attack100"), Hash40::new("top"), 0, 5, 3, 0, 0, 0, 0.85, true);
    }
    frame(fighter.lua_state_agent, 1.0);
    loop {
        if is_excute(fighter.lua_state_agent) {
            EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 15, 11, 0, 0, 0, 0, 0.5, 5, 4, 2, 0, 0, 360, false);
            LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        }
        wait(fighter.lua_state_agent, 5.0);
        if is_excute(fighter.lua_state_agent) {
            EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 16.5, 5.55, 0, 0, 0, 0, 0.55, 5, 4, 2, 0, 0, 360, false);
            LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        }
        wait(fighter.lua_state_agent, 5.0);
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 15, 2, 0, 0, 0, 0, 0.5, 6, 4, 2, 0, 0, 360, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
}

//Attack100End
unsafe extern "C" fn effect_bandana_Attack100End(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("buddy_attack100"), true, true);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -1, 8.5, -90, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 71.0 / 255.0, 31.0 / 255.0, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.15);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_speedline"), true, true);
    }
}

//AttackDash
unsafe extern "C" fn effect_bandana_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 4, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 9, 5.5, 0, 210, 0, 1.0, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        match WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 {
            2 => LAST_EFFECT_SET_COLOR(fighter, 30.0 / 255.0, 1.0, 0.0), //Green
            3 => LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 33.0 / 255.0), //Pink
            4 => LAST_EFFECT_SET_COLOR(fighter, 118.0 / 255.0, 0.0, 255.0/255.0), //Purple
            5 => LAST_EFFECT_SET_COLOR(fighter, 51.0 / 255.0, 54.0 / 255.0, 62.0/255.0), //Grey
            _ => LAST_EFFECT_SET_COLOR(fighter, 1.0, 100.0 / 255.0, 0.0) //Yellow
        }
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 9, -1.5, 0, 30, 0, 0.95, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        match WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 {
            2 => LAST_EFFECT_SET_COLOR(fighter, 30.0 / 255.0, 1.0, 0.0), //Green
            3 => LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 33.0 / 255.0), //Pink
            4 => LAST_EFFECT_SET_COLOR(fighter, 118.0 / 255.0, 0.0, 255.0/255.0), //Purple
            5 => LAST_EFFECT_SET_COLOR(fighter, 51.0 / 255.0, 54.0/255.0, 62.0/255.0), //Grey
            _ => LAST_EFFECT_SET_COLOR(fighter, 1.0, 100.0 / 255.0, 0.0) //Yellow
        }
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, false, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 9, 5.5, 0, 210, 0, 1.0, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        match WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 {
            2 => LAST_EFFECT_SET_COLOR(fighter, 30.0 / 255.0, 1.0, 0.0), //Green
            3 => LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 33.0 / 255.0), //Pink
            4 => LAST_EFFECT_SET_COLOR(fighter, 118.0 / 255.0, 0.0, 255.0/255.0), //Purple
            5 => LAST_EFFECT_SET_COLOR(fighter, 51.0 / 255.0, 54.0/255.0, 62.0/255.0), //Grey
            _ => LAST_EFFECT_SET_COLOR(fighter, 1.0, 100.0 / 255.0, 0.0) //Yellow
        }
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 9, -1.5, 0, 30, 0, 0.95, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        match WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 {
            2 => LAST_EFFECT_SET_COLOR(fighter, 30.0 / 255.0, 1.0, 0.0), //Green
            3 => LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 33.0 / 255.0), //Pink
            4 => LAST_EFFECT_SET_COLOR(fighter, 118.0 / 255.0, 0.0, 255.0/255.0), //Purple
            5 => LAST_EFFECT_SET_COLOR(fighter, 51.0 / 255.0, 54.0/255.0, 62.0/255.0), //Grey
            _ => LAST_EFFECT_SET_COLOR(fighter, 1.0, 100.0 / 255.0, 0.0) //Yellow
        }
    }
    frame(fighter.lua_state_agent, 51.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 3, 0, 0, 0, false);
    }
}

//AttackS3Hi
unsafe extern "C" fn effect_bandana_AttackS3Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear3"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick2"), -2.5, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 6, 10, -17, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 71.0 / 255.0, 31.0 / 255.0, 0.0);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9.5, 24, 0, 0, 0, 0.68, false);
    }   
}

//AttackS3
unsafe extern "C" fn effect_bandana_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear3"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick2"), -2.5, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 6, 10, -17, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 71.0 / 255.0, 31.0 / 255.0, 0.0);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9.5, 24, 0, 0, 0, 0.68, false);
    }
}

//AttackS3Lw
unsafe extern "C" fn effect_bandana_AttackS3Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear3"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick2"), -2.5, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 4, 10, 12, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 71.0 / 255.0, 31.0 / 255.0, 0.0);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 2, 23.5, 0, 0, 0, 0.68, false);
    }
}

//AttackHi3
unsafe extern "C" fn effect_bandana_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear3"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick2"), -2.5, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 14, 0, -90, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 71.0 / 255.0, 31.0 / 255.0, 0.0);
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -0.0, 27.2, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
    }
}

//AttackLw3
unsafe extern "C" fn effect_bandana_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -1, 0, 0, 0, 0.8, 0, 0, 4, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 4, 0, -1, 0, 0, 0, 0.5, 0, 0, 3, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 7, 0, -1, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 7, 0, -1, 0, 0, 0, 0.55, 0, 0, 3, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 4, 0, -1, 0, 0, 0, 0.35, 0, 0, 3, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 4, 0, -1, 0, 0, 0, 0.4, 0, 0, 3, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
}

//AttackS4
unsafe extern "C" fn effect_bandana_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 11, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("kirby_star"), Hash40::new("top"), 6, 3, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.0, 0, 0, 3, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 4.6, 5.0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 1.0);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 4.4, 5.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 165.0 / 255.0, 0.0, 0.0);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("gekkouga_water_impact_air"), Hash40::new("top"), 0, 4.5, 33, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("top"), 0, 4.5, 33, 0, 0, 90, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.75);
        EFFECT(fighter, Hash40::new("kamui_water_sibuki_s"), Hash40::new("top"), 0, 4.5, 33, 0, 180, 90, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.85);
    }
}

//AttackHi4
unsafe extern "C" fn effect_bandana_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 20.5, 4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        match WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 {
            1 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear4"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            2 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear5"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            3 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear6"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            4 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear6"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            5 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear7"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            6 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear8"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            _ => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear1"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1)
        }
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        LAST_EFFECT_SET_ALPHA(fighter, 0.2);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        LAST_EFFECT_SET_ALPHA(fighter, 0.2);
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 47.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
}

//AttackLw4
unsafe extern "C" fn effect_bandana_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 5, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.775, 0, 0, 3, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4.5, 30, 0, 0, 0, 0.9, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_ALPHA(fighter, Hash40::new("wolf_smash_hi"), Hash40::new("top"), 0, 6, 13.5, 90, 0, 0, 0.625, 0, 0, 0, 0, 0, 0, true, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
        EFFECT_ALPHA(fighter, Hash40::new("wolf_smash_hi"), Hash40::new("top"), 0, 6, 13.5, 90, 180, 0, 0.625, 0, 0, 0, 0, 0, 0, true, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.9, 0, 0, 3, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 3, -30, 0, 0, 0, 0.9, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_ALPHA(fighter, Hash40::new("wolf_smash_hi"), Hash40::new("top"), 0, 6, -13.5, -90, 0, 0, 0.625, 0, 0, 0, 0, 0, 0, true, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
        EFFECT_ALPHA(fighter, Hash40::new("wolf_smash_hi"), Hash40::new("top"), 0, 6, -13.5, -90, 180, 0, 0.625, 0, 0, 0, 0, 0, 0, true, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
}

//AttackAirN
unsafe extern "C" fn effect_bandana_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        match WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 {
            1 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear4"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            2 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear5"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            3 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear6"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            4 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear6"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            5 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear7"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            6 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear8"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            _ => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear1"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1)
        }
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
}

//AttackAirF
unsafe extern "C" fn effect_bandana_AttackAirF(fighter: &mut L2CAgentBase) {}

//AttackAirB
unsafe extern "C" fn effect_bandana_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 2.25, 3.25, -6.7, 180, 0, 0, 0.9, true);
        match WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 {
            2 => LAST_EFFECT_SET_COLOR(fighter, 30.0 / 255.0, 1.0, 0.0), 
            3 => LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 33.0 / 255.0), 
            4 => LAST_EFFECT_SET_COLOR(fighter, 118.0 / 255.0, 0.0, 255.0/255.0), 
            5 => LAST_EFFECT_SET_COLOR(fighter, 51.0 / 255.0, 54.0/255.0, 62.0 / 255.0),
            _ => LAST_EFFECT_SET_COLOR(fighter, 1.0, 100.0 / 255.0, 0.0) 
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 3, -16, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
}        

//AttackAirHi
unsafe extern "C" fn effect_bandana_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("kirby_star"), Hash40::new("top"), 0, 5, 1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 21, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.6);
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 21, 1, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        LAST_EFFECT_SET_COLOR(fighter, 165.0/255.0, 0.0, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_sweat"), Hash40::new("top"), 0, 20, 0, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_sweat"), Hash40::new("top"), 0, 20, 0, 0, 0, 0, 0.5, true);
    }
}

//AttackAirLw
unsafe extern "C" fn effect_bandana_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear3"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick2"), -2.5, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -8, 1.0, 0, 0, 0, 0.8, 1, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("littlemac_risinguppercut"), Hash40::new("top"), 0, 0, 0, 180, 0, 0, 1.35, true);
        EFFECT_FOLLOW(fighter, Hash40::new("edge_gokumon_wind"), Hash40::new("top"), -2.2, 25, 1.7, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_sword_light3"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("edge_gokumon_speedline"), Hash40::new("top"), 0, 0, 0, 0, 46, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_light3"), Hash40::new("top"), 0, 25, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_sword_flash2"), -1);
    }
    frame(fighter.lua_state_agent, 64.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("littlemac_risinguppercut"), false, false);
    }
}

//LandingAirLw
unsafe extern "C" fn effect_bandana_LandingAirLw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("littlemac_risinguppercut"), true, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}

//CatchAttack
unsafe extern "C" fn effect_bandana_CatchAttack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -4, 7, -1, -10, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.5);
        EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), -4, 8, 10, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
}

//ThrowF
unsafe extern "C" fn effect_bandana_ThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.7, 0, 0, 3, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3.4, 0, 0, 0, 0, 0, 1.0, 0, 0, 3, 0, 0, 0, false);
    }
}  

//ThrowB
unsafe extern "C" fn effect_bandana_ThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}  

//ThrowHi
unsafe extern "C" fn effect_bandana_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 3, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.93, 0, 0, 3, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("peach_back_atk_heart"), Hash40::new("top"), 0, 19, 1.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("peach_back_atk"), Hash40::new("top"), 1, 19, 1.5, 0, 0, 0, 1.5, true);
    }
}   

//ThrowLw
unsafe extern "C" fn effect_bandana_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//CliffAttack
unsafe extern "C" fn effect_bandana_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 1, 8, -8, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 4, 2.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 3, 13, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 360, true);
    }
}

//SlipAttack
unsafe extern "C" fn effect_bandana_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_XY);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 6, 2.5, 358, -5, -175, 0.85, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_XY);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind_s"), Hash40::new("sys_spin_wind_s"), Hash40::new("rot"), 0, -4, 0, -7, 0, 0, 1.15, true, *EF_FLIP_YZ, 0.3);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 5, 0, 191, 18, 13, 0.85, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
}

//DownAttackD
unsafe extern "C" fn effect_bandana_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 2);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -8, 4, 2.5, 0, 160, 0, 1.1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 2.5, -16, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 5, 5, 0, -50, -90, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

//DownAttackU
unsafe extern "C" fn effect_bandana_DownAttackU(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -8, 4.5, -3, 0, 20, 0, 1.2, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4, 17, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 6, -2, 0, 150, 195, 1.2, true, *EF_FLIP_YZ);
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_XY);
    }
}

//SpecialNStart
unsafe extern "C" fn effect_bandana_SpecialNStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    for _ in 0..14 {
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.7, 20, 0, 12, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 9.0);
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, 2, 3, 3, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("kirby_star"), Hash40::new("top"), -7, 7, 2, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 100.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
    }
    frame(fighter.lua_state_agent, 136.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 0, 0, 4, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 137.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 14, 6, 0, 0, 0, 1.0, 1, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 149.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialAirNStart
unsafe extern "C" fn effect_bandana_SpecialAirNStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    for _ in 0..14 {
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.7, 20, 0, 12, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 9.0);
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("stick"), 0, 0, 0, 0, 0, 0, 1, 2, 3, 3, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("kirby_star"), Hash40::new("top"), -7, 7, 2, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 100.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 1.5, 0, 0, 0, 0, 0.33, true);
    }
    frame(fighter.lua_state_agent, 136.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 0, 0, 4, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 137.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 14, 6, 0, 0, 0, 1.0, 1, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 149.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialN1
unsafe extern "C" fn effect_bandana_SpecialN1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 0, 0, 3, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 14, 6, 0, 0, 0, 0.8, 1, 0, 0, 0, 0, 0, true);
    }
}

//SpecialAirN1
unsafe extern "C" fn effect_bandana_SpecialAirN1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 0, 0, 3, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 14, 6, 0, 0, 0, 0.8, 1, 0, 0, 0, 0, 0, true);
    }
}

//SpecialN2
unsafe extern "C" fn effect_bandana_SpecialN2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 0, 0, 3, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 14, 6, 0, 0, 0, 0.9, 1, 0, 0, 0, 0, 0, true);
    }
}

//SpecialAirN2
unsafe extern "C" fn effect_bandana_SpecialAirN2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 0, 0, 3, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 14, 6, 0, 0, 0, 0.9, 1, 0, 0, 0, 0, 0, true);
    }
}

//SpecialS
unsafe extern "C" fn effect_bandana_SpecialS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ness_pkfl_hold"), Hash40::new("stick2"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("kirby_star"), Hash40::new("top"), 4, 3.5, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ness_pkfl_hold"), true, true);
    }
}

//SpecialAirS
unsafe extern "C" fn effect_bandana_SpecialAirS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ness_pkfl_hold"), Hash40::new("stick2"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("kirby_star"), Hash40::new("top"), 4, 3.5, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ness_pkfl_hold"), true, true);
    }
}

//SpecialSHold
unsafe extern "C" fn effect_bandana_SpecialSHold(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("stick2"), 0, 0, 0, 0, 0, 0, 0.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("ness_pkfl_hold"), Hash40::new("stick2"), 0, 0, 0, 0, 0, 0, 0.4, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.9, 10, 0, 3, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.9, 10, 0, 3, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.9, 10, 0, 3, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.9, 10, 0, 3, 0, 0, 0, false);
    }
}

//SpecialAirSHold
unsafe extern "C" fn effect_bandana_SpecialAirSHold(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("stick2"), 0, 0, 0, 0, 0, 0, 0.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("ness_pkfl_hold"), Hash40::new("stick2"), 0, 0, 0, 0, 0, 0, 0.4, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.7, 10, 0, 3, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.7, 10, 0, 3, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.7, 10, 0, 3, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.7, 10, 0, 3, 0, 0, 0, false);
    }
}

//SpecialHi
unsafe extern "C" fn effect_bandana_SpecialHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        match WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 {
            1 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear4"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            2 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear5"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            3 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear6"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            4 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear6"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            5 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear7"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            6 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear8"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            _ => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear1"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1)
        }
    }
    frame(fighter.lua_state_agent, 105.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
}

//SpecialAirHi
unsafe extern "C" fn effect_bandana_SpecialAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        match WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 {
            1 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear4"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            2 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear5"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            3 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear6"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            4 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear6"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            5 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear7"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            6 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear8"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            _ => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear1"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1)
        }
    }
    frame(fighter.lua_state_agent, 105.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
}

//SpecialLw
unsafe extern "C" fn effect_bandana_SpecialLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 7, 12.0, 0, 0, 0, 0.92, 1, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 4, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_trace"), Hash40::new("top"), -3, 8, 16.5, 0, 0, 0, 1.35, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_trace"), Hash40::new("top"), 0, 8, -1, 0, 0, 0, 1.2, true);
    }
    frame(fighter.lua_state_agent, 47.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_warpstar_trace"), true, true);
    }
}

//SpecialAirLw
unsafe extern "C" fn effect_bandana_SpecialAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -10, 9, 0, 0, 0, 0.92, 1, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_trace"), Hash40::new("stick2"), 0, 0, 0, 0, 0, 0, 1.35, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_warpstar_trace"), Hash40::new("top"), -1, 8.5, -1.5, 0, 0, 0, 1.2, true);
    }
    frame(fighter.lua_state_agent, 47.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_warpstar_trace"), true, true);
    }
}

//SpecialAirLwLand
unsafe extern "C" fn effect_bandana_SpecialAirLwLand(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_warpstar_trace"), true, true);
        EFFECT(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_warpstar_break"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
}

//AppealHiR
unsafe extern "C" fn effect_bandana_AppealHiR(fighter: &mut L2CAgentBase) {}

//AppealHiL
unsafe extern "C" fn effect_bandana_AppealHiL(fighter: &mut L2CAgentBase) {}

//AppealSR
unsafe extern "C" fn effect_bandana_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
}

//AppealSL
unsafe extern "C" fn effect_bandana_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
}

//AppealLwR
unsafe extern "C" fn effect_bandana_AppealLwR(fighter: &mut L2CAgentBase) {}

//AppealLwL
unsafe extern "C" fn effect_bandana_AppealLwL(fighter: &mut L2CAgentBase) {}

//FinalStart
unsafe extern "C" fn effect_bandana_FinalStart(fighter: &mut L2CAgentBase) { 
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bg_black"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("haver"), 0.75, -1, 0, 0, -45, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_r"), Hash40::new("haver"), 0.75, -1, 0, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 70.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -11, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
} 

//FinalHold
unsafe extern "C" fn effect_bandana_FinalHold(fighter: &mut L2CAgentBase) { 
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("haver"), 0.75, -1, 0, 0, -45, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_r"), Hash40::new("haver"), 0.75, -1, 0, 0, 0, 0, 1.0, true);
    }
    for _ in 0..4 {
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 18, 0, 90, 0, 0, 1.0, true, *EF_FLIP_YZ, 1.0);
        }
        wait(fighter.lua_state_agent, 2.0);
    }
} 

//FinalEnd
unsafe extern "C" fn effect_bandana_FinalEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("haver"), 0.75, -1, 0, 0, -45, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_r"), Hash40::new("haver"), 0.75, -1, 0, 0, 0, 0, 1.0, true);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 20, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 30, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 40, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 50, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 60, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 70, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 80, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), -20, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), -30, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), -40, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), -50, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), -60, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), -70, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), -80, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_bullet_r"), false, false);
    }
} 

//FinalAirStart
unsafe extern "C" fn effect_bandana_FinalAirStart(fighter: &mut L2CAgentBase) { 
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bg_black"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("haver"), 0.75, -1, 0, 0, -45, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_r"), Hash40::new("haver"), 0.75, -1, 0, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 70.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -11, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
} 

//FinalAirHold
unsafe extern "C" fn effect_bandana_FinalAirHold(fighter: &mut L2CAgentBase) { 
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("haver"), 0.75, -1, 0, 0, -45, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_r"), Hash40::new("haver"), 0.75, -1, 0, 0, 0, 0, 1.0, true);
    }
    for _ in 0..4 {
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 18, 0, 90, 0, 0, 1.0, true, *EF_FLIP_YZ, 1.0);
        }
        wait(fighter.lua_state_agent, 2.0);
    }
} 

//FinalAirEnd
unsafe extern "C" fn effect_bandana_FinalAirEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("haver"), 0.75, -1, 0, 0, -45, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_r"), Hash40::new("haver"), 0.75, -1, 0, 0, 0, 0, 1.0, true);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 20, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 30, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 40, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 50, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 60, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 70, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), 80, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), -20, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), -30, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), -40, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), -50, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), -60, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), -70, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("master_smash_hi_smoke"), Hash40::new("top"), -80, 0, 0, 0, 0, 0, 5.0, 0.5, 0, 0.5, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_bullet_r"), false, false);
    }
} 

//EntryL
unsafe extern "C" fn effect_bandana_EntryL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        match WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 {
            1 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear4"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            2 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear5"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            3 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear6"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            4 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear6"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            5 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear7"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            6 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear8"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            _ => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear1"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1)
        }    
    }
    frame(fighter.lua_state_agent, 79.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
    frame(fighter.lua_state_agent, 83.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
}

//EntryR
unsafe extern "C" fn effect_bandana_EntryR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        match WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8 {
            1 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear4"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            2 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear5"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            3 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear6"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            4 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear6"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            5 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear7"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            6 => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear8"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1),
            _ => AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_bandana_spear1"), Hash40::new("tex_bandana_spear2"), 7, Hash40::new("stick"), 0.0, 0.0, 0.0, Hash40::new("stick2"), 0.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("stick"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1)
        }    
    }
    frame(fighter.lua_state_agent, 79.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
    frame(fighter.lua_state_agent, 83.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
}

//Win1
unsafe extern "C" fn effect_bandana_Win1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 71.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("trans"), 22.0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 109.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("trans"), -2.0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
    }
}

//Win2
unsafe extern "C" fn effect_bandana_Win2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 110.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("trans"), 9.5, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 163.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("trans"), 22.0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    Agent::new("edge")
    .effect_acmd("effect_attack11_bandana", effect_bandana_Attack11, Low)
    .effect_acmd("effect_attack12_bandana", effect_bandana_Attack12, Low)
    .effect_acmd("effect_attack13_bandana", effect_bandana_Attack13, Low)
    .effect_acmd("effect_attack100_bandana", effect_bandana_Attack100, Low)
    .effect_acmd("effect_attack100end_bandana", effect_bandana_Attack100End, Low)
    .effect_acmd("effect_attackdash_bandana", effect_bandana_AttackDash, Low)
    .effect_acmd("effect_attacks3hi_bandana", effect_bandana_AttackS3Hi, Low)
    .effect_acmd("effect_attacks3_bandana", effect_bandana_AttackS3, Low)
    .effect_acmd("effect_attacks3lw_bandana", effect_bandana_AttackS3Lw, Low)
    .effect_acmd("effect_attackhi3_bandana", effect_bandana_AttackHi3, Low)
    .effect_acmd("effect_attacklw3_bandana", effect_bandana_AttackLw3, Low)
    .effect_acmd("effect_attacks4_bandana", effect_bandana_AttackS4, Low)
    .effect_acmd("effect_attackhi4_bandana", effect_bandana_AttackHi4, Low)
    .effect_acmd("effect_attacklw4_bandana", effect_bandana_AttackLw4, Low)
    .effect_acmd("effect_attackairn_bandana", effect_bandana_AttackAirN, Low)
    .effect_acmd("effect_attackairf_bandana", effect_bandana_AttackAirF, Low) 
    .effect_acmd("effect_attackairb_bandana", effect_bandana_AttackAirB, Low)
    .effect_acmd("effect_attackairhi_bandana", effect_bandana_AttackAirHi, Low)
    .effect_acmd("effect_attackairlw_bandana", effect_bandana_AttackAirLw, Low)
    .effect_acmd("effect_landingairlw_bandana", effect_bandana_LandingAirLw, Low)
    .effect_acmd("effect_catchattack_bandana", effect_bandana_CatchAttack, Low)
    .effect_acmd("effect_throwf_bandana", effect_bandana_ThrowF, Low)
    .effect_acmd("effect_throwb_bandana", effect_bandana_ThrowB, Low)
    .effect_acmd("effect_throwhi_bandana", effect_bandana_ThrowHi, Low)
    .effect_acmd("effect_throwlw_bandana", effect_bandana_ThrowLw, Low)
    .effect_acmd("effect_downattackd_bandana", effect_bandana_DownAttackD, Low)
    .effect_acmd("effect_downattacku_bandana", effect_bandana_DownAttackU, Low)
    .effect_acmd("effect_cliffattack_bandana", effect_bandana_CliffAttack, Low)
    .effect_acmd("effect_slipattack_bandana", effect_bandana_SlipAttack, Low)
    .effect_acmd("effect_specialnstart_bandana", effect_bandana_SpecialNStart, Low)
    .effect_acmd("effect_specialairnstart_bandana", effect_bandana_SpecialAirNStart, Low)
    .effect_acmd("effect_specialn1_bandana", effect_bandana_SpecialN1, Low)
    .effect_acmd("effect_specialairn1_bandana", effect_bandana_SpecialAirN1, Low)
    .effect_acmd("effect_specialn2_bandana", effect_bandana_SpecialN2, Low)
    .effect_acmd("effect_specialairn2_bandana", effect_bandana_SpecialAirN2, Low)
    .effect_acmd("effect_specials_bandana", effect_bandana_SpecialS, Low)
    .effect_acmd("effect_specialairs_bandana", effect_bandana_SpecialAirS, Low)
    .effect_acmd("effect_specialshold_bandana", effect_bandana_SpecialSHold, Low)
    .effect_acmd("effect_specialairshold_bandana", effect_bandana_SpecialAirSHold, Low)
    .effect_acmd("effect_specialhi_bandana", effect_bandana_SpecialHi, Low)
    .effect_acmd("effect_speciallw_bandana", effect_bandana_SpecialLw, Low)
    .effect_acmd("effect_specialairlw_bandana", effect_bandana_SpecialAirLw, Low)
    .effect_acmd("effect_specialairlwland_bandana", effect_bandana_SpecialAirLwLand, Low)
    .effect_acmd("effect_appealsr_bandana", effect_bandana_AppealSR, Low)
    .effect_acmd("effect_appealsl_bandana", effect_bandana_AppealSL, Low)
    .effect_acmd("effect_appealhir_bandana", effect_bandana_AppealHiR, Low)
    .effect_acmd("effect_appealhil_bandana", effect_bandana_AppealHiL, Low)
    .effect_acmd("effect_appeallwr_bandana", effect_bandana_AppealLwR, Low)
    .effect_acmd("effect_appeallwl_bandana", effect_bandana_AppealLwL, Low)
    .effect_acmd("effect_finalstart_bandana", effect_bandana_FinalStart, Low)
    .effect_acmd("effect_finalhold_bandana", effect_bandana_FinalHold, Low)
    .effect_acmd("effect_finalend_bandana", effect_bandana_FinalEnd, Low)
    .effect_acmd("effect_finalairstart_bandana", effect_bandana_FinalAirStart, Low)
    .effect_acmd("effect_finalairhold_bandana", effect_bandana_FinalAirHold, Low)
    .effect_acmd("effect_finalairend_bandana", effect_bandana_FinalAirEnd, Low)
    .effect_acmd("effect_entryl_bandana", effect_bandana_EntryL, Low)
    .effect_acmd("effect_entryr_bandana", effect_bandana_EntryR, Low)
    .effect_acmd("effect_win1_bandana", effect_bandana_Win1, Low)
    .effect_acmd("effect_win2_bandana", effect_bandana_Win2, Low)
    .install();
}