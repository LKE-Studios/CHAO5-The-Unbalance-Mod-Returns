use crate::imports::BuildImports::*;

//Wait3
unsafe extern "C" fn effect_sans_Wait3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 284.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_piyo"), Hash40::new("top"), 0, 9, 2, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

//EscapeB
unsafe extern "C" fn effect_sans_EscapeB(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 8, 0, 0, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//EscapeF
unsafe extern "C" fn effect_sans_EscapeF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//EscapeN
unsafe extern "C" fn effect_sans_EscapeN(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 20.0);
}

//EscapeAir
unsafe extern "C" fn effect_sans_EscapeAir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("rot"), 0, -8, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 23.0);
}

//Attack11
unsafe extern "C" fn effect_sans_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 1, 5, -62.5, 0, 0, 0.5, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
}

//Attack12
unsafe extern "C" fn effect_sans_Attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 1, 13, -135, 0, 0, 0.35, true);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 8.5, 7.5, 0, 0, 0, 0.67, false);
    }
}

//AttackDash
unsafe extern "C" fn effect_sans_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent,5.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 6.5, 17, 0, 0, 0, 1.8, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 6.5, 15.5, 0, 0, 0, 0.4, false);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
}

//AttackS3
unsafe extern "C" fn effect_sans_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 7, 10.0, 0, 0, 0, 2.0, false);
    }
}

//AttackHi3
unsafe extern "C" fn effect_sans_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -1, 10.5, -90, 0, 0, 1.0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("blackheadm"), 0, -4.5, 0.0, 0, 0, 0, 0.4, false);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
}

//AttackLw3
unsafe extern "C" fn effect_sans_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("top"), 2, 2, 2, 0, 0, 0, 0.6, false);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 1, 5.75, 11, -2, -45, 90, 0.46, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("throw"), 0, 0, -6, 0, 0, 0, 0.4, false);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
}

//AttackS4Charge
unsafe extern "C" fn effect_sans_AttackS4Charge(fighter: &mut L2CAgentBase) {
    loop {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("fingerr22"), 0, 0, 0, 0, 0, 0, 1, 2, 3, 3, 0, 0, 0, true);
    }
}

//AttackS4
unsafe extern "C" fn effect_sans_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 1.5, 0, 2.5, -22.5, 0, 0, 0.95, true);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 8, 19.5, 0, 0, 0, 1.75, false);
    }
}

//AttackHi4Charge
unsafe extern "C" fn effect_sans_AttackHi4Charge(fighter: &mut L2CAgentBase) {
    loop {
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("stick"), 0, 8.5, 3, 0, 0, 0, 1, 2, 3, 3, 0, 0, 0, true);
    }
}

//AttackHi4
unsafe extern "C" fn effect_sans_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 11.5, 0, 0, 0, 0, 0.5, true, 1.0);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 16.5, 0, 0, 0, 0, 0.5, true, 1.0);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 21.5, 0, 0, 0, 0, 0.825, true, 1.0);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 23.5, -0.2, 0, 0, 0, 1.6, false);
    }
}

//AttackLw4Charge
unsafe extern "C" fn effect_sans_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    loop {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("fingerl22"), 0, 0, 0, 0, 0, 0, 1, 2, 5, 8, 0, 0, 0, true);
    }
}

//AttackLw4
unsafe extern "C" fn effect_sans_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 6, -4.6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 1.5, 20, 10.2, 90, 0, 0, 0.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 1.5, 20, -10, 90, 0, 0, 0.9, true);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackAirN
unsafe extern "C" fn effect_sans_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -4.5, 3.5, 2.5, 0, 0, 0, 1.0, true);
		LAST_EFFECT_SET_COLOR(fighter, 255.0 / 255.0, 164.0 / 255.0, 231.0 / 255.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
		EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -4.5, 3.5, 2.5, 0, 0, 0, 1.0, true);
		LAST_EFFECT_SET_COLOR(fighter, 255.0 / 255.0, 164.0 / 255.0, 231.0 / 255.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
	}
}

//AttackAirF 
unsafe extern "C" fn effect_sans_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 4.5, 4.5, 0, 0, 0, 0.6, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4.5, 21.5, 0, 0, 0, 0.67, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 4.5, 4.5, 0, 0, 0, 0.6, true);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4.5, 21.5, 0, 0, 0, 0.67, false);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 4.5, 16.5, 0, 0, 0, 0.27, false);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
}

