use crate::imports::BuildImports::*;

//Attack11 
unsafe extern "C" fn effect_funky_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("donkey_attack_arc"), Hash40::new("donkey_attack_arc"), Hash40::new("top"), 3, 15, 2.5, -53, -69, -99, 1.0, true, *EF_FLIP_YZ, 1);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("donkey_attack_arc"), Hash40::new("donkey_attack_arc"), Hash40::new("top"), -3, 11, 3, 75, -77.7, -64, 1.0, true, *EF_FLIP_YZ, 1);
        LAST_EFFECT_SET_RATE(fighter, 1.8);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 22, 8, 0, 0, 0, 0, 2.3, 0, 0, 0, 0, 0, 360, false);
    }
}

//AttackDash
unsafe extern "C" fn effect_funky_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackS3Hi
unsafe extern "C" fn effect_funky_AttackS3Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_funky_sword6"), Hash40::new("tex_funky_sword2"), 10, Hash40::new("board"), 0.0, 0.0, -10.0, Hash40::new("board"), 0.0, 0.0, 31.0, true, Hash40::new("null"), Hash40::new("board"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
}

//AttackS3
unsafe extern "C" fn effect_funky_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_funky_sword6"), Hash40::new("tex_funky_sword2"), 10, Hash40::new("board"), 0.0, 0.0, -10.0, Hash40::new("board"), 0.0, 0.0, 31.0, true, Hash40::new("null"), Hash40::new("board"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
}

//AttackS3Lw
unsafe extern "C" fn effect_funky_AttackS3Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_funky_sword6"), Hash40::new("tex_funky_sword2"), 10, Hash40::new("board"), 0.0, 0.0, -10.0, Hash40::new("board"), 0.0, 0.0, 31.0, true, Hash40::new("null"), Hash40::new("board"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
}

//AttackHi3
unsafe extern "C" fn effect_funky_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_smash_slap"), Hash40::new("donkey_smash_slap"), Hash40::new("top"), 0, 16, 5, -20, 22, 77, 1.4, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.6);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_smash_slap"), Hash40::new("donkey_smash_slap"), Hash40::new("top"), 0, 16, -5, -150, -16, 80, 1.4, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.6);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 30, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 360, false);
    }
}

//AttackLw3
unsafe extern "C" fn effect_funky_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_attack_arc"), Hash40::new("donkey_attack_arc"), Hash40::new("top"), -2, 7, 8.5, -6, -55, 200, 1.5, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackS4
unsafe extern "C" fn effect_funky_AttackS4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 20, -18, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_funky_sword6"), Hash40::new("tex_funky_sword2"), 10, Hash40::new("board"), 0.0, 0.0, -10.0, Hash40::new("board"), 0.0, 0.0, 31.0, true, Hash40::new("null"), Hash40::new("board"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
}

//AttackHi4
unsafe extern "C" fn effect_funky_AttackHi4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 30, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.35, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 1.35, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13, 0, 0, 0, 0, 0.8, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 25, 0, 0, 0, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 40, 0, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13, 0, 0, 0, 0, 0.8, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 25, 0, 0, 0, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 40, 0, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13, 0, 0, 0, 0, 0.8, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 25, 0, 0, 0, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 40, 0, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13, 0, 0, 0, 0, 0.8, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 25, 0, 0, 0, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 40, 0, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
	}
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_spin_wind"), false, false);
    }
    frame(fighter.lua_state_agent, 56.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackLw4
unsafe extern "C" fn effect_funky_AttackLw4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 15, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("donkey_handslap"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("donkey_handslap"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackAirN
unsafe extern "C" fn effect_funky_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("pzenieffect_water_cutter"), Hash40::new("top"), 3, -1.5, -2, 0, 0, 0, 0.9, true);
            EFFECT_FOLLOW(fighter, Hash40::new("pzenieffect_water_smash"), Hash40::new("top"), 3, -2, -2, 10, 180, 0, 0.9, true);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("pzenieffect_water_cutter_r"), Hash40::new("top"), 3, -1.5, -2, 0, 0, 0, 0.9, true);
            EFFECT_FOLLOW(fighter, Hash40::new("pzenieffect_water_smash_r"), Hash40::new("top"), 3, -2, -2, 10, 0, 0, 0.9, true);
        }
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pzenieffect_mouth_water"), false, false);
    }
}

//AttackAirF
unsafe extern "C" fn effect_funky_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_attack_arc"), Hash40::new("donkey_attack_arc"), Hash40::new("top"), 1, 16, 4, -2, 30, 90, 1.7, true, *EF_FLIP_YZ);
    }
}

