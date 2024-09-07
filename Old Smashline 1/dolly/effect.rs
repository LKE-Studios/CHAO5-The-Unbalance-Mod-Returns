use crate::imports::BuildImports::*;

#[acmd_script(//SuperSpecial
    agent = "dolly_burst",
    script = "effect_superspecial",
    category = ACMD_EFFECT,
    low_priority )]
unsafe fn effect_dolly_burst_superspecial(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), -3.0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_ground"), Hash40::new("top"), 0, 0, -3.0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_base"), Hash40::new("top"), 0, 0, -3.0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano"), Hash40::new("top"), 0, 0, -3.0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_flash"), Hash40::new("top"), 0, 0, -3.0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_ground"), Hash40::new("top"), 0, 0, 27.0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_base"), Hash40::new("top"), 0, 0, 27.0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano"), Hash40::new("top"), 0, 0, 27.0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_flash"), Hash40::new("top"), 0, 0, 27.0, 0, 0, 0, 1.1, true);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_ground"), Hash40::new("top"), 0, 0, 57.0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_base"), Hash40::new("top"), 0, 0, 57.0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano"), Hash40::new("top"), 0, 0, 57.0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_flash"), Hash40::new("top"), 0, 0, 57.0, 0, 0, 0, 1.1, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_dolly_burst_superspecial
    );
}