use crate::imports::BuildImports::*;

//Run
unsafe extern "C" fn sound_sans_Run(fighter: &mut L2CAgentBase) {
    loop {
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            PLAY_STEP(fighter, Hash40::new("se_palutena_step_left_m"));
        }
        wait(fighter.lua_state_agent, 18.0);
        if is_excute(fighter) {
            PLAY_STEP(fighter, Hash40::new("se_palutena_step_right_m"));
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
    }
}

//EscapeN
unsafe extern "C" fn sound_sans_EscapeN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_escape"));
    }
}

//EscapeAir
unsafe extern "C" fn sound_sans_EscapeAir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_h01"));
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_h01"));
    }
}

//Attack11
unsafe extern "C" fn sound_sans_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_attack100"));
    }
}

//Attack12
unsafe extern "C" fn sound_sans_Attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_attack100"));
    }
}

//AttackDash
unsafe extern "C" fn sound_sans_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_07"));
    }
}

//AttackS3
unsafe extern "C" fn sound_sans_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_l06"));
    }
}

//AttackHi3
unsafe extern "C" fn sound_sans_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_04"));
    }
}

//AttackLw3
unsafe extern "C" fn sound_sans_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_s"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_m"));
    }
}

//AttackS4Charge
unsafe extern "C" fn sound_sans_AttackS4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_l01"));
    }
}

//AttackS4
unsafe extern "C" fn sound_sans_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_palutena_special_l01"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_h02"));
    }
}

//AttackHi4Charge
unsafe extern "C" fn sound_sans_AttackHi4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_l01"));
    } 
}

//AttackHi4
unsafe extern "C" fn sound_sans_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_palutena_special_l01"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_attackhard_h01"));
    }
}

//AttackLw4Charge
unsafe extern "C" fn sound_sans_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_l01"));
    }    
}

//AttackLw4
unsafe extern "C" fn sound_sans_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_palutena_special_l01"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_attack100"));
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_attack100"));
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_heavy_hit_s"));
    }
}

//AttackAirN
unsafe extern "C" fn sound_sans_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_l04"));
    }
}

//AttackAirF 
unsafe extern "C" fn sound_sans_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_swing_l"));
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_swing_ll"));
    }
}

//AttackAirB 
unsafe extern "C" fn sound_sans_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_05"));
    }
}

//AttackAirHi
unsafe extern "C" fn sound_sans_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_l02"));
    }
}

//AttackAirLw
unsafe extern "C" fn sound_sans_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
    }
}

//ThrowF
unsafe extern "C" fn sound_sans_ThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_attackair_n01"));
    }
}

//ThrowB
unsafe extern "C" fn sound_sans_ThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_l"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_final04"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}

//ThrowHi
unsafe extern "C" fn sound_sans_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}

//ThrowLw
unsafe extern "C" fn sound_sans_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_throw_03"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_l03"));
    }
}

//CliffAttack
unsafe extern "C" fn sound_sans_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_dash_start"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_swing_l"));
    }
}

//SlipAttack
unsafe extern "C" fn sound_sans_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_rise"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_swing_l"));
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_swing_l"));
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_palutena_landing01"));
    }
}

//DownAttackU
unsafe extern "C" fn sound_sans_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_h01"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_h01"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_swing_l"));
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_swing_l"));
    }
}

//DownAttackD
unsafe extern "C" fn sound_sans_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_rise"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_05"));
    }
}

//SpecialN
unsafe extern "C" fn sound_sans_SpecialN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_n02"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_n02"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_n02"));
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_n02"));
    }
}

//SpecialAirN
unsafe extern "C" fn sound_sans_SpecialAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_n02"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_n02"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_n02"));
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_n02"));
    }
}

//SpecialS
unsafe extern "C" fn sound_sans_SpecialS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gohoubi_search_ui"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_lifeup"));
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SANS_GENERATE_ARTICLE_GASTER) {
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_SE);
        }
        else {
            WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_SE);
        }
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_SE) {
            PLAY_SE(fighter, Hash40::new("se_palutena_smash_l01"));
        }
    }
}

//SpecialAirS
unsafe extern "C" fn sound_sans_SpecialAirS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gohoubi_search_ui"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_lifeup"));
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SANS_GENERATE_ARTICLE_GASTER) {
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_SE);
        }
        else {
            WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_SE);
        }
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_SE) {
            PLAY_SE(fighter, Hash40::new("se_palutena_smash_l01"));
        }
    }
}

//SpecialHiStart
unsafe extern "C" fn sound_sans_SpecialHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_h01"));
    }
}

//SpecialAirHiStart
unsafe extern "C" fn sound_sans_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_h01"));
    }
}

//SpecialHi
unsafe extern "C" fn sound_sans_SpecialHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_h01"));
    }
}

//SpecialAirHi
unsafe extern "C" fn sound_sans_SpecialAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_h01"));
    }
}

//SpecialLw
unsafe extern "C" fn sound_sans_SpecialLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_l01"));
    }
}

//SpecialAirLw
unsafe extern "C" fn sound_sans_SpecialAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_l01"));
    }
}

//AppealHiR
unsafe extern "C" fn sound_sans_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_n02"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_n02"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_n02"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_n02"));
    }
}

//AppealHiL
unsafe extern "C" fn sound_sans_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_n02"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_n02"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_n02"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_n02"));
    }
}

//AppealSR
unsafe extern "C" fn sound_sans_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_appeal_s02"));
    }
}

//AppealSL
unsafe extern "C" fn sound_sans_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_appeal_s02"));
    }
}

//AppealLwR
unsafe extern "C" fn sound_sans_AppealLwR(fighter: &mut L2CAgentBase) {}

