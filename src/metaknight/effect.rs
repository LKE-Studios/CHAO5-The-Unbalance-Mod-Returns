use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//GlideStartGFX
    agent = "metaknight", 
    script = "effect_glidestart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn metaknight_glide1gfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -5.3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 5.5, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.2, /*G*/ 2.0, /*B*/ 2.55);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        //macros::FLASH(fighter, /*R*/ 1.2, /*G*/ 2.0, /*B*/ 2.55, /*A*/ 0.88);
    }
}

#[acmd_script(//GlideWingGFX
    agent = "metaknight", 
    script = "effect_glidewing", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn metaknight_glide2gfx(fighter: &mut L2CAgentBase) {
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
unsafe fn metaknight_glideattackgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_lw"), Hash40::new("top"), 0.0, 8.5, 0, -41.3, 28.5, -6.0, 1.9, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}
#[acmd_script(//Attack100GFX
    agent = "metaknight", 
    script = "effect_attack100", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn metaknight_jab100gfx(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
        }
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_attack"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.24, true);
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

#[acmd_script(//AttackS3S3GFX
    agent = "metaknight", 
    script = "effect_attacks3s3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn metaknight_sidetilt3gfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_s3_trace"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
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

#[acmd_script(//AttackS4GFX
    agent = "metaknight", 
    script = "effect_attacks4", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn metaknight_sidesmashgfx(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackHi4GFX
    agent = "metaknight", 
    script = "effect_attackhi4", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn metaknight_upsmashgfx(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackLw4GFX
    agent = "metaknight", 
    script = "effect_attacklw4", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn metaknight_downsmashgfx(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackAirFGFX
    agent = "metaknight", 
    script = "effect_attackairf", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn metaknight_fairgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_f"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.38, true);
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

#[acmd_script(//AttackAirBGFX
    agent = "metaknight", 
    script = "effect_attackairb", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn metaknight_bairgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_air_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.38, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
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

#[acmd_script(//AttackAirHiGFX
    agent = "metaknight", 
    script = "effect_attackairhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn metaknight_uairgfx(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackAirLwGFX
    agent = "metaknight", 
    script = "effect_attackairlw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn metaknight_dairgfx(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//ThrowHiGFX
    agent = "metaknight", 
    script = "effect_throwhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn metaknight_throwhigfx(fighter: &mut L2CAgentBase) {
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
        macros::EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//SpecialNStartGFX
    agent = "metaknight", 
    script = "effect_specialnstart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn metaknight_neutralb1gfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, 10, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_START_FRAME);
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("metaknight_tornado_smoke_l"), Hash40::new("top"), 0, -4.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//SpecialAirNStartGFX
    agent = "metaknight", 
    script = "effect_specialairnstart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn metaknight_neutralbairgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        WorkModule::set_int(fighter.module_accessor, 10, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_START_FRAME);
    }
}

#[acmd_script(//SpecialHiGFX
    agent = "metaknight", 
    script = "effect_specialhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn metaknight_upbgfx(fighter: &mut L2CAgentBase) {
    /*frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
    }*/
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        //macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    /*frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 20, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }*/
}

#[acmd_script(//SpecialHiLoopGFX
    agent = "metaknight", 
    script = "effect_specialhiloop", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn metaknight_upbloopgfx(fighter: &mut L2CAgentBase) {
    /*if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
    }*/
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        //macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0, -5, 2.5, 4, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        //macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 20, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    /*frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }*/
}     

pub fn install() {
    smashline::install_acmd_scripts!(
        metaknight_glide1gfx,
        metaknight_glide2gfx,
        metaknight_glideattackgfx,
        metaknight_jab100gfx,
        metaknight_sidetilt3gfx,
        metaknight_sidesmashgfx,
        metaknight_upsmashgfx,
        metaknight_downsmashgfx,
        metaknight_fairgfx,
        metaknight_bairgfx,
        metaknight_uairgfx,
        metaknight_dairgfx,
        metaknight_throwhigfx,
        metaknight_neutralb1gfx,
        metaknight_neutralbairgfx,
        metaknight_upbgfx,
        metaknight_upbloopgfx,
    );
}