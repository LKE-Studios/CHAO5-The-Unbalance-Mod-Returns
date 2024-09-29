use crate::imports::BuildImports::*;

//ShieldBreakFly
unsafe extern "C" fn effect_metaknight_ShieldBreakFly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("sys_piyopiyo"), Hash40::new("head"), 0, 2, 0, 0, 0, 0, 1.4, true);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.5, /*G*/ 0.178, /*B*/ 1.72);
    }
}

//GlideStart
unsafe extern "C" fn effect_metaknight_GlideStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -5.3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 5.0, 0, 0, 0, 0, 3.2, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.68, /*G*/ 0.87, /*B*/ 2.0);
    }
}

//GlideWing
unsafe extern "C" fn effect_metaknight_GlideWing(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

//GlideAttack
unsafe extern "C" fn effect_metaknight_GlideAttack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_attack_end"), Hash40::new("top"), 9.3, 30.5, -6.25, -11.6, 29.9, 175.0, 1.3, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

//GlideLanding
unsafe extern "C" fn effect_metaknight_GlideLanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

//Attack100
unsafe extern "C" fn effect_metaknight_Attack100(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor); 
        }
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_attack"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.24, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);        
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.2, 10, 0, 2, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.5, 10, 0, 2, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.5, 10, 0, 2, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 20.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
        }
        wait_loop_clear(fighter);
    }
}

//Attack100
unsafe extern "C" fn effect_metaknight_Attack100_B(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), 1, 6, -1, 20, 165, 105, 0.6, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), 0, 7, 0, 90, 0, 25, 1.0, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), 1, 6.5, -1, -165, 20, -80, 1.0, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 9.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), 0, 6, 2, -205, 160, -145, 1.0, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), 2, 6, 1, 0, -155, 105, 1.0, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), 0, 7, 0, 90, 0, -45, 1.0, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 18.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), 2, 4, 4, -165, 20, -95, 1.0, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 19.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), -2, 6, -4, -25, -30, -60, 1.0, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
            EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
        }
        wait_loop_clear(fighter);
    }
}

//Attack100End
unsafe extern "C" fn effect_metaknight_Attack100End(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_attack"), false, false);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_attack_end"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1.25, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), -1.0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackS3S3
unsafe extern "C" fn effect_metaknight_AttackS3S3(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_s3_trace"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 14, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

//AttackS4
unsafe extern "C" fn effect_metaknight_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0.01, 10.0, -0.071, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_smash_trace"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1.35, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

//AttackHi4
unsafe extern "C" fn effect_metaknight_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0.01, 10.0, -0.071, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_smash_u_trace"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1.35, true);
        
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackLw4
unsafe extern "C" fn effect_metaknight_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_smash_d_trace"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1.3, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

//AttackAirN
unsafe extern "C" fn effect_metaknight_AttackAirN(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 0, 0, 0, -90.0, 1.25, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.9, /*B*/ 0.04);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 0, 0, 0, -90, 1.1, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.15, /*G*/ 0.2, /*B*/ 2.0);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 0, 0, 0, -90, 1.0, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 0, 0, 0, -90, 1.0, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.9, /*B*/ 0.04);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 0, 0, 0, -90, 0.9, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 0, 0, 0, -90, 0.9, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
        AFTER_IMAGE_OFF(fighter, 2);
    }
}

//AttackAirF
unsafe extern "C" fn effect_metaknight_AttackAirF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_f"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.38, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

//AttackAirB
unsafe extern "C" fn effect_metaknight_AttackAirB(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 1.5)
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);

    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

//AttackAirHi
unsafe extern "C" fn effect_metaknight_AttackAirHi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_RATE(fighter, 0.75);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

//AttackAirLw
unsafe extern "C" fn effect_metaknight_AttackAirLw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_lw"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, true);
        LAST_EFFECT_SET_RATE(fighter, 0.75);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

