use crate::imports::BuildImports::*;

#[acmd_script(//EntryL, EntryR
    agent = "samusd", 
    scripts = ["effect_entryr", "effect_entryl"], 
    category = ACMD_EFFECT)]
unsafe fn effect_samusd_entry(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackDash
    agent = "samusd", 
    script = "effect_attackdash", 
    category = ACMD_EFFECT)]
unsafe fn effect_samusd_attackdash(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackS3, AttackS3Lw, AttackS3Hi
    agent = "samusd", 
    scripts = ["effect_attacks3", "effect_attacks3hi", "effect_attacks3lw"], 
    category = ACMD_EFFECT)]
unsafe fn effect_samusd_attacks3(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackHi3
    agent = "samusd", 
    script = "effect_attackhi3", 
    category = ACMD_EFFECT)]
unsafe fn effect_samusd_attackhi3(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackLw3
    agent = "samusd", 
    script = "effect_attacklw3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_samusd_attacklw3(fighter: &mut L2CAgentBase) {
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
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("samusd_bomb_a"), Hash40::new("samusd_bomb_b"), Hash40::new("top"), 0, 0, 13.4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT_BRANCH_SITUATION(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("samusd_bomb_a"), Hash40::new("samusd_bomb_b"), Hash40::new("top"), 0, 0, 26.8, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT_BRANCH_SITUATION(fighter.lua_state_agent);
    }
}

#[acmd_script(//AttackLw4
    agent = "samusd", 
    script = "effect_attacklw4", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_samusd_attacklw4(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackAirN
    agent = "samusd", 
    script = "effect_attackairn", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_samusd_attackairn(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackAirF
    agent = "samusd", 
    script = "effect_attackairf", 
    category = ACMD_EFFECT)]
unsafe fn effect_samusd_attackairf(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackAirB
    agent = "samusd", 
    script = "effect_attackairb", 
    category = ACMD_EFFECT)]
unsafe fn effect_samusd_attackairb(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackAirHi
    agent = "samusd", 
    script = "effect_attackairhi", 
    category = ACMD_EFFECT)]
unsafe fn effect_samusd_attackairhi(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackAirLw 
    agent = "samusd", 
    script = "effect_attackairlw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_samusd_attackairlw(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//HBurst 
    agent = "samusd_missile", 
    script = "effect_hburst", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_samusd_missile_hburst(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("samusd_bomb_a"), Hash40::new("samusd_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        EFFECT_BRANCH_SITUATION(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

#[acmd_script(//SBurst
    agent = "samusd_supermissile", 
    script = "effect_sburst", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_samusd_supermissile_sburst(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("samusd_bomb_a"), Hash40::new("samusd_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.04, 0, 0, 0, 0, 0, 0, true);
        EFFECT_BRANCH_SITUATION(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

#[acmd_script(//BurstAttack
    agent = "samusd_bomb", 
    script = "effect_burstattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_samusd_bomb_burstattack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("samusd_bomb_a"), Hash40::new("samusd_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT_BRANCH_SITUATION(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.12, /*G*/ 0.1, /*B*/ 1.85);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_samusd_entry,
        effect_samusd_attackdash,
        effect_samusd_attacks3,
        effect_samusd_attackhi3,
        effect_samusd_attacklw3,
        effect_samusd_attacklw4,
        effect_samusd_attackairn,
        effect_samusd_attackairf,
        effect_samusd_attackairb,
        effect_samusd_attackairhi,
        effect_samusd_attackairlw,
        effect_samusd_missile_hburst,
        effect_samusd_supermissile_sburst,
        effect_samusd_bomb_burstattack
    );
}