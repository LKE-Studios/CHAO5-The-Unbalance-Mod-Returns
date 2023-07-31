use crate::imports::BuildImports::*;

#[acmd_script(//JumpAerialF4, JumpAerialF5, JumpAerialF6, JumpAerialF7
    agent = "pitb", 
    scripts = ["effect_jumpaerialf4", "effect_jumpaerialf5", "effect_jumpaerialf6", "effect_jumpaerialf7"],
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pitb_jumpaerial(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_feather"), Hash40::new("top"), 0, 15, -10, 0, 0, 0, 1, false);
    }
}

#[acmd_script(//GlideStart
    agent = "pitb", 
    script = "effect_glidestart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pitb_glidestart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_sword"), Hash40::new("bowr"), -0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_fly_miracle_start"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 4.2, 0, 0, 0, 0, 3.0, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.9, /*B*/ 0.14);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, 0, 1, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.9, /*B*/ 0.14);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, 0, -1, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.9, /*B*/ 0.14);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_fly_miracle_start"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_sword"), false, false);
    }
}

#[acmd_script(//GlideWing
    agent = "pitb", 
    script = "effect_glidewing", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pitb_glidewing(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

#[acmd_script(//GlideAttack
    agent = "pitb", 
    script = "effect_glideattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pitb_glideattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_sword"), Hash40::new("bowr"), -0, 0, 0, 0, 90, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_feather"), Hash40::new("top"), 0, 10, -10, 0, 0, 0, 1, false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 3, Hash40::new("bowr"), 0.0, 0.0, -0.2, Hash40::new("bowr"), 0.0, 10.4, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("bowr"), 0.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("bowr"), 0.0, -0.9, -0.2, Hash40::new("bowr"), 0.0, -11.0, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("bowr"), 0.0, -0.9, 0.0, 180.0, 90.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_sword"), true, true);
        AFTER_IMAGE_OFF(fighter, 3);
        AFTER_IMAGE_OFF(fighter, 4);
    }
}  

#[acmd_script(//GlideLanding
    agent = "pitb", 
    script = "effect_glidelanding", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pitb_glidelanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), false, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//GlideEnd
    agent = "pitb", 
    script = "effect_glideend", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pitb_glideend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), false, false);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_feather"), Hash40::new("top"), 0, 10, -10, 0, 0, 0, 1, false);
    }
}   

#[acmd_script(//Fly
    agent = "pitb_bowarrow", 
    script = "effect_fly", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pitb_arrow(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_pa_fly_arrow"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_pa_fly_arrow2"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 2.2, true);
    }
}

#[acmd_script(//SpecialHiStart
    agent = "pitb", 
    script = "effect_specialhistart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pitb_specialhistart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_fly_miracle_start"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, 0, 1, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.95, /*G*/ 0.0, /*B*/ 2.1);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, 0, -1, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.95, /*G*/ 0.0, /*B*/ 2.1);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_fly_miracle_start"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_fly_miracle"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

#[acmd_script(//SpecialAirHiStart
    agent = "pitb", 
    script = "effect_specialairhistart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pitb_specialairhistart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_fly_miracle_start"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, 0, 1, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.95, /*G*/ 0.0, /*B*/ 2.1);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, 0, -1, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.95, /*G*/ 0.0, /*B*/ 2.1);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_fly_miracle_start"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_fly_miracle"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_pitb_jumpaerial,
        effect_pitb_glidestart,
        effect_pitb_glidewing,
        effect_pitb_glideattack,
        effect_pitb_glidelanding,
        effect_pitb_glideend,
        effect_pitb_arrow,
        effect_pitb_specialhistart,
        effect_pitb_specialairhistart
    );
}