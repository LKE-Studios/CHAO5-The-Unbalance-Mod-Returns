use crate::imports::BuildImports::*;

//JumpAerialF1
unsafe extern "C" fn effect_krystal_JumpAerialF1(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//GlideStart
unsafe extern "C" fn effect_krystal_GlideStart(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn effect_krystal_GlideWing(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

//GlideAttack
unsafe extern "C" fn effect_krystal_GlideAttack(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn effect_krystal_GlideLanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), false, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

//GlideEnd
unsafe extern "C" fn effect_krystal_GlideEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_start"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), false, false);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_feather"), Hash40::new("top"), 0, 10, -10, 0, 0, 0, 1, false);
    }
}   

//Attack11 
unsafe extern "C" fn effect_krystal_Attack11(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 2.0);
	if is_excute(fighter) {
		FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0.0, 0.0, -0.2, Hash40::new("swordr1"), 0.0, 15.2, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 1.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.5, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
	}
	frame(fighter.lua_state_agent, 6.0);
	if is_excute(fighter) {
		AFTER_IMAGE_OFF(fighter, 2);
	}
}

//Attack12
unsafe extern "C" fn effect_krystal_Attack12(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 4.0);
	if is_excute(fighter) {
		FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0.0, 0.0, -0.2, Hash40::new("swordr1"), 0.0, 15.2, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 1.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.5, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
	}
	frame(fighter.lua_state_agent, 19.0);
	if is_excute(fighter) {
		AFTER_IMAGE_OFF(fighter, 2);
	}
}

//Attack13
unsafe extern "C" fn effect_krystal_Attack13(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 3.0);
	if is_excute(fighter) {
		FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
	frame(fighter.lua_state_agent, 4.0);
	if is_excute(fighter) {
		FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0.0, 0.0, -0.2, Hash40::new("swordr1"), 0.0, 15.2, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 1.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.5, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
	}
	frame(fighter.lua_state_agent, 10.0);
	if is_excute(fighter) {
		AFTER_IMAGE_OFF(fighter, 2);
	}
}

//AttackDash
unsafe extern "C" fn effect_krystal_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0.0, 0.0, -0.2, Hash40::new("swordr1"), 0.0, 15.2, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 1.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.5, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
}

//AttackS3
unsafe extern "C" fn effect_krystal_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 17.5, 0, -90.5, -98, 0.0, 1.1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
	frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackHi3
unsafe extern "C" fn effect_krystal_AttackHi3(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if is_excute(fighter) {
		FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0.0, 0.0, -0.2, Hash40::new("swordr1"), 0.0, -13.2, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 1.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.5, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
	}
	frame(fighter.lua_state_agent, 5.0);
	if is_excute(fighter) {
		AFTER_IMAGE_OFF(fighter, 2);
	}
	frame(fighter.lua_state_agent, 12.0);
	if is_excute(fighter) {
		FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.35, 0, 0, 0, 0, 0, 0, false);
    	EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), -1, 14, 0, 0, 15, 70, 1.05, true, 0.15);
	}
	frame(fighter.lua_state_agent, 16.0);
	if is_excute(fighter) {
		EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc"), true, true);
		EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -1, 25, 7, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 360, true);
	}
	frame(fighter.lua_state_agent, 19.0);
	if is_excute(fighter) {
		EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line"), true, true);
	}
	frame(fighter.lua_state_agent, 33.0);
	if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
	}
}

//AttackLw3
unsafe extern "C" fn effect_krystal_AttackLw3(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0.0, 0.0, -0.2, Hash40::new("swordr1"), 0.0, 15.2, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 1.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.5, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0.0, 0.0, -0.2, Hash40::new("swordr1"), 0.0, 15.2, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("handr"), 5.0, 0.0, 5.0, 0.0, 90.0, 0.0, 1.5, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
	frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.26, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
}