//AttackAirB
unsafe extern "C" fn effect_funky_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_funky_sword6"), Hash40::new("tex_funky_sword2"), 10, Hash40::new("board"), 0.0, 0.0, -10.0, Hash40::new("board"), 0.0, 0.0, 31.0, true, Hash40::new("null"), Hash40::new("board"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
}

//AttackAirHi
unsafe extern "C" fn effect_funky_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        if PostureModule::lr(fighter.module_accessor) < 0.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("pzeniexpression_takinobori"), Hash40::new("top"), -5, 7.5, 2, 0, 180, 0, 0.8, true);
            EFFECT_FOLLOW(fighter, Hash40::new("pzeniexpression_takinobori"), Hash40::new("top"), -5, 7.5, -6, 0, 180, 0, 0.8, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("pzeniexpression_takinobori"), Hash40::new("top"), 5, 7.5, 2, 0, 180, 0, 0.8, true);
            EFFECT_FOLLOW(fighter, Hash40::new("pzeniexpression_takinobori"), Hash40::new("top"), 5, 7.5, -6, 0, 180, 0, 0.8, true);
        }
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_smash_slap"), Hash40::new("donkey_smash_slap"), Hash40::new("top"), 2, 14, 8.5, 4, 0, 90, 1.5, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.75);
        LAST_EFFECT_SET_COLOR(fighter, 255.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pzeniexpression_takinobori"), false, false);
    }
}

//AttackAirLw
unsafe extern "C" fn effect_funky_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pzeniexpression_takinobori_splash"), Hash40::new("top"), 0, 2.0, -5, 0, 0, 0, 1.05, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pzeniexpression_takinobori"), Hash40::new("top"), 0, 8, -5, 5, 0, 0, 0.7, true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pzeniexpression_takinobori_splash"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("pzeniexpression_takinobori"), false, false);
    }
}

//ThrowF
unsafe extern "C" fn effect_funky_ThrowF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_attack_arc_d"), Hash40::new("donkey_attack_arc_d"), Hash40::new("top"), 1, 10, 3, 195, -79.5, 110, 1.6, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 7, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

//ThrowB
unsafe extern "C" fn effect_funky_ThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_attack_arc_d"), Hash40::new("donkey_attack_arc_d"), Hash40::new("top"), 0, 14, -1.5, 249, 74, 93, 1.8, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//ThrowHi
unsafe extern "C" fn effect_funky_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_attack_arc_d"), Hash40::new("donkey_attack_arc_d"), Hash40::new("top"), 0, 19, -1.5, -65, -35, 120, 1.6, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 10, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("donkey_attack_arc_d"), false, true);
    }
}

//ThrowLw
unsafe extern "C" fn effect_funky_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -3, 20, 14, 95, 0, 0, 1, true, *EF_FLIP_YZ);
        LAST_PARTICLE_SET_COLOR(fighter, 0.357, 0.56, 1);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 12, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//CliffAttack
unsafe extern "C" fn effect_funky_CliffAttack(fighter: &mut L2CAgentBase) {}

//SpecialN
unsafe extern "C" fn effect_funky_SpecialN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("diddy_popgun_break_a"), Hash40::new("boot"), 0, 0, 4, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

//SpecialNLoop
unsafe extern "C" fn effect_funky_SpecialNLoop(fighter: &mut L2CAgentBase) {
    loop {
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 20, 0, 12, 0, 0, 0, false);
            FLASH(fighter, 1, 125.0 / 255.0, 125.0 / 255.0, 0.259);
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            FLASH(fighter, 1, 60.0 / 255.0, 79.0 / 255.0, 0.133);
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            FLASH(fighter, 100.0 / 255.0, 3.0 / 255.0, 0, 0.384);
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 8.0);
    }
}

