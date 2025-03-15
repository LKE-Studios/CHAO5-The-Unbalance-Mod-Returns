use crate::imports::BuildImports::*;

//Fly
unsafe extern "C" fn effect_bandana_apple_Fly(fighter: &mut L2CAgentBase) {}

//Haved
unsafe extern "C" fn effect_bandana_apple_Haved(fighter: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("edge_apple")
    .effect_acmd("effect_fly", effect_bandana_apple_Fly, Low)
    .effect_acmd("effect_haved", effect_bandana_apple_Haved, Low)
    .install();
}