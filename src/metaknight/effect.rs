use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//GlideStart
    agent = "metaknight", 
    script = "effect_glidestart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_glidestart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -5.3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 5.0, 0, 0, 0, 0, 3.2, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.68, /*G*/ 0.87, /*B*/ 2.0);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        //macros::FLASH(fighter, /*R*/ 1.2, /*G*/ 2.0, /*B*/ 2.55, /*A*/ 0.88);
    }
}

#[acmd_script(//GlideWing
    agent = "metaknight", 
    script = "effect_glidewing", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_glidewing(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

#[acmd_script(//GlideAttack
    agent = "metaknight", 
    script = "effect_glideattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_glideattack(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_attack_end"), Hash40::new("top"), 9.3, 30.5, -6.25, -11.6, 29.9, 175.0, 1.3, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

#[acmd_script(//GlideLanding
    agent = "metaknight", 
    script = "effect_glidelanding", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_glidelanding(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//Attack100
    agent = "metaknight", 
    script = "effect_attack100", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_attack100(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor); 
        }
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_attack"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.24, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);        
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.2, 10, 0, 2, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.5, 10, 0, 2, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.5, 10, 0, 2, 0, 0, 0, false);
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, 0);
        wait_loop_clear(fighter.lua_state_agent);
    }
}

#[acmd_script(//AttackS3S3
    agent = "metaknight", 
    script = "effect_attacks3s3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_attacks3s3(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_s3_trace"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_sting"), Hash40::new("top"), 0, 7.2, 50.0, 0, 0, 0.0, 0.9, true);                 
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_item_revengeshooter_shot"));
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 14, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

#[acmd_script(//AttackS4
    agent = "metaknight", 
    script = "effect_attacks4", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_attacks4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0.01, 10.0, -0.071, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_smash_trace"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1.35, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

#[acmd_script(//AttackHi4
    agent = "metaknight", 
    script = "effect_attackhi4", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_attackhi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0.01, 10.0, -0.071, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_smash_u_trace"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1.35, true);
        
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//AttackLw4
    agent = "metaknight", 
    script = "effect_attacklw4", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_attacklw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_smash_d_trace"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1.3, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

#[acmd_script(//AttackAirN
    agent = "metaknight", 
    script = "effect_attackairn", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_attackairn(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 12, Hash40::new("haver"), 0.0, 2.5, 0.0, Hash40::new("haver"), 0.0, 15.5, 0.0, true, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.68, /*G*/ 1.2, /*B*/ 0.0);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 0, 0, 0, -90, 0.8, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 0, 0, 0, -90, 0.82, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 0, 0, 0, -90, 0.85, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
        macros::AFTER_IMAGE_OFF(fighter, 2);
    }
}

#[acmd_script(//AttackAirF
    agent = "metaknight", 
    script = "effect_attackairf", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_attackairf(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_f"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.38, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

#[acmd_script(//AttackAirB
    agent = "metaknight", 
    script = "effect_attackairb", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_attackairb(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.38, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5)
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);

    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

#[acmd_script(//AttackAirHi
    agent = "metaknight", 
    script = "effect_attackairhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_attackairhi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_hi"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

#[acmd_script(//AttackAirLw
    agent = "metaknight", 
    script = "effect_attackairlw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_attackairlw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_lw"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

#[acmd_script(//ThrowHi
    agent = "metaknight", 
    script = "effect_throwhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_throwhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.8)
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, 0, -90, 0, 0, 1.6, 0, 1, 0, 0, 0, 0, true, 0.4);
        macros::EFFECT(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2, 0, 1, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.6)
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("metaknight_throw_hi_line"), Hash40::new("top"), 0, 8, 2, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, -3.0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0.0, 0.0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_hit_cut"), Hash40::new("top"), 2, 20, 0, 0, 0, -90.0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 3.5, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.5, /*G*/ 0.178, /*B*/ 1.72);
    }
}

#[acmd_script(//SpecialNStart
    agent = "metaknight", 
    script = "effect_specialnstart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_specialnstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, 10, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_START_FRAME);
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("metaknight_tornado_smoke_l"), Hash40::new("top"), 0, -4.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//SpecialAirNStart
    agent = "metaknight", 
    script = "effect_specialairnstart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_specialairnstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        WorkModule::set_int(fighter.module_accessor, 10, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_START_FRAME);
    }
}