//SpecialNMax
unsafe extern "C" fn effect_funky_SpecialNMax(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("boot"), 0, 0, 0, 0, 0, 0, 1, 5, 5, 5, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 0.502, 0.549);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 5, 1, 60.0 / 255.0, 79.0 / 255.0, 0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FLIP(fighter, Hash40::new("diddy_popgun"), Hash40::new("diddy_popgun"), Hash40::new("boot"), 0, 0, -2.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        EFFECT_FLIP(fighter, Hash40::new("donkey_giantpunch_impact"), Hash40::new("donkey_giantpunch_impact"), Hash40::new("boot"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.0, 0.0);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
}

//SpecialAirN
unsafe extern "C" fn effect_funky_SpecialAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("diddy_popgun_break_a"), Hash40::new("boot"), 0, 0, 4, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

//SpecialAirNLoop
unsafe extern "C" fn effect_funky_SpecialAirNLoop(fighter: &mut L2CAgentBase) {
    loop {
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 20, 0, 12, 0, 0, 0, false);
            FLASH(fighter, 1, 125.0 / 255.0, 125.0 / 255.0, 0.259);
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            FLASH(fighter, 1, 60.0 / 255.0, 79.0 / 255.0, 0.133);
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            FLASH(fighter, 100.0 / 255.0, 3.0 / 255.0, 0, 0.384);
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 8.0);
    }
}

//SpecialAirNMax
unsafe extern "C" fn effect_funky_SpecialAirNMax(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("boot"), 0, 0, 0, 0, 0, 0, 1, 5, 5, 5, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 0.502, 0.549);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 5, 1, 60.0 / 255.0, 79.0 / 255.0, 0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FLIP(fighter, Hash40::new("diddy_popgun"), Hash40::new("diddy_popgun"), Hash40::new("boot"), 0, 0, -2.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
        EFFECT_FLIP(fighter, Hash40::new("donkey_giantpunch_impact"), Hash40::new("donkey_giantpunch_impact"), Hash40::new("boot"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.0, 0.0);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
}

//SpecialSStart
unsafe extern "C" fn effect_funky_SpecialSStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("murabito_entry_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

//SpecialSLanding
unsafe extern "C" fn effect_funky_SpecialSLanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 0.9, 3, 0, 1, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 0.9, 3, 0, 1, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 0.9, 3, 0, 1, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 0.9, 3, 0, 1, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 0.9, 3, 0, 1, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 0.9, 3, 0, 1, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 0.9, 3, 0, 1, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 0.9, 3, 0, 1, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 0.9, 3, 0, 1, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("donkey_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_box00_break"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_box00_break"), Hash40::new("top"), 0, 8, 0, 0, 0, 180, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialAirSDirection
unsafe extern "C" fn effect_funky_SpecialAirSDirection(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("krool_buckpack"), Hash40::new("plane"), 0, 10, -20, 0, 0, 0, 1.5, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

//SpecialAirSEnd
unsafe extern "C" fn effect_funky_SpecialAirSEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("donkey_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_box00_break"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_box00_break"), Hash40::new("top"), 0, 8, 0, 0, 0, 180, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialHiEnd
unsafe extern "C" fn effect_funky_SpecialHiEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_down_smoke"), Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_XY);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind"), Hash40::new("donkey_spin_wind"), Hash40::new("top"), 2, 13, 5, 19, 6, -33, 2, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
}

//SpecialAirHi
unsafe extern "C" fn effect_funky_SpecialAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_item_arrival"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

//SpecialAirHiLaunch
unsafe extern "C" fn effect_funky_SpecialAirHiLaunch(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("donkey_entry"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//SpecialHi_C2
unsafe extern "C" fn effect_funky_SpecialHi_C2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    for _ in 0..2 {
        if is_excute(fighter) {
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_XY);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind"), Hash40::new("donkey_spin_wind"), Hash40::new("rot"), 5.0, -2.0, 3.0, 0, 10.0, 0, 4.5, true, *EF_FLIP_YZ);
        }
        wait(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_XY);
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind"), Hash40::new("donkey_spin_wind"), Hash40::new("rot"), 5.0, -2.0, 3.0, 0, 10.0, 0, 3.5, true, *EF_FLIP_YZ);
        }
        wait(fighter.lua_state_agent, 7.0);
    }
    frame(fighter.lua_state_agent, 58.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_down_smoke"), Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_XY);
    }
}

//SpecialAirHi_C2
unsafe extern "C" fn effect_funky_SpecialAirHi_C2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind"), Hash40::new("donkey_spin_wind"), Hash40::new("rot"), 5.0, 7.0, 0.0, 0.0, 10.0, 0.0, 3.2, true, *EF_FLIP_YZ);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind"), Hash40::new("donkey_spin_wind"), Hash40::new("rot"), 5.0, -20.0, 0.0, 0, 10.0, 0, 3.2, true, *EF_FLIP_YZ);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind"), Hash40::new("donkey_spin_wind"), Hash40::new("rot"), 5.0, -5.0, 0.0, 0, 15.0, 0, 2.7, true, *EF_FLIP_YZ);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind"), Hash40::new("donkey_spin_wind"), Hash40::new("rot"), 5.0, 5.0, 0.0, 0, 15.0, 0, 3.2, true, *EF_FLIP_YZ);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind"), Hash40::new("donkey_spin_wind"), Hash40::new("rot"), 5.0, -10.0, 0.0, 0, 10.0, 0, 3.2, true, *EF_FLIP_YZ);
    }
    wait(fighter.lua_state_agent, 5.0);
}

