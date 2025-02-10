use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn sound_funky_boot_Regular(fighter: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("donkey_boot")
    .sound_acmd("sound_regular", sound_funky_boot_Regular, Low)
    .install();
}