use crate::imports::BuildImports::*;

//SpecialZStart
unsafe extern "C" fn sound_pfushigisou_SpecialZStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_appeal_h01"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_heal_start")); //Index 80
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_special_n01"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_recovery")); //Index 81
    }
}

//SpecialZCharge
unsafe extern "C" fn sound_pfushigisou_SpecialZCharge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
    }
    for _ in 0..8 {
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_appeal_l02"));
        }
        wait(fighter.lua_state_agent, 15.0);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
    }
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
    }
}

//SpecialZShoot
unsafe extern "C" fn sound_pfushigisou_SpecialZShoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_special_beam"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_smash_h02"));
    }
}

//SpecialZEnd
unsafe extern "C" fn sound_pfushigisou_SpecialZEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pfushigisou_appeal_l01"));
        STOP_SE(fighter, Hash40::new("se_pfushigisou_recovery"));
    }
}

//SpecialAirZStart
unsafe extern "C" fn sound_pfushigisou_SpecialAirZStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_appeal_h01"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_heal_start")); //Index 80
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_special_n01"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_recovery")); //Index 81
    }
}

//SpecialAirZCharge
unsafe extern "C" fn sound_pfushigisou_SpecialAirZCharge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
    }
    for _ in 0..8 {
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_appeal_l02"));
        }
        wait(fighter.lua_state_agent, 15.0);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
    }
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
    }
}

//SpecialAirZShoot
unsafe extern "C" fn sound_pfushigisou_SpecialAirZShoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_special_beam"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pfushigisou_smash_h02"));
    }
}

//SpecialAirZEnd
unsafe extern "C" fn sound_pfushigisou_SpecialAirZEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pfushigisou_appeal_l01"));
        STOP_SE(fighter, Hash40::new("se_pfushigisou_recovery"));
    }
}

pub fn install() {
    Agent::new("pfushigisou")
    .sound_acmd("sound_specialzstart", sound_pfushigisou_SpecialZStart, Low)
    .sound_acmd("sound_specialzcharge", sound_pfushigisou_SpecialZCharge, Low)
    .sound_acmd("sound_specialzend", sound_pfushigisou_SpecialZEnd, Low)
    .sound_acmd("sound_specialzshoot", sound_pfushigisou_SpecialZShoot, Low)
    .sound_acmd("sound_specialairzstart", sound_pfushigisou_SpecialAirZStart, Low)
    .sound_acmd("sound_specialairzcharge", sound_pfushigisou_SpecialAirZCharge, Low)
    .sound_acmd("sound_specialairzend", sound_pfushigisou_SpecialAirZEnd, Low)
    .sound_acmd("sound_specialairzshoot", sound_pfushigisou_SpecialAirZShoot, Low)
    .install();
}
