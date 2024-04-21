use crate::imports::BuildImports::*;
use crate::waluigi::waluigi::frame::*;

//Attack11
unsafe extern "C" fn effect_waluigi_Attack11(fighter: &mut L2CAgentBase) {
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

//Attack12
unsafe extern "C" fn effect_waluigi_Attack12(fighter: &mut L2CAgentBase) {
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

//Attack13
unsafe extern "C" fn effect_waluigi_Attack13(fighter: &mut L2CAgentBase) {}

//AttackDash
unsafe extern "C" fn effect_waluigi_AttackDash(fighter: &mut L2CAgentBase) {
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

//AttackS3 
unsafe extern "C" fn effect_waluigi_AttackS3(fighter: &mut L2CAgentBase) {
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

//AttackHi3 
unsafe extern "C" fn effect_waluigi_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT_FLIP(fighter, Hash40::new("sys_run_smoke"), Hash40::new("sys_run_smoke"), Hash40::new("top"), -2.0, 0.0, -2.0, 0.0, 0.0, 0.0, 0.7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("dolly_attack_arc3"), Hash40::new("dolly_attack_arc3"), Hash40::new("top"), 0, 14.5, 0, -34.5, 180, 105, 0.67, true, *EF_FLIP_YZ, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 25, 3, 0, 0, 0, 1.0, true, *EF_FLIP_YZ, 0.8);
    }
}

//AttackLw3 
unsafe extern "C" fn effect_waluigi_AttackLw3(fighter: &mut L2CAgentBase) {}

//AttackS4 
unsafe extern "C" fn effect_waluigi_AttackS4(fighter: &mut L2CAgentBase) {
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

//AttackHi4
unsafe extern "C" fn effect_waluigi_AttackHi4(fighter: &mut L2CAgentBase) {
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

//AttackLw4
unsafe extern "C" fn effect_waluigi_AttackLw4(fighter: &mut L2CAgentBase) {
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

//AttackAirN
unsafe extern "C" fn effect_waluigi_AttackAirN(fighter: &mut L2CAgentBase) {
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

//AttackAirF
unsafe extern "C" fn effect_waluigi_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 14.5, -6, 9, 5, 0, 1.1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
	}
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("fingerl32"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 360, true, 0.85);
    }
}

//AttackAirB
unsafe extern "C" fn effect_waluigi_AttackAirB(fighter: &mut L2CAgentBase) {
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

//AttackAirHi
unsafe extern "C" fn effect_waluigi_AttackAirHi(fighter: &mut L2CAgentBase) {
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

//AttackAirLw
unsafe extern "C" fn effect_waluigi_AttackAirLw(fighter: &mut L2CAgentBase) {
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

//CatchAttack 
unsafe extern "C" fn effect_waluigi_CatchAttack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("dolly_attack_arc2"), Hash40::new("dolly_attack_arc2"), Hash40::new("top"), -2, 11.5, 0.3, -69, -97, 106, 0.63, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//ThrowF
unsafe extern "C" fn effect_waluigi_ThrowF(fighter: &mut L2CAgentBase) {
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

//ThrowB
unsafe extern "C" fn effect_waluigi_ThrowB(fighter: &mut L2CAgentBase) {
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

//ThrowHi 
unsafe extern "C" fn effect_waluigi_ThrowHi(fighter: &mut L2CAgentBase) {
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

//ThrowLw 
unsafe extern "C" fn effect_waluigi_ThrowLw(fighter: &mut L2CAgentBase) {
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

//DownAttackD 
unsafe extern "C" fn effect_waluigi_DownAttackD(fighter: &mut L2CAgentBase) {
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

//DownAttackU 
unsafe extern "C" fn effect_waluigi_DownAttackU(fighter: &mut L2CAgentBase) {
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

//SpecialN
unsafe extern "C" fn effect_waluigi_SpecialN(fighter: &mut L2CAgentBase) {
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
}

//SpecialAirN
unsafe extern "C" fn effect_waluigi_SpecialAirN(fighter: &mut L2CAgentBase) {
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
}

//SpecialSBStart
unsafe extern "C" fn effect_waluigi_SpecialSBStart(fighter: &mut L2CAgentBase) {
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

//SpecialSBAttack
unsafe extern "C" fn effect_waluigi_SpecialSBAttack(fighter: &mut L2CAgentBase) {}

//SpecialSBAttackW
unsafe extern "C" fn effect_waluigi_SpecialSBAttackW(fighter: &mut L2CAgentBase) {}

//SpecialAirSFEnd 
unsafe extern "C" fn effect_waluigi_SpecialAirSFEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_splash"), Hash40::new("top"), 0, 0, 2.5, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_splash"), Hash40::new("top"), 0, 0, 2.5, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
    }
}

//SpecialAirSfEnd2 
unsafe extern "C" fn effect_waluigi_SpecialAirSFEnd2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    for _ in 0..30 {
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 30.0, 0, 90, 0, 0, 1.3, true, *EF_FLIP_YZ, 0.3);
            LAST_EFFECT_SET_RATE(fighter, 0.7);
        }
        wait(fighter.lua_state_agent, 5.0);
    }
}

//SpecialSFStart
unsafe extern "C" fn effect_waluigi_SpecialSFStart(fighter: &mut L2CAgentBase) {
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

//SpecialSFAttack 
unsafe extern "C" fn effect_waluigi_SpecialSFAttack(fighter: &mut L2CAgentBase) {}

//SpecialAirSFStart 
unsafe extern "C" fn effect_waluigi_SpecialAirSFStart(fighter: &mut L2CAgentBase) {
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

//SpecialHi1 
unsafe extern "C" fn effect_waluigi_SpecialHi1(fighter: &mut L2CAgentBase) {
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

//SpecialAirHi1 
unsafe extern "C" fn effect_waluigi_SpecialAirHi1(fighter: &mut L2CAgentBase) {
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

//SpecialAirLw
unsafe extern "C" fn effect_waluigi_SpecialAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -2.5, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialLwStart
unsafe extern "C" fn effect_waluigi_SpecialLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -0.5, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialAirLwStart
unsafe extern "C" fn effect_waluigi_SpecialAirLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -0.5, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialLwShield
unsafe extern "C" fn effect_waluigi_SpecialLwShield(fighter: &mut L2CAgentBase) {
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

//SpecialLwAttack1
unsafe extern "C" fn effect_waluigi_SpecialLwAttack1(fighter: &mut L2CAgentBase) {
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

//SpecialLwAttack2
unsafe extern "C" fn effect_waluigi_SpecialLwAttack2(fighter: &mut L2CAgentBase) {
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

//SpecialLwAttack3
unsafe extern "C" fn effect_waluigi_SpecialLwAttack3(fighter: &mut L2CAgentBase) {
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

//SpecialLwAttackSpecial1
unsafe extern "C" fn effect_waluigi_SpecialLwAttackSpecial1(fighter: &mut L2CAgentBase) {
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

//SpecialLwAttackSpecial2
unsafe extern "C" fn effect_waluigi_SpecialLwAttackSpecial2(fighter: &mut L2CAgentBase) { 
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

//SpecialLwJump
unsafe extern "C" fn effect_waluigi_SpecialLwJump(fighter: &mut L2CAgentBase) {
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

//SpecialLwSpecial
unsafe extern "C" fn effect_waluigi_SpecialLwSpecial(fighter: &mut L2CAgentBase) {
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

//SpecialLwAttackAir
unsafe extern "C" fn effect_waluigi_SpecialLwAttackAir(fighter: &mut L2CAgentBase) {
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

//SpecialLwSpecialAir
unsafe extern "C" fn effect_waluigi_SpecialLwSpecialAir(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FLASH(fighter, 2.55, 0.0, 0.0, 0.55);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) { 
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 12.5, 0, 180, 90, 20, 1.2, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 180, 90, 20, 0.95, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
}

//SpecialLwJumpAir
unsafe extern "C" fn effect_waluigi_SpecialLwJumpAir(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FLASH(fighter, 0.0, 0.09, 2.55, 0.55);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 20.0, -3.0, 0, 0, 0.0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 19.0, -3.0, 0, 0, 0.0, 1.0, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.4, /*G*/ 0.0, /*B*/ 2.0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 20.0, -3.0, 0, 0, 0.0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 19.0, -3.0, 0, 0, 0.0, 1.0, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.4, /*G*/ 0.0, /*B*/ 2.0);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 20.0, -3.0, 0, 0, 0.0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 19.0, -3.0, 0, 0, 0.0, 1.0, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.4, /*G*/ 0.0, /*B*/ 2.0);
    }
}

//SuperSpecial 
unsafe extern "C" fn effect_waluigi_SuperSpecial(fighter: &mut L2CAgentBase) {
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

//EntryR 
unsafe extern "C" fn effect_waluigi_EntryR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

//EntryL 
unsafe extern "C" fn effect_waluigi_EntryL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

//FinalStart
unsafe extern "C" fn effect_waluigi_FinalStart(fighter: &mut L2CAgentBase) {
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

//FinalAirStart
unsafe extern "C" fn effect_waluigi_FinalAirStart(fighter: &mut L2CAgentBase) {
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

//SuperSpecial2 
unsafe extern "C" fn effect_waluigi_SuperSpecial2(fighter: &mut L2CAgentBase) {
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

//SuperSpecial2Start 
unsafe extern "C" fn effect_waluigi_SuperSpecial2Start(fighter: &mut L2CAgentBase) {
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
    Agent::new("dolly")
    .effect_acmd("effect_attack11_waluigi", effect_waluigi_Attack11)
    .effect_acmd("effect_attack12_waluigi", effect_waluigi_Attack12)
    .effect_acmd("effect_attack13_waluigi", effect_waluigi_Attack13)
    .effect_acmd("effect_attackdash_waluigi", effect_waluigi_AttackDash)
    .effect_acmd("effect_attacks3_waluigi", effect_waluigi_AttackS3)
    .effect_acmd("effect_attackhi3_waluigi", effect_waluigi_AttackHi3)
    .effect_acmd("effect_attacklw3_waluigi", effect_waluigi_AttackLw3)
    .effect_acmd("effect_attacks4_waluigi", effect_waluigi_AttackS4)
    .effect_acmd("effect_attackhi4_waluigi", effect_waluigi_AttackHi4)
    .effect_acmd("effect_attacklw4_waluigi", effect_waluigi_AttackLw4)
    .effect_acmd("effect_attackairn_waluigi", effect_waluigi_AttackAirN)
    .effect_acmd("effect_attackairf_waluigi", effect_waluigi_AttackAirF)
    .effect_acmd("effect_attackairb_waluigi", effect_waluigi_AttackAirB)
    .effect_acmd("effect_attackairhi_waluigi", effect_waluigi_AttackAirHi)
    .effect_acmd("effect_attackairlw_waluigi", effect_waluigi_AttackAirLw)
    .effect_acmd("effect_catchattack_waluigi", effect_waluigi_CatchAttack)
    .effect_acmd("effect_throwf_waluigi", effect_waluigi_ThrowF)
    .effect_acmd("effect_throwb_waluigi", effect_waluigi_ThrowB)
    .effect_acmd("effect_throwhi_waluigi", effect_waluigi_ThrowHi)
    .effect_acmd("effect_throwlw_waluigi", effect_waluigi_ThrowLw)
    .effect_acmd("effect_downattacku_waluigi", effect_waluigi_DownAttackU)
    .effect_acmd("effect_downattackd_waluigi", effect_waluigi_DownAttackD)
    .effect_acmd("effect_specialn_waluigi", effect_waluigi_SpecialN)
    .effect_acmd("effect_specialairn_waluigi", effect_waluigi_SpecialAirN)
    .effect_acmd("effect_specialsbstart_waluigi", effect_waluigi_SpecialSBStart)
    .effect_acmd("effect_specialsbattackw_waluigi", effect_waluigi_SpecialSBAttackW)
    .effect_acmd("effect_specialsbattack_waluigi", effect_waluigi_SpecialSBAttack)
    .effect_acmd("effect_specialairsfend_waluigi", effect_waluigi_SpecialAirSFEnd)
    .effect_acmd("effect_specialairsfend2", effect_waluigi_SpecialAirSFEnd2)
    .effect_acmd("effect_specialsfattack_waluigi", effect_waluigi_SpecialSFAttack)
    .effect_acmd("effect_specialsfstart_waluigi", effect_waluigi_SpecialSFStart)
    .effect_acmd("effect_specialairsfstart_waluigi", effect_waluigi_SpecialAirSFStart)
    .effect_acmd("effect_specialhi1_waluigi", effect_waluigi_SpecialHi1)
    .effect_acmd("effect_specialairhi1_waluigi", effect_waluigi_SpecialAirHi1)
    .effect_acmd("effect_speciallwstart_waluigi", effect_waluigi_SpecialLwStart)
    .effect_acmd("effect_specialairlwstart_waluigi", effect_waluigi_SpecialAirLwStart)
    .effect_acmd("effect_specialairlw_waluigi", effect_waluigi_SpecialAirLw)
    .effect_acmd("effect_speciallwshield", effect_waluigi_SpecialLwShield)
    .effect_acmd("effect_speciallwattack1", effect_waluigi_SpecialLwAttack1)
    .effect_acmd("effect_speciallwattack2", effect_waluigi_SpecialLwAttack2)
    .effect_acmd("effect_speciallwattack3", effect_waluigi_SpecialLwAttack3)
    .effect_acmd("effect_speciallwattackspecial1", effect_waluigi_SpecialLwAttackSpecial1)
    .effect_acmd("effect_speciallwattackspecial2", effect_waluigi_SpecialLwAttackSpecial2)
    .effect_acmd("effect_speciallwjump", effect_waluigi_SpecialLwJump)
    .effect_acmd("effect_speciallwattackair", effect_waluigi_SpecialLwAttackAir)
    .effect_acmd("effect_speciallwspecialair", effect_waluigi_SpecialLwSpecialAir)
    .effect_acmd("effect_speciallwspecial", effect_waluigi_SpecialLwSpecial)
    .effect_acmd("effect_speciallwjumpair", effect_waluigi_SpecialLwJumpAir)
    .effect_acmd("effect_superspecial_waluigi", effect_waluigi_SuperSpecial)
    .effect_acmd("effect_superspecial2_waluigi", effect_waluigi_SuperSpecial2)
    .effect_acmd("effect_superspecial2start_waluigi", effect_waluigi_SuperSpecial2Start)
    .effect_acmd("effect_finalstart_waluigi", effect_waluigi_FinalStart)
    .effect_acmd("effect_finalairstart_waluigi", effect_waluigi_FinalAirStart)
    .effect_acmd("effect_entryl_waluigi", effect_waluigi_EntryL)
    .effect_acmd("effect_entryr_waluigi", effect_waluigi_EntryR)
    .install();
}