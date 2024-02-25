use crate::imports::BuildImports::*;

//SpecialLwLoop
unsafe extern "C" fn sound_cloud_SpecialLwLoop(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
        }
        wait(fighter.lua_state_agent, 9.0);
        wait_loop_clear(fighter);
    }
}

//SpecialAirLwLoop
unsafe extern "C" fn sound_cloud_SpecialAirLwLoop(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
        }
        wait(fighter.lua_state_agent, 9.0);
        wait_loop_clear(fighter);
    }
}

pub fn install() {
    Agent::new("cloud")
    .sound_acmd("sound_speciallwloop", sound_cloud_SpecialLwLoop)
    .sound_acmd("sound_specialairlwloop", sound_cloud_SpecialAirLwLoop)
    .install();
}
