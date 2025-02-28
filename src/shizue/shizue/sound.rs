use crate::imports::BuildImports::*;

//AttackDash
unsafe extern "C" fn sound_shizue_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_attackdash_01"));
        PLAY_DAMAGE_VC(fighter);
    }
}

//AttackS3
unsafe extern "C" fn sound_shizue_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_attackhard_s01"));
        PLAY_ATTACK_VC(fighter);
    }
}

//AttackHi3
unsafe extern "C" fn sound_shizue_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_attackhard_h01"));
        PLAY_ATTACK_VC(fighter);
    }
}

//AttackLw3
unsafe extern "C" fn sound_shizue_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_attackhard_l01"));
        PLAY_ATTACK_VC(fighter);
    }
}

//AttackS4
unsafe extern "C" fn sound_shizue_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        STOP_SE(fighter, Hash40::new("se_shizue_smash_s02"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_smash_s01"));
        PLAY_SE(fighter, Hash40::new("vc_shizue_attack_heavy01"));
    }
}

//AttackHi4
unsafe extern "C" fn sound_shizue_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_smash_h01"));
        PLAY_SE(fighter, Hash40::new("vc_shizue_attack_heavy02"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_smash_h02"));
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_smash_h03"));
    }
}

//AttackLw4
unsafe extern "C" fn sound_shizue_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_smash_l01"));
        PLAY_SE(fighter, Hash40::new("vc_shizue_attack_heavy04"));
    }
}

//AttackS4Charge
unsafe extern "C" fn sound_shizue_AttackS4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
}

//AttackHi4Charge
unsafe extern "C" fn sound_shizue_AttackHi4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
}

//AttackLw4Charge
unsafe extern "C" fn sound_shizue_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_attackhard_h01"));
    }
}

//AttackAirN
unsafe extern "C" fn sound_shizue_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_attackair_n01"));
        PLAY_ATTACK_VC(fighter);
    }
}

//AttackAirF
unsafe extern "C" fn sound_shizue_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_attackair_f01"));
        PLAY_ATTACK_VC(fighter);
    }
}

//AttackAirB
unsafe extern "C" fn sound_shizue_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_attackair_b01"));
        PLAY_SE(fighter, Hash40::new("se_shizue_attackhard_s01"));
        PLAY_ATTACK_VC(fighter);
    }
}

//AttackAirHi
unsafe extern "C" fn sound_shizue_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_attackair_h02"));
        PLAY_ATTACK_VC(fighter);
    }
}

//AttackAirLw
unsafe extern "C" fn sound_shizue_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_attackair_l02"));
        PLAY_ATTACK_VC(fighter);
    }
}

//CliffCatch
unsafe extern "C" fn sound_shizue_CliffCatch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_cliff_catch"));
        PLAY_SE(fighter, Hash40::new("vc_shizue_cliffcatch"));
    }
}

//Ottotto
unsafe extern "C" fn sound_shizue_Ottotto(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        if sv_math::rand(hash40("fighter"), 4) == 0 {
            PLAY_SE(fighter, Hash40::new("vc_shizue_ottootto"));
        }
    }
}

//FuraFura
unsafe extern "C" fn sound_shizue_FuraFura(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        if sv_math::rand(hash40("fighter"), 4) == 0 {
            PLAY_SE(fighter, Hash40::new("vc_shizue_ottootto"));
        }
    }
}

//SpecialN
unsafe extern "C" fn sound_shizue_SpecialN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_attackdash_01"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_special_n02"));
    }
}

//SpecialAirN
unsafe extern "C" fn sound_shizue_SpecialAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_attackdash_01"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_special_n02"));
    }
}

//SpecialSThrowF
unsafe extern "C" fn sound_shizue_SpecialSThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_shizue_special_s08"));
        PLAY_SE(fighter, Hash40::new("se_shizue_special_s04"));
        PLAY_ATTACK_HEAVY_VC(fighter);
    }
}

//SpecialSThrowB
unsafe extern "C" fn sound_shizue_SpecialSThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_shizue_special_s08"));
        PLAY_SE(fighter, Hash40::new("se_shizue_special_s04"));
        PLAY_ATTACK_HEAVY_VC(fighter);
    }
}

//SpecialSThrowHi
unsafe extern "C" fn sound_shizue_SpecialSThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_shizue_special_s08"));
        PLAY_SE(fighter, Hash40::new("se_shizue_special_s04"));
        PLAY_ATTACK_HEAVY_VC(fighter);
    }
}

//SpecialSThrowLw
unsafe extern "C" fn sound_shizue_SpecialSThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_shizue_special_s08"));
        PLAY_SE(fighter, Hash40::new("se_shizue_special_s04"));
        PLAY_ATTACK_HEAVY_VC(fighter);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_kick_hit_m"));
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_m_01"));
    }
}

