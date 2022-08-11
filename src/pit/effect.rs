use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//GlideAttack
    agent = "pit", 
    script = "effect_glideattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn pit_glideattackgfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pit_sword"), Hash40::new("swordr2"), 0, 0.8, 0, 0, 90, 0, 1, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pit_sword1"), Hash40::new("tex_pit_sword2"), 3, Hash40::new("swordr1"), 0.0, 0.0, -2.0, Hash40::new("swordr1"), 0, 10.4, -1.2, true, Hash40::new("pit_sword"), Hash40::new("swordr1"), 0, 0, 0, -9.7, 90.0, -21.0, 1.0, 0.0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), true, true);
        macros::AFTER_IMAGE_OFF(fighter, 3);
    }
}  

pub fn install() {
    smashline::install_acmd_scripts!(
        pit_glideattackgfx
    );
}