//AttackAirB 
unsafe extern "C" fn effect_sans_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0.75, -11, 0, 0, 0, 0.8, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 1, 5, -4.55, 176, -12, 259, 1.0, true, *EF_FLIP_YZ, 0.7);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
    }
}

//AttackAirHi
unsafe extern "C" fn effect_sans_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("poke_meloetta_end"), Hash40::new("top"), 0, 18.5, -2.5, 0, 0, 0, 0.6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
    }
}

//AttackAirLw
unsafe extern "C" fn effect_sans_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -4.2, 2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
}

//CatchAttack
unsafe extern "C" fn effect_sans_CatchAttack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_matchless_smoke1"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_windwave"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
}

//ThrowF
unsafe extern "C" fn effect_sans_ThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 5.0, 25.0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -1.7, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    }
}

//ThrowB
unsafe extern "C" fn effect_sans_ThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -1.7, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
    }
}

//ThrowHi
unsafe extern "C" fn effect_sans_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

//ThrowLw
unsafe extern "C" fn effect_sans_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_matchless_smoke1"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_windwave"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("palutena_catch_wind"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -1.5, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

//CliffEscape
unsafe extern "C" fn effect_sans_CliffEscape(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//CliffAttack
unsafe extern "C" fn effect_sans_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 4, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 4, 1, -5, 25, -170, 0.85, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 1.8);
    }
}

//SlipAttack
unsafe extern "C" fn effect_sans_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -1, 3.9, 3.5, 0, 190, -30, 0.8, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 5, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -1.5, 4.8, -8.3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true, 0.7);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -1.5, 3.9, -3.5, 0, 10, 30, 0.8, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5.4, 9.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true, 0.7);
    }
}

//DownAttackU
unsafe extern "C" fn effect_sans_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 5, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 6.0, 18.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true, 0.7);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 6.0, -17.75, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true, 0.7);
    }
}

//DownAttackD
unsafe extern "C" fn effect_sans_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 5, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5.0, 7.0, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 360, true, 0.7);
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5.0, -8.0, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 360, true, 0.7);
    }
}

//SpecialN
unsafe extern "C" fn effect_sans_SpecialN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.0, 0.0, 1.0, 0.65);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 7, 6, 0, 0, 0, 0.5, false);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//SpecialAirN
unsafe extern "C" fn effect_sans_SpecialAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 7, 6, 0, 0, 0, 0.5, false);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
}

//SpecialS
unsafe extern "C" fn effect_sans_SpecialS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -1, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SANS_GENERATE_ARTICLE_GASTER) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, 11, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
    }
}

//SpecialAirS
unsafe extern "C" fn effect_sans_SpecialAirS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SANS_GENERATE_ARTICLE_GASTER) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, 11, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
    }
}

