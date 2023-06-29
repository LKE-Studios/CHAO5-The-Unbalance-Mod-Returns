use crate::imports::BuildImports::*;

#[acmd_script(//GlideStart
    agent = "trail", 
    script = "effect_glidestart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_trail_glidestart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -4.2, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 4.5, 0, 0, 0, 0, 2.9, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.8, /*G*/ 0.13, /*B*/ 0.53);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_special_all_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.8, /*G*/ 0.13, /*B*/ 0.52);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_status_attack_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.8, /*G*/ 0.13, /*B*/ 0.52);
    }
}

#[acmd_script(//GlideWing
    agent = "trail", 
    script = "effect_glidewing", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_trail_glidewing(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

#[acmd_script(//GlideAttack
    agent = "trail", 
    script = "effect_glideattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_trail_glideattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.2, true);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new_raw(0x13dcc5dae1), Hash40::new_raw(0x1345cc8b5b), 7, Hash40::new("haver"), 0.0, 2.0, 0.0, Hash40::new("haver"), 0.0, 13.8, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("trail_keyblade_flare"), false, true);
        AFTER_IMAGE_OFF(fighter, 6);
    }
}  

#[acmd_script(//GlideLanding
    agent = "trail", 
    script = "effect_glidelanding", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_trail_glidelanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.48, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//Fly
    agent = "trail_fire", 
    script = "effect_fly", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_trail_fire_fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("trail_fire_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 4.0, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

#[acmd_script(//Fly
    agent = "trail_fire", 
    script = "effect_fly2", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_trail_fire_fly2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("trail_fire_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

#[acmd_script(//Fly
    agent = "trail_ice", 
    script = "effect_fly", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_trail_ice_fly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("trail_ice_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 4.5, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

#[acmd_script(//FlyLast
    agent = "trail_ice", 
    script = "effect_flylast", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_trail_ice_flylast(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("trail_ice_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 6.0, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_trail_glidestart,
        effect_trail_glidewing,
        effect_trail_glideattack,
        effect_trail_glidelanding,
        effect_trail_fire_fly,
        effect_trail_fire_fly2,
        effect_trail_ice_fly,
        effect_trail_ice_flylast
    );
}
