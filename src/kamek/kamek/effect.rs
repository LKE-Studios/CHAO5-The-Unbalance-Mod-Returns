use crate::imports::BuildImports::*;

//Win3
unsafe extern "C" fn effect_ninten_Win3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 1.5, 0, 0, 0, -30, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

//Wait2
unsafe extern "C" fn effect_ninten_Wait2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 55.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 5.2, 6.85, 1, 0, 0, 0, 0.17, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
	}
    frame(fighter.lua_state_agent, 82.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 5.2, 6.85, 1, 0, 0, 0, 0.17, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
	}
}

//Wait3
unsafe extern "C" fn effect_ninten_Wait3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 32.0);
	if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
	}
    frame(fighter.lua_state_agent, 66.0);
	if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
	}
}

//Attack11 
unsafe extern "C" fn effect_ninten_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0.5, 4.9, -6.5, 0, -2, 0, 0.95, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.2, 1.0);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5, 10.5, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, true, 0.6);
    }
}

//Attack12
unsafe extern "C" fn effect_ninten_Attack12(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 5.5, -4.5, 0, 7, 0, 0.95, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.2, 1.0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5.5, 12.5, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, true, 0.5);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line"), true, true);
    }
}

//Attack13
unsafe extern "C" fn effect_ninten_Attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ness_smash_arc"), Hash40::new("ness_smash_arc"), Hash40::new("top"), 2, 4, 2, 0, 0, 13, 1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -2, 6, 12, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true, 0.6);
    }
}

//AttackDash
unsafe extern "C" fn effect_ninten_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackS3Hi
unsafe extern "C" fn effect_ninten_AttackS3Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 1, 6.7, 1.8, -25, -45, 45, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.2, 1.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackS3
unsafe extern "C" fn effect_ninten_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 1, 5.3, 1.5, 0, -60, 15, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.2, 1.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackS3Lw
unsafe extern "C" fn effect_ninten_AttackS3Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 2, 4, 1.8, 15, -60, 3, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.2, 1.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackHi3
unsafe extern "C" fn effect_ninten_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ness_smash_arc"), Hash40::new("ness_smash_arc"), Hash40::new("top"), 0, 7.3, -0.5, 0, -15, -90, 1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ness_smash_arc"), true, true);
    }
}

//AttackLw3
unsafe extern "C" fn effect_ninten_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 3.5, 10.0, 0, 0, -7, 0.45, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 1.5, 10.0, 0, 180, 7, 0.45, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 3.5, 10.0, 0, 0, -7, 0.45, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 1.5, 10.0, 0, 180, 7, 0.45, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 3.5, 10.0, 0, 0, -7, 0.45, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 1.5, 10.0, 0, 180, 7, 0.45, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 3.5, 10.0, 0, 0, -7, 0.45, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 1.5, 10.0, 0, 180, 7, 0.45, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 48.0);
	if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
	}
}

//AttackS4
unsafe extern "C" fn effect_ninten_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0.0, 8, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ness_smash_arc"), Hash40::new("ness_smash_arc"), Hash40::new("top"), 2, 4, 2, 0, 0, 13, 1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -2, 6, 12, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 360, true, 0.6);
    }
}

//AttackHi4Charge
unsafe extern "C" fn effect_ninten_AttackHi4Charge(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -5, 0, 0, 0, 1, 8, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0.0, 9, 5.5, 0, 0, 0, 1, 2, 5, 2, 0, 0, 0, true);
    }
}

//AttackHi4
unsafe extern "C" fn effect_ninten_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 11, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
	}
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0.0, 6.0, 1.5, 0, 18, 90, 0.8, true);
		LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.2, 1.0);
	}
}

//AttackLw4
unsafe extern "C" fn effect_ninten_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 5, 7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("throw"), 1.0, 0.0, 0.0, 0, 0, 83, 0.45, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("throw"), -1.0, 0.0, 0.0, 0, 180, 97, 0.45, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("throw"), 1.0, 0.0, 0.0, 0, 0, 83, 0.45, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("throw"), -1.0, 0.0, 0.0, 0, 180, 97, 0.45, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("throw"), 1.0, 0.0, 0.0, 0, 0, 83, 0.45, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("throw"), -1.0, 0.0, 0.0, 0, 180, 97, 0.45, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("throw"), 1.0, 0.0, 0.0, 0, 0, 83, 0.45, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("throw"), -1.0, 0.0, 0.0, 0, 180, 97, 0.45, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
}

//AttackLw4Charge
unsafe extern "C" fn effect_ninten_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 1, 8, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0.0, 3.5, 6, 0, 0, 0, 1, 2, 5, 2, 0, 0, 0, true);
    }
}

