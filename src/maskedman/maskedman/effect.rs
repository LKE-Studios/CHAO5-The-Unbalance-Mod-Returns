use crate::imports::BuildImports::*;

//EntryL
unsafe extern "C" fn effect_maskedman_EntryL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 8, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

//EntryR
unsafe extern "C" fn effect_maskedman_EntryR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 8, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

//Attack11
unsafe extern "C" fn effect_maskedman_Attack11(fighter: &mut L2CAgentBase) {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        if color == 121 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword3"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 122 { 
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword4"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 124 { 
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword5"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 126 || color == 127 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword6"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword1"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
}

//AttackDash
unsafe extern "C" fn effect_maskedman_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 6, -3, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 0.0, 1.0, 0.0, 180, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 6, -3, 0, 0, 0, 0.7, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 6, -3, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 0.0, 1.0, 0.0, 180, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 6, -3, 0, 0, 0, 0.7, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 6, -3, 0, 0, 0, 0.7, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 6, -3, 0, 0, 0, 0.7, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_spark"), false, false);
    }
}

//AttackS3Hi
unsafe extern "C" fn effect_maskedman_AttackS3Hi(fighter: &mut L2CAgentBase) {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        if color == 121 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword3"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 122 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword4"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 124 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword5"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 126 || color == 127 { //purple
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword6"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword1"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
}

//AttackS3
unsafe extern "C" fn effect_maskedman_AttackS3(fighter: &mut L2CAgentBase) {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        if color == 121 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword3"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 122 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword4"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 124 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword5"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 126 || color == 127 { //purple
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword6"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword1"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
}

//AttackS3Lw
unsafe extern "C" fn effect_maskedman_AttackS3Lw(fighter: &mut L2CAgentBase) {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        if color == 121 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword3"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 122 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword4"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 124 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword5"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 126 || color == 127 { //purple
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword6"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword1"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
}

//AttackHi3
unsafe extern "C" fn effect_maskedman_AttackHi3(fighter: &mut L2CAgentBase) {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }    
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        if color == 121 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword3"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 122 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword4"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 124 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword5"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 126 || color == 127 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword6"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword1"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
}

//AttackLw3
unsafe extern "C" fn effect_maskedman_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -2.5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("havel"), 0, -2.5, 0, -90, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, true);
        LAST_PARTICLE_SET_COLOR(fighter, 1.2, 0.8, 0.2);
        LAST_EFFECT_SET_RATE(fighter, 1.7);
    }
}

//AttackS4
unsafe extern "C" fn effect_maskedman_AttackS4(fighter: &mut L2CAgentBase) {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 6, 7, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        if color == 121 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword3"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 122 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword4"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 124 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword5"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 126 || color == 127 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword6"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword1"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 7, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 7, 0, 0, 0, 0, 0, 0.725, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackHi4Charge
unsafe extern "C" fn effect_maskedman_AttackHi4Charge(fighter: &mut L2CAgentBase) {
    loop {
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 1, 8, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0.0, 9, 0.0, 0, 0, 0, 1, 2, 5, 2, 0, 0, 0, true);
    }
}

//AttackHi4
unsafe extern "C" fn effect_maskedman_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0.0, 7.0, 7.0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 15, 0, -90, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_PARTICLE_SET_COLOR(fighter, 1.2, 0.8, 0.2);
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 22.0, 0.5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        EFFECT(fighter, Hash40::new("lucas_psi_atk_hi"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 10, 0, -90, 0, 0, 1.05, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 21, 0, -90, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 30, 0, -90, 0, 0, 0.6, true);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
    }
}

//AttackLw4Charge
unsafe extern "C" fn effect_maskedman_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    loop {
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 1, 8, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0.0, 9, 0.0, 0, 0, 0, 1, 2, 5, 2, 0, 0, 0, true);
    }
}

//AttackLw4
unsafe extern "C" fn effect_maskedman_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_psi_hold"), Hash40::new("lucas_psi_hold"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0, 0, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("lucas_psi_atk_lw"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0,  true);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.7, 2, 2, 2, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.7, 2, 2, 2, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
        EFFECT(fighter, Hash40::new("lucas_psi_atk_lw"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 52.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_hold"), false, false);
    }
}

//AttackAirN
unsafe extern "C" fn effect_maskedman_AttackAirN(fighter: &mut L2CAgentBase) {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        if color == 121 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword3"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 122 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword4"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 124 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword5"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 126 || color == 127 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword6"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword1"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
}

