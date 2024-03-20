use crate::imports::BuildImports::*;

//JumpAerialF4
unsafe extern "C" fn sound_pit_JumpAerialF4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pit_jump02"));
    }
}

//JumpAerialF5
unsafe extern "C" fn sound_pit_JumpAerialF5(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pit_jump02"));
    }
}

//JumpAerialF6
unsafe extern "C" fn sound_pit_JumpAerialF6(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pit_jump02"));
    }
}

//JumpAerialF7
unsafe extern "C" fn sound_pit_JumpAerialF7(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pit_jump02"));
    }
}

//GlideStart
unsafe extern "C" fn sound_pit_GlideStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pit_jump02"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pit_glide_start"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        //PLAY_SE_REMAIN(fighter, Hash40::new("se_pit_bowsplit"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pit_glide_loop"));
    }
}

//GlideAttack
unsafe extern "C" fn sound_pit_GlideAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pit_swing_m"));
    }
}

//GlideLanding
unsafe extern "C" fn sound_pit_GlideLanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_pit_landing02"));
    }
}

//GlideEnd
unsafe extern "C" fn sound_pit_GlideEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_pit_bowsplit"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pit_jump01"));
    }
}  

//SpecialHiStart
unsafe extern "C" fn sound_pit_SpecialHiStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pit_bowsplit"));
        PLAY_SE(fighter, Hash40::new("se_pit_special_h01"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pit_special_h02"));
    }
}

//SpecialAirHiStart
unsafe extern "C" fn sound_pit_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pit_bowsplit"));
        PLAY_SE(fighter, Hash40::new("se_pit_special_h01"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pit_special_h02"));
    }
}

//SpecialHiFly
unsafe extern "C" fn sound_pit_SpecialHiFly(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_pit_jump02"));
        }
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_pit_special_h03"));
        }
        wait_loop_clear(fighter);
    }
}

pub fn install() {
    Agent::new("pit")
    .sound_acmd("sound_jumpaerialf4", sound_pit_JumpAerialF4)
    .sound_acmd("sound_jumpaerialf5", sound_pit_JumpAerialF5)
    .sound_acmd("sound_jumpaerialf6", sound_pit_JumpAerialF6)
    .sound_acmd("sound_jumpaerialf4", sound_pit_JumpAerialF7)
    .sound_acmd("sound_glidestart", sound_pit_GlideStart)
    .sound_acmd("sound_glideattack", sound_pit_GlideAttack)
    .sound_acmd("sound_glidelanding", sound_pit_GlideLanding)
    .sound_acmd("sound_glideend", sound_pit_GlideEnd)
    .sound_acmd("sound_specialhistart", sound_pit_SpecialHiStart)
    .sound_acmd("sound_specialairhistart", sound_pit_SpecialAirHiStart)
    .sound_acmd("sound_specialhifly", sound_pit_SpecialHiFly)
    .install();
}