use crate::imports::BuildImports::*;

//AttackAirLw
unsafe extern "C" fn effect_marth_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_marth_sword1"), Hash40::new("tex_marth_sword2"), 8, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 6);
    }
}

pub fn install() {
    Agent::new("marth")
    .effect_acmd("effect_attackairlw", effect_marth_AttackAirLw)
    .install();
}