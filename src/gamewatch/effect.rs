use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//SpecialS2GFX
    agent = "gamewatch", 
    script = "effect_specials2", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn gamewatch_sideb2gfx(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirS2GFX
    agent = "gamewatch", 
    script = "effect_specialairs2", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn gamewatch_sidebair2gfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    for _ in 0..12 {
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("rot"), 0, 2, 0, 0, 0, 0, 1.2, 16, 20, 16, 0, 0, 0, false);
        }
    }
}

#[acmd_script(//SpecialS6GFX
    agent = "gamewatch", 
    script = "effect_specials6", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn gamewatch_sideb6gfx(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirS6GFX
    agent = "gamewatch", 
    script = "effect_specialairs6", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn gamewatch_sidebair6gfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    for _ in 0..12 {
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("rot"), 0, 2, 0, 0, 0, 0, 1.2, 16, 20, 16, 0, 0, 0, false);
        }
    }
}

#[acmd_script(//SpecialS7GFX
    agent = "gamewatch", 
    script = "effect_specials7", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn gamewatch_sideb7gfx(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirS7GFX
    agent = "gamewatch", 
    script = "effect_specialairs7", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn gamewatch_sidebair7gfx(fighter: &mut L2CAgentBase) {
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
        gamewatch_sideb2gfx,
        gamewatch_sidebair2gfx,
        gamewatch_sideb6gfx,
        gamewatch_sidebair6gfx,
        gamewatch_sideb7gfx,
        gamewatch_sidebair7gfx
    );
}