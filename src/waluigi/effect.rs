use crate::imports::BuildImports::*;
use crate::dolly::frame::*;

#[acmd_script(//Attack11
    agent = "dolly",
    script =  "effect_attack11_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("null"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("dolly_kick_arc_w02"), Hash40::new("dolly_kick_arc_w02"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, 1.3);}
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("hat"), 0, 0, 0, 0, 0, 0, 1.4, true, *EF_FLIP_YZ, 0.5);
    }
}

#[acmd_script(//Attack12
    agent = "dolly",
    script =  "effect_attack12_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 12, 3, 90, -10, 0, 0.5, true, *EF_FLIP_YZ, 0.5);
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 25, 3, 0, 0, 0, 0.7, true, *EF_FLIP_YZ, 0.5);
    }
}

#[acmd_script(//Attack13
    agent = "dolly",
    script =  "effect_attack13_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_attack13(fighter: &mut L2CAgentBase) {}

#[acmd_script(//AttackDash
    agent = "dolly",
    script =  "effect_attackdash_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_attackdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 12.5, 0, 180, 0, 20, 1.25, true);
        LAST_EFFECT_SET_RATE(fighter, 1.7);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 180, 0, 20, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.7);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) { 
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 12.5, 0, 180, 90, 20, 1.2, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 180, 90, 20, 0.95, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -0.5, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 14, 0, 0, 0, 0, 0.95, true);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
}

#[acmd_script(//AttackS3 
    agent = "dolly", 
    script = "effect_attacks3_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_attacks3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("dolly_attack_arc"), Hash40::new("dolly_attack_arc"), Hash40::new("top"), 0, 10, 1.5, -5.3, -51.7, 42.2, 1, true, *EF_FLIP_YZ, 1);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, -2, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
    }
}

#[acmd_script(//AttackHi3 
    agent = "dolly", 
    script = "effect_attackhi3_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_attackhi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("sys_run_smoke"), Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, -2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
        sv_animcmd::FOOT_EFFECT_FLIP(fighter.lua_state_agent);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("dolly_attack_arc3"), Hash40::new("dolly_attack_arc3"), Hash40::new("top"), 5, 14.5, 2, -34.5, 0.4, 105, 0.67, true, *EF_FLIP_YZ, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 2, 25, 2, 0, 0, 0, 0.7, true, *EF_FLIP_YZ, 0.7);
    }
}

#[acmd_script(//AttackLw3 
    agent = "dolly", 
    script = "effect_attacklw3_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_attacklw3(fighter: &mut L2CAgentBase) {}

#[acmd_script(//AttackS4 
    agent = "dolly", 
    script = "effect_attacks4_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_attacks4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -5, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("dolly_attack_arc6"), Hash40::new("dolly_attack_arc6"), Hash40::new("top"), 2, 12, 5, -9, -46, 32, 0.8, true, *EF_FLIP_YZ, 1);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//AttackHi4
    agent = "dolly",
    script =  "effect_attackhi4_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_attackhi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("head"), 4.0, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("throw"), 0, 13, 2, -15, -90, -90, 0.95, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//AttackLw4
    agent = "dolly",
    script =  "effect_attacklw4_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_attacklw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.5, -1.5, 180, 0, 20, 1.35, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.5, -1.5, 180, 90, 20, 1.3, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//AttackAirN
    agent = "dolly",
    script =  "effect_attackairn_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_attackairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 7.5, 0, 180, 0, 20, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 7.5, 0, 180, 90, 20, 0.95, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
}

#[acmd_script(//AttackAirF
    agent = "dolly",
    script =  "effect_attackairf_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_attackairf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("dolly_attack_arc3"), Hash40::new("dolly_attack_arc3"), Hash40::new("top"), 4, 10.0, 4, -34.5, 0.4, 105, 0.67, true, *EF_FLIP_YZ, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
}

#[acmd_script(//AttackAirB
    agent = "dolly",
    script =  "effect_attackairb_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_attackairb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 3, 8.4, 5, 180, 0, 0, 1.0, true);
        LAST_PARTICLE_SET_COLOR(fighter, 1.0, 1.0, 1.0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -16.5, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
}

