use crate::imports::BuildImports::*;

//AppealHiL
unsafe extern "C" fn sound_littlemac_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_littlemac_appeal04"));
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_landing03"));
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_swing_m"));
    }
}

//AppealHiR
unsafe extern "C" fn sound_littlemac_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_littlemac_appeal04"));
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_landing03"));
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_swing_m"));
    }
}

//AppealLwR
unsafe extern "C" fn sound_littlemac_AppealLwR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_littlemac_appeal06"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("vc_littlemac_appeal03"));
	}
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_swing_m"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_smash_s02"));
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_swing_m"));
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_special_l01"));
    }
}

//AppealLwL
unsafe extern "C" fn sound_littlemac_AppealLwL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_littlemac_appeal06"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("vc_littlemac_appeal03"));
	}
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_swing_m"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_smash_s02"));
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_swing_m"));
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_special_l01"));
    }
}

//AppealSR
unsafe extern "C" fn sound_littlemac_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_littlemac_appeal05"));
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_appeal_s01"));
    }
    frame(fighter.lua_state_agent, 63.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_appeal_s02"));
    }
}

//AppealSL
unsafe extern "C" fn sound_littlemac_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_littlemac_appeal05"));
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_appeal_s01"));
    }
    frame(fighter.lua_state_agent, 63.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_littlemac_appeal_s02"));
    }
}

pub fn install() {
    Agent::new("littlemac")
    .sound_acmd("sound_appealhir", sound_littlemac_AppealHiR, Low)
    .sound_acmd("sound_appealhil", sound_littlemac_AppealHiL, Low)
    .sound_acmd("sound_appeallwr", sound_littlemac_AppealLwR, Low)
    .sound_acmd("sound_appeallwl", sound_littlemac_AppealLwL, Low)
    .sound_acmd("sound_appealsr", sound_littlemac_AppealSR, Low)
    .sound_acmd("sound_appealsl", sound_littlemac_AppealSL, Low)
    .install();
}