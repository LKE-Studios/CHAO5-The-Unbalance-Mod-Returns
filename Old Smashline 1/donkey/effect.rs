use crate::imports::BuildImports::*;

#[acmd_script(//SpecialHi2
    agent = "donkey", 
    script = "effect_specialhi2", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_donkey_specialhi2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    for _ in 0..5 {
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind"), Hash40::new("donkey_spin_wind"), Hash40::new("top"), 0, 10.0, -3.0, 0, 0, 5.0, 2.0, true, *EF_FLIP_YZ);
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_XY);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind"), Hash40::new("donkey_spin_wind"), Hash40::new("top"), 0, 10.0, -3.0, 0, 0, 5.0, 2.0, true, *EF_FLIP_YZ);
        }
        wait(fighter.lua_state_agent, 4.0);
    }
    frame(fighter.lua_state_agent, 57.0);
    if is_excute(fighter) {
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 58.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_down_smoke"), Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_XY);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind_flash"), Hash40::new("donkey_spin_wind_flash"), Hash40::new("top"), -3.5, 10, 15.5, 0, 0, 60.0, 1.3, true, *EF_FLIP_YZ);
    }
}

#[acmd_script(//SpecialAirHi
    agent = "donkey", 
    script = "effect_specialairhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_donkey_specialairhi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    for _ in 0..8 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, true);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind"), Hash40::new("donkey_spin_wind"), Hash40::new("top"), 0, 10.0, -3.0, 0, 0, 5, 2.7, true, *EF_FLIP_YZ);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind"), Hash40::new("donkey_spin_wind"), Hash40::new("top"), 0, 2.4, 0.0, 0, 0, 5, 0.8, true, *EF_FLIP_YZ);
        }
        wait(fighter.lua_state_agent, 8.0);
    }
}

#[acmd_script(//SpecialLwLoop
    agent = "donkey", 
    script = "effect_speciallwloop", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_donkey_speciallwloop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("donkey_handslap"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 2.2, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 2.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("donkey_handslap"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 2.2, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 2.8, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_donkey_specialhi2,
        effect_donkey_specialairhi,
        effect_donkey_speciallwloop
    );
}