#[acmd_script(//AttackAirHi
    agent = "dolly",
    script =  "effect_attackairhi_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_attackairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("dolly_attack_arc3"), Hash40::new("dolly_attack_arc3"), Hash40::new("top"), 4, 15.0, 2, -34.5, 0.4, 105, 0.67, true, *EF_FLIP_YZ, 0.5);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("dolly_attack_arc3"), Hash40::new("dolly_attack_arc3"), Hash40::new("top"), 4, 15.0, -2, 214.5, 0.4, -255, 0.67, true, *EF_FLIP_YZ, 0.5);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 23, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, false);
    }
}

#[acmd_script(//AttackAirLw
    agent = "dolly",
    script =  "effect_attackairlw_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_attackairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("top"), 5, 5, 4.5, 90, 0, 0, 0.7, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("footL"), 0, -5, 4.5, 90, 0, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("top"), 5, 5, 4.5, 90, 0, 0, 0.7, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("footL"), 0, -5, 4.5, 90, 0, 0, 0.6, true);
        LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.2, 0.7);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("top"), 5, 5, 4.5, 90, 0, 0, 0.7, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("footL"), 0, -5, 4.5, 90, 0, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
}

#[acmd_script(//CatchAttack 
    agent = "dolly", 
    script = "effect_catchattack_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_catchattack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("dolly_attack_arc2"), Hash40::new("dolly_attack_arc2"), Hash40::new("top"), -2, 11.5, 0.3, -69, -97, 106, 0.63, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//ThrowF
    agent = "dolly",
    script =  "effect_throwf_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_throwf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new_raw(0x12147c5df3), Hash40::new("top"), 4, 0, 1, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 14, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 14, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 14, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 12, 14, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script(//ThrowB
    agent = "dolly",
    script =  "effect_throwb_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_throwb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 1, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new_raw(0x12147c5df3), Hash40::new("top"), -4, 0, 1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -6, 0, 2, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 12, -14, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script(//ThrowHi 
    agent = "dolly", 
    script = "effect_throwhi_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_throwhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("dolly_attack_arc3"), Hash40::new("dolly_attack_arc3"), Hash40::new("top"), 2, 14, 2, -34.5, 0.4, 105, 0.67, true, *EF_FLIP_YZ, 1);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 24, 2, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 92.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 4.0, 27.0, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script(//ThrowLw 
    agent = "dolly", 
    script = "effect_throwlw_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_throwlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -2, 20, 0, 90, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
}

#[acmd_script(//DownAttackD 
    agent = "dolly", 
    script = "effect_downattackd_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_downattackd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 10, -15, 180, 0, 20, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 10, -14, 180, 0, 20, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 10, 15, 180, 0, 20, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 0.4);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 10, 15, 180, 0, 20, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 0.4);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
}

#[acmd_script(//DownAttackU 
    agent = "dolly", 
    script = "effect_downattacku_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_downattacku(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 10, -15, 180, 0, 20, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 10, -14, 180, 0, 20, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 10, 15, 180, 0, 20, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 0.4);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 10, 15, 180, 0, 20, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 0.4);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
}

#[acmd_script(//SpecialN
    agent = "dolly", 
    script =  "effect_specialn_waluigi", 
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_specialn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0.0, 30.0, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("handl"), 2.0, 3, -9.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_falling_smoke"), false, false);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_assist"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//SpecialAirN
    agent = "dolly", 
    script =  "effect_specialairn_waluigi", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn effect_waluigi_specialairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0.0, 30.0, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("handl"), 2.0, 2, -9.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_falling_smoke"), false, false);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_assist"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }	
}

#[acmd_script(//SpecialSBStart
    agent = "dolly",
    script =  "effect_specialsbstart_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn effect_waluigi_specialsbstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_absorption"), Hash40::new("hat"), 0, 0, 0, 0, 0, 0, 0.4, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.6);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_muzzleflash"), Hash40::new("hat"), 0, 0, 0, 360, 0, 0, 0.4, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.6);
    }
}

#[acmd_script(//SpecialSBAttack
    agent = "dolly",
    script =  "effect_specialsbattack_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_specialsbattack(fighter: &mut L2CAgentBase) {}

#[acmd_script(//SpecialSBAttackW
    agent = "dolly",
    script =  "effect_specialsbattackw_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_specialsbattackw(fighter: &mut L2CAgentBase) {}

