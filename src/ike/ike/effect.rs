use crate::imports::BuildImports::*;

//AttackAirHi
unsafe extern "C" fn effect_ike_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
}

pub fn install() {
    Agent::new("ike")
    .effect_acmd("effect_attackairhi", effect_ike_AttackAirHi)
    .install();
}