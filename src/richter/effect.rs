use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
//use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//Fly
    agent = "richter_axe", 
    script = "effect_fly", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn richter_axegfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("richter_ax_blur"), Hash40::new("top"), 0.0, 6.0, 0.0, 0.0, -90.0, 0.0, 3.7, true);
    }
}

#[acmd_script(//Fly
    agent = "richter_cross", 
    script = "effect_fly", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn richter_cross1gfx(fighter: &mut L2CAgentBase) {
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("richter_cross"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, -0.0, 0.0, 2.1, true);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("richter_cross"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.1, true);
        }
    }
}

#[acmd_script(//Turn
    agent = "richter_cross", 
    script = "effect_turn", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn richter_cross2gfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("richter_cross"), true, true);
    }
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("richter_cross"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.1, true);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("richter_cross"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, -0.0, 0.0, 2.1, true);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_erase_smoke"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        richter_axegfx,
        richter_cross1gfx,
        richter_cross2gfx
    );
}