#[acmd_script(//SpecialAirSfEnd 
    agent = "dolly", 
    script = "effect_specialairsfend_waluigi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_waluigi_specialairsfend(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_splash"), Hash40::new("top"), 0, 0, 2.5, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_splash"), Hash40::new("top"), 0, 0, 2.5, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
    }
}

#[acmd_script(//SpecialAirSfEnd2 
    agent = "dolly", 
    script = "effect_specialairsfend2", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_specialairsfend2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    for _ in 0..30 {
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 30.0, 0, 90, 0, 0, 1.3, true, *EF_FLIP_YZ, 0.3);
            LAST_EFFECT_SET_RATE(fighter, 0.7);
        }
        wait(fighter.lua_state_agent, 5.0);
    }
}

#[acmd_script(//SpecialSFStart
    agent = "dolly",
    script =  "effect_specialsfstart_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_specialsfstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_absorption"), Hash40::new("hat"), 0, 0, 0, 0, 0, 0, 0.4, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.6);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_muzzleflash"), Hash40::new("hat"), 0, 0, 0, 360, 0, 0, 0.4, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.6);
    }
}

#[acmd_script(//SpecialSFAttack 
    agent = "dolly", 
    script = "effect_specialsfattack_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_specialsfattack(fighter: &mut L2CAgentBase) {}

#[acmd_script(//SpecialAirSFStart 
    agent = "dolly", 
    script = "effect_specialairsfstart_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_specialairsfstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_water_landing"), Hash40::new("top"), 0, 13, 6.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_water_landing"), Hash40::new("top"), 0, 13, 6.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_water_landing"), Hash40::new("top"), 0, 11, 11, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_water_landing"), Hash40::new("top"), 0, 11, 11, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
    }
    frame(fighter.lua_state_agent, 66.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_water_landing"), Hash40::new("top"), 0, 7.5, 8.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_water_landing"), Hash40::new("top"), 0, 7.5, 8.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
    }
}

#[acmd_script(//SpecialHi1 
    agent = "dolly", 
    script = "effect_specialhi1_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_specialhi1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 0, 1.2, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8.5, 0, 180, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.0, 0, 180, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 90, 0, 1.15, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8.5, 0, 180, 90, 0, 0.95, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.0, 0, 180, 90, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12.5, 1, 0, 0, 0, 1.9, true, 0.5);
    }
}

#[acmd_script(//SpecialAirHi1 
    agent = "dolly", 
    script = "effect_specialairhi1_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_specialairhi1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 0, 1.2, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8.5, 0, 180, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.0, 0, 180, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 90, 0, 1.15, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8.5, 0, 180, 90, 0, 0.95, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.0, 0, 180, 90, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12.5, 1, 0, 0, 0, 1.9, true, 0.5);
    }
}

#[acmd_script(//SpecialAirLw
    agent = "dolly",
    script =  "effect_specialairlw_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_specialairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -2.5, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//SpecialLwStart
    agent = "dolly",
    script =  "effect_speciallwstart_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_speciallwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -0.5, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//SpecialAirLwStart
    agent = "dolly",
    script =  "effect_specialairlwstart_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_specialairlwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -0.5, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//SpecialLwShield
    agent = "dolly",
    script =  "effect_speciallwshield",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_speciallwshield(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_masterball"), Hash40::new("top"), 0, 2.5, 0, 0, 0, 0, 0.55, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 180, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 180, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 180, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//SpecialLwAttack1
    agent = "dolly",
    script =  "effect_speciallwattack1",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_speciallwattack1(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FLASH(fighter, 0.13, 1.55, 0.0, 0.65);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_absorption"), Hash40::new("top"), -4, 10, 8.5, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.35);
    }
}

#[acmd_script(//SpecialLwAttack2
    agent = "dolly",
    script =  "effect_speciallwattack2",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_speciallwattack2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_absorption"), false, false);
        FLASH(fighter, 0.13, 1.55, 0.0, 0.65);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_absorption"), Hash40::new("top"), -4, 12, 10, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.35);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_absorption"), false, false);
    }
}

#[acmd_script(//SpecialLwAttack3
    agent = "dolly",
    script =  "effect_speciallwattack3",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_speciallwattack3(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_absorption"), false, false);
        FLASH(fighter, 0.13, 1.55, 0.0, 0.55);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_masterball"), Hash40::new("top"), 0, 3.5, 1, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("handl"), 4, 0, 0, 0, 0, 0, 0.35, true);
    }
}

