use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
//use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//Crash1
    agent = "brave_crash", 
    script = "effect_crash1", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_brave_crash1(fighter: &mut L2CAgentBase) {
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("brave_fullburst"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 90.0, 0.0, 3.0, true);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("brave_fullburst"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, -90.0, 0.0, 3.0, true);
        }
    }
    frame(fighter.lua_state_agent, 80.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("brave_fullburst_finish"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 90.0, 0.0, 2.5, true);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("brave_fullburst_finish"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, -90.0, 0.0, 2.5, true);
        }
    }
    frame(fighter.lua_state_agent, 82.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_DETACH_KIND(fighter, Hash40::new("brave_fullburst_finish"), -1);
    }
}

#[acmd_script(//CrashEnd1
    agent = "brave_crash", 
    script = "effect_crashend1", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_brave_crashend1(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("brave_fullburst_end"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, true);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_DETACH_KIND(fighter, Hash40::new("brave_fullburst_end"), -1);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_brave_crash1,
        effect_brave_crashend1
    );
}