//AttackAirN
unsafe extern "C" fn effect_ninten_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("hip"), 1, 0, 0, 0, 0, 0, 1.3, true);
		LAST_EFFECT_SET_RATE(fighter, 0.5);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
		EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("hip"), 1, 0, 0, 0, 0, 0, 1.3, true);
		LAST_EFFECT_SET_RATE(fighter, 0.5);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
	}
}

//AttackAirF 
unsafe extern "C" fn effect_ninten_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ness_smash_arc"), Hash40::new("ness_smash_arc"), Hash40::new("top"), 2, 4, 2, 0, 0, 13, 1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ness_smash_arc"), true, true);
    }
}

//AttackAirB
unsafe extern "C" fn effect_ninten_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -2, 3.7, 0, 0, 180, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4.5, -10, 0, 0, 0, 0.8, true);
    }
}

//AttackAirHi
unsafe extern "C" fn effect_ninten_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0.5, -2, 2.5, -90, 0, 0, 0.8, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.2, 1.0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("throw"), 1, 0, 0, 0, 0, 0, 1.3, true);
		LAST_EFFECT_SET_RATE(fighter, 0.5);
        LAST_EFFECT_SET_ALPHA(fighter, 0.75);
    }
}

//AttackAirLw
unsafe extern "C" fn effect_ninten_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("hip"), 1, 0, 0, 0, 0, 0, 1.3, true);
		LAST_EFFECT_SET_RATE(fighter, 0.5);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
		EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("hip"), 1, 0, 0, 0, 0, 0, 1.3, true);
		LAST_EFFECT_SET_RATE(fighter, 0.5);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
	}
}

//ThrowF
unsafe extern "C" fn effect_ninten_ThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1);
    }
}

//ThrowB 
unsafe extern "C" fn effect_ninten_ThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1);
    }
}

//ThrowHi
unsafe extern "C" fn effect_ninten_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("ness_psi_rush"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 0, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("ness_psi_rush"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("ness_psi_rush"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.3, 5, 5, 5, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1);
    }
}      

//ThrowLw
unsafe extern "C" fn effect_ninten_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1);
    }
}

//CliffAttack
unsafe extern "C" fn effect_ninten_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 4.0, -8.0, 0, 7, 0, 0.95, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.2, 1.0);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4.0, 10.0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, true, 0.5);
    }
}

//DownAttackU
unsafe extern "C" fn effect_ninten_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 4.5, -6, 0, 120, 13, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.2, 1.0);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        LAST_EFFECT_SET_ALPHA(fighter, 0.55);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 4.5, 5, 0, -50, -10, 1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.2, 1.0);
    }
}

//DownAttackD
unsafe extern "C" fn effect_ninten_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 5, 5, 12, -30, 0, 1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.2, 1.0);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        LAST_EFFECT_SET_ALPHA(fighter, 0.55);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc"), false, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 3.8, -5, 0, 120, 23, 1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.2, 1.0);
    }
}

//SpecialNStart
unsafe extern "C" fn effect_ninten_SpecialNStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialNHold
unsafe extern "C" fn effect_ninten_SpecialNHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("sys_special_all_up"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter,0.7,0.0,1.0);
    }
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 9, 0, 9, 0, 0, 0, false);
            FLASH(fighter, 0.7, 1, 0.2, 0.4);
        }
        wait(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            FLASH(fighter, 0.8, 1, 1, 0.6);
        }
        wait(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
}

//SpecialNFire
unsafe extern "C" fn effect_ninten_SpecialNFire(fighter: &mut L2CAgentBase) {
    for _ in 0..3 {
        if is_excute(fighter) {
            FLASH(fighter, 0.7, 1, 0.2, 0.4);
        }
        wait(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            FLASH(fighter, 2, 1, 1, 0.6);
        }
        wait(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("bust"), 1.5, -2, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("ness_pkfl_hold"), false, false);
    }
}

//SpecialAirNStart
unsafe extern "C" fn effect_ninten_SpecialAirNStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialAirNHold
unsafe extern "C" fn effect_ninten_SpecialAirNHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("sys_special_all_up"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter,0.7,0.0,1.0);
    }
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 9, 0, 9, 0, 0, 0, false);
            FLASH(fighter, 0.7, 1, 0.2, 0.4);
        }
        wait(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            FLASH(fighter, 0.8, 1, 1, 0.6);
        }
        wait(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
}