#[acmd_script(//SpecialHi
    agent = "metaknight", 
    script = "effect_specialhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_specialhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 20, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

#[acmd_script(//SpecialHiLoop
    agent = "metaknight", 
    script = "effect_specialhiloop", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_specialhiloop(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0, -5, 2.5, 4, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 20, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

#[acmd_script(//SpecialLw
    agent = "metaknight", 
    script = "effect_speciallw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_speciallw(fighter: &mut L2CAgentBase) {
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, true);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle"), Hash40::new("top"), 0, -1.4, -0.5, 0, 0, 0, 1.3, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_end"), Hash40::new("top"), 0, 8, 0, 0, -90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_mantle_end"), false, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//SpecialLwB
    agent = "metaknight", 
    script = "effect_speciallwb", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_speciallwb(fighter: &mut L2CAgentBase) {
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_b"), Hash40::new("top"), 0, -2.4, -16.5, 0, 0, 0, 1.3, true);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_b"), Hash40::new("top"), 0, 0, -15, 0, 0, 0, 1.3, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
    }
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("metaknight_attack_smoke"), Hash40::new("top"), 23, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_end"), Hash40::new("top"), 0, 8, 0, 0, -90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_mantle_end"), false, false);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

#[acmd_script(//SpecialLwF
    agent = "metaknight", 
    script = "effect_speciallwf", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_speciallwf(fighter: &mut L2CAgentBase) {
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_f"), Hash40::new("top"), 0, 2.6, 14.9, 0, 0, 0, 1.3, true);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_f"), Hash40::new("top"), 0, 0, 15.3, 0, 0, 0, 1.3, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
    }
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("metaknight_attack_smoke"), Hash40::new("top"), -22, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 16, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_end"), Hash40::new("top"), 0, 8, 0, 0, -90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_mantle_end"), false, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

#[acmd_script(//SpecialAirLw
    agent = "metaknight", 
    script = "effect_specialairlw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_specialairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_air"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, true);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_air"), Hash40::new("top"), 0, -2.6, -1, 0, 0, 0, 1.3, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_air"), Hash40::new("top"), 0, -2.6, -1, 0, 0, 0, 1.3, true);
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_end"), Hash40::new("top"), 0, 8, 0, 0, -90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_mantle_end"), false, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

#[acmd_script(//SpecialAirLwB
    agent = "metaknight", 
    script = "effect_specialairlwb", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_specialairlwb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_airb"), Hash40::new("top"), 0, -3.7, -17.5, 0, 0, 0, 1.3, true);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_airb"), Hash40::new("top"), 0, 0, -18, 0, 0, 0, 1.3, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_airb"), Hash40::new("top"), 0, 0, -18, 0, 0, 0, 1.3, true);
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_end"), Hash40::new("top"), 0, 8, 0, 0, -90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_mantle_end"), false, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

#[acmd_script(//SpecialAirLwF
    agent = "metaknight", 
    script = "effect_specialairlwf", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_metaknight_specialairlwf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_airf"), Hash40::new("top"), 0, 2.5, 14.5, 0, 0, 0, 1.3, true);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_airf"), Hash40::new("top"), 0, 0, 15.3, 0, 0, 0, 1.3, true);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_airf"), Hash40::new("top"), 0, 0, 15.3, 0, 0, 0, 1.3, true);
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 16, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_mantle_end"), Hash40::new("top"), 0, 8, 0, 0, -90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_mantle_end"), false, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_metaknight_glidestart,
        effect_metaknight_glidewing,
        effect_metaknight_glideattack,
        effect_metaknight_glidelanding,
        effect_metaknight_attack100,
        effect_metaknight_attacks3s3,
        effect_metaknight_attacks4,
        effect_metaknight_attackhi4,
        effect_metaknight_attacklw4,
        effect_metaknight_attackairn,
        effect_metaknight_attackairf,
        effect_metaknight_attackairb,
        effect_metaknight_attackairhi,
        effect_metaknight_attackairlw,
        effect_metaknight_throwhi,
        effect_metaknight_specialnstart,
        effect_metaknight_specialairnstart,
        effect_metaknight_specialhi,
        effect_metaknight_specialhiloop,
        effect_metaknight_speciallw,
        effect_metaknight_speciallwb,
        effect_metaknight_speciallwf,
        effect_metaknight_specialairlw,
        effect_metaknight_specialairlwb,
        effect_metaknight_specialairlwf
    );
}