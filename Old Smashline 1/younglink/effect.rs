use crate::imports::BuildImports::*;

#[acmd_script(//AttackDash
    agent = "younglink", 
    script = "effect_attackdash", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_younglink_attackdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//AttackAirF
    agent = "younglink", 
    script = "effect_attackairf", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_younglink_attackairf(fighter: &mut L2CAgentBase) {
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
    smashline::install_acmd_scripts!(
        effect_younglink_attackdash,
        effect_younglink_attackairf
    );
}