//AttackAirF
unsafe extern "C" fn effect_maskedman_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("throw"), 0.0, 0, 0.0, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, true);
    }
}

//AttackAirB
unsafe extern "C" fn effect_maskedman_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.8, true);
        EFFECT(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("throw"), 0, 0, 0, 0, 90, 0, 0.6,0,0,0,0, 0,0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("throw"), 0, 0, 0, 0, 90, 0, 0.6, 0,0,0,0, 0,0,true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("throw"), 0, -1, 0, 0, 90, 0, 0.75, 0,0,0,0, 0,0,true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_hold"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
    }
}

//AttackAirHi
unsafe extern "C" fn effect_maskedman_AttackAirHi(fighter: &mut L2CAgentBase) {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        if color == 121 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword3"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 122 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword4"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 124 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword5"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else if color == 126 || color == 127 {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword6"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
        else {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_maskedman_sword1"), Hash40::new("tex_maskedman_sword2"), 5, Hash40::new("batl"), -1.0, 0.0, 1.0, Hash40::new("batl"), -1.0, 12.0, 3.0, true, Hash40::new("null"), Hash40::new("batl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
}

//AttackAirLw
unsafe extern "C" fn effect_maskedman_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("throw"), 0.0, 0, 0.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

//CatchAttack
unsafe extern "C" fn effect_maskedman_CatchAttack(fighter: &mut L2CAgentBase) {}

//ThrowF
unsafe extern "C" fn effect_maskedman_ThrowF(fighter: &mut L2CAgentBase) {    
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("throw"), 0, 3, 0, 0, 0, 0, 0.6, true);
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("throw"), 0, 3, 0, 0, 0, 0, 0.6, true);
    }
    wait(fighter.lua_state_agent, 10.0);
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk_lw"), Hash40::new("throw"), 0, 3, -4, 0, 0, 0, 0.75, true);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 17, 15, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

//ThrowB
unsafe extern "C" fn effect_maskedman_ThrowB(fighter: &mut L2CAgentBase) {  
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("armr"), 4, 0, 0, 0, 90, 0, 0.6, true);
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("armr"), 4, 0, 0, 0, 90, 0, 0.6, true);
    }
    wait(fighter.lua_state_agent, 8.0);
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk_lw"), Hash40::new("armr"), 4, 0, 0, 0, 90, 0, 1.0, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

//ThrowHi
unsafe extern "C" fn effect_maskedman_ThrowHi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("throw"), 0, 2, 4, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("throw"), 0, 2, 4, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk_lw"), Hash40::new("throw"), 0, 2, 4, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}      

//ThrowLw
unsafe extern "C" fn effect_maskedman_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.25);
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1);
    }
}

//CliffAttack
unsafe extern "C" fn effect_maskedman_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8.5, -10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 4, 1, -5, 25, -170, 0.85, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.8);
    }
}

//SlipAttack
unsafe extern "C" fn effect_maskedman_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 4, 6, 0, 190, -20, 0.9, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 5, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 1, 5, -8.7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.7);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 5, -8, 0, 0, 30, 1, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 1, 5.2, 9.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.7);
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3.5, 0, 0.5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//DownAttackU
unsafe extern "C" fn effect_maskedman_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 5, 0, 0, 190, 10, 1.1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 5, 0, 0, -40, 10, 1.1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

//DownAttackD
unsafe extern "C" fn effect_maskedman_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 4, 0, 0, 25, 10, 1.1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 4, 0, 0, 160, 10, 1.1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

//SpecialNDash
unsafe extern "C" fn effect_maskedman_SpecialNDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_assist_steam_max"), false, false);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_assist_steam_max"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 2.0, true);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        COL_NORMAL(fighter);
        FLASH(fighter, 2.5, 2.5, 0.0, 0.25);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_assist_steam_max"), false, false);
    }
}

//SpecialAirNDash
unsafe extern "C" fn effect_maskedman_SpecialAirNDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_assist_steam_max"), false, false);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_assist_steam_max"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 2.0, true);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        COL_NORMAL(fighter);
        FLASH(fighter, 2.5, 2.5, 0.0, 0.25);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ColorBlendModule::off_flash(fighter.module_accessor, false);
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_assist_steam_max"), false, false);
    }
}

//SpecialS
unsafe extern "C" fn effect_maskedman_SpecialS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("top"), 0, 5, 7, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}       