//SpecialHiStart
unsafe extern "C" fn effect_sans_SpecialHiStart(fighter: &mut L2CAgentBase) {
    let lr = PostureModule::lr(fighter.module_accessor);
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 3.0 * lr, 6, 0, 0, 0, 0, 3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.88);
        EFFECT(fighter, Hash40::new("rockman_hardknuckle_fire"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.4);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 0.4);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 0.2);
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 1, 0.3);
        EFFECT(fighter, Hash40::new("rockman_hardknuckle"), Hash40::new("top"), -4.0 * lr, 6, 0.5, 0, 0, 0, 1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rockman_hardknuckle"), false, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 0);
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//SpecialAirHiStart
unsafe extern "C" fn effect_sans_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
    let lr = PostureModule::lr(fighter.module_accessor);
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 3.0 * lr, 6, 0, 0, 0, 0, 3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.88);
        EFFECT(fighter, Hash40::new("rockman_hardknuckle_fire"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.4);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 0.4);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 0.2);
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 1, 0.3);
        EFFECT(fighter, Hash40::new("rockman_hardknuckle"), Hash40::new("top"), -4.0 * lr, 6, 0.5, 0, 0, 0, 1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rockman_hardknuckle"), false, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 0);
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//SpecialHi
unsafe extern "C" fn effect_sans_SpecialHi(fighter: &mut L2CAgentBase) {
    let lr = PostureModule::lr(fighter.module_accessor);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 3.0 * lr, 6, 0, 0, 0, 0, 5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.88);
        EFFECT(fighter, Hash40::new("rockman_hardknuckle_fire"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT(fighter, Hash40::new("rockman_hardknuckle"), Hash40::new("top"), -4.0 * lr, 6, 0, 0, 0, 0, 1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 0.3);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 0.2);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rockman_hardknuckle"), false, false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//SpecialAirHi
unsafe extern "C" fn effect_sans_SpecialAirHi(fighter: &mut L2CAgentBase) {
    let lr = PostureModule::lr(fighter.module_accessor);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 3.0 * lr, 6, 0, 0, 0, 0, 5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.88);
        EFFECT(fighter, Hash40::new("rockman_hardknuckle_fire"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT(fighter, Hash40::new("rockman_hardknuckle"), Hash40::new("top"), -4.0 * lr, 6, 0, 0, 0, 0, 1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 0.3);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 0.2);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rockman_hardknuckle"), false, false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//SpecialLw
unsafe extern "C" fn effect_sans_SpecialLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.0, 0.0, 1.0, 0.65);
    }
    frame(fighter.lua_state_agent, 20.0);
    for _ in 0..9 {
        if is_excute(fighter) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_BONECAGE_EFFECT) {
                LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("articlebone"), 0, 0, 0, 0, 0, 90, 0.1, 0, 0, 0, 0, 0, 0, true);
                LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("articlebone"), 0, 0, -5, 0, 0, 90, 0.1, 0, 0, 0, 0, 0, 0, true);
                EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("articlebone"), -2, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
                LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
                EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("articlebone"), -2, 0, -5, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
                LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
            }
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//SpecialAirLw
unsafe extern "C" fn effect_sans_SpecialAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.0, 0.0, 1.0, 0.65);
    }
    frame(fighter.lua_state_agent, 20.0);
    for _ in 0..9 {
        if is_excute(fighter) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_BONECAGE_EFFECT) {
                LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("articlebone"), 0, 0, 0, 0, 0, 90, 0.1, 0, 0, 0, 0, 0, 0, true);
                LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("articlebone"), 0, 0, -5, 0, 0, 90, 0.1, 0, 0, 0, 0, 0, 0, true);
                EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("articlebone"), -2, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
                LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
                EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("articlebone"), -2, 0, -5, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
                LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
            }
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//AppealHiR
unsafe extern "C" fn effect_sans_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), -5, 23, 0, 0, 90, 0, 1, true, 0.7);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("palutena_appeal_feather"), Hash40::new("top"), 0, 18, 0, 0, 180, 0, 1, true);
    }
}

//AppealHiL
unsafe extern "C" fn effect_sans_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), -5, 23, 0, 0, 90, 0, 1, true, 0.7);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("palutena_appeal_feather"), Hash40::new("top"), 0, 18, 0, 0, 180, 0, 1, true);
    }
}

//AppealSR
unsafe extern "C" fn effect_sans_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pacman_appeal_down"), Hash40::new("top"), 0, 7, 10.5, 0, 0, 0, 0.4, true);
    }
}

//AppealSL
unsafe extern "C" fn effect_sans_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pacman_appeal_down"), Hash40::new("top"), 0, 7, 10.5, 0, 0, 0, 0.4, true);
    }
}

//AppealLwR
unsafe extern "C" fn effect_sans_AppealLwR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 54.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("palutena_appeal_twinkle"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), 3, 21, -2, 0, -50, 0, 1, true, 0.9);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("palutena_appeal_twinkle"), false, false);
    }
}

//AppealLwL
unsafe extern "C" fn effect_sans_AppealLwL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 54.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("palutena_appeal_twinkle"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), 3, 21, -2, 0, -50, 0, 1, true, 0.9);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("palutena_appeal_twinkle"), false, false);
    }
}

//EntryL
unsafe extern "C" fn effect_sans_EntryL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 10.0);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 10.0);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 10.0);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 10.0);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 10.0);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 10.0);
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//EntryR
unsafe extern "C" fn effect_sans_EntryR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 10.0);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 10.0);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 10.0);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 10.0);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 10.0);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        FLASH(fighter, 0, 0, 0, 10.0);
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//Final
unsafe extern "C" fn effect_sans_Final(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bg_black"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -1, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SANS_GENERATE_ARTICLE_GASTER) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, 11, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
    }
}

//FinalAir
unsafe extern "C" fn effect_sans_FinalAir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bg_black"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SANS_GENERATE_ARTICLE_GASTER) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, 11, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
    }
}

