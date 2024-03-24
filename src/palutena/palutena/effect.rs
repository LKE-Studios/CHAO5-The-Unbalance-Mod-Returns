use crate::imports::BuildImports::*;

//ShieldBreakFly
unsafe extern "C" fn effect_palutena_ShieldBreakFly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("sys_piyopiyo"), Hash40::new("head"), 0, 2, 0, 0, 0, 0, 1.4, true);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
    }
}

//GlideStart
unsafe extern "C" fn effect_palutena_GlideStart(fighter: &mut L2CAgentBase) {
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

//GlideWing
unsafe extern "C" fn effect_palutena_GlideWing(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        EFFECT_FOLLOW(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), 1.0, 21.0, 2.5, 0, -50.0, 0, 1.0, true);
    }
}

//GlideAttack
unsafe extern "C" fn effect_palutena_GlideAttack(fighter: &mut L2CAgentBase) {
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

//GlideLanding
unsafe extern "C" fn effect_palutena_GlideLanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

//GlideEnd
unsafe extern "C" fn effect_palutena_GlideEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), 1.0, 21.0, 2.5, 0, -50.0, 0, 1.0, true);
        EFFECT(fighter, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 6.0, -6.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}

//AttackHi4
unsafe extern "C" fn effect_palutena_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("stick"), 0, 8.5, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1, true);
            LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), 4, 21.5, 2, 0, -60, 0, 1, true, 0.7);
            LAST_EFFECT_SET_RATE(fighter, 1.1);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("palutena_backlight"), Hash40::new("top"), 4, 21.5, 2.5, 0, -60, 0, 1, true);
            LAST_EFFECT_SET_RATE(fighter, 1.1);
            EFFECT_FOLLOW(fighter, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1, true);
            LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(fighter.lua_state_agent, 17.0);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_HI4_IS_CHARGED) {
        frame(fighter.lua_state_agent, 23.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 25.0, 0, 0, 0, 0.9, true);
            LAST_EFFECT_SET_COLOR(fighter, /*R*/ 3.0, /*G*/ 0.8, /*B*/ 0.0);
            EFFECT_FOLLOW(fighter, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, -10.0, 0, 0, 0, 0.9, true);
            LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.3, /*G*/ 1.9, /*B*/ 0.0);
            PLAY_SE(fighter, Hash40::new("se_palutena_smash_h01"));
        }
        frame(fighter.lua_state_agent, 30.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 40.0, 0, 0, 0, 0.8, true);
            LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.6, /*G*/ 0.1, /*B*/ 0.1);
            EFFECT_FOLLOW(fighter, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, -25.0, 0, 0, 0, 0.8, true);
            LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.39, /*G*/ 0.045, /*B*/ 1.55);
            PLAY_SE(fighter, Hash40::new("se_palutena_smash_h01"));
        }
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("palutena_wand_light2"), false, false);
    }
}

pub fn install() {
    Agent::new("palutena")
    .effect_acmd("effect_shieldbreakfly", effect_palutena_ShieldBreakFly)
    .effect_acmd("effect_glidestart", effect_palutena_GlideStart)
    .effect_acmd("effect_glidewing", effect_palutena_GlideWing)
    .effect_acmd("effect_glideattack", effect_palutena_GlideAttack)
    .effect_acmd("effect_glidelanding", effect_palutena_GlideLanding)
    .effect_acmd("effect_glideend", effect_palutena_GlideEnd)
    .effect_acmd("effect_attackhi4", effect_palutena_AttackHi4)
    .install();
}