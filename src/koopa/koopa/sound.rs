use crate::imports::BuildImports::*;

//AttackLw3
unsafe extern "C" fn sound_koopa_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_l01"));
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_l02"));
    }
}

//AttackAirLw 
unsafe extern "C" fn sound_koopa_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_koopa_attackair_l01"));
    }
}

pub fn install() {
    Agent::new("koopa")
    .sound_acmd("sound_attacklw3", sound_koopa_AttackLw3)
    .sound_acmd("sound_attackairlw", sound_koopa_AttackAirLw)
    .install();
}