#[acmd_script(//SpecialLwAttackSpecial1
    agent = "dolly",
    script =  "effect_speciallwattackspecial1",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_speciallwattackspecial1(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_absorption"), false, false);
        FLASH(fighter, 2.55, 0.0, 0.0, 0.55);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire_fly"), Hash40::new("handr"), 0, -2, 0, 0, 0, 0, 0.45, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("handr"), 3, 0, 0, 0, 0, 0, 0.35, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_fire_fly"), false, false);
    }
}

#[acmd_script(//SpecialLwAttackSpecial2
    agent = "dolly",
    script =  "effect_speciallwattackspecial2",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_speciallwattackspecial2(fighter: &mut L2CAgentBase) { 
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_absorption"), false, false);
        FLASH(fighter, 2.55, 0.0, 0.0, 0.55);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("footl"), 7.0, 0, 0, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
}

#[acmd_script(//SpecialLwJump
    agent = "dolly",
    script =  "effect_speciallwjump",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_speciallwjump(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FLASH(fighter, 0.0, 0.09, 2.55, 0.55);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//SpecialLwSpecial
    agent = "dolly",
    script =  "effect_speciallwspecial",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_speciallwspecial(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FLASH(fighter, 2.55, 0.0, 0.0, 0.55);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("handl"), 0, 2, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
}

#[acmd_script(//SpecialLwAttackAir
    agent = "dolly",
    script =  "effect_speciallwattackair",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_speciallwattackair(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FLASH(fighter, 0.13, 1.55, 0.0, 0.55);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("footl"), 0, 2, 0, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("footr"), 0, 2, 0, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
}

#[acmd_script(//SpecialLwSpecialAir
    agent = "dolly",
    script =  "effect_speciallwspecialair",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_speciallwspecialair(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FLASH(fighter, 2.55, 0.0, 0.0, 0.55);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
}

#[acmd_script(//SpecialLwJumpAir
    agent = "dolly",
    script =  "effect_speciallwjumpair",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_speciallwjumpair(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FLASH(fighter, 0.0, 0.09, 2.55, 0.55);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
}

#[acmd_script(//SuperSpecial 
    agent = "dolly", 
    script = "effect_superspecial_waluigi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_waluigi_superspecial(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("dolly_wave_aura"), Hash40::new("handr"), 1, 0, 0, 0, 0, 0, 1, true);
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 13, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("dolly_wave_arc"), Hash40::new("dolly_wave_arc"), Hash40::new("top"), 0, 10, 4, 69, -46, -45, 1.2, true, *EF_FLIP_YZ);
    }
}

#[acmd_script(//EntryR 
    agent = "dolly", 
    script = "effect_entryr_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_entryr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//EntryL 
    agent = "dolly", 
    script = "effect_entryl_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_entryl(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//FinalStart
    agent = "dolly",
    script =  "effect_finalstart_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_finalstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_shield"), Hash40::new("top"), 0.0, 18, 0, 0, 0, 0, 7.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.2, 0.9);
        LAST_EFFECT_SET_ALPHA(fighter, 0.3);
    }
	frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_shield"), Hash40::new("top"), 0.0, 18, 0, 0, 0, 0, 4.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.2, 0.9);
	    LAST_EFFECT_SET_ALPHA(fighter, 0.01);
	}
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_special_all_up"), Hash40::new("top"), 0, 18, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.2);
    }
	frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bg_vortex2"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 354.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_shield"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_bg_vortex2"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_special_all_up"), false, false);

    }
}

#[acmd_script(//FinalAirStart
    agent = "dolly",
    script =  "effect_finalairstart_waluigi",
    category = ACMD_EFFECT, low_priority)]
unsafe fn effect_waluigi_finalairstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_shield"), Hash40::new("top"), 0.0, 18, 0, 0, 0, 0, 7.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.2, 0.9);
        LAST_EFFECT_SET_ALPHA(fighter, 0.3);

    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_shield"), Hash40::new("top"), 0.0, 18, 0, 0, 0, 0, 4.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.2, 0.9);
	    LAST_EFFECT_SET_ALPHA(fighter, 0.01);
	}
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_special_all_up"), Hash40::new("top"), 0, 18, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.2);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bg_vortex2"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 354.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_shield"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_bg_vortex2"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_special_all_up"), false, false);

    }
}