//AttackS4
unsafe extern "C" fn effect_krystal_AttackS4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, 6, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
	frame(fighter.lua_state_agent, 10.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("klystal_rod_flare"), Hash40::new("haver"), 0, 11.4, -0.4, 0, 0, 0, 0.225, true);
	}
	frame(fighter.lua_state_agent, 12.0);
	if is_excute(fighter) {
		FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    	EFFECT_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 1, 9.0, 11.3, 0, 0.0, 0.0, 1.0, 0, 0, 0, 0, 0, 0, true, 1);
		EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9, 28, 0, 0, 0, 1.88, true);
		LAST_EFFECT_SET_RATE(fighter, 0.8);
	}
}

//AttackHi4
unsafe extern "C" fn effect_krystal_AttackHi4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 18, 6, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
	frame(fighter.lua_state_agent, 6.0);
	if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 3, 18, 5, -3, 23.6, 96.3, 1.1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackLw4
unsafe extern "C" fn effect_krystal_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, -6, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_KRYSTAL_INSTANCE_WORK_ID_FLAG_ATTACK_LW4_SUCCESS) { 
            QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
            EFFECT(fighter, Hash40::new("pitb_gouwan_dash_ring"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("sys_soil_landing"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

//AttackAirN
unsafe extern "C" fn effect_krystal_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7.2, -1.8, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9.4, 11, 0, 0, 0, 1, true, 0.7);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_speedline"), true, true);
    }
}

//AttackAirF 
unsafe extern "C" fn effect_krystal_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 12, 3, 14, -22, -31, 1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 10, 3, -20, -29, 10, 0.8, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
	frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 13, 11.5, 0, 0, 0, 1.1, true, 0.8);
    }
}

//AttackAirB
unsafe extern "C" fn effect_krystal_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 2, 8, -2.5, 23, 120, 200, 0.9, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 10.5, -12.5, 0, 0, 0, 1.35, 0, 0, 0, 0, 0, 360, true);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
}

//AttackAirHi
unsafe extern "C" fn effect_krystal_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0.0, 0.0, -0.2, Hash40::new("swordr1"), 0.0, 15.2, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 1.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.5, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 13.5, 0, -90, 0, 0, 0.7, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.75, 0.9, 1);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 25, 0, 0, 0, -8, 0.2, true, 1);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 22, 0, 0, 90, 8, 0.4, true, 0.8);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 25, 0, 0, 0, 0, 1.2, true, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_rod_flare"), Hash40::new("top"), 0, 25, 0, 0, 0, 0, 0.15, true);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 19, 0, 0, 180, -8, 0.5, true, 0.55);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 15.5, 0, 0, 270, 8, 0.6, true, 0.4);
        AFTER_IMAGE_OFF(fighter, 2);
        LAST_EFFECT_SET_RATE(fighter, 1.15);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_spin_wind"), false, true);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
}

//AttackAirLw
unsafe extern "C" fn effect_krystal_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);                     
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 1, -2.5, -1.5, 90, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true, 1);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, -18, 0, 0, 0, 0, 1.65, true);
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_rod_flare"), Hash40::new("haver"), 0, 11.5, -0.4, -4.7, 0, 0, 0.15, true);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
}

//ThrowF
unsafe extern "C" fn effect_krystal_ThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, -7, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -2, 6, 10.5, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, 0.75);
        EFFECT(fighter, Hash40::new("sys_hit_normal"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 360, true);
    }
}

//ThrowB
unsafe extern "C" fn effect_krystal_ThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 2, 9, -5, 0, 0, 0, 1.5, true, *EF_FLIP_YZ, 0.6);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 2, 9.5, -3.5, 0, 0, 0, 1.5, true, *EF_FLIP_YZ, 0.6);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_hit_normal"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 360, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 2, 11.8, -1.1, 0, 0, 0, 1.5, true, *EF_FLIP_YZ, 0.6);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 2, 13.8, 2.5, 0, 0, 0, 1.5, true, *EF_FLIP_YZ, 0.6);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 2, 16.5, 4.0, 0, 0, 0, 1.5, true, *EF_FLIP_YZ, 0.6);
    }
}

//ThrowHi
unsafe extern "C" fn effect_krystal_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("haver"), 0, 12.6, 0, 0, 0, 0, 0.25, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("haver"), 0, 12.6, 0, 0, 0, 0, 0.25, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("haver"), 0, 12, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_pa_fly_arrow"), Hash40::new("throw"), 0, 0, 0, 90, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_pa_fly_arrow2"), Hash40::new("throw"), 0, 0, 0, 90, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 5);
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_pa_fly_arrow"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_pa_fly_arrow2"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_pa_max_flash"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_pa_max"), true, true);
    }
}      

