use crate::imports::BuildImports::*;

//Run
unsafe extern "C" fn sound_koopag_Run(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_koopag_run_loop"));
    }
}

//SpecialNStart
unsafe extern "C" fn sound_koopag_SpecialNStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_special_n01"));
    }
    wait(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_koopag_step_left_m"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_koopag_special_n03"));
    }
}

//SpecialAirNStart
unsafe extern "C" fn sound_koopag_SpecialAirNStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_special_n01"));
    }
    wait(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_koopag_special_n03"));
    }
}

//DownBoundD
unsafe extern "C" fn sound_koopag_DownBoundD(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_blowaway_s"));
        STOP_SE(fighter, Hash40::new("se_common_blowaway_m"));
        STOP_SE(fighter, Hash40::new("se_common_blowaway_l"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_m_01"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_common_down_m_01"), 0.75);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_m_02"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_common_down_m_02"), 0.75);
    }
}

//DownBoundU
unsafe extern "C" fn sound_koopag_DownBoundU(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_blowaway_s"));
        STOP_SE(fighter, Hash40::new("se_common_blowaway_m"));
        STOP_SE(fighter, Hash40::new("se_common_blowaway_l"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_m_01"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_common_down_m_01"), 0.75);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_m_02"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_common_down_m_02"), 0.75);
    }
}

//Damage
unsafe extern "C" fn sound_koopag_Damage(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGE_VC(fighter);
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_koopag_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGEFLY_VC(fighter);
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_koopag_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGEFLY_VC(fighter);
    }
}

//DamageFlyN
unsafe extern "C" fn sound_koopag_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGEFLY_VC(fighter);
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_koopag_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGEFLY_VC(fighter);
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_koopag_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGEFLY_VC(fighter);
    }
}

pub fn install() {
    Agent::new("koopag")
	.sound_acmd("sound_specialnstart", sound_koopag_SpecialNStart, Low)
    .sound_acmd("sound_specialairnstart", sound_koopag_SpecialAirNStart, Low)
    .sound_acmd("sound_run", sound_koopag_Run, Low)
    .sound_acmd("sound_downboundd", sound_koopag_DownBoundD, Low)
    .sound_acmd("sound_downboundu", sound_koopag_DownBoundU, Low)
    .sound_acmd("sound_damagen1", sound_koopag_Damage, Low)
    .sound_acmd("sound_damagen2", sound_koopag_Damage, Low)
    .sound_acmd("sound_damagen3", sound_koopag_Damage, Low)
    .sound_acmd("sound_damagehi1", sound_koopag_Damage, Low)
    .sound_acmd("sound_damagehi2", sound_koopag_Damage, Low)
    .sound_acmd("sound_damagehi3", sound_koopag_Damage, Low)
    .sound_acmd("sound_damagelw1", sound_koopag_Damage, Low)
    .sound_acmd("sound_damagelw2", sound_koopag_Damage, Low)
    .sound_acmd("sound_damagelw3", sound_koopag_Damage, Low)
    .sound_acmd("sound_damageflyhi", sound_koopag_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_koopag_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_koopag_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_koopag_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_koopag_DamageFlyRoll, Low)
    .install();
}