//SpecialAirNFire
unsafe extern "C" fn effect_ninten_SpecialAirNFire(fighter: &mut L2CAgentBase) {
    for _ in 0..3 {
        if is_excute(fighter) {
            FLASH(fighter, 0.7, 1, 0.2, 0.4);
        }
        wait(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            FLASH(fighter, 2, 1, 1, 0.6);
        }
        wait(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("bust"), 1.5, -2, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("ness_pkfl_hold"), false, false);
    }
}

//SpecialHiStart
unsafe extern "C" fn effect_ninten_SpecialHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.25, 10, 10, 10, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 30.0);
    for _ in 0..12 {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.85, 10, 10, 10, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 12.0);
    }
}

//SpecialAirHiStart
unsafe extern "C" fn effect_ninten_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.25, 10, 10, 10, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 30.0);
    for _ in 0..12 {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.85, 10, 10, 10, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 12.0);
    }
}

//SpecialAirHiEnd
unsafe extern "C" fn effect_ninten_SpecialAirHiEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.25, 10, 10, 10, 0, 0, 0, false);
    }
}

//SpecialAirHiAttack
unsafe extern "C" fn effect_ninten_SpecialAirHiAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smartbomb_finish"), Hash40::new("claviclel"), 0.0, 0.0, 0.0, 0, 0, 0, 0.25, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.4, 0.0, 2.3);
        LAST_EFFECT_SET_ALPHA(fighter, 0.45);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_smartbomb_finish"), false, false);
    }
}

//SpecialLwStart
unsafe extern "C" fn effect_ninten_SpecialLwStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ness_psimagnet_start"), Hash40::new("trans"), 0, 6.5, 0, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 2.5, 0.5);
        BURN_COLOR(fighter, 10.4, 2.4, 0.2, 0.3);
    }
}

//SpecialAirLwStart
unsafe extern "C" fn effect_ninten_SpecialAirLwStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ness_psimagnet_start"), Hash40::new("trans"), 0, 6.5, 0, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 2.5, 0.5);
        BURN_COLOR(fighter, 10.4, 2.4, 0.2, 0.3);
    }
}

//SpecialLwHold
unsafe extern "C" fn effect_ninten_SpecialLwHold(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            FLASH(fighter, 0, 1, 1, 0.2);
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 2.5, 0.5);
            BURN_COLOR(fighter, 10.4, 2.4, 0.2, 0.3);
        }
        wait(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
}

//SpecialAirLwHold
unsafe extern "C" fn effect_ninten_SpecialAirLwHold(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            FLASH(fighter, 0, 1, 1, 0.2);
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 2.5, 0.5);
            BURN_COLOR(fighter, 10.4, 2.4, 0.2, 0.3);
        }
        wait(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
}

//SpecialLwEnd
unsafe extern "C" fn effect_ninten_SpecialLwEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 1.3);
        }
        let time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("time"));
        let time_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_TIME);
        let ratio = 1.0;
        EFFECT_FOLLOW(fighter, Hash40::new("ness_psimagnet_end"), Hash40::new("trans"), 0, 6.5, 0, 0, 0, 0, 1.0 * ratio, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 2.5, 0.5);
        FLASH(fighter, 0.5, 1, 1, 0.4);
        BURN_COLOR(fighter, 10.4, 2.4, 0.2, 0.3);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 10, 0, 1, 1, 0.1);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//SpecialAirLwEnd
unsafe extern "C" fn effect_ninten_SpecialAirLwEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 1.3);
        }
        let time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("time"));
        let time_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_TIME);
        let ratio = 1.0;
        EFFECT_FOLLOW(fighter, Hash40::new("ness_psimagnet_end"), Hash40::new("trans"), 0, 6.5, 0, 0, 0, 0, 1.0 * ratio, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 2.5, 0.5);
        FLASH(fighter, 0.5, 1, 1, 0.4);
        BURN_COLOR(fighter, 10.4, 2.4, 0.2, 0.3);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 10, 0, 1, 1, 0.1);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//AppealLwR
unsafe extern "C" fn effect_ninten_AppealLwR(fighter: &mut L2CAgentBase) {}

//AppealLwL
unsafe extern "C" fn effect_ninten_AppealLwL(fighter: &mut L2CAgentBase) {}

//AppealSR
unsafe extern "C" fn effect_ninten_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 7, 8, 1, 0, 0, 0, 0.22, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
	}
}

//AppealSL
unsafe extern "C" fn effect_ninten_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 7, 8, 1, 0, 0, 0, 0.22, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
	}
}

