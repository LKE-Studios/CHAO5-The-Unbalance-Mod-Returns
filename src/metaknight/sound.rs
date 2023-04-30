use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;

#[acmd_script(//GlideStart
    agent = "metaknight", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_metaknight_glidestart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_jump04"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_appeal_s03"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_start"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_loop"));
    }
}

#[acmd_script(//GlideAttack
    agent = "metaknight", 
    script = "sound_glideattack", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_metaknight_glideattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_attack100_03"));
    }
}

#[acmd_script(//GlideLanding
    agent = "metaknight", 
    script = "sound_glidelanding", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_metaknight_glidelanding(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_start"));
        macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_loop"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_metaknight_landing02"));
    }
}

#[acmd_script(//GlideEnd
    agent = "metaknight", 
    script = "sound_glideend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_metaknight_glideend(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 10 {//Brawl Meta Knight 
        if macros::is_excute(fighter) {
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_start"));
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_loop"));
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_metaknight_dash_turn"));
        }
    } else {
        if macros::is_excute(fighter) {
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_start"));
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_loop"));
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_metaknight_dash_stop"));
        }
    }
}    

#[acmd_script(//ThrowHi
    agent = "metaknight", 
    script = "sound_throwhi", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_metaknight_throwhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        macros::PLAY_SE(fighter, Hash40::new("vc_metaknight_win01"));
    }
    wait(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
        //macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_metaknight_rnd_attack"));
    }
    wait(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_s"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_kick_hit_m"));
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_final_hit"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
}   

#[acmd_script(//SpecialNStart
    agent = "metaknight", 
    script = "sound_specialnstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_metaknight_specialnstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_n01"));
    }
}

#[acmd_script(//SpecialAirNStart
    agent = "metaknight", 
    script = "sound_specialairnstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_metaknight_specialairnstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_n01"));
    }
}

#[acmd_script(//SpecialSStart
    agent = "metaknight", 
    script = "sound_specialsstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_metaknight_specialsstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_s01"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_jump01"));
    }
}

#[acmd_script(//SpecialHi
    agent = "metaknight", 
    script = "sound_specialhi", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_metaknight_specialhi(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 10 {//Brawl Meta Knight 
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_dash_start"));
            macros::PLAY_SE(fighter, Hash40::new("se_metaknight_special_h01"));
            macros::PLAY_STATUS(fighter, Hash40::new("vc_metaknight_special_h01"));
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_h02"));
        }
    } else {
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_dash_start"));
            macros::PLAY_SE(fighter, Hash40::new("se_metaknight_special_h01"));
            macros::PLAY_STATUS(fighter, Hash40::new("vc_metaknight_special_h01"));
        }
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_h02"));
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STATUS(fighter, Hash40::new("se_metaknight_special_h03"));
        }
    }
}    

#[acmd_script(//SpecialHiLoop
    agent = "metaknight", 
    script = "sound_specialhiloop", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_metaknight_specialhiloop(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 10 {//Brawl Meta Knight 
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_dash_start"));
            macros::PLAY_SE(fighter, Hash40::new("se_metaknight_special_h01"));
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STATUS(fighter, Hash40::new("vc_metaknight_special_h01"));
            macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_h02"));
        }
    } else {
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_dash_start"));
            macros::PLAY_SE(fighter, Hash40::new("se_metaknight_special_h01"));
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STATUS(fighter, Hash40::new("vc_metaknight_special_h01"));
            macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_h02"));
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STATUS(fighter, Hash40::new("se_metaknight_special_h03"));
        }
    }
}   

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_metaknight_glidestart,
        sound_metaknight_glideattack,
        sound_metaknight_glidelanding,
        sound_metaknight_glideend,
        sound_metaknight_throwhi,
        sound_metaknight_specialnstart,
        sound_metaknight_specialairnstart,
        sound_metaknight_specialsstart,
        sound_metaknight_specialhi,
        sound_metaknight_specialhiloop
    );
}