//ThrowLw
unsafe extern "C" fn effect_krystal_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 1, 0, 4.2, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_gouwan_dash_end"), Hash40::new("haver"), 0, 14.4, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_flash"), Hash40::new("haver"), 0, 18, 3, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_gouwan_dash_end"), Hash40::new("haver"), 0, 14.4, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_gouwan_dash_end"), Hash40::new("haver"), 0, 14.4, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_gouwan_dash_flash"), true, true);
        AFTER_IMAGE_OFF(fighter, 4);
    }
    frame(fighter.lua_state_agent, 52.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_gouwan_dash_end"), true, true);
        AFTER_IMAGE_OFF(fighter, 4);
    }
}

//SlipAttack
unsafe extern "C" fn effect_krystal_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0.0, 0.0, -0.2, Hash40::new("swordr1"), 0.0, 15.2, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 1.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.5, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.6);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0.0, 0.0, -0.2, Hash40::new("swordr1"), 0.0, 15.2, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 1.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.5, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
}

//DownAttackD
unsafe extern "C" fn effect_krystal_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0.0, 0.0, -0.2, Hash40::new("swordr1"), 0.0, 15.2, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 1.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.5, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.6);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0.0, 0.0, -0.2, Hash40::new("swordr1"), 0.0, 15.2, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 1.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.5, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
}

//DownAttackU
unsafe extern "C" fn effect_krystal_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pitb_sword1"), Hash40::new("tex_pitb_sword2"), 4, Hash40::new("swordr1"), 0.0, 0.0, -0.2, Hash40::new("swordr1"), 0.0, 15.2, -1.2, true, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 1.0, 0.0, 0.0, 0.0, 90.0, 0.0, 1.5, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.6);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 2, 8.5, 2.2, -8.5, 17, 9.5, 1, true);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 10.5, 14.0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true, 0.5);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.7);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), true, true);
    }
}

//SpecialNFireS
unsafe extern "C" fn effect_krystal_SpecialNFireS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("haver"), 0, 12.6, 0, 0, 0, 0, 0.25, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("haver"), 0, 12.6, 0, 0, 0, 0, 0.25, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("haver"), 0, 12.6, 0, 0, 0, 0, 0.25, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("haver"), 0, 12.6, 0, 0, 0, 0, 0.25, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("haver"), 0, 13, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 21.0);
	if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -5, 0, 0, 0, 0.94, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialAirNFireS
unsafe extern "C" fn effect_krystal_SpecialAirNFireS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("haver"), 0, 12.6, 0, 0, 0, 0, 0.25, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("haver"), 0, 12.6, 0, 0, 0, 0, 0.25, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("haver"), 0, 12.6, 0, 0, 0, 0, 0.25, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_shot"), Hash40::new("haver"), 0, 12.6, 0, 0, 0, 0, 0.25, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("haver"), 0, 12.5, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    wait(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//SpecialSStart
unsafe extern "C" fn effect_krystal_SpecialSStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("pitb_gouwan_dash_end"), Hash40::new("havel"), 0, 14, 1.5, 0, 0, 0, 1, true);
	}
    frame(fighter.lua_state_agent, 12.0);
	if is_excute(fighter) {
		LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -5, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
	frame(fighter.lua_state_agent, 19.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_bullet_r"), Hash40::new("top"), 0, 8, 20, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_splash"), Hash40::new("top"), 0, 8, 20, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_shot"), Hash40::new("top"), 0, 8, 20, 0, 0, 0, 1, true);
	}
    frame(fighter.lua_state_agent, 23.0);
	if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_shot"), Hash40::new("top"), 0, 8, 20, 0, 0, 0, 1, true);
	}
    frame(fighter.lua_state_agent, 26.0);
	if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_shot"), Hash40::new("top"), 0, 8, 20, 0, 0, 0, 1, true);
	}
    frame(fighter.lua_state_agent, 29.0);
	if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_shot"), Hash40::new("top"), 0, 8, 20, 0, 0, 0, 1, true);
	}
    frame(fighter.lua_state_agent, 32.0);
	if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_shot"), Hash40::new("top"), 0, 8, 20, 0, 0, 0, 1, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("klystal_iceblast_splash"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("klystal_iceblast_bullet_r"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_splash"), Hash40::new("havel"), 0, 14, 1.5, 0, 0, 0, 0.75, true);
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_bullet_r"), Hash40::new("havel"), 0, 14, 1.5, 0, 0, 0, 0.4, true);
	}
	frame(fighter.lua_state_agent, 36.0);
	if is_excute(fighter) {
		EFFECT_OFF_KIND(fighter, Hash40::new("pitb_gouwan_dash_end"), true, true);
	}
}

