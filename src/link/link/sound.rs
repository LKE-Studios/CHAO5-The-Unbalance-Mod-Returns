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
    loop {
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

//DamageFlyHi
unsafe extern "C" fn sound_link_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_link_rnd_futtobi01"), Hash40::new("seq_link_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_link_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_link_rnd_futtobi01"), Hash40::new("seq_link_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_link_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_link_rnd_futtobi01"), Hash40::new("seq_link_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_link_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_link_rnd_futtobi01"), Hash40::new("seq_link_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_link_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_link_rnd_futtobi01"), Hash40::new("seq_link_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("link")
    .sound_acmd("sound_ascendjump", sound_link_AscendJump, Low)
    .sound_acmd("sound_ascendairjump", sound_link_AscendAirJump, Low)
    .sound_acmd("sound_ascendstart", sound_link_AscendStart, Low)
    .sound_acmd("sound_ascend", sound_link_Ascend, Low)
    .sound_acmd("sound_ascendend", sound_link_AscendEnd, Low)
    .sound_acmd("sound_revaliglidelanding", sound_link_RevaliGlideLanding, Low)
    .sound_acmd("sound_attachwall", sound_link_AttachWall, Low)
    .sound_acmd("sound_attachwallclimb", sound_link_AttachWallClimb, Low)
    .sound_acmd("sound_damageflyhi", sound_link_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_link_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_link_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_link_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_link_DamageFlyRoll, Low)
    .install();
}