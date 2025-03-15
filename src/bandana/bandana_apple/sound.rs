use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn sound_bandana_apple_Fly(fighter: &mut L2CAgentBase) {}

//Haved
unsafe extern "C" fn sound_bandana_apple_Haved(fighter: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("edge_apple")
    .sound_acmd("sound_fly", sound_bandana_apple_Fly, Low)
    .sound_acmd("sound_haved", sound_bandana_apple_Haved, Low)
    .install();
}