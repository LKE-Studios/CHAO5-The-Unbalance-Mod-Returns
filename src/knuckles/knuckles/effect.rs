use crate::imports::BuildImports::*;
use crate::knuckles::*;

//Run
unsafe extern "C" fn effect_knuckles_Run(fighter: &mut L2CAgentBase) {
    loop {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 4.0);
        wait_loop_clear(fighter);
    }
}

//EntryL
unsafe extern "C" fn effect_knuckles_EntryL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 2.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

//EntryR
unsafe extern "C" fn effect_knuckles_EntryR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 2.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

//Attack11
unsafe extern "C" fn effect_knuckles_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7.8, -2, 0, 0, 0, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 13.5, 7.8, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
}

//Attack12
unsafe extern "C" fn effect_knuckles_Attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 3, 5.0, 3, -6, -15, 30, 0.97, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 13.5, 7.8, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
}

//Attack13
unsafe extern "C" fn effect_knuckles_Attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 6.3, 6.6, -7, 0, -100, 0.8, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
    }
}

//Attack100
unsafe extern "C" fn effect_knuckles_Attack100(fighter: &mut L2CAgentBase) {
    loop {
        if is_excute(fighter) {
            EFFECT_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 6, -2, 0, 0, 0, 1.0, 0, 0, 0, 50, 0, 360, true, 0.4);
            LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 16, 10, 0, 0, 0, 0, 0.8, 5, 4, 2, 0, 0, 360, false, 0.7);
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 15, 3.5, 0, 0, 0, 0, 0.7, 5, 4, 2, 0, 0, 360, false, 0.6);
        }
        wait(fighter.lua_state_agent, 4.0);
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 15.5, 6.5, 0, 0, 0, 0, 0.8, 6, 4, 2, 0, 0, 360, false, 0.6);
    }
}

//Attack100End
unsafe extern "C" fn effect_knuckles_Attack100End(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -1, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 8, 15, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
    }
}

//AttackDash
unsafe extern "C" fn effect_knuckles_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7.8, -2, -24, 25, 360, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 14.8, 11.0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 360, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.9);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter.lua_state_agent) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 1.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.2);
    }
}

//AttackS3Hi
unsafe extern "C" fn effect_knuckles_AttackS3Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, -3, -42, 0, 0, 1.1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 13, 13.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
    }
}

//AttackS3
unsafe extern "C" fn effect_knuckles_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 3, -3, -22, 0, 0, 1.1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9, 15.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
    }
}

//AttackS3Lw
unsafe extern "C" fn effect_knuckles_AttackS3Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 3, -3, -22, 0, 0, 1.1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9, 15.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
    }
}

//AttackHi3
unsafe extern "C" fn effect_knuckles_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, -3, -42, 0, 0, 1.1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 13, 13.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
    }
}

//AttackLw3
unsafe extern "C" fn effect_knuckles_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -2, 2, 2, 1, 5, 5, 0.925, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.86);
        LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackS4Charge
unsafe extern "C" fn effect_knuckles_AttackS4Charge(fighter: &mut L2CAgentBase) {
    loop {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("handr"), 0.75, -1, 0, 0, -45, 0, 0.4, true);
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 5, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 7.0);
        if is_excute(fighter) {
            EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -3, 11.5, -8, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, true, *EF_FLIP_YZ);
        }
        wait_loop_clear(fighter);
    }
}

//AttackS4Hi
unsafe extern "C" fn effect_knuckles_AttackS4Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("handr"), 0.75, -1, 0, 0, -45, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 2.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_r"), Hash40::new("handr"), 0.75, -1, 0, 0, 0, 0, 0.35, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 6.5, -7, 0, 0, 0, 1.3, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 17, 6.5, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, false);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_bullet_r"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
}

//AttackS4
unsafe extern "C" fn effect_knuckles_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("handr"), 0.75, -1, 0, 0, -45, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 2.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_r"), Hash40::new("handr"), 0.75, -1, 0, 0, 0, 0, 0.35, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 6.5, -7, 0, 0, 0, 1.3, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 17, 6.5, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, false);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_bullet_r"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
}

