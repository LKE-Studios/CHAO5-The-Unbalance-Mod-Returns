use crate::imports::BuildImports::*;

//EntryR
unsafe extern "C" fn effect_samusd_EntryR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_water_walk"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.8);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        ColorBlendModule::set_disable_camera_depth_influence(fighter.module_accessor, true);
        FLASH(fighter, 0.012, 0.012, 0.032, 0.9);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 25, 0.012, 0.012, 0.032, 0);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_ripple"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.8);
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
        ColorBlendModule::set_disable_camera_depth_influence(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 75.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_splash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.8);
    }
}

//EntryL
unsafe extern "C" fn effect_samusd_EntryL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_water_walk"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.8);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        ColorBlendModule::set_disable_camera_depth_influence(fighter.module_accessor, true);
        FLASH(fighter, 0.012, 0.012, 0.032, 0.9);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 25, 0.012, 0.012, 0.032, 0);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_ripple"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.8);
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
        ColorBlendModule::set_disable_camera_depth_influence(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 75.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_splash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.8);
    }
}

//AttackDash
unsafe extern "C" fn effect_samusd_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2.0, 0.0, 0.0, 0, 0, 0, 2.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new_raw(0x09aee445d1), 2.0, 0.0, 0.5, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0.0, 0.0, -0.5, 0, 0, 0, 1.70000005, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0999999, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new_raw(0x0954eb78b2), 2.0, 0.0, -0.5, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0.0, 0.0, 0.0, 0, 0, 0, 1.70000005, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0999999, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_dash_attack"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.8);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("bust"), 0.0, 0.0, 0.0, 0, 0, 0, 1.65, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
        BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
        BURN_COLOR_NORMAL(fighter);
    }
}

//AttackS3Hi
unsafe extern "C" fn effect_samusd_AttackS3Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
        BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_throw_hi"), Hash40::new("haver"), 0, 0, 4.5, 90, 0, 0, 0.88, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
        BURN_COLOR_NORMAL(fighter);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("samusd_throw_hi"), false, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackS3
unsafe extern "C" fn effect_samusd_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
        BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_throw_hi"), Hash40::new("haver"), 0, 0, 4.5, 90, 0, 0, 0.88, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
        BURN_COLOR_NORMAL(fighter);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("samusd_throw_hi"), false, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackS3Lw
unsafe extern "C" fn effect_samusd_AttackS3Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
        BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_throw_hi"), Hash40::new("haver"), 0, 0, 4.5, 90, 0, 0, 0.88, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
        BURN_COLOR_NORMAL(fighter);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("samusd_throw_hi"), false, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackHi3
unsafe extern "C" fn effect_samusd_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 10, 0, 0, -40, -90, 1.2, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
        BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
        BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
        BURN_COLOR_NORMAL(fighter);
    }
}

//AttackLw3
unsafe extern "C" fn effect_samusd_AttackLw3(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("armr"), 28.387, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_BRANCH_SITUATION(fighter, Hash40::new("samusd_bomb_a"), Hash40::new("samusd_bomb_b"), Hash40::new("top"), 0.0, 0.0, 13.4, 0.0, 0.0, 0.0, 0.7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
        EFFECT_BRANCH_SITUATION(fighter, Hash40::new("samusd_bomb_a"), Hash40::new("samusd_bomb_b"), Hash40::new("top"), 0.0, 0.0, 26.8, 0.0, 0.0, 0.0, 0.7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
    }
}

//AttackLw4
unsafe extern "C" fn effect_samusd_AttackLw4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("claviclel"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("samusd_throw_hi"), Hash40::new("top"), 0, 0, 17.4, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("samusd_throw_hi"), Hash40::new("top"), 0, 0, -19.4, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("samusd_throw_hi"), Hash40::new("top"), 0, 0, 8.4, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("samusd_throw_hi"), Hash40::new("top"), 0, 0, -9.4, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
    }
}

//AttackAirN
unsafe extern "C" fn effect_samusd_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("shoulderr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("clavicler"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("claviclel"), 0, 0, 0, 0, 0, 0, 1.9, true);
        BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 2.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 3.0 / 255.0, 194.0 / 255.0, 252.0 / 255.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 2.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 3.0 / 255.0, 194.0 / 255.0, 252.0 / 255.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
        BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
        BURN_COLOR_NORMAL(fighter);
    }
}

//AttackAirF
unsafe extern "C" fn effect_samusd_AttackAirF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 1.3, true, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 12, 6, 180, -180, 100, 1.1, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.8);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
        BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
        BURN_COLOR_NORMAL(fighter);
    }
}

//AttackAirB
unsafe extern "C" fn effect_samusd_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new_raw(0x0e27bc68a2u64), Hash40::new_raw(0x0e27bc68a2u64), Hash40::new("top"), 7.0, 9.0, 0.0, 0, 180, 90, 1.0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.8);
        LAST_EFFECT_SET_RATE(fighter, 2.5);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
        BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
        BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
        BURN_COLOR_NORMAL(fighter);
    }
}

//AttackAirHi
unsafe extern "C" fn effect_samusd_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new_raw(0x0e27bc68a2u64), Hash40::new_raw(0x0e27bc68a2u64), Hash40::new("top"), 6.0, 9.0, 0.0, 0, 0, 90, 1.0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.8);
        LAST_EFFECT_SET_RATE(fighter, 2.5);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
        BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new_raw(0x0e27bc68a2u64), Hash40::new_raw(0x0e27bc68a2u64), Hash40::new("top"), 6.0, 9.0, 0.0, 0, 90, 90, 1.0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.8);
        LAST_EFFECT_SET_RATE(fighter, 3.0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new_raw(0x0e27bc68a2u64), Hash40::new_raw(0x0e27bc68a2u64), Hash40::new("top"), 6.0, 9.0, 0.0, 0, 180, 90, 1.0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.8);
        LAST_EFFECT_SET_RATE(fighter, 3.0);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
        BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
        BURN_COLOR_NORMAL(fighter);
    }
}

//AttackAirLw 
unsafe extern "C" fn effect_samusd_AttackAirLw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("shoulderr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("clavicler"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("claviclel"), 0, 0, 0, 0, 0, 0, 1.9, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 15, 0, 90, 0, 0, 1.7, true);
        LAST_EFFECT_SET_COLOR(fighter, 3.0 / 255.0, 194.0 / 255.0, 252.0 / 255.0);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
    }
}

pub fn install() {
    Agent::new("samusd")
    .effect_acmd("effect_entryr", effect_samusd_EntryR, Low)
    .effect_acmd("effect_entryl", effect_samusd_EntryL, Low)
    .effect_acmd("effect_attackdash", effect_samusd_AttackDash, Low)
    .effect_acmd("effect_attacks3hi", effect_samusd_AttackS3Hi, Low)
    .effect_acmd("effect_attacks3", effect_samusd_AttackS3, Low)
    .effect_acmd("effect_attacks3lw", effect_samusd_AttackS3Lw, Low)
    .effect_acmd("effect_attackhi3", effect_samusd_AttackHi3, Low)
    .effect_acmd("effect_attacklw3", effect_samusd_AttackLw3, Low)
    .effect_acmd("effect_attackairn", effect_samusd_AttackAirN, Low)
    .effect_acmd("effect_attackairf", effect_samusd_AttackAirF, Low)
    .effect_acmd("effect_attackairb", effect_samusd_AttackAirB, Low)
    .effect_acmd("effect_attackairhi", effect_samusd_AttackAirHi, Low)
    .effect_acmd("effect_attackairlw", effect_samusd_AttackAirLw, Low)
    .install();
}