//SpecialAirSStart
unsafe extern "C" fn effect_krystal_SpecialAirSStart(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 4.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("pitb_gouwan_dash_end"), Hash40::new("havel"), 0, 14, 1.5, 0, 0, 0, 1, true);
	}
	frame(fighter.lua_state_agent, 19.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_bullet_r"), Hash40::new("top"), 0, 2, 17, 33, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_splash"), Hash40::new("top"), 0, 2, 17, 33, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_shot"), Hash40::new("top"), 0, 2, 17, 33, 0, 0, 1, true);
	}
    frame(fighter.lua_state_agent, 23.0);
	if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_shot"), Hash40::new("top"), 0, 2, 17, 33, 0, 0, 1, true);
	}
    frame(fighter.lua_state_agent, 26.0);
	if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_shot"), Hash40::new("top"), 0, 2, 17, 33, 0, 0, 1, true);
	}
    frame(fighter.lua_state_agent, 29.0);
	if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_shot"), Hash40::new("top"), 0, 2, 17, 33, 0, 0, 1, true);
	}
    frame(fighter.lua_state_agent, 32.0);
	if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_shot"), Hash40::new("top"), 0, 2, 17, 33, 0, 0, 1, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("klystal_iceblast_splash"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("klystal_iceblast_bullet_r"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_splash"), Hash40::new("havel"), 0, 14, 1.5, 0, 0, 0, 0.75, true);
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_iceblast_bullet_r"), Hash40::new("havel"), 0, 14, 1.5, 0, 0, 0, 0.4, true);
    }
	frame(fighter.lua_state_agent, 36.0);
	if is_excute(fighter) {
		EFFECT_OFF_KIND(fighter, Hash40::new("pitb_gouwan_dash_end"), true, true);
	}
}

//SpecialHi
unsafe extern "C" fn effect_krystal_SpecialHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        LAST_EFFECT_SET_RATE(fighter, 1.3);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_fly_miracle_b"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.6, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_fly_miracle_b"), Hash40::new("throw"), 0, -2, 0.7, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_fly_miracle_b"), Hash40::new("throw"), 0, -1, -0.7, 0, 0, 0, 0.5, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        LAST_EFFECT_SET_COLOR(fighter, 0.75, 0.9, 1);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_steam"), Hash40::new("throw"), 0, -3, 0, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("klystal_rod_flare"), Hash40::new("haver"), 0, 11.5, -0.4, -4.7, 0, 0, 0.15, true);
		LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_steam"), Hash40::new("throw"), 0, -3, 0, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_steam"), Hash40::new("throw"), 0, -3, 0, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_steam"), Hash40::new("throw"), 0, -3, 0, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_fly_mirable_b"), true, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_steam"), Hash40::new("throw"), 0, -3, 0, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_steam"), Hash40::new("throw"), 0, -3, 0, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_steam"), Hash40::new("throw"), 0, -3, 0, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_steam"), true, true);
    }
}

//SpecialHiStart
unsafe extern "C" fn effect_krystal_SpecialHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialAirHiStart
unsafe extern "C" fn effect_krystal_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialLwStartL
unsafe extern "C" fn effect_krystal_SpecialLwStartL(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 6.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("pitb_guardian_shield"), Hash40::new("top"), 0.0, 12.0, 0.0, 0, 300, 0, 1.0, true);
		LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
	}
}

