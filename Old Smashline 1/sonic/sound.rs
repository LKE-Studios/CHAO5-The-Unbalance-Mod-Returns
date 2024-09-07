use crate::imports::BuildImports::*;

#[acmd_script(//Attack12
    agent = "sonic", 
    script = "sound_attack12", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_sonic_attack12(fighter: &mut L2CAgentBase) {
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_swing_m"));
        }
    }
    else {//Sonic
		frame(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_swing_m"));
        }
    }
}

#[acmd_script(//Attack13
    agent = "sonic", 
    script = "sound_attack13", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_sonic_attack13(fighter: &mut L2CAgentBase) {
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 7.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
        }
    } else {//Sonic
		    frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
            PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
        }
    }
}

#[acmd_script(//AttackDash
    agent = "sonic", 
    script = "sound_attackdash", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_sonic_attackdash(fighter: &mut L2CAgentBase) {
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            PLAY_SEQUENCE(fighter, Hash40::new("se_sonic_landing01"));
        }
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_swing_ll"));
        }
		frame(fighter.lua_state_agent, 27.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
        }
    } else {//Sonic
		frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
        }
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_swing_ll"));
        }
        frame(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_step_right_m"));
        }
        frame(fighter.lua_state_agent, 30.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_step_left_m"));
        }
    }
}

#[acmd_script(//AttackS4Charge
    agent = "sonic", 
    script = "sound_attacks4charge", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_sonic_attacks4charge(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
        }
    } else {
        frame(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
            PLAY_STATUS(fighter, Hash40::new("se_sonic_smash_s01"));
        }
        frame(fighter.lua_state_agent, 61.0);
        if is_excute(fighter) {
            STOP_SE(fighter, Hash40::new("se_sonic_smash_s01"));
        }
    }
}

#[acmd_script(//AttackHi4
    agent = "sonic", 
    script = "sound_attackhi4", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_sonic_attackhi4(fighter: &mut L2CAgentBase) {
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        if is_excute(fighter) {
            STOP_SE(fighter, Hash40::new_raw(0x13589f8893));
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("vc_sonic_attack06"));
            PLAY_SE(fighter, Hash40::new("se_sonic_swing_ll"));
        }
        wait(fighter.lua_state_agent, 28.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
        }
    } else { 
        if is_excute(fighter) {
            STOP_SE(fighter, Hash40::new_raw(0x13589f8893));
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("vc_sonic_attack06"));
            PLAY_SE(fighter, Hash40::new("se_sonic_smash_h01"));
        }
        wait(fighter.lua_state_agent, 19.0);
        if is_excute(fighter) {
            STOP_SE(fighter, Hash40::new("se_sonic_smash_h01"));
        }
        wait(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
        }
    }
}

#[acmd_script(//AttackLw4
    agent = "sonic", 
    script = "sound_attacklw4", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_sonic_attacklw4(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        }
        wait(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("vc_sonic_attack07"));
            PLAY_STATUS(fighter, Hash40::new("se_sonic_smash_h01"));
        }
        frame(fighter.lua_state_agent, 30.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        }
        wait(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("vc_sonic_attack07"));
            PLAY_STATUS(fighter, Hash40::new("se_sonic_smash_l01"));
        }
        wait(fighter.lua_state_agent, 41.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
        }
    }
}

#[acmd_script(//AttackAirN
    agent = "sonic", 
    script = "sound_attackairn", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_sonic_attackairn(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_swing_m"));
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
            PLAY_SE(fighter, Hash40::new("se_sonic_attackair_n01"));
        }
    }
}

#[acmd_script(//AttackAirF
    agent = "sonic", 
    script = "sound_attackairf", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_sonic_attackairf(fighter: &mut L2CAgentBase) { 
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
            PLAY_SE(fighter, Hash40::new("se_sonic_attackair_f01"));
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
                PLAY_SE(fighter, Hash40::new("se_sonic_attackair_f02"));
            }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_attackair_f03"));
        }
    }
}

#[acmd_script(//AttackAirB
    agent = "sonic", 
    script = "sound_attackairb", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_sonic_attackairb(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
            PLAY_SE(fighter, Hash40::new("se_sonic_swing_m"));
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
            PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
        }
    }
}

#[acmd_script(//AttackAirHi
    agent = "sonic", 
    script = "sound_attackairhi", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_sonic_attackairhi(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_swing_m"));
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_swing_s"));
        }
        wait(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
           PLAY_SE(fighter, Hash40::new("se_sonic_swing_s"));
        }
        wait(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_attackair_h01"));
        }
    }
}

#[acmd_script(//AttackAirLw
    agent = "sonic", 
    script = "sound_attackairlw", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_sonic_attackairlw(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles 
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
        }
    } else {//Sonic
        if is_excute(fighter) {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
        }
        frame(fighter.lua_state_agent, 16.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_swing_m"));
        }
        wait(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_sonic_attackair_l01"));
        }
    }
}

#[acmd_script(//ThrowHi
    agent = "sonic", 
    script = "sound_throwhi", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_sonic_throwhi(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {
        frame(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
        }
        frame(fighter.lua_state_agent, THROWHI_FRAME_LAND + 1.0);
        if is_excute(fighter) {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
        }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        }
    } else {
        frame(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
        }
        wait(fighter.lua_state_agent, 18.0);
        if is_excute(fighter) {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
        }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        }
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_sonic_attack12,
        sound_sonic_attack13,
        sound_sonic_attackdash,
        sound_sonic_attacks4charge,
        sound_sonic_attackhi4,
        sound_sonic_attacklw4,
        sound_sonic_attackairn,
        sound_sonic_attackairf,
        sound_sonic_attackairb,
        sound_sonic_attackairhi,
        sound_sonic_attackairlw,
        sound_sonic_throwhi
    );
}