pub fn install() {
    Agent::new("ness")
    .effect_acmd("effect_win3_ninten", effect_ninten_Win3, Low)
    .effect_acmd("effect_wait2_ninten", effect_ninten_Wait2, Low)
    .effect_acmd("effect_wait3_ninten", effect_ninten_Wait3, Low)
    .effect_acmd("effect_attack11_ninten", effect_ninten_Attack11, Low)
    .effect_acmd("effect_attack12_ninten", effect_ninten_Attack12, Low)
    .effect_acmd("effect_attack13_ninten", effect_ninten_Attack13, Low)
    .effect_acmd("effect_attackdash_ninten", effect_ninten_AttackDash, Low)
    .effect_acmd("effect_attacks3hi_ninten", effect_ninten_AttackS3Hi, Low)
    .effect_acmd("effect_attacks3_ninten", effect_ninten_AttackS3, Low)
    .effect_acmd("effect_attacks3lw_ninten", effect_ninten_AttackS3Lw, Low)
    .effect_acmd("effect_attackhi3_ninten", effect_ninten_AttackHi3, Low)
    .effect_acmd("effect_attacklw3_ninten", effect_ninten_AttackLw3, Low)
    .effect_acmd("effect_attacks4_ninten", effect_ninten_AttackS4, Low)
    .effect_acmd("effect_attackhi4charge_ninten", effect_ninten_AttackHi4Charge, Low)
    .effect_acmd("effect_attackhi4_ninten", effect_ninten_AttackHi4, Low)
    .effect_acmd("effect_attacklw4charge_ninten", effect_ninten_AttackLw4Charge, Low)
    .effect_acmd("effect_attacklw4_ninten", effect_ninten_AttackLw4, Low)
    .effect_acmd("effect_attackairn_ninten", effect_ninten_AttackAirN, Low)
    .effect_acmd("effect_attackairf_ninten", effect_ninten_AttackAirF, Low)    
    .effect_acmd("effect_attackairb_ninten", effect_ninten_AttackAirB, Low)
    .effect_acmd("effect_attackairhi_ninten", effect_ninten_AttackAirHi, Low)
    .effect_acmd("effect_attackairlw_ninten", effect_ninten_AttackAirLw, Low)
    .effect_acmd("effect_throwf_ninten", effect_ninten_ThrowF, Low)
    .effect_acmd("effect_throwb_ninten", effect_ninten_ThrowB, Low)
    .effect_acmd("effect_throwhi_ninten", effect_ninten_ThrowHi, Low)
    .effect_acmd("effect_throwlw_ninten", effect_ninten_ThrowLw, Low)
    .effect_acmd("effect_downattackd_ninten", effect_ninten_DownAttackD, Low)
    .effect_acmd("effect_downattacku_ninten", effect_ninten_DownAttackU, Low)
    .effect_acmd("effect_specialnfire_ninten", effect_ninten_SpecialNFire, Low) 
    .effect_acmd("effect_specialnhold_ninten", effect_ninten_SpecialNHold, Low)  
    .effect_acmd("effect_specialnstart_ninten", effect_ninten_SpecialNStart, Low) 
    .effect_acmd("effect_specialairnfire_ninten", effect_ninten_SpecialAirNFire, Low) 
    .effect_acmd("effect_specialairnhold_ninten", effect_ninten_SpecialAirNHold, Low)  
    .effect_acmd("effect_specialairnstart_ninten", effect_ninten_SpecialAirNStart, Low) 
    .effect_acmd("effect_specialhistart_ninten", effect_ninten_SpecialHiStart, Low)
    .effect_acmd("effect_specialairhistart_ninten", effect_ninten_SpecialAirHiStart, Low)
    .effect_acmd("effect_specialairhiend_ninten", effect_ninten_SpecialAirHiEnd, Low)
    .effect_acmd("effect_specialairhiattack_ninten", effect_ninten_SpecialAirHiAttack, Low)
    .effect_acmd("effect_speciallwstart_ninten", effect_ninten_SpecialLwStart, Low)
    .effect_acmd("effect_speciallwhold_ninten", effect_ninten_SpecialLwHold, Low)
    .effect_acmd("effect_speciallwend_ninten", effect_ninten_SpecialLwEnd, Low)
    .effect_acmd("effect_specialairlwstart_ninten", effect_ninten_SpecialAirLwStart, Low)
    .effect_acmd("effect_specialairlwhold_ninten", effect_ninten_SpecialAirLwHold, Low)
    .effect_acmd("effect_specialairlwend_ninten", effect_ninten_SpecialAirLwEnd, Low)
    .effect_acmd("effect_appealsr_ninten", effect_ninten_AppealSR, Low)
    .effect_acmd("effect_appealsl_ninten", effect_ninten_AppealSL, Low)
    .effect_acmd("effect_appeallwr_ninten", effect_ninten_AppealLwR, Low)
    .effect_acmd("effect_appeallwl_ninten", effect_ninten_AppealLwL, Low)
    .install();
}