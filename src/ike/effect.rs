use crate::imports::BuildImports::*;

#[acmd_script(//AttackAirHi
    agent = "ike", 
    script = "effect_attackairhi",
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_ike_attackairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_ike_attackairhi
    );
}