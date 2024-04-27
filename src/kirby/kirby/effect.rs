use crate::imports::BuildImports::*;

//SpecialHiH
unsafe extern "C" fn effect_kirby_SpecialHiH(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_fcut_arc"), Hash40::new("kirby_fcut_arc"), Hash40::new("top"), 0, 1, 6, 85, 120, -90, 1, false, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.66);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        effect!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE3_ON, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0, 2, 0.25, Hash40::new("haver"), 0, 14, 0.5, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_fcut_arc"), Hash40::new("kirby_fcut_arc"), Hash40::new("top"), 0, 4, 6, -65, -100, -90, 1, false, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.66);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 0);
        effect!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE3_ON, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0, 2, 0.25, Hash40::new("haver"), 0, 14, 0.5, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 0);
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 6, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 6, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialAirHiH
unsafe extern "C" fn effect_kirby_SpecialAirHiH(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_fcut_arc"), Hash40::new("kirby_fcut_arc"), Hash40::new("top"), 0, 2, 0, 0, 90, -90, 1, false, *EF_FLIP_XY);
        LAST_EFFECT_SET_SCALE_W(fighter, 0.65, 1.0, 0.55);
        LAST_EFFECT_SET_RATE(fighter, 0.66);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        effect!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE3_ON, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0, 2, 0.25, Hash40::new("haver"), 0, 14, 0.5, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1);
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 0);
    }
}

pub fn install() {
    Agent::new("kirby")
    .effect_acmd("effect_specialhih", effect_kirby_SpecialHiH, Low)
    .effect_acmd("effect_specialairhih", effect_kirby_SpecialAirHiH, Low)
    .install();
}