//SpecialLwStartR
unsafe extern "C" fn effect_krystal_SpecialLwStartR(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 6.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("pitb_guardian_shield"), Hash40::new("top"), 0.0, 12.0, 0.0, 0, 300, 0, 1.0, true);
		LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
	}
}

//SpecialAirLwStartL
unsafe extern "C" fn effect_krystal_SpecialAirLwStartL(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 6.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("pitb_guardian_shield"), Hash40::new("top"), 0.0, 12.0, 0.0, 0, 300, 0, 1.0, true);
		LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
	}
}

//SpecialAirLwStartR
unsafe extern "C" fn effect_krystal_SpecialAirLwStartR(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 6.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("pitb_guardian_shield"), Hash40::new("top"), 0.0, 12.0, 0.0, 0, 300, 0, 1.0, true);
		LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
	}
}

//SpecialLwHold
unsafe extern "C" fn effect_krystal_SpecialLwHold(fighter: &mut L2CAgentBase) {}

//SpecialAirLwHold
unsafe extern "C" fn effect_krystal_SpecialAirLwHold(fighter: &mut L2CAgentBase) {}

//SpecialLwEndR
unsafe extern "C" fn effect_krystal_SpecialLwEndR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4.5, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 0, 5.6, 2.6, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 0, 5.6, 2.1, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 0, 8.2, 1.2, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 0, 8.2, -0.7, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_guardian_shield"), false, false);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, 0.7, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, -0.7, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, 4, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, -2, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, 0.7, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, -0.7, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, 4, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, -2, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, 0.7, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, -0.7, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, 4, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, -2, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, 0.7, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, -0.7, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, 4, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, -2, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_guardian_shield"), false, false);
    }
}

//SpecialLwEndL
unsafe extern "C" fn effect_krystal_SpecialLwEndL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4.5, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 0, 5.6, 2.6, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 0, 5.6, 2.1, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 0, 8.2, 1.2, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 0, 8.2, -0.7, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_guardian_shield"), false, false);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, 0.7, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, -0.7, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, 4, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, -2, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, 0.7, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, -0.7, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, 4, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, -2, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, 0.7, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, -0.7, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, 4, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, -2, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, 0.7, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, -0.7, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, 4, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, -2, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_guardian_shield"), false, false);
    }
}

//SpecialAirLwEndL
unsafe extern "C" fn effect_krystal_SpecialAirLwEndL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4.5, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 0, 5.6, 2.6, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 0, 5.6, 2.1, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 0, 8.2, 1.2, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 0, 8.2, -0.7, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_guardian_shield"), false, false);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, 0.7, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, -0.7, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, 4, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, -2, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, 0.7, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, -0.7, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, 4, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, -2, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, 0.7, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, -0.7, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, 4, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, -2, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, 0.7, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, -0.7, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, 4, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, -2, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_guardian_shield"), false, false);
    }
}

//SpecialAirLwEndR
unsafe extern "C" fn effect_krystal_SpecialAirLwEndR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4.5, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 0, 5.6, 2.6, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 0, 5.6, 2.1, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 0, 8.2, 1.2, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 0, 8.2, -0.7, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_guardian_shield"), false, false);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, 0.7, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, -0.7, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, 4, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, -2, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, 0.7, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, -0.7, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, 4, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, -2, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, 0.7, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, -0.7, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, 4, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, -2, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, 0.7, 0, 240, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 4.3, -0.7, 0, 120, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, 4, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_guardian_shield_end"), Hash40::new("top"), 0, 12.5, -2, 0, 120, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_guardian_shield"), false, false);
    }
}

