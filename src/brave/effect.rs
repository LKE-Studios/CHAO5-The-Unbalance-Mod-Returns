use crate::imports::BuildImports::*;

#[acmd_script(//Crash1
    agent = "brave_crash", 
    script = "effect_crash1", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_brave_crash1(fighter: &mut L2CAgentBase) {
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("brave_fullburst"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 90.0, 0.0, 4.5, true);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("brave_fullburst"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, -90.0, 0.0, 4.5, true);
        }
    }
    frame(fighter.lua_state_agent, 80.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("brave_fullburst_finish"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 90.0, 0.0, 3.75, true);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("brave_fullburst_finish"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, -90.0, 0.0, 3.75, true);
        }
    }
    frame(fighter.lua_state_agent, 82.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("brave_fullburst_finish"), -1);
    }
}

#[acmd_script(//CrashEnd1
    agent = "brave_crash", 
    script = "effect_crashend1", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_brave_crashend1(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("brave_fullburst_end"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0, true);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("brave_fullburst_end"), -1);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_brave_crash1,
        effect_brave_crashend1
    );
}