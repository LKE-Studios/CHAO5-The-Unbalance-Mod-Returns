use crate::imports::BuildImports::*;

#[acmd_script(//Win3
    agent = "ness", 
    script = "sound_win3_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_win3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_ninten_win02"));
    }
}

#[acmd_script(//Wait2
    agent = "ness", 
    script = "sound_wait2_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_wait2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 54.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal02"));
    }
    frame(fighter.lua_state_agent, 81.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal02"));
    }
}

#[acmd_script(//Wait3
    agent = "ness", 
    script = "sound_wait3_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_wait3(fighter: &mut L2CAgentBase) {}

#[acmd_script(//Attack11 
    agent = "ness", 
    script = "sound_attack11_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_swing_s"));
    }
}

#[acmd_script(//Attack12
    agent = "ness", 
    script = "sound_attack12_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_swing_m"));
    }
}

#[acmd_script(//Attack13
    agent = "ness", 
    script = "sound_attack13_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_smash_s02"));
    }
}

#[acmd_script(//AttackDash
    agent = "ness", 
    script = "sound_attackdash_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_attackdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_smash_s02"));
    }
}

#[acmd_script(//AttackHi3
    agent = "ness", 
    script = "sound_attackhi3_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_attackhi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_smash_s02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
}

#[acmd_script(//AttackLw3
    agent = "ness", 
    script = "sound_attacklw3_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_attacklw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
    }
}

#[acmd_script(//AttackS4
    agent = "ness", 
    script = "sound_attacks4_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_attacks4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
    wait(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack05"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_smash_s02"));
    }
}

#[acmd_script(//AttackHi4Charge
    agent = "ness", 
    script = "sound_attackhi4charge_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_attackhi4charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
}

#[acmd_script(//AttackHi4
    agent = "ness", 
    script = "sound_attackhi4_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_attackhi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal02"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("vc_ness_attack02"));
    }
}

#[acmd_script(//AttackLw4Charge
    agent = "ness", 
    script = "sound_attacklw4charge_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_attacklw4charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
}

#[acmd_script(//AttackLw4
    agent = "ness", 
    script = "sound_attacklw4_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_attacklw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
    }
}

#[acmd_script(//AttackAirN
    agent = "ness", 
    script = "sound_attackairn_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_attackairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    } 
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_swing_s"));
    }
}

#[acmd_script(//AttackAirF 
    agent = "ness", 
    script = "sound_attackairf_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_attackairf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_smash_s02"));
    }
}

#[acmd_script(//AttackAirB
    agent = "ness", 
    script = "sound_attackairb_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_attackairb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    } 
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_swing_m"));
    }
}

#[acmd_script(//AttackAirHi
    agent = "ness", 
    script = "sound_attackairhi_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_attackairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
    }
}

#[acmd_script(//AttackAirLw
    agent = "ness", 
    script = "sound_attackairlw_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_attackairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    } 
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_swing_m"));
    }
}

#[acmd_script(//ThrowF
    agent = "ness", 
    script = "sound_throwf_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_throwf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_down_l_01"));
    }
}

#[acmd_script(//ThrowB
    agent = "ness", 
    script = "sound_throwb_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_throwb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_down_l_01"));
    }
}

#[acmd_script(//ThrowHi
    agent = "ness", 
    script = "sound_throwhi_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_throwhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_throw_pk01"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_throw_pk02"));
    }
}      

#[acmd_script(//ThrowLw
    agent = "ness", 
    script = "sound_throwlw_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_throwlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_down_l_01"));
    }
}

#[acmd_script(//SpecialHiStart
    agent = "ness", 
    script = "sound_specialhistart_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_specialhistart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal01"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gohoubi_search_ui"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
    }
    frame(fighter.lua_state_agent, 144.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
    }
}

#[acmd_script(//SpecialAirHiStart
    agent = "ness", 
    script = "sound_specialairhistart_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_specialairhistart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal01"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gohoubi_search_ui"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
    }
    frame(fighter.lua_state_agent, 144.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
    }
}

#[acmd_script(//SpecialAirHiEnd
    agent = "ness", 
    script = "sound_specialairhiend_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_specialairhiend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gohoubi_search_ui"));
    }
}

#[acmd_script(//SpecialAirHiAttack
    agent = "ness", 
    script = "sound_specialairhiattack_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_specialairhiattack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gohoubi_search_ui"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_m"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack05"));
    }
}

#[acmd_script(//SpecialLwHit
    agent = "ness", 
    script = "sound_speciallwhit_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_speciallwhit(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_003"));
    }
}

#[acmd_script(//SpecialAirLwHit
    agent = "ness", 
    script = "sound_specialairlwhit_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_specialairlwhit(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_003"));
    }
}

#[acmd_script(//AppealHiR
    agent = "ness", 
    script = "sound_appealhir_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_appealhir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack01"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal01"));
    }
}

#[acmd_script(//AppealHiL
    agent = "ness", 
    script = "sound_appealhil_ninten", 
    category = ACMD_SOUND )]
unsafe fn sound_ninten_appealhil(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack01"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal01"));
    }
}

#[acmd_script(//AppealSR
    agent = "ness", 
    script = "sound_appealsr_ninten", 
    category = ACMD_GAME )]
unsafe fn sound_ninten_appealsr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal02"));
    }
}

#[acmd_script(//AppealSL
    agent = "ness", 
    script = "sound_appealsl_ninten", 
    category = ACMD_GAME )]
unsafe fn sound_ninten_appealsl(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal02"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_ninten_win3,
        sound_ninten_wait2,
        sound_ninten_wait3,
        sound_ninten_attack11,
        sound_ninten_attack12,
        sound_ninten_attack13,
        sound_ninten_attackdash,
        sound_ninten_attackhi3,
        sound_ninten_attacklw3,
        sound_ninten_attacks4,
        sound_ninten_attackhi4charge,
        sound_ninten_attackhi4,
        sound_ninten_attacklw4charge,
        sound_ninten_attacklw4,
        sound_ninten_attackairn,
        sound_ninten_attackairf,
        sound_ninten_attackairb,
        sound_ninten_attackairhi,
        sound_ninten_attackairlw,
        sound_ninten_throwf,
        sound_ninten_throwb,
        sound_ninten_throwhi,
        sound_ninten_throwlw,
        sound_ninten_specialhistart,
        sound_ninten_specialairhistart,
        sound_ninten_specialairhiend,
        sound_ninten_specialairhiattack,
        sound_ninten_speciallwhit,
        sound_ninten_specialairlwhit,
        sound_ninten_appealsr,
        sound_ninten_appealsl,
    );
}