//AppealHiR
unsafe extern "C" fn effect_krystal_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
       EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("pitb_atk_air_n"), Hash40::new("swordr2"), 0, -1, 0, -90, -90, 0, 1, true, 0.3);
       EFFECT_FOLLOW(fighter, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 0, 0, 0, 0, -90, 0, 1, true);
       EFFECT_FOLLOW(fighter, Hash40::new("pitb_sword"), Hash40::new("swordl"), 0, -0.06, 0, 180, -90, 0, 1, true);
    }
     frame(fighter.lua_state_agent, 30.0);
     if is_excute(fighter) {
       EFFECT_OFF_KIND(fighter, Hash40::new("pitb_atk_air_n"), false, false);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
       EFFECT_OFF_KIND(fighter, Hash40::new("pitb_sword"), false, false);
    }
    frame(fighter.lua_state_agent, 58.0);
    if is_excute(fighter) {
       EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -4.2, 4.1, 5.2, -4.5, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

//AppealHiL 
unsafe extern "C" fn effect_krystal_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
       EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("pitb_atk_air_n"), Hash40::new("swordr2"), 0, -1, 0, -90, -90, 0, 1, true, 0.3);
       EFFECT_FOLLOW(fighter, Hash40::new("pitb_sword"), Hash40::new("swordr1"), 0, 0, 0, 0, -90, 0, 1, true);
       EFFECT_FOLLOW(fighter, Hash40::new("pitb_sword"), Hash40::new("swordl"), 0, -0.06, 0, 180, -90, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
       EFFECT_OFF_KIND(fighter, Hash40::new("pitb_atk_air_n"), false, false);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
       EFFECT_OFF_KIND(fighter, Hash40::new("pitb_sword"), false, false);
    }
    frame(fighter.lua_state_agent, 54.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -4.2, 4.1, 5.2, -4.5, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

//AppealSR
unsafe extern "C" fn effect_krystal_AppealSR(fighter: &mut L2CAgentBase) {}

//AppealSL
unsafe extern "C" fn effect_krystal_AppealSL(fighter: &mut L2CAgentBase) {}

//AppealLwR
unsafe extern "C" fn effect_krystal_AppealLwR(fighter: &mut L2CAgentBase) {}

//AppealLwL
unsafe extern "C" fn effect_krystal_AppealLwL(fighter: &mut L2CAgentBase) {}

//Final
unsafe extern "C" fn effect_krystal_Final(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EffectModule::req_screen(fighter.module_accessor, Hash40::new("bg_pitb_final"), false, false, false);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_staff_shot"), Hash40::new("haver"), 0, 14, 1.5, 0, 0, 0, 0.75, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_explosion_flash"), Hash40::new("haver"), 0, 13, 1.5, 0, 0, 0, 0.15, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_end"), Hash40::new("haver"), 0, 14, 1.5, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_gouwan_dash_end"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_staff_shot"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_bomb_a"), Hash40::new("haver"), 0, 14, 1.5, 0, 0, 0, 0.50, true);
    }

    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_fireflower_bullet"), Hash40::new("throw"), 0, 6, 3, 0, 0, 0, 2.8, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_fireflower_bullet"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 2.8, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_fireflower_bullet"), Hash40::new("throw"), 0, -5, -3, 0, 0, 0, 2.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.46);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_ring"), Hash40::new("throw"), 0, -5, 2.2, 90, 0, 0, 2.0, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash"), Hash40::new("throw"), 0, -5, 3, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_start"), Hash40::new("throw"), 0, -5, 3, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_flash"), Hash40::new("throw"), 0, -5, 2, 0, 0, 0, 1.7, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_flash"), Hash40::new("throw"), 0, -5, 2, 0, 0, 0, 1.7, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_flash"), Hash40::new("throw"), 0, -5, 2, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_ring"), Hash40::new("throw"), 0, 5, -3.2, 90, 0, 0, 2.0, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash"), Hash40::new("throw"), 0, 5, -4, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_start"), Hash40::new("throw"), 0, 5, -4, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_flash"), Hash40::new("throw"), 0, 5, -3, 0, 0, 0, 1.7, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_flash"), Hash40::new("throw"), 0, 5, -3, 0, 0, 0, 1.7, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_flash"), Hash40::new("throw"), 0, 5, -3, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(fighter, 1.46);
    }
    frame(fighter.lua_state_agent, 83.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 2.4, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_explosion_flash"), Hash40::new("haver"), 0, 13, 1.5, 0, 0, 0, 0.5, true);
    }
    frame(fighter.lua_state_agent, 85.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_gouwan_dash_ring"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_gouwan_dash"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_gouwan_dash_start"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_gouwan_dash_flash"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_fireflower_bullet"), true, true);
    }
}

