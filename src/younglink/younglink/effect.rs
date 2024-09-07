use crate::imports::BuildImports::*;

//AttackDash
unsafe extern "C" fn effect_younglink_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackHi4
unsafe extern "C" fn effect_younglink_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("younglink_sword3"), Hash40::new("younglink_sword2"), 4, Hash40::new("sword"), 0.5, 0.0, 0.0, Hash40::new("sword"), 9.7, 0.0, -0.25, true, Hash40::new("younglink_sword_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.6, 0.2);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("younglink_sword3"), Hash40::new("younglink_sword2"), 4, Hash40::new("sword"), 0.5, 0.0, 0.0, Hash40::new("sword"), 9.7, 0.0, -0.25, true, Hash40::new("younglink_sword_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.6, 0.2);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
}

//AttackAirF
unsafe extern "C" fn effect_younglink_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("younglink_sword1"), Hash40::new("younglink_sword2"), 4, Hash40::new("sword"), 0.5, 0.0, 0.0, Hash40::new("sword"), 9.7, 0.0, -0.25, true, Hash40::new("younglink_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
}

pub fn install() {
    Agent::new("younglink")
    .effect_acmd("effect_attackdash", effect_younglink_AttackDash, Low)
    .effect_acmd("effect_attackhi4", effect_younglink_AttackHi4, Low)
    .effect_acmd("effect_attackairf", effect_younglink_AttackAirF, Low)    
    .install();
}