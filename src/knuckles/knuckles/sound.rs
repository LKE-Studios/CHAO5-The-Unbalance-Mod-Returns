use crate::imports::BuildImports::*;
use crate::knuckles::*;

//Run
unsafe extern "C" fn sound_knuckles_Run(fighter: &mut L2CAgentBase) {
    loop {
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            PLAY_STEP(fighter, Hash40::new("se_sonic_step_right_m"));
        }
        wait(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            PLAY_STEP(fighter, Hash40::new("se_sonic_step_left_m"));
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
    }
}

//EntryL
unsafe extern "C" fn sound_knuckles_EntryL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing03"));
    }
}

//EntryR
unsafe extern "C" fn sound_knuckles_EntryR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing03"));
    }
}

//Attack11
unsafe extern "C" fn sound_knuckles_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_s"));
    }
}

//Attack12
unsafe extern "C" fn sound_knuckles_Attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_m"));
    }
}

//Attack13
unsafe extern "C" fn sound_knuckles_Attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
    }
}

//Attack100
unsafe extern "C" fn sound_knuckles_Attack100(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    loop {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_s"));
        wait(fighter.lua_state_agent, 6.0);
    }
}

//Attack100End
unsafe extern "C" fn sound_knuckles_Attack100End(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
    }
}

//AttackDash
unsafe extern "C" fn sound_knuckles_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing01"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_ll"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
    }
}

//AttackS3Hi
unsafe extern "C" fn sound_knuckles_AttackS3Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
    }
}

//AttackS3
unsafe extern "C" fn sound_knuckles_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
    }
}

//AttackS3Lw
unsafe extern "C" fn sound_knuckles_AttackS3Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
    }
}

//AttackHi3
unsafe extern "C" fn sound_knuckles_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
    }
}

//AttackLw3
unsafe extern "C" fn sound_knuckles_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_ll"));
    }
}

//AttackS4Charge
unsafe extern "C" fn sound_knuckles_AttackS4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_sonic_smash_s01"));
    }
}

//AttackS4Hi
unsafe extern "C" fn sound_knuckles_AttackS4Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_sonic_attack05"));
        PLAY_SE(fighter, Hash40::new("se_sonic_smash_s02"));
    }
}

//AttackS4
unsafe extern "C" fn sound_knuckles_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_sonic_attack05"));
        PLAY_SE(fighter, Hash40::new("se_sonic_smash_s02"));
    }
}

//AttackS4Lw
unsafe extern "C" fn sound_knuckles_AttackS4Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_sonic_attack05"));
        PLAY_SE(fighter, Hash40::new("se_sonic_smash_s02"));
    }
}

//AttackHi4Charge
unsafe extern "C" fn sound_knuckles_AttackHi4Charge(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new_raw(0x13589f8893));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
    }
}

//AttackHi4
unsafe extern "C" fn sound_knuckles_AttackHi4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new_raw(0x13589f8893));
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_sonic_attack06"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_08"));
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
    }
}

//AttackLw4Charge
unsafe extern "C" fn sound_knuckles_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
    }
}

//AttackLw4
unsafe extern "C" fn sound_knuckles_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_sonic_roundholdloop"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_sonic_attack07"));
        PLAY_SE(fighter, Hash40::new("se_sonic_smash_h01"));
        STOP_SE(fighter, Hash40::new("se_sonic_roundholdloop"));
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
    }
}

//AttackAirN
unsafe extern "C" fn sound_knuckles_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_s"));
    }
}

//AttackAirF
unsafe extern "C" fn sound_knuckles_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
    }
}

//AttackAirB
unsafe extern "C" fn sound_knuckles_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_common_swing_04"));
    }
}

//AttackAirHi
unsafe extern "C" fn sound_knuckles_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_common_swing_05"));
    }
}

//AttackAirLw
unsafe extern "C" fn sound_knuckles_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
    }
}

//ThrowF
unsafe extern "C" fn sound_knuckles_ThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}

//ThrowB
unsafe extern "C" fn sound_knuckles_ThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 57.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_attackair_l01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}

//ThrowHi
unsafe extern "C" fn sound_knuckles_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
}

//ThrowLw
unsafe extern "C" fn sound_knuckles_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
    }
    wait(fighter.lua_state_agent, 62.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}

//DownAttackD
unsafe extern "C" fn sound_knuckles_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_rise"));
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_smash_h01"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing01"));
    }
}

//DownAttackU
unsafe extern "C" fn sound_knuckles_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_rise"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing01"));
    }
}

//SlipAttack
unsafe extern "C" fn sound_knuckles_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
    }
    wait(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_step_right_m"));
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_step_left_m"));
    }
}

