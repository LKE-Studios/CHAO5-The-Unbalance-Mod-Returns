use crate::imports::BuildImports::*;

#[acmd_script(//JumpAerialF4, JumpAerialF5, JumpAerialF6, JumpAerialF7
    agent = "pit", 
    scripts = ["effect_jumpaerialf4", "effect_jumpaerialf5", "effect_jumpaerialf6", "effect_jumpaerialf7"],
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pit_jumpaerialf(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pit_feather"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, false);
    }
}

#[acmd_script(//GlideStart
    agent = "pit", 
    script = "effect_glidestart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pit_glidestart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pit_sword"), Hash40::new("bowr"), -0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pit_fly_miracle_start"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 4.2, 0, 0, 0, 0, 3.0, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.9, /*B*/ 0.04);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pit_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, 0, 1, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 1.0, /*B*/ 0.0);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("pit_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, 0, -1, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 1.0, /*B*/ 0.0);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), false, false);
    }
}

#[acmd_script(//GlideWing
    agent = "pit", 
    script = "effect_glidewing", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pit_glidewing(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

#[acmd_script(//GlideAttack
    agent = "pit", 
    script = "effect_glideattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pit_glideattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pit_sword"), Hash40::new("bowr"), -0, 0, 0, 0, 90, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pit_feather"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pit_sword1"), Hash40::new("tex_pit_sword2"), 3, Hash40::new("bowr"), 0.0, 0.0, -0.2, Hash40::new("bowr"), 0.0, 10.4, -1.2, true, Hash40::new("pit_sword"), Hash40::new("bowr"), 0.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pit_sword1"), Hash40::new("tex_pit_sword2"), 4, Hash40::new("bowr"), 0.0, -0.9, -0.2, Hash40::new("bowr"), 0.0, -11.0, -1.2, true, Hash40::new("pit_sword"), Hash40::new("bowr"), 0.0, -0.9, 0.0, 180.0, 90.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), true, true);
        AFTER_IMAGE_OFF(fighter, 3);
        AFTER_IMAGE_OFF(fighter, 4);
    }
}  

#[acmd_script(//GlideLanding
    agent = "pit", 
    script = "effect_glidelanding", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pit_glidelanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), false, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//GlideEnd
    agent = "pit", 
    script = "effect_glideend", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pit_glideend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), false, false);
        EFFECT_FOLLOW(fighter, Hash40::new("pit_feather"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, false);
    }
}   

#[acmd_script(//SpecialHiStart 
    agent = "pit", 
    script = "effect_specialhistart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pit_specialhistart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pit_fly_miracle_start"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pit_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, 0, 1, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 0.1, /*B*/ 2.2);
        EFFECT_FOLLOW(fighter, Hash40::new("pit_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, 0, -1, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 0.1, /*B*/ 2.2);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pit_fly_miracle"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

#[acmd_script(//SpecialAirHiStart 
    agent = "pit", 
    script = "effect_specialairhistart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pit_specialairhistart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pit_fly_miracle_start"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pit_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, 0, 1, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 0.1, /*B*/ 2.2);
        EFFECT_FOLLOW(fighter, Hash40::new("pit_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, 0, -1, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 0.1, /*B*/ 2.2);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pit_fly_miracle"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_pit_jumpaerialf,
        effect_pit_glidestart,
        effect_pit_glidewing,
        effect_pit_glideattack,
        effect_pit_glidelanding,
        effect_pit_glideend,
        effect_pit_specialhistart,
        effect_pit_specialairhistart
    );
}