//AttackS4Lw
unsafe extern "C" fn effect_knuckles_AttackS4Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("handr"), 0.75, -1, 0, 0, -45, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 2.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_r"), Hash40::new("handr"), 0.75, -1, 0, 0, 0, 0, 0.35, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 6.5, -7, 0, 0, 0, 1.3, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 17, 6.5, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, false);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_bullet_r"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
}

//AttackHi4Charge
unsafe extern "C" fn effect_knuckles_AttackHi4Charge(fighter: &mut L2CAgentBase) {
    loop {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("handr"), 0.75, -1, 0, 0, -45, 0, 0.4, true);
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 12, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 12, -2, 0, 0, 0, 0.9, 10, 8, 10, 0, 0, 0, true);
        }
        wait_loop_clear(fighter);
    }
}

//AttackHi4
unsafe extern "C" fn effect_knuckles_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("handr"), 0.75, -1, 0, 0, -45, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 15, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_r"), Hash40::new("handr"), 0.75, -1, 0, 0, 0, 0, 0.35, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 16.5, 2.0, 0, 45, 90, 0.85, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.4, 0.14, 0.05);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_bullet_r"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 1.3, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
}

//AttackLw4Charge
unsafe extern "C" fn effect_knuckles_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    loop {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_soil_landing"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
            EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            let pos = PostureModule::pos(fighter.module_accessor);
            let handle = EffectModule::req(fighter.module_accessor, Hash40::new("sys_merikomi"), pos, &Vector3f{ x: 0.0, y: 0.0, z: 0.0 }, 1.4, 0, -1, false, 0);
            KNUCKLES_MERIKOMI_EFFECT_HANDLE[ENTRY_ID] = handle as i32;
        }
        wait(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, true);
        }
        wait_loop_clear(fighter);
    }
}

//AttackLw4
unsafe extern "C" fn effect_knuckles_AttackLw4(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sphere"), 0, 0, 0, 0, -160, 270, 0.75, true, 1);
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spintrace"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.4, 0.14, 0.05);
    }
    frame(fighter.lua_state_agent, 5.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_soil_landing"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
            EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            let pos = PostureModule::pos(fighter.module_accessor);
            let handle = EffectModule::req(fighter.module_accessor, Hash40::new("sys_merikomi"), pos, &Vector3f{ x: 0.0, y: 0.0, z: 0.0 }, 1.4, 0, -1, false, 0);
            KNUCKLES_MERIKOMI_EFFECT_HANDLE[ENTRY_ID] = handle as i32;
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spintrace"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_merikomi"), false, false);
    }
}

//AttackAirN
unsafe extern "C" fn effect_knuckles_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 12.5, -8, 35, 0, 0, 1.1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 3, 8.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
    }
}

//AttackAirF
unsafe extern "C" fn effect_knuckles_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("handr"), 0.75, -1, 0, 0, -45, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_r"), Hash40::new("handr"), 0.75, -1, 0, 0, 0, 0, 0.35, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 7, 0, -3, -11, -113, 1.2, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_bullet_r"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
}

//AttackAirB
unsafe extern "C" fn effect_knuckles_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("armr"), 0, 0, 0.0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
    }
}

//AttackAirHi
unsafe extern "C" fn effect_knuckles_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("head"), 0, 0, 0.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 360, true);
    }
}

//AttackAirLw
unsafe extern "C" fn effect_knuckles_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0.5, -7, 0, 0, 0, 0, 1.45, 0, 0, 0, 0, 0, 360, false);
    }
}

//ThrowF
unsafe extern "C" fn effect_knuckles_ThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_hit_normal"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 360, true);
    }
}

//ThrowB
unsafe extern "C" fn effect_knuckles_ThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("trans"), 0, 12, 0, 0, 0, 90, 0.65, true, *EF_FLIP_YZ, 0.6);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("trans"), 0, 12, 0, 0, 180, 90, 1, true, *EF_FLIP_YZ, 0.6);
    }
    frame(fighter.lua_state_agent, 58.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//ThrowHi
unsafe extern "C" fn effect_knuckles_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}

