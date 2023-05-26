use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//SpecialS6
    agent = "gamewatch", 
    script = "effect_specials6", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_gamewatch_specials6(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 0..12 {
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("rot"), 0, 2, 0, 0, 0, 0, 1.2, 16, 20, 16, 0, 0, 0, false);
        }
    }
}

#[acmd_script(//SpecialAirS6
    agent = "gamewatch", 
    script = "effect_specialairs6", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_gamewatch_specialairs6(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    for _ in 0..12 {
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("rot"), 0, 2, 0, 0, 0, 0, 1.2, 16, 20, 16, 0, 0, 0, false);
        }
    }
}

#[acmd_script(//SpecialS7
    agent = "gamewatch", 
    script = "effect_specials7", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_gamewatch_specials7(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 0..12 {
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("rot"), 0, 2, 0, 0, 0, 0, 1.2, 16, 20, 16, 0, 0, 0, false);
        }
    }
}

#[acmd_script(//SpecialAirS7
    agent = "gamewatch", 
    script = "effect_specialairs7", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_gamewatch_specialairs7(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    for _ in 0..12 {
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("rot"), 0, 2, 0, 0, 0, 0, 1.2, 16, 20, 16, 0, 0, 0, false);
        }
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_gamewatch_specials6,
        effect_gamewatch_specialairs6,
        effect_gamewatch_specials7,
        effect_gamewatch_specialairs7
    );
}