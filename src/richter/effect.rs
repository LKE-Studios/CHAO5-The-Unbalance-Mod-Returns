use crate::imports::BuildImports::*;

#[acmd_script(//Fly
    agent = "richter_axe", 
    script = "effect_fly", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_richter_axe_fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("richter_ax_blur"), Hash40::new("top"), 0.0, 6.0, 0.0, 0.0, -90.0, 0.0, 3.7, true);
    }
}

#[acmd_script(//Fly
    agent = "richter_cross", 
    script = "effect_fly", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_richter_cross_fly(fighter: &mut L2CAgentBase) {
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("richter_cross"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, -0.0, 0.0, 2.1, true);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("richter_cross"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.1, true);
        }
    }
}

#[acmd_script(//Turn
    agent = "richter_cross", 
    script = "effect_turn", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_richter_cross_turn(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("richter_cross"), true, true);
    }
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("richter_cross"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.1, true);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("richter_cross"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, -0.0, 0.0, 2.1, true);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_erase_smoke"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_richter_axe_fly,
        effect_richter_cross_fly,
        effect_richter_cross_turn
    );
}