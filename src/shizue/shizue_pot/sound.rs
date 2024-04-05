use crate::imports::BuildImports::*;

//Burst
unsafe extern "C" fn sound_shizue_pot_Burst(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_shizue_attackdash_03"));
    }
}

pub fn install() {
    Agent::new("shizue_pot")
    .sound_acmd("sound_burst", sound_shizue_pot_Burst)
    .install();
}