//FinalAir
unsafe extern "C" fn effect_krystal_FinalAir(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EffectModule::req_screen(fighter.module_accessor, Hash40::new("bg_pitb_final"), false, false, false);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_staff_shot"), Hash40::new("haver"), 0, 14, 1.5, 0, 0, 0, 0.75, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_explosion_flash"), Hash40::new("haver"), 0, 13, 1.5, 0, 0, 0, 0.15, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_end"), Hash40::new("haver"), 0, 14, 1.5, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_gouwan_dash_end"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_staff_shot"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_bomb_a"), Hash40::new("haver"), 0, 14, 1.5, 0, 0, 0, 0.50, true);
    }

    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_fireflower_bullet"), Hash40::new("throw"), 0, 6, 3, 0, 0, 0, 2.8, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_fireflower_bullet"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 2.8, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_fireflower_bullet"), Hash40::new("throw"), 0, -5, -3, 0, 0, 0, 2.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.46);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_ring"), Hash40::new("throw"), 0, -5, 2.2, 90, 0, 0, 2.0, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash"), Hash40::new("throw"), 0, -5, 3, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_start"), Hash40::new("throw"), 0, -5, 3, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_flash"), Hash40::new("throw"), 0, -5, 2, 0, 0, 0, 1.7, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_flash"), Hash40::new("throw"), 0, -5, 2, 0, 0, 0, 1.7, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_flash"), Hash40::new("throw"), 0, -5, 2, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_ring"), Hash40::new("throw"), 0, 5, -3.2, 90, 0, 0, 2.0, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash"), Hash40::new("throw"), 0, 5, -4, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_start"), Hash40::new("throw"), 0, 5, -4, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_flash"), Hash40::new("throw"), 0, 5, -3, 0, 0, 0, 1.7, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_flash"), Hash40::new("throw"), 0, 5, -3, 0, 0, 0, 1.7, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pitb_gouwan_dash_flash"), Hash40::new("throw"), 0, 5, -3, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(fighter, 1.46);
    }
    frame(fighter.lua_state_agent, 83.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 2.4, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_explosion_flash"), Hash40::new("haver"), 0, 13, 1.5, 0, 0, 0, 0.5, true);
    }
    frame(fighter.lua_state_agent, 85.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_gouwan_dash_ring"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_gouwan_dash"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_gouwan_dash_start"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_gouwan_dash_flash"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_fireflower_bullet"), true, true);
    }
}

//EntryL
unsafe extern "C" fn effect_krystal_EntryL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 5.5, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 4.5, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 4.5, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 8.0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 8.0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 8.0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 8.0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 8.0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7.6, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7.6, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 2.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
}

//EntryR
unsafe extern "C" fn effect_krystal_EntryR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 5.5, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 4.5, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 4.5, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 8.0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 8.0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 8.0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 8.0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 8.0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7.6, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7.6, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 2.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pitb_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("pit_entry"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
}

