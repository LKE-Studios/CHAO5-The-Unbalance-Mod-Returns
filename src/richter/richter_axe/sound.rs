use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn sound_richter_axe_Fly(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_richter_special_n02"), 0.75);
        PLAY_SE(fighter, Hash40::new("se_richter_special_n02"));
    }
}

pub fn install() {
    Agent::new("richter_axe")
    .game_acmd("sound_fly", sound_richter_axe_Fly)
    .install();
}