//ThrowLw
unsafe extern "C" fn effect_knuckles_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_r"), Hash40::new("handr"), 0.75, -1, 0, 0, 0, 0, 0.35, true);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.57, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_r"), Hash40::new("handl"), 0.75, -1, 0, 0, 0, 0, 0.35, true);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.57, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 59.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.73, 0, 0, 0, 0, 0, 0, false);
    }
}

//DownAttackD
unsafe extern "C" fn effect_knuckles_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinblur_plain"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spinblur_plain"), true, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spintrace"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinblur_plain"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, -3, 0, 0, 90, 1.4, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 15.0);
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 15, 0, 0, 0, 180, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_speedline"), false, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, -0.25, 0, 180, 90, 1.4, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinwind"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinblur_plain"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -17, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spinwind"), true, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spinblur_plain"), true, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spintrace"), false, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinblur_plain"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spinblur_plain"), true, false);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinblur_plain"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spinblur_plain"), true, false);
    }
}

//DownAttackU
unsafe extern "C" fn effect_knuckles_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -1, 5, 8, 185, 150, 5, 1.1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), false, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 5, -6.5, 185, -20, 8, 1.1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), false, false);
    }
}

//SlipAttack
unsafe extern "C" fn effect_knuckles_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 3, 4, 0, 35, 10, 0.9, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), false, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 4, -2.5, 0, 165, 10, 0.9, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), false, false);
    }
}

//CliffAttack
unsafe extern "C" fn effect_knuckles_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -3, 13.5, -1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinwind_throw"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1.1, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 7.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spintrace"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spinwind_throw"), true, true);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spintrace"), false, true);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialNDig
unsafe extern "C" fn effect_knuckles_SpecialNDig(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        let pos = PostureModule::pos(fighter.module_accessor);
        let handle = EffectModule::req(fighter.module_accessor, Hash40::new("sys_merikomi"), pos, &Vector3f{ x: 0.0, y: 0.0, z: 0.0 }, 1.4, 0, -1, false, 0);
        KNUCKLES_MERIKOMI_EFFECT_HANDLE[ENTRY_ID] = handle as i32;
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_merikomi"), false, false);
        let pos = PostureModule::pos(fighter.module_accessor);
        let handle = EffectModule::req(fighter.module_accessor, Hash40::new("sys_merikomi"), pos, &Vector3f{ x: 0.0, y: 0.0, z: 0.0 }, 1.4, 0, -1, false, 0);
        KNUCKLES_MERIKOMI_EFFECT_HANDLE[ENTRY_ID] = handle as i32;
    }
}

//SpecialAirNDig
unsafe extern "C" fn effect_knuckles_SpecialAirNDig(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_soil_landing"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        let pos = PostureModule::pos(fighter.module_accessor);
        let handle = EffectModule::req(fighter.module_accessor, Hash40::new("sys_merikomi"), pos, &Vector3f{ x: 0.0, y: 0.0, z: 0.0 }, 1.4, 0, -1, false, 0);
        KNUCKLES_MERIKOMI_EFFECT_HANDLE[ENTRY_ID] = handle as i32;
    }
}

//SpecialAirNDrillStart
unsafe extern "C" fn effect_knuckles_SpecialAirNDrillStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("littlemac_risinguppercut"), Hash40::new("rot"), 0, 4.8, 0, 180, 0, 0, 1.25, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
}

//SpecialAirNDrillLoop
unsafe extern "C" fn effect_knuckles_SpecialAirNDrillLoop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spintrace"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 0.88, true);
    }
}

//LandingFallSpecial
unsafe extern "C" fn effect_knuckles_LandingFallSpecial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 1.5, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
}

//SpecialNUpper
unsafe extern "C" fn effect_knuckles_SpecialNUpper(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_soil_landing"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        let pos = PostureModule::pos(fighter.module_accessor);
        let handle = EffectModule::req(fighter.module_accessor, Hash40::new("sys_merikomi"), pos, &Vector3f{ x: 0.0, y: 0.0, z: 0.0 }, 1.4, 0, -1, false, 0);
        KNUCKLES_MERIKOMI_EFFECT_HANDLE[ENTRY_ID] = handle as i32;
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 14.5, 3.0, 0, 45, 90, 0.87, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.4, 0.14, 0.05);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 1.3, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
}

