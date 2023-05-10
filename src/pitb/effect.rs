use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//JumpAerialF4, JumpAerialF5, JumpAerialF6, JumpAerialF7
    agent = "pitb", 
    scripts = ["effect_jumpaerialf4", "effect_jumpaerialf5", "effect_jumpaerialf6", "effect_jumpaerialf7"],
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pitb_jumpaerial(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pitb_feather"), Hash40::new("top"), 0, 15, -10, 0, 0, 0, 1, false);
    }
}

#[acmd_script(//GlideStart
    agent = "pitb", 
    script = "effect_glidestart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pitb_glidestart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pitb_sword"), Hash40::new("bowr"), -0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pitb_fly_miracle_start"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 4.2, 0, 0, 0, 0, 3.0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.9, /*B*/ 0.14);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, 0, 1, 0, 0, 0, 1, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pitb_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, 0, -1, 0, 0, 0, 1, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pitb_fly_miracle_start"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pitb_sword"), false, false);
    }
}

#[acmd_script(//GlideWing
    agent = "pitb", 
    script = "effect_glidewing", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pitb_glidewing(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

#[acmd_script(//GlideAttack
    agent = "pitb", 
    script = "effect_glideattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pitb_glideattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pitb_sword"), Hash40::new("bowr"), -0, 0, 0, 0, 90, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pitb_feather"), Hash40::new("top"), 0, 10, -10, 0, 0, 0, 1, false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 3, Hash40::new("bowr"), 0.0, 0.0, -0.2, Hash40::new("bowr"), 0.0, 10.4, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("bowr"), 0.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("bowr"), 0.0, -0.9, -0.2, Hash40::new("bowr"), 0.0, -11.0, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("bowr"), 0.0, -0.9, 0.0, 180.0, 90.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        //macros::EFFECT_FOLLOW(fighter, Hash40::new("pit_atk_wind"), Hash40::new("top"), -7, 6, 2.3, -12, -42, 48, 1, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pitb_sword"), true, true);
        //macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_atk_wind"), true, true);
        macros::AFTER_IMAGE_OFF(fighter, 3);
        macros::AFTER_IMAGE_OFF(fighter, 4);
    }
}  

#[acmd_script(//GlideLanding
    agent = "pitb", 
    script = "effect_glidelanding", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pitb_glidelanding(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), false, false);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//GlideEnd
    agent = "pitb", 
    script = "effect_glideend", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pitb_glideend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), false, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pitb_feather"), Hash40::new("top"), 0, 10, -10, 0, 0, 0, 1, false);
    }
}   

#[acmd_script(//Fly
    agent = "pitb_bowarrow", 
    script = "effect_fly", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pitb_arrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pitb_pa_fly_arrow"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 2.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pitb_pa_fly_arrow2"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 2.2, true);
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
        effect_pitb_arrow
    );
}