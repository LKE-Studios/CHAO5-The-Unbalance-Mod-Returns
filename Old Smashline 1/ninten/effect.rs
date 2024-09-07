use crate::imports::BuildImports::*;

#[acmd_script(//Win3
    agent = "ness", 
    script = "effect_win3_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_win3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 1.5, 0, 0, 0, -30, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script(//Wait2
    agent = "ness", 
    script = "effect_wait2_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_wait2(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//Wait3
    agent = "ness", 
    script = "effect_wait3_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_wait3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 32.0);
	if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
	}
    frame(fighter.lua_state_agent, 66.0);
	if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
	}
}

#[acmd_script(//Attack11 
    agent = "ness", 
    script = "effect_attack11_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attack11(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//Attack12
    agent = "ness", 
    script = "effect_attack12_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attack12(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//Attack13
    agent = "ness", 
    script = "effect_attack13_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attack13(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackDash
    agent = "ness", 
    script = "effect_attackdash_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attackdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//AttackS3Hi
    agent = "ness", 
    script = "effect_attacks3hi_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attacks3hi(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackS3
    agent = "ness", 
    script = "effect_attacks3_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attacks3(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackS3Lw
    agent = "ness", 
    script = "effect_attacks3lw_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attacks3lw(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackHi3
    agent = "ness", 
    script = "effect_attackhi3_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attackhi3(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackLw3
    agent = "ness", 
    script = "effect_attacklw3_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attacklw3(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackS4
    agent = "ness", 
    script = "effect_attacks4_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attacks4(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackHi4Charge
    agent = "ness", 
    script = "effect_attackhi4charge_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attackhi4charge(fighter: &mut L2CAgentBase) {
    for _ in 0..25 {
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -5, 0, 0, 0, 1, 8, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0.0, 9, 5.5, 0, 0, 0, 1, 2, 5, 2, 0, 0, 0, true);
    }
}

#[acmd_script(//AttackHi4
    agent = "ness", 
    script = "effect_attackhi4_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attackhi4(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackLw4
    agent = "ness", 
    script = "effect_attacklw4_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attacklw4(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackLw4Charge
    agent = "ness", 
    script = "effect_attacklw4charge_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attacklw4charge(fighter: &mut L2CAgentBase) {
    for _ in 0..25 {
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 1, 8, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0.0, 3.5, 6, 0, 0, 0, 1, 2, 5, 2, 0, 0, 0, true);
    }
}

#[acmd_script(//AttackAirN
    agent = "ness", 
    script = "effect_attackairn_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attackairn(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackAirF 
    agent = "ness", 
    script = "effect_attackairf_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attackairf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ness_smash_arc"), Hash40::new("ness_smash_arc"), Hash40::new("top"), 2, 4, 2, 0, 0, 13, 1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ness_smash_arc"), true, true);
    }
}

#[acmd_script(//AttackAirB
    agent = "ness", 
    script = "effect_attackairb_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attackairb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -2, 3.7, 0, 0, 180, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4.5, -10, 0, 0, 0, 0.8, true);
    }
}

#[acmd_script(//AttackAirHi
    agent = "ness", 
    script = "effect_attackairhi_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attackairhi(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackAirLw
    agent = "ness", 
    script = "effect_attackairlw_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_attackairlw(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//ThrowF
    agent = "ness", 
    script = "effect_throwf_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_throwf(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//ThrowB
    agent = "ness", 
    script = "effect_throwb_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_throwb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1);
    }
}

#[acmd_script(//ThrowHi
    agent = "ness", 
    script = "effect_throwhi_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_throwhi(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//ThrowLw
    agent = "ness", 
    script = "effect_throwlw_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_throwlw(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//CliffAttack
    agent = "ness", 
    script = "effect_cliffattack_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_cliffattack(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//DownAttackU
    agent = "ness", 
    script = "effect_downattacku", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_downattacku(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//DownAttackD
    agent = "ness", 
    script = "effect_downattackd", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_downattackd(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialNStart
    agent = "ness", 
    script = "effect_specialnstart_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_specialnstart(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialNHold
    agent = "ness", 
    script = "effect_specialnhold_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_specialnhold(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialNFire
    agent = "ness", 
    script = "effect_specialnfire_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_specialnfire(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirNStart
    agent = "ness", 
    script = "effect_specialairnstart_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_specialairnstart(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirNHold
    agent = "ness", 
    script = "effect_specialairnhold_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_specialairnhold(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirNFire
    agent = "ness", 
    script = "effect_specialairnfire_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_specialairnfire(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//Move
    agent = "ness_pkflash", 
    script = "effect_move_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_pkhypnosis_move(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("sys_special_all_up"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.7, 0.0, 1.0);
        EFFECT_OFF_KIND(fighter,Hash40::new("ness_pkfl_bullet"),false,false);
        EFFECT_OFF_KIND(fighter,Hash40::new("ness_pkfl_bullet_ed"),false,false);
    }
}

#[acmd_script(//Tame
    agent = "ness_pkflash", 
    script = "effect_tame_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_pkhypnosis_tame(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("ness_pkfl_bullet"),false,false);
        EFFECT_OFF_KIND(fighter,Hash40::new("ness_pkfl_bullet_ed"),false,false);
        EFFECT(fighter, Hash40::new("sys_explosion_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter,0.7,0.0,1.0);
    }
}

#[acmd_script(//Bang
    agent = "ness_pkflash", 
    script = "effect_bang_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_pkhypnosis_bang(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_level_up"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter,0.7,0.0,1.0);
        EFFECT_OFF_KIND(fighter,Hash40::new("ness_pkfl_bomb"), false, false);
        EFFECT_OFF_KIND(fighter,Hash40::new("ness_pkfl_bomb_max"), false, false);
        EFFECT_OFF_KIND(fighter,Hash40::new("ness_pkfl_bullet_ed"), false, false);
    }
}

