use crate::imports::BuildImports::*;

//Move
unsafe extern "C" fn effect_kamek_beam_Regular(fighter: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("ness_pinkmagic")
    .effect_acmd("effect_finalcutterregular", effect_kamek_beam_Regular, Low)
    .install();
}