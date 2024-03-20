use crate::imports::BuildImports::*;

//AttackS4Hi
unsafe extern "C" fn effect_mariod_AttackS4Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 10, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_smash_aura"), Hash40::new("mariod_smash_aura"), Hash40::new("havel"), -1.5, 0, 0, 0, 0, 0, 0.3, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("mariod_smash_wind"), Hash40::new("top"), 0, 10.0, 11.0, -25.0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("top"), 1.0, 10.0, 8.75, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.85);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_aura"), false, true);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), true, true);
    }
}

//AttackS4
unsafe extern "C" fn effect_mariod_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 10, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_smash_aura"), Hash40::new("mariod_smash_aura"), Hash40::new("havel"), -1.5, 0, 0, 0, 0, 0, 0.3, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("mariod_smash_wind"), Hash40::new("top"), 0, 9, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("top"), 1, 8, 9, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.85);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_aura"), false, true);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), true, true);
    }
}

//AttackS4Lw
unsafe extern "C" fn effect_mariod_AttackS4Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 10, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_smash_aura"), Hash40::new("mariod_smash_aura"), Hash40::new("havel"), -1.5, 0, 0, 0, 0, 0, 0.3, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("mariod_smash_wind"), Hash40::new("top"), 0, 5.0, 11.5, 30.0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("top"), 1.0, 4.5, 8.75, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.85);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_aura"), false, true);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), true, true);
    }
}

pub fn install() {
    Agent::new("mariod")
    .effect_acmd("effect_attacks4hi", effect_mariod_AttackS4Hi)
    .effect_acmd("effect_attacks4", effect_mariod_AttackS4)
    .effect_acmd("effect_attacks4lw", effect_mariod_AttackS4Lw)
    .install();
}