//SpecialS
unsafe extern "C" fn effect_knuckles_SpecialS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
}

//SpecialAirSEndLoop
unsafe extern "C" fn effect_knuckles_SpecialAirSEndLoop(fighter: &mut L2CAgentBase) {
    for _ in 0..5 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter,Hash40::new("sys_attack_speedline"),Hash40::new("top"), 2, 9, -17.0, 5, 0, 0, 1, true);
            LAST_EFFECT_SET_RATE(fighter,0.3);
            LAST_EFFECT_SET_ALPHA(fighter,0.5);
            LAST_EFFECT_SET_COLOR(fighter, 0.9, 0.2, 0.2);
        }
        wait(fighter.lua_state_agent, 14.0);
    }
}

//SpecialAirSLanding
unsafe extern "C" fn effect_knuckles_SpecialAirSLanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
}

//SpecialHi
unsafe extern "C" fn effect_knuckles_SpecialHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sonic_gimmck_impact"), Hash40::new("top"), 0, 0, -0.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_gimmckjump"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, true);
    }
}

//SpecialLwHold
unsafe extern "C" fn effect_knuckles_SpecialLwHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_impact"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinwind"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinblur"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 1, 0, 0, 5, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_idling"), Hash40::new("top"), 0, 4.5, 0, 0, 0, 0, 1, true);
    }
    wait(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -12, 0, 0, 0, 0.8, 5, 0, 15, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_idling"), Hash40::new("top"), 0, 4.5, 0, 0, 0, 0, 1, true);
    }
    wait(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_idling"), Hash40::new("top"), 0, 4.5, 0, 0, 0, 0, 1, true);
    }
    wait(fighter.lua_state_agent, 10.0);
}

//SpecialAirLwHold
unsafe extern "C" fn effect_knuckles_SpecialAirLwHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_impact"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinwind"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spinblur"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 1, 0, 0, 5, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_idling"), Hash40::new("top"), 0, 4.5, 0, 0, 0, 0, 1, true);
    }
    wait(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -12, 0, 0, 0, 0.8, 5, 0, 15, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_idling"), Hash40::new("top"), 0, 4.5, 0, 0, 0, 0, 1, true);
    }
    wait(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_idling"), Hash40::new("top"), 0, 4.5, 0, 0, 0, 0, 1, true);
    }
    wait(fighter.lua_state_agent, 10.0);
}

//AppealSR
unsafe extern "C" fn effect_knuckles_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.2);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -5, 6.0, 0, 0, 0, 0, 0.8, true);
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -5, 6.5, 1, 0, 0, 0, 0.8, true);
    }
}

//AppealSL
unsafe extern "C" fn effect_knuckles_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.2);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -5, 6.0, 0, 0, 0, 0, 0.8, true);
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -5, 6.5, 1, 0, 0, 0, 0.8, true);
    }
}

//AppealLwR
unsafe extern "C" fn effect_knuckles_AppealLwR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.4);
    }
}

//AppealLwL
unsafe extern "C" fn effect_knuckles_AppealLwL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.4);
    }
}

//Win1
unsafe extern "C" fn effect_knuckles_Win1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter.lua_state_agent) {
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spintrace"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.88, true);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter.lua_state_agent) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spintrace"), false, false);
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter.lua_state_agent) {
        EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 118.0);
    if is_excute(fighter.lua_state_agent) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

//Win2
unsafe extern "C" fn effect_knuckles_Win2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 56.0);
    if is_excute(fighter.lua_state_agent) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("handl"), 0, 0, 3, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 360, false);
    }
    frame(fighter.lua_state_agent, 70.0);
    if is_excute(fighter.lua_state_agent) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("handl"), 0, 0, 5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 360, false);
    }
    frame(fighter.lua_state_agent, 97.0);
    if is_excute(fighter.lua_state_agent) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

