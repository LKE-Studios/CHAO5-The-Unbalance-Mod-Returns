use crate::imports::BuildImports::*;

//SpecialHiH
unsafe extern "C" fn sound_kirby_SpecialHiH(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_special_h02"));
        match sv_math::rand(hash40("fighter"), 2) {
            0 => PLAY_SE(fighter, Hash40::new("vc_kirby_002")),
            1 => PLAY_SE(fighter, Hash40::new("vc_kirby_attack07")),
            _ => PLAY_SE(fighter, Hash40::new("vc_kirby_attack07")),
        };
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_special_h04"));
    }
}

//SpecialAirHiH
unsafe extern "C" fn sound_kirby_SpecialAirHiH(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_special_h02"));
        match sv_math::rand(hash40("fighter"), 2) {
            0 => PLAY_SE(fighter, Hash40::new("vc_kirby_002")),
            1 => PLAY_SE(fighter, Hash40::new("vc_kirby_attack07")),
            _ => PLAY_SE(fighter, Hash40::new("vc_kirby_attack07")),
        };
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_special_h04"));
    }
}

pub fn install() {
    Agent::new("kirby")
    .sound_acmd("sound_specialhih", sound_kirby_SpecialHiH)
    .sound_acmd("sound_specialairhih", sound_kirby_SpecialAirHiH)
    .install();
}