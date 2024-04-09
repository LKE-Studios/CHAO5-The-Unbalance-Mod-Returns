use crate::imports::BuildImports::*;

//JumpAerialF4
unsafe extern "C" fn effect_pit_JumpAerialF4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pit_feather"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, false);
    }
}

//JumpAerialF5
unsafe extern "C" fn effect_pit_JumpAerialF5(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pit_feather"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, false);
    }
}

//JumpAerialF6
unsafe extern "C" fn effect_pit_JumpAerialF6(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pit_feather"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, false);
    }
}

//JumpAerialF7
unsafe extern "C" fn effect_pit_JumpAerialF7(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pit_feather"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, false);
    }
}

//GlideStart
unsafe extern "C" fn effect_pit_GlideStart(fighter: &mut L2CAgentBase) {
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

//GlideWing
unsafe extern "C" fn effect_pit_GlideWing(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

//GlideAttack
unsafe extern "C" fn effect_pit_GlideAttack(fighter: &mut L2CAgentBase) {
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

//GlideLanding
unsafe extern "C" fn effect_pit_GlideLanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), false, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

//GlideEnd
unsafe extern "C" fn effect_pit_GlideEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), false, false);
        EFFECT_FOLLOW(fighter, Hash40::new("pit_feather"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, false);
    }
}   

//SpecialHiStart 
unsafe extern "C" fn effect_pit_SpecialHiStart(fighter: &mut L2CAgentBase) {
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

//SpecialHiFly
unsafe extern "C" fn effect_pit_SpecialHiFly(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("pit_fly_miracle_start"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("pit_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, 0, 1, 0, 0, 0, 1, false);
            LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 0.1, /*B*/ 2.2);
            EFFECT_FOLLOW(fighter, Hash40::new("pit_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, 0, -1, 0, 0, 0, 1, false);
            LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 0.1, /*B*/ 2.2);
            EFFECT_FOLLOW(fighter, Hash40::new("pit_feather"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, false);
        }
        for _ in 0..2 {
            if is_excute(fighter) {
                if WorkModule::is_flag(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLAG_BURN) {
                    EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
                    EFFECT_FOLLOW(fighter, Hash40::new("pit_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, 0, 1, 0, 0, 0, 1, false);
                    LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.2, /*G*/ 0.1, /*B*/ 0.0);
                    EFFECT_FOLLOW(fighter, Hash40::new("pit_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, 0, -1, 0, 0, 0, 1, false);
                    LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.2, /*G*/ 0.1, /*B*/ 0.0);
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("s_wingl1"), 0, -1.0, 1.0, 0, 0, 0, 0.7, false);
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("s_wingr1"), 0, -1.0, -1.0, 0, 0, 0, 0.7, false);
                }
            }
            wait(fighter.lua_state_agent, 7.0);
        }
        frame(fighter.lua_state_agent, 40.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("pit_fly_miracle"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, false);
        }
        wait_loop_clear(fighter);
    }
}

//SpecialHiFlyTurn
unsafe extern "C" fn effect_pit_SpecialHiFlyTurn(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pit_feather"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, false);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pit_fly_miracle"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

//SpecialHiEnd
unsafe extern "C" fn effect_pit_SpecialHiEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialAirHiStart 
unsafe extern "C" fn effect_pit_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
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
    Agent::new("pit")
    .effect_acmd("effect_jumpaerialf4", effect_pit_JumpAerialF4)
    .effect_acmd("effect_jumpaerialf5", effect_pit_JumpAerialF5)
    .effect_acmd("effect_jumpaerialf6", effect_pit_JumpAerialF6)
    .effect_acmd("effect_jumpaerialf4", effect_pit_JumpAerialF7)
    .effect_acmd("effect_glidestart", effect_pit_GlideStart)
    .effect_acmd("effect_glidewing", effect_pit_GlideWing)
    .effect_acmd("effect_glideattack", effect_pit_GlideAttack)
    .effect_acmd("effect_glidelanding", effect_pit_GlideLanding)
    .effect_acmd("effect_specialhistart", effect_pit_SpecialHiStart)
    .effect_acmd("effect_specialhifly", effect_pit_SpecialHiFly)
    .effect_acmd("effect_specialhiflyturn", effect_pit_SpecialHiFlyTurn)
    .effect_acmd("effect_specialhiend", effect_pit_SpecialHiEnd)
    .effect_acmd("effect_specialairhistart", effect_pit_SpecialAirHiStart)
    .install();
}