//ThrowHi
unsafe extern "C" fn effect_metaknight_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.8)
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, 0, -90, 0, 0, 1.6, 0, 1, 0, 0, 0, 0, true, 0.4);
        EFFECT(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2, 0, 1, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.6)
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("metaknight_throw_hi_line"), Hash40::new("top"), 0, 8, 2, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, -3.0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0.0, 0.0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_hit_cut"), Hash40::new("top"), 2, 20, 0, 0, 0, -90.0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 3.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.5, /*G*/ 0.178, /*B*/ 1.72);
    }
}

//SpecialNStart
unsafe extern "C" fn effect_metaknight_SpecialNStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, 8, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_START_FRAME);
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("metaknight_tornado_smoke_l"), Hash40::new("top"), 0, -4.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialAirNStart
unsafe extern "C" fn effect_metaknight_SpecialAirNStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        WorkModule::set_int(fighter.module_accessor, 8, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_START_FRAME);
    }
}

//SpecialNSpinGroundEffect
unsafe extern "C" fn effect_metaknight_SpecialNSpinGroundEffect(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("metaknight_tornado_smoke_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        WorkModule::set_float(fighter.module_accessor, 500.0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_FLOAT_GROUND_EFFECT_COUNTER);
    }
}

//SpecialHi
unsafe extern "C" fn effect_metaknight_SpecialHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 20, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_shuttleloop2"), Hash40::new("top"), 0, 14.6, 46.3, -102.0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

//SpecialHiLoop
unsafe extern "C" fn effect_metaknight_SpecialHiLoop(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0, -5, 2.5, 4, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 20, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_shuttleloop2"), Hash40::new("top"), 0, 10.6, 46.3, -102.0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

//SpecialLwStart
unsafe extern "C" fn effect_metaknight_SpecialLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_start"), Hash40::new("top"), 0, 6, 0, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

//SpecialLw
unsafe extern "C" fn effect_metaknight_SpecialLw(fighter: &mut L2CAgentBase) {
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, true);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle"), Hash40::new("top"), 0, -1.4, -0.5, 0, 0, 0, 1.3, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_end"), Hash40::new("top"), 0, 8, 0, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_mantle_end"), false, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialLwB
unsafe extern "C" fn effect_metaknight_SpecialLwB(fighter: &mut L2CAgentBase) {
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_b"), Hash40::new("top"), 0, -2.4, -16.5, 0, 0, 0, 1.3, true);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_b"), Hash40::new("top"), 0, 0, -15, 0, 0, 0, 1.3, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
    }
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("metaknight_attack_smoke"), Hash40::new("top"), 23, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_end"), Hash40::new("top"), 0, 8, 0, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_mantle_end"), false, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

//SpecialLwF
unsafe extern "C" fn effect_metaknight_SpecialLwF(fighter: &mut L2CAgentBase) {
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_f"), Hash40::new("top"), 0, 2.6, 14.9, 0, 0, 0, 1.3, true);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_f"), Hash40::new("top"), 0, 0, 15.3, 0, 0, 0, 1.3, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
    }
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("metaknight_attack_smoke"), Hash40::new("top"), -22, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 16, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_end"), Hash40::new("top"), 0, 8, 0, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_mantle_end"), false, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

//SpecialAirLw
unsafe extern "C" fn effect_metaknight_SpecialAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_air"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, true);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_air"), Hash40::new("top"), 0, -2.6, -1, 0, 0, 0, 1.3, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_air"), Hash40::new("top"), 0, -2.6, -1, 0, 0, 0, 1.3, true);
        EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_end"), Hash40::new("top"), 0, 8, 0, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_mantle_end"), false, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

//SpecialAirLwB
unsafe extern "C" fn effect_metaknight_SpecialAirLwB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_airb"), Hash40::new("top"), 0, -3.7, -17.5, 0, 0, 0, 1.3, true);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_airb"), Hash40::new("top"), 0, 0, -18, 0, 0, 0, 1.3, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_airb"), Hash40::new("top"), 0, 0, -18, 0, 0, 0, 1.3, true);
        EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_end"), Hash40::new("top"), 0, 8, 0, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_mantle_end"), false, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