//CliffAttack
unsafe extern "C" fn sound_knuckles_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_dash_start"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_special_s01"));
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_s"));
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
    }
}

//SpecialNDig
unsafe extern "C" fn sound_knuckles_SpecialNDig(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
    }
}

//SpecialAirNDig
unsafe extern "C" fn sound_knuckles_SpecialAirNDig(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
    }
}

//SpecialAirNDrillStart
unsafe extern "C" fn sound_knuckles_SpecialAirNDrillStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_roundholdloop"));
    }
}

//SpecialAirNDrillLoop
unsafe extern "C" fn sound_knuckles_SpecialAirNDrillLoop(fighter: &mut L2CAgentBase) {}

//LandingFallSpecial
unsafe extern "C" fn sound_knuckles_LandingFallSpecial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing03"));
    }
}

//SpecialNUpper
unsafe extern "C" fn sound_knuckles_SpecialNUpper(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_sonic_attack06"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_08"));
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing03"));
    }
}

//SpecialS
unsafe extern "C" fn sound_knuckles_SpecialS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
    }
}

//SpecialAirSEndLoop
unsafe extern "C" fn sound_knuckles_SpecialAirSEndLoop(fighter: &mut L2CAgentBase) {}

//SpecialAirSLanding
unsafe extern "C" fn sound_knuckles_SpecialAirSLanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
    }
}

//SpecialHi
unsafe extern "C" fn sound_knuckles_SpecialHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_special_h01"));
    }
}

//SpecialLwHold
unsafe extern "C" fn sound_knuckles_SpecialLwHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_sonic_roundholdloop"));
    }
}

//SpecialAirLwHold
unsafe extern "C" fn sound_knuckles_SpecialAirLwHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_sonic_roundholdloop"));
    }
}

//AppealSR
unsafe extern "C" fn sound_knuckles_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("vc_sonic_appeal02"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_s"));
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_s"));
    }
}

//AppealSL
unsafe extern "C" fn sound_knuckles_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("vc_sonic_appeal02"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_s"));
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_s"));
    }
}

//AppealHiR
unsafe extern "C" fn sound_knuckles_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("vc_sonic_appeal01"));
    }
}

//AppealHiL
unsafe extern "C" fn sound_knuckles_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("vc_sonic_appeal01"));
    }
}

//AppealLwR
unsafe extern "C" fn sound_knuckles_AppealLwR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("vc_sonic_appeal03"));
    }
}

//AppealLwL
unsafe extern "C" fn sound_knuckles_AppealLwL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("vc_sonic_appeal03"));
    }
}

//Win1
unsafe extern "C" fn sound_knuckles_Win1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        let handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_sonic_attack04"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, handle as i32, 1.5, 0);
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        let handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_sonic_landing04"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, handle as i32, 2.0, 0);
    }
}

//Win2
unsafe extern "C" fn sound_knuckles_Win2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_sonic_win03"));
    }
}

//Win3
unsafe extern "C" fn sound_knuckles_Win3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_sonic_roundholdloop"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_sonic_roundholdloop"));
        PLAY_SE_NO_3D(fighter, Hash40::new("se_sonic_landing02"));
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        let handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_smashswing_04"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, handle as i32, 1.8, 0);
        let handle_2 = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_pitin_move_01"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, handle_2 as i32, 2.0, 0);
    }
    frame(fighter.lua_state_agent, 79.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_sonic_win01"));
    }
}