//SpecialAirSThrowF
unsafe extern "C" fn sound_shizue_SpecialAirSThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_shizue_special_s08"));
        PLAY_SE(fighter, Hash40::new("se_shizue_special_s04"));
        PLAY_ATTACK_HEAVY_VC(fighter);
    }
}

//SpecialAirSThrowB
unsafe extern "C" fn sound_shizue_SpecialAirSThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_shizue_special_s08"));
        PLAY_SE(fighter, Hash40::new("se_shizue_special_s04"));
        PLAY_ATTACK_HEAVY_VC(fighter);
    }
}

//SpecialAirSThrowHi
unsafe extern "C" fn sound_shizue_SpecialAirSThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_shizue_special_s08"));
        PLAY_SE(fighter, Hash40::new("se_shizue_special_s04"));
        PLAY_ATTACK_HEAVY_VC(fighter);
    }
}

//SpecialAirSThrowLw
unsafe extern "C" fn sound_shizue_SpecialAirSThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_shizue_special_s08"));
        PLAY_SE(fighter, Hash40::new("se_shizue_special_s04"));
        PLAY_ATTACK_HEAVY_VC(fighter);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_kick_hit_m"));
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_m_01"));
    }
}

//AppealSR
unsafe extern "C" fn sound_shizue_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        let handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_shizue_appeal_s01"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, handle as i32, 0.75, 0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_s01"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_s01"));
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_s01"));
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_s01"));
    }
}

//AppealSL
unsafe extern "C" fn sound_shizue_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        let handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_shizue_appeal_s01"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, handle as i32, 0.75, 0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_s01"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_s01"));
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_s01"));
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_s01"));
    }
}

//AppealHiR
unsafe extern "C" fn sound_shizue_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_h01"));
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        let handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_shizue_appeal_h01"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, handle as i32, 0.75, 0);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_h01"));
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_h01"));
    }
}

//AppealHiL
unsafe extern "C" fn sound_shizue_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_h01"));
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        let handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_shizue_appeal_h01"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, handle as i32, 0.75, 0);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_h01"));
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_h01"));
    }
}

//AppealLwR
unsafe extern "C" fn sound_shizue_AppealLwR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_l01"));
        PLAY_SE(fighter, Hash40::new("se_shizue_step_left_s"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_step_right_s"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_step_left_s"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_step_right_s"));
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_step_left_s"));
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_step_right_s"));
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        let handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_shizue_appeal_l01"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, handle as i32, 0.75, 0);
    }
    frame(fighter.lua_state_agent, 52.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_l02"));
    }
    frame(fighter.lua_state_agent, 69.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_shizue_landing01"));
    }
}

//AppealLwL
unsafe extern "C" fn sound_shizue_AppealLwL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_l01"));
        PLAY_SE(fighter, Hash40::new("se_shizue_step_left_s"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_step_right_s"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_step_left_s"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_step_right_s"));
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_step_left_s"));
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_step_right_s"));
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        let handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_shizue_appeal_l01"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, handle as i32, 0.75, 0);
    }
    frame(fighter.lua_state_agent, 52.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_appeal_l02"));
    }
    frame(fighter.lua_state_agent, 69.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_shizue_landing01"));
    }
}

//Win1
unsafe extern "C" fn sound_shizue_Win1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_shizue_appeal_h01_win01"));
        PLAY_SE_NO_3D(fighter, Hash40::new("se_shizue_bell01_win01"));
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_shizue_appeal_h01_win01"));
    }
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_shizue_appeal_h01_win01"));
        PLAY_SE_NO_3D(fighter, Hash40::new("se_shizue_bell02_win01"));
    }
    frame(fighter.lua_state_agent, 76.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_shizue_appeal_h01_win01"));
    }
    frame(fighter.lua_state_agent, 94.0);
    if is_excute(fighter) {
        let handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_shizue_win03"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, handle as i32, 1.2, 0);
    }
    frame(fighter.lua_state_agent, 113.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_shizue_appeal_l02_win01"));
        PLAY_SE_NO_3D(fighter, Hash40::new("se_shizue_bell03_win01"));
    }
}

//Win3
unsafe extern "C" fn sound_shizue_Win3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_shizue_win03_cloth_swish"));
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_shizue_win03_cloth_swish"));
    }
    frame(fighter.lua_state_agent, 67.0);
    if is_excute(fighter) {
        let handle = SoundModule::play_se_no3d(fighter.module_accessor, Hash40::new("vc_shizue_win02"), true, false);
        SoundModule::set_se_vol(fighter.module_accessor, handle as i32, 1.3, 0);
    }
    frame(fighter.lua_state_agent, 71.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_shizue_appeal_l02_win03"));
    }
    frame(fighter.lua_state_agent, 91.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_shizue_landing01_win03"));
    }
    frame(fighter.lua_state_agent, 97.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_shizue_appeal_l02_win03"));
    }
    frame(fighter.lua_state_agent, 118.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_shizue_landing01_win03"));
    }
}