//LandingLight
unsafe extern "C" fn effect_krystal_LandingLight(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

//Win2
unsafe extern "C" fn effect_krystal_Win2(fighter: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("pitb")
    .effect_acmd("effect_jumpaerialf1_krystal", effect_krystal_JumpAerialF1, Default)
    .effect_acmd("effect_attack11_krystal", effect_krystal_Attack11, Low)
    .effect_acmd("effect_attack12_krystal", effect_krystal_Attack12, Low)
    .effect_acmd("effect_attack13_krystal", effect_krystal_Attack13, Low)
    .effect_acmd("effect_attackdash_krystal", effect_krystal_AttackDash, Low)
    .effect_acmd("effect_attacks3_krystal", effect_krystal_AttackS3, Low)
    .effect_acmd("effect_attackhi3_krystal", effect_krystal_AttackHi3, Low)
    .effect_acmd("effect_attacklw3_krystal", effect_krystal_AttackLw3, Low)
    .effect_acmd("effect_attacks4_krystal", effect_krystal_AttackS4, Low)
    .effect_acmd("effect_attackhi4_krystal", effect_krystal_AttackHi4, Low)
    .effect_acmd("effect_attacklw4_krystal", effect_krystal_AttackLw4, Low)
    .effect_acmd("effect_attackairn_krystal", effect_krystal_AttackAirN, Low)
    .effect_acmd("effect_attackairf_krystal", effect_krystal_AttackAirF, Low)    
    .effect_acmd("effect_attackairb_krystal", effect_krystal_AttackAirB, Low)
    .effect_acmd("effect_attackairhi_krystal", effect_krystal_AttackAirHi, Low)
    .effect_acmd("effect_attackairlw_krystal", effect_krystal_AttackAirLw, Low)
    .effect_acmd("effect_throwf_krystal", effect_krystal_ThrowF, Low)
    .effect_acmd("effect_throwb_krystal", effect_krystal_ThrowB, Low)
    .effect_acmd("effect_throwhi_krystal", effect_krystal_ThrowHi, Low)
    .effect_acmd("effect_throwlw_krystal", effect_krystal_ThrowLw, Low)
    .effect_acmd("effect_downattackd_krystal", effect_krystal_DownAttackD, Low)
    .effect_acmd("effect_downattacku_krystal", effect_krystal_DownAttackU, Low)
    .effect_acmd("effect_slipattack_krystal", effect_krystal_SlipAttack, Low)
    .effect_acmd("effect_specialnfires_krystal", effect_krystal_SpecialNFireS, Low)  
    .effect_acmd("effect_specialairnfires_krystal", effect_krystal_SpecialAirNFireS, Low)  
    .effect_acmd("effect_specialsstart_krystal", effect_krystal_SpecialSStart, Low)  
    .effect_acmd("effect_specialairsstart_krystal", effect_krystal_SpecialAirSStart, Low)  
    .effect_acmd("effect_specialhi_krystal", effect_krystal_SpecialHi, Low)
    .effect_acmd("effect_specialhistart_krystal", effect_krystal_SpecialHiStart, Low)
    .effect_acmd("effect_specialairhistart_krystal", effect_krystal_SpecialAirHiStart, Low)
    .effect_acmd("effect_speciallwstartr_krystal", effect_krystal_SpecialLwStartR, Low)
    .effect_acmd("effect_speciallwstartl_krystal", effect_krystal_SpecialLwStartL, Low)
    .effect_acmd("effect_specialairlwstartr_krystal", effect_krystal_SpecialAirLwStartR, Low)
    .effect_acmd("effect_specialairlwstartl_krystal", effect_krystal_SpecialAirLwStartL, Low)
    .effect_acmd("effect_speciallwhold_krystal", effect_krystal_SpecialLwHold, Low)
    .effect_acmd("effect_specialairlwhold_krystal", effect_krystal_SpecialAirLwHold, Low)
    .effect_acmd("effect_speciallwendl_krystal", effect_krystal_SpecialLwEndL, Low)
    .effect_acmd("effect_specialairlwendl_krystal", effect_krystal_SpecialAirLwEndL, Low)
    .effect_acmd("effect_speciallwendr_krystal", effect_krystal_SpecialLwEndR, Low)
    .effect_acmd("effect_specialairlwendr_krystal", effect_krystal_SpecialAirLwEndR, Low)
    .effect_acmd("effect_appealsr_krystal", effect_krystal_AppealSR, Low)
    .effect_acmd("effect_appealsl_krystal", effect_krystal_AppealSL, Low)
    .effect_acmd("effect_appealhir_krystal", effect_krystal_AppealHiR, Low)
    .effect_acmd("effect_appealhil_krystal", effect_krystal_AppealHiL, Low)
    .effect_acmd("effect_appeallwr_krystal", effect_krystal_AppealLwR, Low)
    .effect_acmd("effect_appeallwl_krystal", effect_krystal_AppealLwL, Low)
    .effect_acmd("effect_final_krystal", effect_krystal_Final, Low)
    .effect_acmd("effect_finalair_krystal", effect_krystal_FinalAir, Low)
    .effect_acmd("effect_entryl_krystal", effect_krystal_EntryL, Low)
    .effect_acmd("effect_entryr_krystal", effect_krystal_EntryR, Low)
    .effect_acmd("effect_landinglight_krystal", effect_krystal_LandingLight, Low)
    .effect_acmd("effect_win2_krystal", effect_krystal_Win2, Low)
    .install();
}