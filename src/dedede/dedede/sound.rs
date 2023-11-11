use crate::imports::BuildImports::*;

//JumpAerialF3
unsafe extern "C" fn sound_dedede_JumpAerialF3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dedede_jump02_05"));
    }
}

//JumpAerialF4
unsafe extern "C" fn sound_dedede_JumpAerialF4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dedede_jump02_05"));
    }
}

//JumpAerialF5
unsafe extern "C" fn sound_dedede_JumpAerialF5(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dedede_jump02_05"));
    }
}

//JumpAerialF6
unsafe extern "C" fn sound_dedede_JumpAerialF6(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dedede_jump02_05"));
    }
}

//JumpAerialF7
unsafe extern "C" fn sound_dedede_JumpAerialF7(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dedede_jump02_05"));
    }
}

//JumpAerialF8
unsafe extern "C" fn sound_dedede_JumpAerialF8(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dedede_jump02_05"));
    }
}

//JumpAerialF9
unsafe extern "C" fn sound_dedede_JumpAerialF9(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dedede_jump02_05"));
    }
}

pub fn install() {
    Agent::new("dedede")
    .sound_acmd("sound_jumpaerialf3", sound_dedede_JumpAerialF3)
    .sound_acmd("sound_jumpaerialf4", sound_dedede_JumpAerialF4)
    .sound_acmd("sound_jumpaerialf5", sound_dedede_JumpAerialF5)
    .sound_acmd("sound_jumpaerialf6", sound_dedede_JumpAerialF6)
    .sound_acmd("sound_jumpaerialf7", sound_dedede_JumpAerialF7)
    .sound_acmd("sound_jumpaerialf8", sound_dedede_JumpAerialF8)
    .sound_acmd("sound_jumpaerialf9", sound_dedede_JumpAerialF9)
    .install();
}