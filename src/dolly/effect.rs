use smash::app::sv_animcmd::*;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::Hash40;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;

#[acmd_script(//SuperSpecial
    agent = "dolly_burst",
    script = "effect_superspecial",
    category = ACMD_EFFECT,
    low_priority )]
unsafe fn effect_dolly_burst_superspecial(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), -3.0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_ground"), Hash40::new("top"), 0, 0, -3.0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_base"), Hash40::new("top"), 0, 0, -3.0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano"), Hash40::new("top"), 0, 0, -3.0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_flash"), Hash40::new("top"), 0, 0, -3.0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_ground"), Hash40::new("top"), 0, 0, 27.0, 0, 0, 0, 1.1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_base"), Hash40::new("top"), 0, 0, 27.0, 0, 0, 0, 1.1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano"), Hash40::new("top"), 0, 0, 27.0, 0, 0, 0, 1.1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_flash"), Hash40::new("top"), 0, 0, 27.0, 0, 0, 0, 1.1, true);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_ground"), Hash40::new("top"), 0, 0, 57.0, 0, 0, 0, 1.1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_base"), Hash40::new("top"), 0, 0, 57.0, 0, 0, 0, 1.1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano"), Hash40::new("top"), 0, 0, 57.0, 0, 0, 0, 1.1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("dolly_volcano_flash"), Hash40::new("top"), 0, 0, 57.0, 0, 0, 0, 1.1, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_dolly_burst_superspecial
    );
}