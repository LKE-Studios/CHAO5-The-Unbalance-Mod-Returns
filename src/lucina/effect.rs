use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script(//AttackAirN
    agent = "lucina", 
    script = "effect_attackairn", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn effect_lucina_attackairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 8, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_sword_light"), Hash40::new("sword1"), 0, 0, 11, 0, 0, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 5);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("lucina_sword_light"), false, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_lucina_attackairn
    );
}