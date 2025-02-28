use crate::imports::BuildImports::*;

//Fire
unsafe extern "C" fn sound_sans_gaster_Fire(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_smash_s01"));
    }
}

//Final
unsafe extern "C" fn sound_sans_gaster_Final(fighter: &mut L2CAgentBase) {}

//FinalFire
unsafe extern "C" fn sound_sans_gaster_FinalFire(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_smash_s01"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_palutena_smash_s01"), -semitone_2_up);
    }
}

pub fn install() {
    Agent::new("palutena_gaster")
    .sound_acmd("sound_fire", sound_sans_gaster_Fire, Low)
    .sound_acmd("sound_final", sound_sans_gaster_Final, Low)
    .sound_acmd("sound_finalfire", sound_sans_gaster_FinalFire, Low)
    .install();
}