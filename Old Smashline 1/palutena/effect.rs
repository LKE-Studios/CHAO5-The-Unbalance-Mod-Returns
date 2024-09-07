use crate::imports::BuildImports::*;

#[acmd_script(//GlideStart
    agent = "palutena", 
    script = "effect_glidestart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_palutena_glidestart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -5.3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 4.4, 0, 0, 0, 0, 3.0, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.9, /*B*/ 0.65);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), 1.0, 21.0, 2.5, 0, -50.0, 0, 1.0, true);
        EFFECT(fighter, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 6.0, -6.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script(//GlideWing
    agent = "palutena", 
    script = "effect_glidewing", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_palutena_glidewing(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        EFFECT_FOLLOW(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), 1.0, 21.0, 2.5, 0, -50.0, 0, 1.0, true);
    }
}

#[acmd_script(//GlideAttack
    agent = "palutena", 
    script = "effect_glideattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_palutena_glideattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), 1.0, 21.0, 2.5, 0, -50.0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 6.0, -6.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FLW_POS(fighter, Hash40::new("palutena_counter_attack"), Hash40::new("stick"), 0, 8.5, 0, 0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
}  

#[acmd_script(//GlideLanding
    agent = "palutena", 
    script = "effect_glidelanding", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_palutena_glidelanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//GlideEnd
    agent = "palutena", 
    script = "effect_glideend", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_palutena_glideend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), 1.0, 21.0, 2.5, 0, -50.0, 0, 1.0, true);
        EFFECT(fighter, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 6.0, -6.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script(//Explode
    agent = "palutena_explosiveflame", 
    script = "effect_explode", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_palutena_explosiveflame_explode(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("palutena_bomb"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("palutena_bomb_appear"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("palutena_bomb_finish"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_palutena_glidestart,
        effect_palutena_glidewing,
        effect_palutena_glideattack,
        effect_palutena_glidelanding,
        effect_palutena_glideend,
        effect_palutena_explosiveflame_explode
    );
}