//SpecialAirS
unsafe extern "C" fn effect_maskedman_SpecialAirS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("top"), 0, 5, 7, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialHiStart
unsafe extern "C" fn effect_maskedman_SpecialHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialAirHiStart
unsafe extern "C" fn effect_maskedman_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        EFFECT(fighter, Hash40::new("sys_bomb_b"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bomb_b"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialHiHold
unsafe extern "C" fn effect_maskedman_SpecialHiHold(fighter: &mut L2CAgentBase) {
    loop {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_misfire"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 2.0);
    }
}

//SpecialLwStart
unsafe extern "C" fn effect_maskedman_SpecialLwStart(fighter: &mut L2CAgentBase) {
    let team_color = FighterUtil::get_team_color(fighter.module_accessor);
    let color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_direction"), Hash40::new("throw"), 0, -5, 0, 0, 90, 0, 1, true);
        EffectModule::set_rgb_partial_last(fighter.module_accessor, color.value[0], color.value[1], color.value[2]);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_direction"), Hash40::new("throw"), 0, 5, 0, 180, 90, 0, 1, true);
        EffectModule::set_rgb_partial_last(fighter.module_accessor, color.value[0], color.value[1], color.value[2]);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_direction"), Hash40::new("throw"), 0, 0, -5, 90, 0, 90, 1, true);
        EffectModule::set_rgb_partial_last(fighter.module_accessor, color.value[0], color.value[1], color.value[2]);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_direction"), Hash40::new("throw"), 0, 0, 5, -90, 0, 90, 1, true);
        EffectModule::set_rgb_partial_last(fighter.module_accessor, color.value[0], color.value[1], color.value[2]);
    }
}

//SpecialLwEnd
unsafe extern "C" fn effect_maskedman_SpecialLwEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psimagnet_end"), Hash40::new("trans"), 0, 6.5, 12, 0, 0, 0, 1, true);
        FLASH(fighter, 0.5, 1, 1, 0.4);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 10, 0, 1, 1, 0.1);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//SpecialAirLwStart
unsafe extern "C" fn effect_maskedman_SpecialAirLwStart(fighter: &mut L2CAgentBase) {
    let team_color = FighterUtil::get_team_color(fighter.module_accessor);
    let color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_direction"), Hash40::new("throw"), 0, -5, 0, 0, 90, 0, 1, true);
        EffectModule::set_rgb_partial_last(fighter.module_accessor, color.value[0], color.value[1], color.value[2]);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_direction"), Hash40::new("throw"), 0, 5, 0, 180, 90, 0, 1, true);
        EffectModule::set_rgb_partial_last(fighter.module_accessor, color.value[0], color.value[1], color.value[2]);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_direction"), Hash40::new("throw"), 0, 0, -5, 90, 0, 90, 1, true);
        EffectModule::set_rgb_partial_last(fighter.module_accessor, color.value[0], color.value[1], color.value[2]);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_direction"), Hash40::new("throw"), 0, 0, 5, -90, 0, 90, 1, true);
        EffectModule::set_rgb_partial_last(fighter.module_accessor, color.value[0], color.value[1], color.value[2]);
    }
}

//SpecialAirLwEnd
unsafe extern "C" fn effect_maskedman_SpecialAirLwEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psimagnet_end"), Hash40::new("trans"), 0, 6.5, 12, 0, 0, 0, 1, true);
        FLASH(fighter, 0.5, 1, 1, 0.4);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 10, 0, 1, 1, 0.1);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//AppealHiR
unsafe extern "C" fn effect_maskedman_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("throw"), 0.0, 0, 0.0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    frame(fighter.lua_state_agent, 88.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

//AppealHiL
unsafe extern "C" fn effect_maskedman_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("throw"), 0.0, 0, 0.0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    frame(fighter.lua_state_agent, 88.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

//AppealLwR
unsafe extern "C" fn effect_maskedman_AppealLwR(fighter: &mut L2CAgentBase) {

}

//AppealLwL
unsafe extern "C" fn effect_maskedman_AppealLwL(fighter: &mut L2CAgentBase) {

}

//Final
unsafe extern "C" fn effect_maskedman_Final(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bg_black"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_final_psi"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.1, false);
    }
    frame(fighter.lua_state_agent, 90.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_bg_black"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_final_psi"), false, false);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_assist_deathflash"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 42.0, false);
    }
}

