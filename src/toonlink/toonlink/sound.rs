use crate::imports::BuildImports::*;

//AttackS4Charge
unsafe extern "C" fn sound_toonlink_AttackS4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
}

//AttackHi4Charge
unsafe extern "C" fn sound_toonlink_AttackHi4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
}

//AttackLw4Charge
unsafe extern "C" fn sound_toonlink_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
}

pub fn install() {
    Agent::new("toonlink")
    .sound_acmd("sound_attacks4charge", sound_toonlink_AttackS4Charge)
    .sound_acmd("sound_attackhi4charge", sound_toonlink_AttackHi4Charge)
    .sound_acmd("sound_attacklw4charge", sound_toonlink_AttackLw4Charge)
    .install();
}