pub fn install() {
    Agent::new("palutena")
    .effect_acmd("effect_wait3_sans", effect_sans_Wait3, Low)
    .effect_acmd("effect_escapeb_sans", effect_sans_EscapeB, Low)
    .effect_acmd("effect_escapef_sans", effect_sans_EscapeF, Low)
    .effect_acmd("effect_escapen_sans", effect_sans_EscapeN, Low)
    .effect_acmd("effect_escapeair_sans", effect_sans_EscapeAir, Low)
    .effect_acmd("effect_attack11_sans", effect_sans_Attack11, Low)
    .effect_acmd("effect_attack12_sans", effect_sans_Attack12, Low)
    .effect_acmd("effect_attackdash_sans", effect_sans_AttackDash, Low)
    .effect_acmd("effect_attacks3_sans", effect_sans_AttackS3, Low)
    .effect_acmd("effect_attackhi3_sans", effect_sans_AttackHi3, Low)
    .effect_acmd("effect_attacklw3_sans", effect_sans_AttackLw3, Low)
    .effect_acmd("effect_attacks4charge_sans", effect_sans_AttackS4Charge, Low)
    .effect_acmd("effect_attacks4_sans", effect_sans_AttackS4, Low)
    .effect_acmd("effect_attackhi4charge_sans", effect_sans_AttackHi4Charge, Low)
    .effect_acmd("effect_attackhi4_sans", effect_sans_AttackHi4, Low)
    .effect_acmd("effect_attacklw4charge_sans", effect_sans_AttackLw4Charge, Low)
    .effect_acmd("effect_attacklw4_sans", effect_sans_AttackLw4, Low)
    .effect_acmd("effect_attackairn_sans", effect_sans_AttackAirN, Low)
    .effect_acmd("effect_attackairf_sans", effect_sans_AttackAirF, Low)    
    .effect_acmd("effect_attackairb_sans", effect_sans_AttackAirB, Low)
    .effect_acmd("effect_attackairhi_sans", effect_sans_AttackAirHi, Low)
    .effect_acmd("effect_attackairlw_sans", effect_sans_AttackAirLw, Low)
    .effect_acmd("effect_catchattack_sans", effect_sans_CatchAttack, Low)
    .effect_acmd("effect_throwf_sans", effect_sans_ThrowF, Low)
    .effect_acmd("effect_throwb_sans", effect_sans_ThrowB, Low)
    .effect_acmd("effect_throwhi_sans", effect_sans_ThrowHi, Low)
    .effect_acmd("effect_throwlw_sans", effect_sans_ThrowLw, Low)
    .effect_acmd("effect_downattackd_sans", effect_sans_DownAttackD, Low)
    .effect_acmd("effect_downattacku_sans", effect_sans_DownAttackU, Low)
    .effect_acmd("effect_cliffattack_sans", effect_sans_CliffAttack, Low)
    .effect_acmd("effect_cliffescape_sans", effect_sans_CliffEscape, Low)
    .effect_acmd("effect_slipattack_sans", effect_sans_SlipAttack, Low)
    .effect_acmd("effect_specialn_sans", effect_sans_SpecialN, Low)
    .effect_acmd("effect_specialairn_sans", effect_sans_SpecialAirN, Low)
    .effect_acmd("effect_specials_sans", effect_sans_SpecialS, Low)
    .effect_acmd("effect_specialairs_sans", effect_sans_SpecialAirS, Low)
    .effect_acmd("effect_specialhistart_sans", effect_sans_SpecialHiStart, Low)
    .effect_acmd("effect_specialairhistart_sans", effect_sans_SpecialAirHiStart, Low)
    .effect_acmd("effect_specialhi_sans", effect_sans_SpecialHi, Low)
    .effect_acmd("effect_specialairhi_sans", effect_sans_SpecialAirHi, Low)
    .effect_acmd("effect_speciallw_sans", effect_sans_SpecialLw, Low)
    .effect_acmd("effect_specialairlw_sans", effect_sans_SpecialAirLw, Low)
    .effect_acmd("effect_appealsr_sans", effect_sans_AppealSR, Low)
    .effect_acmd("effect_appealsl_sans", effect_sans_AppealSL, Low)
    .effect_acmd("effect_appealhir_sans", effect_sans_AppealHiR, Low)
    .effect_acmd("effect_appealhil_sans", effect_sans_AppealHiL, Low)
    .effect_acmd("effect_appeallwr_sans", effect_sans_AppealLwR, Low)
    .effect_acmd("effect_appeallwl_sans", effect_sans_AppealLwL, Low)
    .effect_acmd("effect_entryl_sans", effect_sans_EntryL, Low)
    .effect_acmd("effect_entryr_sans", effect_sans_EntryR, Low)
    .effect_acmd("effect_final_sans", effect_sans_Final, Low)
    .effect_acmd("effect_finalair_sans", effect_sans_FinalAir, Low)
    .install();
}