pub fn install() {
    Agent::new("lucas")
    .effect_acmd("effect_entryl_maskedman", effect_maskedman_EntryL, Low)
    .effect_acmd("effect_entryr_maskedman", effect_maskedman_EntryR, Low)
    .effect_acmd("effect_attack11_maskedman", effect_maskedman_Attack11, Low)
    .effect_acmd("effect_attackdash_maskedman", effect_maskedman_AttackDash, Low)
    .effect_acmd("effect_attacks3hi_maskedman", effect_maskedman_AttackS3Hi, Low)
    .effect_acmd("effect_attacks3_maskedman", effect_maskedman_AttackS3, Low)
    .effect_acmd("effect_attacks3lw_maskedman", effect_maskedman_AttackS3Lw, Low)
    .effect_acmd("effect_attackhi3_maskedman", effect_maskedman_AttackHi3, Low)
    .effect_acmd("effect_attacklw3_maskedman", effect_maskedman_AttackLw3, Low)
    .effect_acmd("effect_attacks4_maskedman", effect_maskedman_AttackS4, Low)
    .effect_acmd("effect_attackhi4charge_maskedman", effect_maskedman_AttackHi4Charge, Low)
    .effect_acmd("effect_attackhi4_maskedman", effect_maskedman_AttackHi4, Low)
    .effect_acmd("effect_attacklw4charge_maskedman", effect_maskedman_AttackLw4Charge, Low)
    .effect_acmd("effect_attacklw4_maskedman", effect_maskedman_AttackLw4, Low)
    .effect_acmd("effect_attackairn_maskedman", effect_maskedman_AttackAirN, Low)
    .effect_acmd("effect_attackairf_maskedman", effect_maskedman_AttackAirF, Low)    
    .effect_acmd("effect_attackairb_maskedman", effect_maskedman_AttackAirB, Low)
    .effect_acmd("effect_attackairhi_maskedman", effect_maskedman_AttackAirHi, Low)
    .effect_acmd("effect_attackairlw_maskedman", effect_maskedman_AttackAirLw, Low)
    .effect_acmd("effect_catchattack_maskedman", effect_maskedman_CatchAttack, Low)
    .effect_acmd("effect_throwf_maskedman", effect_maskedman_ThrowF, Low)
    .effect_acmd("effect_throwb_maskedman", effect_maskedman_ThrowB, Low)
    .effect_acmd("effect_throwhi_maskedman", effect_maskedman_ThrowHi, Low)
    .effect_acmd("effect_throwlw_maskedman", effect_maskedman_ThrowLw, Low)
    .effect_acmd("effect_downattackd_maskedman", effect_maskedman_DownAttackD, Low)
    .effect_acmd("effect_downattacku_maskedman", effect_maskedman_DownAttackU, Low)
    .effect_acmd("effect_cliffattack_maskedman", effect_maskedman_CliffAttack, Low)
    .effect_acmd("effect_slipattack_maskedman", effect_maskedman_SlipAttack, Low)
    .effect_acmd("effect_specialndash_maskedman", effect_maskedman_SpecialNDash, Low)
    .effect_acmd("effect_specialairndash_maskedman", effect_maskedman_SpecialAirNDash, Low)
    .effect_acmd("effect_specials_maskedman", effect_maskedman_SpecialS, Low)
    .effect_acmd("effect_specialairs_maskedman", effect_maskedman_SpecialAirS, Low)
    .effect_acmd("effect_specialhistart_maskedman", effect_maskedman_SpecialHiStart, Low)
    .effect_acmd("effect_specialhihold_maskedman", effect_maskedman_SpecialHiHold, Low)
    .effect_acmd("effect_specialairhistart_maskedman", effect_maskedman_SpecialAirHiStart, Low)
    .effect_acmd("effect_speciallwstart_maskedman", effect_maskedman_SpecialLwStart, Low)
    .effect_acmd("effect_speciallwend_maskedman", effect_maskedman_SpecialLwEnd, Low)
    .effect_acmd("effect_specialairlwstart_maskedman", effect_maskedman_SpecialAirLwStart, Low)
    .effect_acmd("effect_specialairlwend_maskedman", effect_maskedman_SpecialAirLwEnd, Low)
    .effect_acmd("effect_appealhir_maskedman", effect_maskedman_AppealHiR, Low)
    .effect_acmd("effect_appealhil_maskedman", effect_maskedman_AppealHiL, Low)
    .effect_acmd("effect_appeallwr_maskedman", effect_maskedman_AppealLwR, Low)
    .effect_acmd("effect_appeallwl_maskedman", effect_maskedman_AppealLwL, Low)
    .effect_acmd("effect_final_maskedman", effect_maskedman_Final, Low)
    .install();
}
