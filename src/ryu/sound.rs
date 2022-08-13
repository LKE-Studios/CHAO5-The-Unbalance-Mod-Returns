use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::Hash40;
use smash_script::*;
use smashline::*;

#[acmd_script(//MoveSpM
    agent = "ryu_hadoken",
    script = "sound_movespm",
    category = ACMD_SOUND,
    low_priority )]
unsafe fn sound_hadoken1(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ryu_special_n04"));
    }
}

#[acmd_script(//MoveSpS
    agent = "ryu_hadoken",
    script = "sound_movesps",
    category = ACMD_SOUND,
    low_priority )]
unsafe fn sound_hadoken2(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ryu_special_n04"));
    }
}

#[acmd_script(//MoveSpW
    agent = "ryu_hadoken",
    script = "sound_movespw",
    category = ACMD_SOUND,
    low_priority )]
unsafe fn sound_hadoken3(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ryu_special_n04"));
    }
}

#[acmd_script(//MoveSpW
    agent = "ryu_hadoken",
    script = "sound_movespw",
    category = ACMD_SOUND,
    low_priority )]
unsafe fn sound_hadoken4(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ryu_special_n04"));
    }
}

#[acmd_script(//SpecialN, SpecialAirN
    agent = "ryu",
    scripts = ["sound_specialn", "sound_specialairn"],
    category = ACMD_SOUND,
    low_priority )]
unsafe fn sound_neutralb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) == true {
        wait(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_ryu_swing_m"));
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("vc_ryu_special_n03"));
        } 
    }
    else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) == false {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_ryu_special_n04"));
        }
        wait(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_ryu_swing_m"));
        } 
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("vc_ryu_special_n01"));
        } 
    }
    else {
        if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_TYPE) == 0 {
            if macros::is_excute(fighter) {
                macros::PLAY_SE(fighter, Hash40::new("se_ryu_command_success"));
                macros::PLAY_SE(fighter, Hash40::new("se_ryu_special_n04"));
            }
            wait(fighter.lua_state_agent, 10.0);
            if macros::is_excute(fighter) {
                macros::PLAY_SE(fighter, Hash40::new("se_ryu_swing_m"));
            }
            wait(fighter.lua_state_agent, 2.0);
            if macros::is_excute(fighter) {
                macros::PLAY_SE(fighter, Hash40::new("vc_ryu_special_n01_command"));
            } 
        }
        else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_TYPE) == 1 {
            if macros::is_excute(fighter) {
                macros::PLAY_SE(fighter, Hash40::new("se_ryu_command_success"));
                macros::PLAY_SE(fighter, Hash40::new("se_ryu_special_n04"));
            }
            wait(fighter.lua_state_agent, 3.0);
            if macros::is_excute(fighter) {
                macros::PLAY_SE(fighter, Hash40::new("vc_ryu_special_n02_command"));
            }
            wait(fighter.lua_state_agent, 6.0);
            if macros::is_excute(fighter) {
                macros::PLAY_SE(fighter, Hash40::new("se_ryu_swing_m"));
            }
        }
    }
}

#[acmd_script(
agent = "ryu",
script = "sound_kamehameha_start",
category = ACMD_SOUND,
low_priority )]
unsafe fn sound_ryu_kamehameha_start(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_ryu_special_n01"));
    }
}

#[acmd_script(
agent = "ryu",
script = "sound_kamehameha_charge",
category = ACMD_SOUND,
low_priority )]
unsafe fn sound_ryu_kamehameha_charge(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ryu_special_n02"));
    }
}

#[acmd_script(
agent = "ryu",
script = "sound_kamehameha_fire",
category = ACMD_SOUND,
low_priority )]
unsafe fn sound_ryu_kamehameha_fire(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ryu_special_n03"));
    }
    frame(fighter.lua_state_agent, 70.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_ryu_special_n03"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_hadoken1,
        sound_hadoken2,
        sound_hadoken3,
        sound_hadoken4,
        sound_neutralb,
        //sound_ryu_kamehameha_start,
        //sound_ryu_kamehameha_charge,
        //sound_ryu_kamehameha_fire
    );
}