pub fn install() {
    Agent::new("sonic")
    .sound_acmd("sound_attack11_knuckles", sound_knuckles_Attack11, Low)
    .sound_acmd("sound_attack12_knuckles", sound_knuckles_Attack12, Low)
    .sound_acmd("sound_attack13_knuckles", sound_knuckles_Attack13, Low)
    .sound_acmd("sound_attack100_knuckles", sound_knuckles_Attack100, Low)
    .sound_acmd("sound_attack100end_knuckles", sound_knuckles_Attack100End, Low)
    .sound_acmd("sound_attackdash_knuckles", sound_knuckles_AttackDash, Low)
    .sound_acmd("sound_attacks3hi_knuckles", sound_knuckles_AttackS3Hi, Low)
    .sound_acmd("sound_attacks3_knuckles", sound_knuckles_AttackS3, Low)
    .sound_acmd("sound_attacks3lw_knuckles", sound_knuckles_AttackS3Lw, Low)
    .sound_acmd("sound_attackhi3_knuckles", sound_knuckles_AttackHi3, Low)
    .sound_acmd("sound_attacklw3_knuckles", sound_knuckles_AttackLw3, Low)
    .sound_acmd("sound_attacks4charge_knuckles", sound_knuckles_AttackS4Charge, Low)
    .sound_acmd("sound_attacks4hi_knuckles", sound_knuckles_AttackS4Hi, Low)
    .sound_acmd("sound_attacks4_knuckles", sound_knuckles_AttackS4, Low)
    .sound_acmd("sound_attacks4lw_knuckles", sound_knuckles_AttackS4Lw, Low)
    .sound_acmd("sound_attackhi4charge_knuckles", sound_knuckles_AttackHi4Charge, Low)
    .sound_acmd("sound_attackhi4_knuckles", sound_knuckles_AttackHi4, Low)
    .sound_acmd("sound_attacklw4charge_knuckles", sound_knuckles_AttackLw4Charge, Low)
    .sound_acmd("sound_attacklw4_knuckles", sound_knuckles_AttackLw4, Low)
    .sound_acmd("sound_attackairn_knuckles", sound_knuckles_AttackAirN, Low)
    .sound_acmd("sound_attackairf_knuckles", sound_knuckles_AttackAirF, Low)    
    .sound_acmd("sound_attackairb_knuckles", sound_knuckles_AttackAirB, Low)
    .sound_acmd("sound_attackairhi_knuckles", sound_knuckles_AttackAirHi, Low)
    .sound_acmd("sound_attackairlw_knuckles", sound_knuckles_AttackAirLw, Low)
    .sound_acmd("sound_throwf_knuckles", sound_knuckles_ThrowF, Low)
    .sound_acmd("sound_throwb_knuckles", sound_knuckles_ThrowB, Low)
    .sound_acmd("sound_throwhi_knuckles", sound_knuckles_ThrowHi, Low)
    .sound_acmd("sound_throwlw_knuckles", sound_knuckles_ThrowLw, Low)
    .sound_acmd("sound_downattackd_knuckles", sound_knuckles_DownAttackD, Low)
    .sound_acmd("sound_downattacku_knuckles", sound_knuckles_DownAttackU, Low)
    .sound_acmd("sound_cliffattack_knuckles", sound_knuckles_CliffAttack, Low)
    .sound_acmd("sound_slipattack_knuckles", sound_knuckles_SlipAttack, Low)
    .sound_acmd("sound_specialndig_knuckles", sound_knuckles_SpecialNDig, Low)
    .sound_acmd("sound_specialairndig_knuckles", sound_knuckles_SpecialAirNDig, Low)
    .sound_acmd("sound_specialairndrillstart_knuckles", sound_knuckles_SpecialAirNDrillStart, Low)
    .sound_acmd("sound_specialairndrillloop_knuckles", sound_knuckles_SpecialAirNDrillLoop, Low)
    .sound_acmd("sound_landingfallspecial_knuckles", sound_knuckles_LandingFallSpecial, Low)
    .sound_acmd("sound_specialnupper_knuckles", sound_knuckles_SpecialNUpper, Low)  
    .sound_acmd("sound_specials_knuckles", sound_knuckles_SpecialS, Low)
    .sound_acmd("sound_specialairslanding_knuckles", sound_knuckles_SpecialAirSLanding, Low)
    .sound_acmd("sound_specialairsendloop_knuckles", sound_knuckles_SpecialAirSEndLoop, Low)
    .sound_acmd("sound_specialhi_knuckles", sound_knuckles_SpecialHi, Low)
    .sound_acmd("sound_speciallwhold_knuckles", sound_knuckles_SpecialLwHold, Low)
    .sound_acmd("sound_specialairlwhold_knuckles", sound_knuckles_SpecialAirLwHold, Low)
    .sound_acmd("sound_appealsr_knuckles", sound_knuckles_AppealSR, Low)
    .sound_acmd("sound_appealsl_knuckles", sound_knuckles_AppealSL, Low)
    .sound_acmd("sound_appealhir_knuckles", sound_knuckles_AppealHiR, Low)
    .sound_acmd("sound_appealhil_knuckles", sound_knuckles_AppealHiL, Low)
    .sound_acmd("sound_appeallwr_knuckles", sound_knuckles_AppealLwR, Low)
    .sound_acmd("sound_appeallwl_knuckles", sound_knuckles_AppealLwL, Low)
    .sound_acmd("sound_win1_knuckles", sound_knuckles_Win1, Low)
    .sound_acmd("sound_win2_knuckles", sound_knuckles_Win2, Low)
    .sound_acmd("sound_win3_knuckles", sound_knuckles_Win3, Low)
    .install();
}