#[acmd_script(//SuperSpecial2 
    agent = "dolly", 
    script = "effect_superspecial2_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_superspecial2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("dolly_buster_punch"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("dolly_buster_dash"), false, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        BURN_COLOR(fighter, 2, 0.4, 0.008, 0);
        BURN_COLOR_FRAME(fighter, 4, 2, 0.4, 0.008, 0.4);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("dolly_buster_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("dolly_buster_ground_blow"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("dolly_buster"), Hash40::new("top"), 0, 11, 4, 90, 0, 0, 1, false);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        CANCEL_FILL_SCREEN(fighter, 1, 20);
        BURN_COLOR(fighter, 2, 0.4, 0.008, 0.4);
        BURN_COLOR_FRAME(fighter, 12, 2, 0.4, 0.008, 0);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("dolly_buster_aura"), Hash40::new("handr"), 1, 0, 0, 0, 0, 0, 1, true);
        EFFECT_DETACH_KIND(fighter, Hash40::new("dolly_buster_ground"), -1);
        EFFECT_DETACH_KIND(fighter, Hash40::new("dolly_buster_ground_blow"), -1);
        EFFECT_DETACH_KIND(fighter, Hash40::new("dolly_buster"), -1);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        BURN_COLOR_NORMAL(fighter);
    }
}

#[acmd_script(//SuperSpecial2Start 
    agent = "dolly", 
    script = "effect_superspecial2start_waluigi", 
    category = ACMD_EFFECT, low_priority )]
unsafe fn effect_waluigi_superspecial2start(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -15, 16, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
        EFFECT_FOLLOW(fighter, Hash40::new("dolly_wave_hold"), Hash40::new("handr"), 1, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FLW_POS_NO_STOP(fighter, Hash40::new("dolly_buster_punch"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("dolly_buster_dash"), Hash40::new("top"), 0, 0, -8, 0, 0, 0, 1, false);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("dolly_buster_punch"), 6);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("dolly_buster_dash"), false, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_waluigi_attack11,
        effect_waluigi_attack12,
        effect_waluigi_attack13,
        effect_waluigi_attackdash,
        effect_waluigi_attacks3,
        effect_waluigi_attackhi3,
        effect_waluigi_attacklw3,
        effect_waluigi_attacks4,
        effect_waluigi_attackhi4,
        effect_waluigi_attacklw4,
        effect_waluigi_attackairn,
        effect_waluigi_attackairf,
        effect_waluigi_attackairb,
        effect_waluigi_attackairhi,
        effect_waluigi_attackairlw,
        effect_waluigi_catchattack,
        effect_waluigi_throwf,
        effect_waluigi_throwb,
        effect_waluigi_throwhi,
        effect_waluigi_throwlw,
        effect_waluigi_downattacku,
        effect_waluigi_downattackd,
        effect_waluigi_specialn,
        effect_waluigi_specialairn,
        effect_waluigi_specialsbstart,
        effect_waluigi_specialsbattackw,
        effect_waluigi_specialsbattack,
        effect_waluigi_specialairsfend,
        effect_waluigi_specialairsfend2,
        effect_waluigi_specialsfattack,
        effect_waluigi_specialsfstart,
        effect_waluigi_specialairsfstart,
        effect_waluigi_specialhi1,
        effect_waluigi_specialairhi1,
        effect_waluigi_speciallwstart,
        effect_waluigi_specialairlwstart,
        effect_waluigi_specialairlw,
        effect_waluigi_speciallwshield,
        effect_waluigi_speciallwattack1,
        effect_waluigi_speciallwattack2,
        effect_waluigi_speciallwattack3,
        effect_waluigi_speciallwattackspecial1,
        effect_waluigi_speciallwattackspecial2,
        effect_waluigi_speciallwjump,
        effect_waluigi_speciallwattackair,
        effect_waluigi_speciallwspecialair,
        effect_waluigi_speciallwspecial,
        effect_waluigi_speciallwjumpair,
        effect_waluigi_superspecial,
        effect_waluigi_superspecial2,
        effect_waluigi_superspecial2start,
        effect_waluigi_entryl,
        effect_waluigi_entryr,
        effect_waluigi_finalstart,
        effect_waluigi_finalairstart
    );
}