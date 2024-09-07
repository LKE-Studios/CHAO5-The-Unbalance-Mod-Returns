use crate::imports::BuildImports::*;

//JumpAerialF4
unsafe extern "C" fn effect_pitb_JumpAerialF4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_feather"), Hash40::new("top"), 0, 15, -10, 0, 0, 0, 1, false);
    }
}

//JumpAerialF5
unsafe extern "C" fn effect_pitb_JumpAerialF5(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_feather"), Hash40::new("top"), 0, 15, -10, 0, 0, 0, 1, false);
    }
}

//JumpAerialF6
unsafe extern "C" fn effect_pitb_JumpAerialF6(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_feather"), Hash40::new("top"), 0, 15, -10, 0, 0, 0, 1, false);
    }
}

//JumpAerialF7
unsafe extern "C" fn effect_pitb_JumpAerialF7(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_feather"), Hash40::new("top"), 0, 15, -10, 0, 0, 0, 1, false);
    }
}

//GlideStart
unsafe extern "C" fn effect_pitb_GlideStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_sword"), Hash40::new("bowr"), -0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_fly_miracle_start"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 4.2, 0, 0, 0, 0, 3.0, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.5, /*B*/ 0.14);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, 0, 1, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.5, /*B*/ 0.14);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, 0, -1, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.5, /*B*/ 0.14);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_fly_miracle_start"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_sword"), false, false);
    }
}

//GlideWing
unsafe extern "C" fn effect_pitb_GlideWing(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

//GlideAttack
unsafe extern "C" fn effect_pitb_GlideAttack(fighter: &mut L2CAgentBase) {
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

//GlideLanding
unsafe extern "C" fn effect_pitb_GlideLanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), false, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

//GlideEnd
unsafe extern "C" fn effect_pitb_GlideEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), false, false);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_feather"), Hash40::new("top"), 0, 10, -10, 0, 0, 0, 1, false);
    }
}   

//SpecialHiStart
unsafe extern "C" fn effect_pitb_SpecialHiStart(fighter: &mut L2CAgentBase) {
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

//SpecialAirHiStart
unsafe extern "C" fn effect_pitb_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
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
    Agent::new("pitb")
    .effect_acmd("effect_jumpaerialf4", effect_pitb_JumpAerialF4, Low)
    .effect_acmd("effect_jumpaerialf5", effect_pitb_JumpAerialF5, Low)
    .effect_acmd("effect_jumpaerialf6", effect_pitb_JumpAerialF6, Low)
    .effect_acmd("effect_jumpaerialf4", effect_pitb_JumpAerialF7, Low)
    .effect_acmd("effect_glidestart", effect_pitb_GlideStart, Low)
    .effect_acmd("effect_glidewing", effect_pitb_GlideWing, Low)
    .effect_acmd("effect_glideattack", effect_pitb_GlideAttack, Low)
    .effect_acmd("effect_glidelanding", effect_pitb_GlideLanding, Low)
    .effect_acmd("effect_specialhistart", effect_pitb_SpecialHiStart, Low)
    .effect_acmd("effect_specialairhistart", effect_pitb_SpecialAirHiStart, Low)
    .install();
}