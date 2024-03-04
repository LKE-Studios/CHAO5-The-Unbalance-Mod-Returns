use crate::imports::BuildImports::*;

//AscendJump
unsafe extern "C" fn sound_link_AscendJump(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_link_jump02"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_link_special_h01"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_link_ottotto"));
    }
}

//AscendAirJump
unsafe extern "C" fn sound_link_AscendAirJump(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_link_jump02"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_link_special_h01"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_link_ottotto"));
    }
}

//AscendStart
unsafe extern "C" fn sound_link_AscendStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {        
        STOP_SE(fighter, Hash40::new("se_link_special_h01"));
        PLAY_SE(fighter, Hash40::new("se_link_special_h02"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_link_jump01"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_link_escapeair"));
    }
}

//Ascend
unsafe extern "C" fn sound_link_Ascend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {        
        PLAY_STATUS(fighter, Hash40::new("se_link_special_h03"));
    }
}

//AscendEnd
unsafe extern "C" fn sound_link_AscendEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_special_h04"));
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("vc_link_cliffcatch"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_link_jump02"));
    }
    wait(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_05"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_05"));
    }
}

//RevaliGlideLanding
unsafe extern "C" fn sound_link_RevaliGlideLanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {        
        PLAY_LANDING_SE(fighter, Hash40::new("se_link_landing02"));
    }
}

//AttachWall
unsafe extern "C" fn sound_link_AttachWall(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_link_step_left_s_ft"));
        STOP_SE(fighter, Hash40::new("se_link_step_right_s_ft"));
    }
}

//AttachWallClimb
unsafe extern "C" fn sound_link_AttachWallClimb(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_link_step_right_s_ft"));
        }
        wait(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_link_step_left_s_ft"));
        }
        wait(fighter.lua_state_agent, 5.0);
    }
}

pub fn install() {
    Agent::new("link")
    .sound_acmd("sound_ascendjump", sound_link_AscendJump)
    .sound_acmd("sound_ascendairjump", sound_link_AscendAirJump)
    .sound_acmd("sound_ascendstart", sound_link_AscendStart)
    .sound_acmd("sound_ascend", sound_link_Ascend)
    .sound_acmd("sound_ascendend", sound_link_AscendEnd)
    .sound_acmd("sound_revaliglidelanding", sound_link_RevaliGlideLanding)
    .sound_acmd("sound_attachwall", sound_link_AttachWall)
    .sound_acmd("sound_attachwallclimb", sound_link_AttachWallClimb)
    .install();
}