//SpecialLwStart
unsafe extern "C" fn effect_funky_SpecialLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_naminori_f"), false, false);
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_wave"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_ripple"), false, false);
    }
}

//SpecialLwJump
unsafe extern "C" fn effect_funky_SpecialLwJump(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_naminori_f"), false, false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_wave"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_ripple"), false, false);
    }
}

//SpecialLwMusic
unsafe extern "C" fn effect_funky_SpecialLwMusic(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pacman_appeal_down"), Hash40::new("top"), 0, 20, 0, 0, 0, 0, 1.1, true);
    }
    frame(fighter.lua_state_agent, 70.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_naminori_f"), false, false);
    }
    frame(fighter.lua_state_agent, 80.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_wave"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_ripple"), false, false);
    }
}

//SpecialLwPose
unsafe extern "C" fn effect_funky_SpecialLwPose(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -3, 46, 5, 0, 0, 0, 2.1, false);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_naminori_f"), false, false);
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_wave"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_ripple"), false, false);
    }
}

//SpecialLwFlip
unsafe extern "C" fn effect_funky_SpecialLwFlip(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_naminori_f"), false, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_wave"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_ripple"), false, false);
    }
}

//SpecialAirLwStart
unsafe extern "C" fn effect_funky_SpecialAirLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_naminori_f"), false, false);
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_wave"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_ripple"), false, false);
    }
}

//SpecialAirLwJump
unsafe extern "C" fn effect_funky_SpecialAirLwJump(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_naminori_f"), false, false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_wave"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_ripple"), false, false);
    }
}

//SpecialAirLwMusic
unsafe extern "C" fn effect_funky_SpecialAirLwMusic(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pacman_appeal_down"), Hash40::new("top"), 0, 20, 0, 0, 0, 0, 1.1, true);
    }
    frame(fighter.lua_state_agent, 70.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_naminori_f"), false, false);
    }
    frame(fighter.lua_state_agent, 80.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_wave"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_ripple"), false, false);
    }
}

//SpecialAirLwPose
unsafe extern "C" fn effect_funky_SpecialAirLwPose(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -3, 46, 5, 0, 0, 0, 2.1, false);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_naminori_f"), false, false);
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_wave"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_ripple"), false, false);
    }
}

//SpecialAirLwFlip
unsafe extern "C" fn effect_funky_SpecialAirLwFlip(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_naminori_f"), false, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_wave"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("poke_mijumaeu_ripple"), false, false);
    }
}