#[acmd_script(//Shoot
    agent = "ness_pkfire", 
    script = "effect_shoot_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_pkbeam_shoot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ness_pkfi_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, true);
    }
}

#[acmd_script(//Pillar
    agent = "ness_pkfire", 
    script = "effect_pillar_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_pkbeam_pillar(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("ness_pkfl_bomb"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

#[acmd_script(//PillarAir
    agent = "ness_pkfire", 
    script = "effect_pillarair_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_pkbeam_pillarair(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("ness_pkfl_bomb"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

#[acmd_script(//SpecialHiStart
    agent = "ness", 
    script = "effect_specialhistart_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_specialhistart(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirHiStart
    agent = "ness", 
    script = "effect_specialairhistart_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_specialairhistart(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirHiEnd
    agent = "ness", 
    script = "effect_specialairhiend_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_specialairhiend(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.25, 10, 10, 10, 0, 0, 0, false);
    }
}

#[acmd_script(//SpecialAirHiAttack
    agent = "ness", 
    script = "effect_specialairhiattack_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_specialairhiattack(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialLwStart
    agent = "ness", 
    script = "effect_speciallwstart_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_speciallwstart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ness_psimagnet_start"), Hash40::new("trans"), 0, 6.5, 0, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 2.5, 0.5);
        BURN_COLOR(fighter, 10.4, 2.4, 0.2, 0.3);
    }
}

#[acmd_script(//SpecialAirLwStart
    agent = "ness", 
    script = "effect_specialairlwstart_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_specialairlwstart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ness_psimagnet_start"), Hash40::new("trans"), 0, 6.5, 0, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 2.5, 0.5);
        BURN_COLOR(fighter, 10.4, 2.4, 0.2, 0.3);
    }
}

#[acmd_script(//SpecialLwHold
    agent = "ness", 
    script = "effect_speciallwhold_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_speciallwhold(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirLwHold
    agent = "ness", 
    script = "effect_specialairlwhold_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_specialairlwhold(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialLwEnd
    agent = "ness", 
    script = "effect_speciallwend_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_speciallwend(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirLwEnd
    agent = "ness", 
    script = "effect_specialairlwend_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_specialairlwend(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AppealLwR
    agent = "ness", 
    script = "effect_appeallwr_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_appeallwr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack01"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal01"));
    }
}

#[acmd_script(//AppealLwL
    agent = "ness", 
    script = "effect_appeallwl_ninten", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_appeallwl(fighter: &mut L2CAgentBase) {}

#[acmd_script(//AppealSR
    agent = "ness", 
    script = "effect_appealsr_ninten", 
    category = ACMD_GAME )]
unsafe fn effect_ninten_appealsr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 7, 8, 1, 0, 0, 0, 0.22, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
	}
}

#[acmd_script(//AppealSL
    agent = "ness", 
    script = "effect_appealsl_ninten", 
    category = ACMD_GAME )]
unsafe fn effect_ninten_appealsl(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 7, 8, 1, 0, 0, 0, 0.22, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
	}
}

#[acmd_script(//Move
    agent = "ness_pkstarstorm_ninten", 
    script = "effect_move", 
    category = ACMD_EFFECT )]
unsafe fn effect_ninten_pkstarstorm_move(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ness_final_main"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.55, 2.15, 0.0);
        EFFECT(fighter, Hash40::new("ness_final_stardust"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.55, 2.15, 0.0);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_ninten_win3,
        effect_ninten_wait2,
        effect_ninten_wait3,
        effect_ninten_attack11,
        effect_ninten_attack12,
        effect_ninten_attack13,
        effect_ninten_attackdash,
        effect_ninten_attackhi3,
        effect_ninten_attacklw3,
        effect_ninten_attacks4,
        effect_ninten_attackhi4charge,
        effect_ninten_attackhi4,
        effect_ninten_attacklw4charge,
        effect_ninten_attacklw4,
        effect_ninten_attackairn,
        effect_ninten_attackairf,
        effect_ninten_attackairb,
        effect_ninten_attackairhi,
        effect_ninten_attackairlw,
        effect_ninten_throwf,
        effect_ninten_throwb,
        effect_ninten_throwhi,
        effect_ninten_throwlw,
        effect_ninten_cliffattack,
        effect_ninten_downattacku,
        effect_ninten_downattackd,
        effect_ninten_specialnstart,
        effect_ninten_specialnhold,
        effect_ninten_specialnfire,
        effect_ninten_specialairnstart,
        effect_ninten_specialairnhold,
        effect_ninten_specialairnfire,
        effect_ninten_pkhypnosis_move,
        effect_ninten_pkhypnosis_tame,
        effect_ninten_pkhypnosis_bang,
        effect_ninten_pkbeam_shoot,
        effect_ninten_pkbeam_pillar,
        effect_ninten_pkbeam_pillarair,
        effect_ninten_specialhistart,
        effect_ninten_specialairhistart,
        effect_ninten_specialairhiend,
        effect_ninten_specialairhiattack,
        effect_ninten_speciallwstart,
        effect_ninten_specialairlwstart,
        effect_ninten_speciallwhold,
        effect_ninten_specialairlwhold,
        effect_ninten_speciallwend,
        effect_ninten_specialairlwend,
        effect_ninten_appeallwr,
        effect_ninten_appeallwl,
        effect_ninten_appealsr,
        effect_ninten_appealsl,
        effect_ninten_pkstarstorm_move
    );
}