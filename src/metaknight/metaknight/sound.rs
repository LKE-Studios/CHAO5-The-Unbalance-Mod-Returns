use crate::imports::BuildImports::*;

//ShieldBreakFly
unsafe extern "C" fn sound_metaknight_ShieldBreakFly(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_metaknight_rnd_futtobi01"));
    }
}

//GlideStart
unsafe extern "C" fn sound_metaknight_GlideStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_metaknight_jump04"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_appeal_s03"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_start"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_loop"));
    }
}

//GlideAttack
unsafe extern "C" fn sound_metaknight_GlideAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_metaknight_attack100_03"));
    }
}

//GlideLanding
unsafe extern "C" fn sound_metaknight_GlideLanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_metaknight_glide_start"));
        STOP_SE(fighter, Hash40::new("se_metaknight_glide_loop"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_metaknight_landing02"));
    }
}

//GlideEnd
unsafe extern "C" fn sound_metaknight_GlideEnd(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 10 {//Brawl Meta Knight 
        if is_excute(fighter) {
            STOP_SE(fighter, Hash40::new("se_metaknight_glide_start"));
            STOP_SE(fighter, Hash40::new("se_metaknight_glide_loop"));
        }
        frame(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_metaknight_dash_turn"));
        }
    } 
    else {
        if is_excute(fighter) {
            STOP_SE(fighter, Hash40::new("se_metaknight_glide_start"));
            STOP_SE(fighter, Hash40::new("se_metaknight_glide_loop"));
        }
        frame(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_metaknight_dash_stop"));
        }
    }
}    

//ThrowHi
unsafe extern "C" fn sound_metaknight_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        PLAY_SE(fighter, Hash40::new("vc_metaknight_win01"));
    }
    wait(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
    }
    wait(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_s"));
        PLAY_SE(fighter, Hash40::new("se_common_kick_hit_m"));
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_metaknight_final_hit"));
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
}   

//SpecialNStart
unsafe extern "C" fn sound_metaknight_SpecialNStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_metaknight_special_n01"));
    }
}

//SpecialAirNStart
unsafe extern "C" fn sound_metaknight_SpecialAirNStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_metaknight_special_n01"));
    }
}

//SpecialNSpin
unsafe extern "C" fn sound_metaknight_SpecialNSpin(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        REG_LANDING_SE(fighter, Hash40::new("se_metaknight_landing02"));
    }
}

//SpecialNAddHit
unsafe extern "C" fn sound_metaknight_SpecialNAddHit(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_swish08"));
    }
}

//SpecialSStart
unsafe extern "C" fn sound_metaknight_SpecialSStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_s01"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_metaknight_jump01"));
    }
}

//SpecialHi
unsafe extern "C" fn sound_metaknight_SpecialHi(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 10 {//Brawl Meta Knight 
        frame(fighter.lua_state_agent, 7.0);
        if is_excute(fighter) {
            STOP_SE(fighter, Hash40::new("se_metaknight_dash_start"));
            PLAY_SE(fighter, Hash40::new("se_metaknight_special_h01"));
            PLAY_STATUS(fighter, Hash40::new("vc_metaknight_special_h01"));
        }
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_h02"));
        }
        frame(fighter.lua_state_agent, 29.0);
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_start"));
            PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_loop"));
        }
    } 
    else {
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            STOP_SE(fighter, Hash40::new("se_metaknight_dash_start"));
            PLAY_SE(fighter, Hash40::new("se_metaknight_special_h01"));
            PLAY_STATUS(fighter, Hash40::new("vc_metaknight_special_h01"));
        }
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_h02"));
        }
        frame(fighter.lua_state_agent, 16.0);
        if is_excute(fighter) {
            PLAY_STATUS(fighter, Hash40::new("se_metaknight_special_h03"));
        }
        frame(fighter.lua_state_agent, 29.0);
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_start"));
            PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_loop"));
        }
    }
}    

//SpecialHiLoop
unsafe extern "C" fn sound_metaknight_SpecialHiLoop(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 10 {//Brawl Meta Knight 
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            STOP_SE(fighter, Hash40::new("se_metaknight_dash_start"));
            PLAY_SE(fighter, Hash40::new("se_metaknight_special_h01"));
        }
        frame(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            PLAY_STATUS(fighter, Hash40::new("vc_metaknight_special_h01"));
            PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_h02"));
        }
        frame(fighter.lua_state_agent, 29.0);
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_start"));
            PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_loop"));
        }
    } 
    else {
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            STOP_SE(fighter, Hash40::new("se_metaknight_dash_start"));
            PLAY_SE(fighter, Hash40::new("se_metaknight_special_h01"));
        }
        frame(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            PLAY_STATUS(fighter, Hash40::new("vc_metaknight_special_h01"));
            PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_h02"));
        }
        frame(fighter.lua_state_agent, 11.0);
        if is_excute(fighter) {
            PLAY_STATUS(fighter, Hash40::new("se_metaknight_special_h03"));
        }
        frame(fighter.lua_state_agent, 29.0);
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_start"));
            PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_loop"));
        }
    }
}   

//SpecialLwStart 
unsafe extern "C" fn sound_metaknight_SpecialLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_metaknight_special_l01"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_l01"));
    }
}

//SpecialAirLwStart
unsafe extern "C" fn sound_metaknight_SpecialAirLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_metaknight_special_l01"));
        STOP_SE(fighter, Hash40::new("se_metaknight_dash_start"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_metaknight_special_l01"));
    }
}

pub fn install() {
    Agent::new("metaknight")
    .sound_acmd("sound_shieldbreakfly", sound_metaknight_ShieldBreakFly)
    .sound_acmd("sound_glidestart", sound_metaknight_GlideStart)
    .sound_acmd("sound_glideattack", sound_metaknight_GlideAttack)
    .sound_acmd("sound_glideend", sound_metaknight_GlideEnd)
    .sound_acmd("sound_glidelanding", sound_metaknight_GlideLanding)
    .sound_acmd("sound_throwhi", sound_metaknight_ThrowHi)
    .sound_acmd("sound_specialnstart", sound_metaknight_SpecialNStart)
    .sound_acmd("sound_specialairnstart", sound_metaknight_SpecialAirNStart)
    .sound_acmd("sound_specialnspin", sound_metaknight_SpecialNSpin)
    .sound_acmd("sound_specialnaddhit", sound_metaknight_SpecialNAddHit)
    .sound_acmd("sound_specialsstart", sound_metaknight_SpecialSStart)
    .sound_acmd("sound_specialhi", sound_metaknight_SpecialHi)
    .sound_acmd("sound_specialhiloop", sound_metaknight_SpecialHiLoop)
    .sound_acmd("sound_speciallwstart", sound_metaknight_SpecialLwStart)
    .sound_acmd("sound_specialairlwstart", sound_metaknight_SpecialAirLwStart)
    .install();
}