//Win3
unsafe extern "C" fn effect_knuckles_Win3(fighter: &mut L2CAgentBase) {
    let mut handle = 0;
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter.lua_state_agent) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("littlemac_risinguppercut"), Hash40::new("sphere"), 0, 4.8, 0, 0, 90, 0, 1.25, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("sonic_spintrace"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.88, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter.lua_state_agent) {
        EFFECT_OFF_KIND(fighter, Hash40::new("littlemac_risinguppercut"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spintrace"), false, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter.lua_state_agent) {
        let mut pos = Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        let offset = ModelModule::joint_global_offset_from_top(fighter.module_accessor, Hash40::new("hip"), &mut pos);
        let new_pos = Vector3f{
            x: PostureModule::pos_x(fighter.module_accessor) + pos.x,
            y: 0.0, 
            z: PostureModule::pos_z(fighter.module_accessor) + pos.z
        };
        handle = EffectModule::req(fighter.module_accessor, Hash40::new("sys_merikomi"), &new_pos, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);
        EffectModule::set_scale(fighter.module_accessor, handle as u32, &Vector3f{x: 1.0, y: 1.0, z: 1.0});
    }
    frame(fighter.lua_state_agent, 47.0);
    for i in 0..8 {
        if is_excute(fighter.lua_state_agent) {
            let scale = 1.0 - (0.14 * i as f32);
            EffectModule::set_scale(fighter.module_accessor, handle as u32, &Vector3f{x: scale, y: 1.0, z: scale});
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter.lua_state_agent) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_merikomi"), false, false);
    }
    frame(fighter.lua_state_agent, 62.0);
    if is_excute(fighter.lua_state_agent) {
        let mut pos = Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        let offset = ModelModule::joint_global_offset_from_top(fighter.module_accessor, Hash40::new("hip"), &mut pos);
        let new_pos = Vector3f{
            x: PostureModule::pos_x(fighter.module_accessor) + pos.x + 2.5,
            y: 0.0, 
            z: PostureModule::pos_z(fighter.module_accessor) + pos.z + 1.0
        };
        handle = EffectModule::req(fighter.module_accessor, Hash40::new("sys_merikomi"), &new_pos, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);
        EffectModule::set_scale(fighter.module_accessor, handle as u32, &Vector3f{x: 1.0, y: 1.0, z: 1.0});
    }
    frame(fighter.lua_state_agent, 91.0);
    for i in 0..8 {
        if is_excute(fighter.lua_state_agent) {
            let scale = 1.0 - (0.14 * i as f32);
            EffectModule::set_scale(fighter.module_accessor, handle as u32, &Vector3f{x: scale, y: 1.0, z: scale});
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 99.0);
    if is_excute(fighter.lua_state_agent) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_merikomi"), false, false);
    }
}

pub fn install() {
    Agent::new("sonic")
    .effect_acmd("effect_attack11_knuckles", effect_knuckles_Attack11, Low)
    .effect_acmd("effect_attack12_knuckles", effect_knuckles_Attack12, Low)
    .effect_acmd("effect_attack13_knuckles", effect_knuckles_Attack13, Low)
    .effect_acmd("effect_attack100_knuckles", effect_knuckles_Attack100, Low)
    .effect_acmd("effect_attack100end_knuckles", effect_knuckles_Attack100End, Low)
    .effect_acmd("effect_attackdash_knuckles", effect_knuckles_AttackDash, Low)
    .effect_acmd("effect_attacks3hi_knuckles", effect_knuckles_AttackS3Hi, Low)
    .effect_acmd("effect_attacks3_knuckles", effect_knuckles_AttackS3, Low)
    .effect_acmd("effect_attacks3lw_knuckles", effect_knuckles_AttackS3Lw, Low)
    .effect_acmd("effect_attackhi3_knuckles", effect_knuckles_AttackHi3, Low)
    .effect_acmd("effect_attacklw3_knuckles", effect_knuckles_AttackLw3, Low)
    .effect_acmd("effect_attacks4charge_knuckles", effect_knuckles_AttackS4Charge, Low)
    .effect_acmd("effect_attacks4hi_knuckles", effect_knuckles_AttackS4Hi, Low)
    .effect_acmd("effect_attacks4_knuckles", effect_knuckles_AttackS4, Low)
    .effect_acmd("effect_attacks4lw_knuckles", effect_knuckles_AttackS4Lw, Low)
    .effect_acmd("effect_attackhi4charge_knuckles", effect_knuckles_AttackHi4Charge, Low)
    .effect_acmd("effect_attackhi4_knuckles", effect_knuckles_AttackHi4, Low)
    .effect_acmd("effect_attacklw4charge_knuckles", effect_knuckles_AttackLw4Charge, Low)
    .effect_acmd("effect_attacklw4_knuckles", effect_knuckles_AttackLw4, Low)
    .effect_acmd("effect_attackairn_knuckles", effect_knuckles_AttackAirN, Low)
    .effect_acmd("effect_attackairf_knuckles", effect_knuckles_AttackAirF, Low)    
    .effect_acmd("effect_attackairb_knuckles", effect_knuckles_AttackAirB, Low)
    .effect_acmd("effect_attackairhi_knuckles", effect_knuckles_AttackAirHi, Low)
    .effect_acmd("effect_attackairlw_knuckles", effect_knuckles_AttackAirLw, Low)
    .effect_acmd("effect_throwf_knuckles", effect_knuckles_ThrowF, Low)
    .effect_acmd("effect_throwb_knuckles", effect_knuckles_ThrowB, Low)
    .effect_acmd("effect_throwhi_knuckles", effect_knuckles_ThrowHi, Low)
    .effect_acmd("effect_throwlw_knuckles", effect_knuckles_ThrowLw, Low)
    .effect_acmd("effect_downattackd_knuckles", effect_knuckles_DownAttackD, Low)
    .effect_acmd("effect_downattacku_knuckles", effect_knuckles_DownAttackU, Low)
    .effect_acmd("effect_cliffattack_knuckles", effect_knuckles_CliffAttack, Low)
    .effect_acmd("effect_slipattack_knuckles", effect_knuckles_SlipAttack, Low)
    .effect_acmd("effect_specialndig_knuckles", effect_knuckles_SpecialNDig, Low)
    .effect_acmd("effect_specialairndig_knuckles", effect_knuckles_SpecialAirNDig, Low)
    .effect_acmd("effect_specialairndrillstart_knuckles", effect_knuckles_SpecialAirNDrillStart, Low)
    .effect_acmd("effect_specialairndrillloop_knuckles", effect_knuckles_SpecialAirNDrillLoop, Low)
    .effect_acmd("effect_landingfallspecial_knuckles", effect_knuckles_LandingFallSpecial, Low)
    .effect_acmd("effect_specialnupper_knuckles", effect_knuckles_SpecialNUpper, Low)  
    .effect_acmd("effect_specials_knuckles", effect_knuckles_SpecialS, Low)
    .effect_acmd("effect_specialairslanding_knuckles", effect_knuckles_SpecialAirSLanding, Low)
    .effect_acmd("effect_specialairsendloop_knuckles", effect_knuckles_SpecialAirSEndLoop, Low)
    .effect_acmd("effect_specialhi_knuckles", effect_knuckles_SpecialHi, Low)
    .effect_acmd("effect_speciallwhold_knuckles", effect_knuckles_SpecialLwHold, Low)
    .effect_acmd("effect_specialairlwhold_knuckles", effect_knuckles_SpecialAirLwHold, Low)
    .effect_acmd("effect_appealsr_knuckles", effect_knuckles_AppealSR, Low)
    .effect_acmd("effect_appealsl_knuckles", effect_knuckles_AppealSL, Low)
    .effect_acmd("effect_appeallwr_knuckles", effect_knuckles_AppealLwR, Low)
    .effect_acmd("effect_appeallwl_knuckles", effect_knuckles_AppealLwL, Low)
    .effect_acmd("effect_win1_knuckles", effect_knuckles_Win1, Low)
    .effect_acmd("effect_win2_knuckles", effect_knuckles_Win2, Low)
    .effect_acmd("effect_win3_knuckles", effect_knuckles_Win3, Low)
    .install();
}