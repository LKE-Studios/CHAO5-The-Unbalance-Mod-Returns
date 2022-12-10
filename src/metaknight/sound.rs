use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//GlideStart
    agent = "metaknight", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_glidestartsfx(fighter: &mut L2CAgentBase) {
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
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_h01"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_jump05_win02"));
    }
}

/*#[acmd_script(//GlideWing
    agent = "metaknight", 
    script = "sound_glidewing", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_glidesfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_jump05_win02"));
    }
}*/

#[acmd_script(//GlideAttack
    agent = "metaknight", 
    script = "sound_glideattack", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_glideattacksfx(fighter: &mut L2CAgentBase) {
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
unsafe fn metaknight_glidelandingsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_metaknight_special_h01"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_s"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_ss"));
    }
}

#[acmd_script(//GlideEnd
    agent = "metaknight", 
    script = "sound_glideend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_glideendsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_metaknight_special_h01"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_dash_stop"));
    }
}    

#[acmd_script(//ThrowHi
    agent = "metaknight", 
    script = "sound_throwhi", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_throwupsfx(fighter: &mut L2CAgentBase) {
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
unsafe fn metaknight_neutralbstartsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_n01"));
    }
}

#[acmd_script(//SpecialAirNStart
    agent = "metaknight", 
    script = "sound_specialairnstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_neutralbairstartsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_special_n01"));
    }
}

#[acmd_script(//SpecialSStart
    agent = "metaknight", 
    script = "sound_specialsstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_sidebstartsfx(fighter: &mut L2CAgentBase) {
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
unsafe fn metaknight_upbsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_metaknight_dash_start"));
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_smash_h03"));
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

#[acmd_script(//SpecialHiLoop
    agent = "metaknight", 
    script = "sound_specialhiloop", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_upbloopsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_metaknight_dash_start"));
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_smash_h03"));
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

#[acmd_script(//Win2
    agent = "metaknight", 
    scripts = ["sound_win2", "sound_win2_default", "sound_win2_us_en"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_win2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_jump04"));
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_jump04"));
    }
    frame(fighter.lua_state_agent, 68.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_jump05"));
    }
    frame(fighter.lua_state_agent, 75.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_metaknight_win02"));
    }
    frame(fighter.lua_state_agent, 89.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_landing02"));
    }
    frame(fighter.lua_state_agent, 124.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_appeal_l01"));
    }
}


pub fn install() {
    smashline::install_acmd_scripts!(
        metaknight_glidestartsfx,
        //metaknight_glidesfx,
        metaknight_glideattacksfx,
        metaknight_glidelandingsfx,
        metaknight_glideendsfx,
        metaknight_throwupsfx,
        metaknight_neutralbstartsfx,
        metaknight_neutralbairstartsfx,
        metaknight_sidebstartsfx,
        metaknight_upbsfx,
        metaknight_upbloopsfx,
        metaknight_win2
    );
}