//SpecialAirLwF
unsafe extern "C" fn effect_metaknight_SpecialAirLwF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_airf"), Hash40::new("top"), 0, 2.5, 14.5, 0, 0, 0, 1.3, true);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_airf"), Hash40::new("top"), 0, 0, 15.3, 0, 0, 0, 1.3, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_airf"), Hash40::new("top"), 0, 0, 15.3, 0, 0, 0, 1.3, true);
        EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 16, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_end"), Hash40::new("top"), 0, 8, 0, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_mantle_end"), false, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

//SpecialZ
unsafe extern "C" fn effect_metaknight_SpecialZ(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 16, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_end"), Hash40::new("top"), 0, 8, 0, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("metaknight_attack_smoke"), Hash40::new("top"), -22, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_mantle_end"), false, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

//SpecialAirZ
unsafe extern "C" fn effect_metaknight_SpecialAirZ(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 16, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_end"), Hash40::new("top"), 0, 8, 0, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_mantle_end"), false, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

pub fn install() {
    Agent::new("metaknight")
    .effect_acmd("effect_shieldbreakfly", effect_metaknight_ShieldBreakFly, Low)
    .effect_acmd("effect_glidestart", effect_metaknight_GlideStart, Low)
    .effect_acmd("effect_glidewing", effect_metaknight_GlideWing, Low)
    .effect_acmd("effect_glideattack", effect_metaknight_GlideAttack, Low)
    .effect_acmd("effect_glidelanding", effect_metaknight_GlideLanding, Low)
    .effect_acmd("effect_attack100", effect_metaknight_Attack100, Low)
    .effect_acmd("effect_attack100_brawl", effect_metaknight_Attack100_B, Low)
    .effect_acmd("effect_attack100end", effect_metaknight_Attack100End, Low)
    .effect_acmd("effect_attacks3s3", effect_metaknight_AttackS3S3, Low)
    .effect_acmd("effect_attacks4", effect_metaknight_AttackS4, Low)
    .effect_acmd("effect_attackhi4", effect_metaknight_AttackHi4, Low)
    .effect_acmd("effect_attacklw4", effect_metaknight_AttackLw4, Low)
    .effect_acmd("effect_attackairn", effect_metaknight_AttackAirN, Low)
    .effect_acmd("effect_attackairf", effect_metaknight_AttackAirF, Low)    
    .effect_acmd("effect_attackairb", effect_metaknight_AttackAirB, Low)
    .effect_acmd("effect_attackairhi", effect_metaknight_AttackAirHi, Low)
    .effect_acmd("effect_attackairlw", effect_metaknight_AttackAirLw, Low)
    .effect_acmd("effect_throwhi", effect_metaknight_ThrowHi, Low)
    .effect_acmd("effect_specialnstart", effect_metaknight_SpecialNStart, Low)
    .effect_acmd("effect_specialairnstart", effect_metaknight_SpecialAirNStart, Low)
    .effect_acmd("effect_specialnspingroundeffect", effect_metaknight_SpecialNSpinGroundEffect, Low)
    .effect_acmd("effect_specialhi", effect_metaknight_SpecialHi, Low)
    .effect_acmd("effect_specialhiloop", effect_metaknight_SpecialHiLoop, Low)
    .effect_acmd("effect_speciallwstart", effect_metaknight_SpecialLwStart, Low)
    .effect_acmd("effect_speciallw", effect_metaknight_SpecialLw, Low)
    .effect_acmd("effect_speciallwb", effect_metaknight_SpecialLwB, Low)
    .effect_acmd("effect_speciallwf", effect_metaknight_SpecialLwF, Low)
    .effect_acmd("effect_specialairlw", effect_metaknight_SpecialAirLw, Low)
    .effect_acmd("effect_specialairlwb", effect_metaknight_SpecialAirLwB, Low)
    .effect_acmd("effect_specialairlwf", effect_metaknight_SpecialAirLwF, Low)
    .effect_acmd("effect_specialz", effect_metaknight_SpecialZ, Low)
    .effect_acmd("effect_specialairz", effect_metaknight_SpecialAirZ, Low)
    .install();
}