//Damage
unsafe extern "C" fn sound_shizue_Damage(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGE_VC(fighter);
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_shizue_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGEFLY_VC(fighter);
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_shizue_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGEFLY_VC(fighter);
    }
}

//DamageFlyN
unsafe extern "C" fn sound_shizue_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGEFLY_VC(fighter);
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_shizue_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGEFLY_VC(fighter);
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_shizue_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGEFLY_VC(fighter);
    }
}

pub fn install() {
    Agent::new("shizue")
    .sound_acmd("sound_attackdash", sound_shizue_AttackDash, Low)
    .sound_acmd("sound_attacks3", sound_shizue_AttackS3, Low)
    .sound_acmd("sound_attackhi3", sound_shizue_AttackHi3, Low)
    .sound_acmd("sound_attacklw3", sound_shizue_AttackLw3, Low)
    .sound_acmd("sound_attacks4charge", sound_shizue_AttackS4Charge, Low)
    .sound_acmd("sound_attackhi4charge", sound_shizue_AttackHi4Charge, Low)
    .sound_acmd("sound_attacklw4charge", sound_shizue_AttackLw4Charge, Low)
    .sound_acmd("sound_attacks4", sound_shizue_AttackS4, Low)
    .sound_acmd("sound_attackhi4", sound_shizue_AttackHi4, Low)
    .sound_acmd("sound_attacklw4", sound_shizue_AttackLw4, Low)
    .sound_acmd("sound_attackairn", sound_shizue_AttackAirN, Low)
    .sound_acmd("sound_attackairf", sound_shizue_AttackAirF, Low)
    .sound_acmd("sound_attackairb", sound_shizue_AttackAirB, Low)
    .sound_acmd("sound_attackairhi", sound_shizue_AttackAirHi, Low)
    .sound_acmd("sound_attackairlw", sound_shizue_AttackAirLw, Low)
    .sound_acmd("sound_cliffcatch", sound_shizue_CliffCatch, Low)
    .sound_acmd("sound_ottotto", sound_shizue_Ottotto, Low)
    .sound_acmd("sound_furafura", sound_shizue_FuraFura, Low)
    .sound_acmd("sound_specialn", sound_shizue_SpecialN, Low)
    .sound_acmd("sound_specialairn", sound_shizue_SpecialAirN, Low)
    .sound_acmd("sound_specialsthrowf", sound_shizue_SpecialSThrowF, Low)
    .sound_acmd("sound_specialsthrowb", sound_shizue_SpecialSThrowB, Low)
    .sound_acmd("sound_specialsthrowhi", sound_shizue_SpecialSThrowHi, Low)
    .sound_acmd("sound_specialsthrowlw", sound_shizue_SpecialSThrowLw, Low)
    .sound_acmd("sound_specialairsthrowf", sound_shizue_SpecialAirSThrowF, Low)
    .sound_acmd("sound_specialairsthrowb", sound_shizue_SpecialAirSThrowB, Low)
    .sound_acmd("sound_specialairsthrowhi", sound_shizue_SpecialAirSThrowHi, Low)
    .sound_acmd("sound_specialairsthrowlw", sound_shizue_SpecialAirSThrowLw, Low)
    .sound_acmd("sound_appealsr", sound_shizue_AppealSR, Low)
    .sound_acmd("sound_appealsl", sound_shizue_AppealSL, Low)
    .sound_acmd("sound_appealhir", sound_shizue_AppealHiR, Low)
    .sound_acmd("sound_appealhil", sound_shizue_AppealHiL, Low)
    .sound_acmd("sound_appeallwr", sound_shizue_AppealLwR, Low)
    .sound_acmd("sound_appeallwl", sound_shizue_AppealLwL, Low)
    .sound_acmd("sound_win1", sound_shizue_Win1, Low)
    .sound_acmd("sound_win3", sound_shizue_Win3, Low)
    .sound_acmd("sound_damagen1", sound_shizue_Damage, Low)
    .sound_acmd("sound_damagen2", sound_shizue_Damage, Low)
    .sound_acmd("sound_damagen3", sound_shizue_Damage, Low)
    .sound_acmd("sound_damagehi1", sound_shizue_Damage, Low)
    .sound_acmd("sound_damagehi2", sound_shizue_Damage, Low)
    .sound_acmd("sound_damagehi3", sound_shizue_Damage, Low)
    .sound_acmd("sound_damagelw1", sound_shizue_Damage, Low)
    .sound_acmd("sound_damagelw2", sound_shizue_Damage, Low)
    .sound_acmd("sound_damagelw3", sound_shizue_Damage, Low)
    .sound_acmd("sound_damageflyhi", sound_shizue_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_shizue_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_shizue_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_shizue_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_shizue_DamageFlyRoll, Low)
    .install();
}