use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase; 

#[acmd_script(//AttackS4Hi
    agent = "mariod", 
    script = "effect_attacks4hi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn mariod_sidesmashupgfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 10, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_smash_aura"), Hash40::new("mariod_smash_aura"), Hash40::new("havel"), -1.5, 0, 0, 0, 0, 0, 0.3, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("mariod_smash_wind"), Hash40::new("top"), 0, 10.0, 11.0, -25.0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FLIP(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("top"), 1.0, 10.0, 8.75, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.85);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_aura"), false, true);
    }
    frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), true, true);
    }
}

#[acmd_script(//AttackS4
    agent = "mariod", 
    script = "effect_attacks4", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn mariod_sidesmashgfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 10, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_smash_aura"), Hash40::new("mariod_smash_aura"), Hash40::new("havel"), -1.5, 0, 0, 0, 0, 0, 0.3, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("mariod_smash_wind"), Hash40::new("top"), 0, 9, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FLIP(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("top"), 1, 8, 9, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.85);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_aura"), false, true);
    }
    frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), true, true);
    }
}

#[acmd_script(//AttackS4Lw
    agent = "mariod", 
    script = "effect_attacks4lw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn mariod_sidesmashdowngfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 10, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_smash_aura"), Hash40::new("mariod_smash_aura"), Hash40::new("havel"), -1.5, 0, 0, 0, 0, 0, 0.3, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("mariod_smash_wind"), Hash40::new("top"), 0, 5.0, 11.5, 30.0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FLIP(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("top"), 1.0, 4.5, 8.75, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.85);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_aura"), false, true);
    }
    frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), true, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        mariod_sidesmashupgfx,
        mariod_sidesmashgfx,
        mariod_sidesmashdowngfx
    );
}
