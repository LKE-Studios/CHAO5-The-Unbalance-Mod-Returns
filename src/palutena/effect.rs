use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
//use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//GlideStart
    agent = "palutena", 
    script = "effect_glidestart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn palutena_glidestartgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -5.3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 5.2, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.9, /*B*/ 0.65);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), 1.0, 21.0, 2.5, 0, -50.0, 0, 1.0, true);
        macros::EFFECT(fighter, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 6.0, -6.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script(//GlideWingGFX
    agent = "palutena", 
    script = "effect_glidewing", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn palutena_glide2gfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), 1.0, 21.0, 2.5, 0, -50.0, 0, 1.0, true);
    }
}

#[acmd_script(//GlideAttack
    agent = "palutena", 
    script = "effect_glideattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn palutena_glideattackgfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), 1.0, 21.0, 2.5, 0, -50.0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 6.0, -6.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FLW_POS(fighter, Hash40::new("palutena_counter_attack"), Hash40::new("stick"), 0, 8.5, 0, 0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
}  

#[acmd_script(//GlideLanding
    agent = "palutena", 
    script = "effect_glidelanding", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn palutena_glidelandinggfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//GlideEnd
    agent = "palutena", 
    script = "effect_glideend", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn palutena_glideendgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), 1.0, 21.0, 2.5, 0, -50.0, 0, 1.0, true);
        macros::EFFECT(fighter, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 6.0, -6.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}   

pub fn install() {
    smashline::install_acmd_scripts!(
        palutena_glidestartgfx,
        palutena_glide2gfx,
        palutena_glideattackgfx,
        palutena_glidelandinggfx,
        palutena_glideendgfx
    );
}