//AppealLwL
unsafe extern "C" fn sound_sans_AppealLwL(fighter: &mut L2CAgentBase) {}

//EntryL
unsafe extern "C" fn sound_sans_EntryL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_s02"));
    }
}

//EntryR
unsafe extern "C" fn sound_sans_EntryR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_special_s02"));
    }
}

//Final
unsafe extern "C" fn sound_sans_Final(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gohoubi_search_ui"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SANS_GENERATE_ARTICLE_GASTER) {
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_SE);
        }
        else {
            WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_SE);
        }
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_SE) {
            PLAY_SE(fighter, Hash40::new("se_palutena_smash_l01"));
        }
    }
}

//FinalAir
unsafe extern "C" fn sound_sans_FinalAir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gohoubi_search_ui"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SANS_GENERATE_ARTICLE_GASTER) {
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_SE);
        }
        else {
            WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_SE);
        }
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_SE) {
            PLAY_SE(fighter, Hash40::new("se_palutena_smash_l01"));
        }
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_sans_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_sans_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_sans_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_sans_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_sans_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_palutena_rnd_futtobi01"), Hash40::new("seq_palutena_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("palutena")
    .sound_acmd("sound_run_sans", sound_sans_Run, Low)
    .sound_acmd("sound_escapen_sans", sound_sans_EscapeN, Low)
    .sound_acmd("sound_escapeair_sans", sound_sans_EscapeAir, Low)
    .sound_acmd("sound_attack11_sans", sound_sans_Attack11, Low)
    .sound_acmd("sound_attack12_sans", sound_sans_Attack12, Low)
    .sound_acmd("sound_attackdash_sans", sound_sans_AttackDash, Low)
    .sound_acmd("sound_attacks3_sans", sound_sans_AttackS3, Low)
    .sound_acmd("sound_attackhi3_sans", sound_sans_AttackHi3, Low)
    .sound_acmd("sound_attacklw3_sans", sound_sans_AttackLw3, Low)
    .sound_acmd("sound_attacks4charge_sans", sound_sans_AttackS4Charge, Low)
    .sound_acmd("sound_attacks4_sans", sound_sans_AttackS4, Low)
    .sound_acmd("sound_attackhi4charge_sans", sound_sans_AttackHi4Charge, Low)
    .sound_acmd("sound_attackhi4_sans", sound_sans_AttackHi4, Low)
    .sound_acmd("sound_attacklw4charge_sans", sound_sans_AttackLw4Charge, Low)
    .sound_acmd("sound_attacklw4_sans", sound_sans_AttackLw4, Low)
    .sound_acmd("sound_attackairn_sans", sound_sans_AttackAirN, Low)
    .sound_acmd("sound_attackairf_sans", sound_sans_AttackAirF, Low)    
    .sound_acmd("sound_attackairb_sans", sound_sans_AttackAirB, Low)
    .sound_acmd("sound_attackairhi_sans", sound_sans_AttackAirHi, Low)
    .sound_acmd("sound_attackairlw_sans", sound_sans_AttackAirLw, Low)
    .sound_acmd("sound_throwf_sans", sound_sans_ThrowF, Low)
    .sound_acmd("sound_throwb_sans", sound_sans_ThrowB, Low)
    .sound_acmd("sound_throwhi_sans", sound_sans_ThrowHi, Low)
    .sound_acmd("sound_throwlw_sans", sound_sans_ThrowLw, Low)
    .sound_acmd("sound_downattackd_sans", sound_sans_DownAttackD, Low)
    .sound_acmd("sound_downattacku_sans", sound_sans_DownAttackU, Low)
    .sound_acmd("sound_cliffattack_sans", sound_sans_CliffAttack, Low)
    .sound_acmd("sound_slipattack_sans", sound_sans_SlipAttack, Low)
    .sound_acmd("sound_specialn_sans", sound_sans_SpecialN, Low)
    .sound_acmd("sound_specialairn_sans", sound_sans_SpecialAirN, Low)
    .sound_acmd("sound_specials_sans", sound_sans_SpecialS, Low)
    .sound_acmd("sound_specialairs_sans", sound_sans_SpecialAirS, Low)
    .sound_acmd("sound_specialhistart_sans", sound_sans_SpecialHiStart, Low)
    .sound_acmd("sound_specialairhistart_sans", sound_sans_SpecialAirHiStart, Low)
    .sound_acmd("sound_specialhi_sans", sound_sans_SpecialHi, Low)
    .sound_acmd("sound_specialairhi_sans", sound_sans_SpecialAirHi, Low)
    .sound_acmd("sound_speciallw_sans", sound_sans_SpecialLw, Low)
    .sound_acmd("sound_specialairlw_sans", sound_sans_SpecialAirLw, Low)
    .sound_acmd("sound_appealsr_sans", sound_sans_AppealSR, Low)
    .sound_acmd("sound_appealsl_sans", sound_sans_AppealSL, Low)
    .sound_acmd("sound_appealhir_sans", sound_sans_AppealHiR, Low)
    .sound_acmd("sound_appealhil_sans", sound_sans_AppealHiL, Low)
    .sound_acmd("sound_appeallwr_sans", sound_sans_AppealLwR, Low)
    .sound_acmd("sound_appeallwl_sans", sound_sans_AppealLwL, Low)
    .sound_acmd("sound_entryl_sans", sound_sans_EntryL, Low)
    .sound_acmd("sound_entryr_sans", sound_sans_EntryR, Low)
    .sound_acmd("sound_final_sans", sound_sans_Final, Low)
    .sound_acmd("sound_finalair_sans", sound_sans_FinalAir, Low)
    .sound_acmd("sound_damageflyhi", sound_sans_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_sans_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_sans_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_sans_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_sans_DamageFlyRoll, Low)
    .install();
}