//AppealHiR
unsafe extern "C" fn effect_funky_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//AppealHiL
unsafe extern "C" fn effect_funky_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//FinalStart
unsafe extern "C" fn effect_funky_FinalStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bg_black"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
    }
    frame(fighter.lua_state_agent, 58.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("stg_midgar_leviathan_wavesplash"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("stg_midgar_leviathan_wavesplash"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 82.0 / 255.0, 190.0 / 255.0, 245.0 / 255.0);
    }
    frame(fighter.lua_state_agent, 116.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("stg_midgar_leviathan_wavesplash"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_bg_black"), false, false);
    }
}

//FinalAirStart
unsafe extern "C" fn effect_funky_FinalAirStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bg_black"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
    }
    frame(fighter.lua_state_agent, 58.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("stg_midgar_leviathan_wavesplash"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("stg_midgar_leviathan_wavesplash"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 82.0 / 255.0, 190.0 / 255.0, 245.0 / 255.0);
    }
    frame(fighter.lua_state_agent, 116.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("stg_midgar_leviathan_wavesplash"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_bg_black"), false, false);
    }
}

pub fn install() {
    Agent::new("donkey")
    .effect_acmd("effect_attack11_funky", effect_funky_Attack11, Low)
    .effect_acmd("effect_attackdash_funky", effect_funky_AttackDash, Low)
    .effect_acmd("effect_attacks3hi_funky", effect_funky_AttackS3Hi, Low)
    .effect_acmd("effect_attacks3_funky", effect_funky_AttackS3, Low)
    .effect_acmd("effect_attacks3lw_funky", effect_funky_AttackS3Lw, Low)
    .effect_acmd("effect_attackhi3_funky", effect_funky_AttackHi3, Low)
    .effect_acmd("effect_attacklw3_funky", effect_funky_AttackLw3, Low)
    .effect_acmd("effect_attacks4_funky", effect_funky_AttackS4, Low)
    .effect_acmd("effect_attackhi4_funky", effect_funky_AttackHi4, Low)
    .effect_acmd("effect_attacklw4_funky", effect_funky_AttackLw4, Low)
    .effect_acmd("effect_attackairn_funky", effect_funky_AttackAirN, Low)
    .effect_acmd("effect_attackairf_funky", effect_funky_AttackAirF, Low)    
    .effect_acmd("effect_attackairb_funky", effect_funky_AttackAirB, Low)
    .effect_acmd("effect_attackairhi_funky", effect_funky_AttackAirHi, Low)
    .effect_acmd("effect_attackairlw_funky", effect_funky_AttackAirLw, Low)
    .effect_acmd("effect_throwf_funky", effect_funky_ThrowF, Low)
    .effect_acmd("effect_throwb_funky", effect_funky_ThrowB, Low)
    .effect_acmd("effect_throwhi_funky", effect_funky_ThrowHi, Low)
    .effect_acmd("effect_throwlw_funky", effect_funky_ThrowLw, Low)
    .effect_acmd("effect_cliffattack_funky", effect_funky_CliffAttack, Low)
    .effect_acmd("effect_specialn_funky", effect_funky_SpecialN, Low)
    .effect_acmd("effect_specialnloop_funky", effect_funky_SpecialNLoop, Low)
    .effect_acmd("effect_specialnmax_funky", effect_funky_SpecialNMax, Low)
    .effect_acmd("effect_specialairn_funky", effect_funky_SpecialAirN, Low)
    .effect_acmd("effect_specialairnloop_funky", effect_funky_SpecialAirNLoop, Low)
    .effect_acmd("effect_specialairnmax_funky", effect_funky_SpecialAirNMax, Low)
    .effect_acmd("effect_specialsstart_funky", effect_funky_SpecialSStart, Low)
    .effect_acmd("effect_specialslanding_funky", effect_funky_SpecialSLanding, Low)
    .effect_acmd("effect_specialairsdirection_funky", effect_funky_SpecialAirSDirection, Low)
    .effect_acmd("effect_specialairsend_funky", effect_funky_SpecialAirSEnd, Low)
    .effect_acmd("effect_specialairhi_funky", effect_funky_SpecialAirHi, Low)
    .effect_acmd("effect_specialairhilaunch_funky", effect_funky_SpecialAirHiLaunch, Low)
    .effect_acmd("effect_specialhic2_funky", effect_funky_SpecialHi_C2, Low)
    .effect_acmd("effect_specialairhic2_funky", effect_funky_SpecialAirHi_C2, Low)
    .effect_acmd("effect_speciallwstart_funky", effect_funky_SpecialLwStart, Low)
    .effect_acmd("effect_speciallwjump_funky", effect_funky_SpecialLwJump, Low)
    .effect_acmd("effect_speciallwmusic_funky", effect_funky_SpecialLwMusic, Low)
    .effect_acmd("effect_speciallwflip_funky", effect_funky_SpecialLwFlip, Low)
    .effect_acmd("effect_speciallwpose_funky", effect_funky_SpecialLwPose, Low)
    .effect_acmd("effect_specialairlwstart_funky", effect_funky_SpecialAirLwStart, Low)
    .effect_acmd("effect_specialairlwjump_funky", effect_funky_SpecialAirLwJump, Low)
    .effect_acmd("effect_specialairlwmusic_funky", effect_funky_SpecialAirLwMusic, Low)
    .effect_acmd("effect_specialairlwflip_funky", effect_funky_SpecialAirLwFlip, Low)
    .effect_acmd("effect_specialairlwpose_funky", effect_funky_SpecialAirLwPose, Low)
    .effect_acmd("effect_appealhir_funky", effect_funky_AppealHiR, Low)
    .effect_acmd("effect_appealhil_funky", effect_funky_AppealHiL, Low)
    .effect_acmd("effect_finalstart_funky", effect_funky_FinalStart, Low)
    .effect_acmd("effect_finalairstart_funky", effect_funky_FinalAirStart, Low)
    .install();
}