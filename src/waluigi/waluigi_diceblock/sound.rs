use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn sound_waluigi_diceblock_Regular(fighter: &mut L2CAgentBase) {}

//Bound
unsafe extern "C" fn sound_waluigi_diceblock_Bound(fighter: &mut L2CAgentBase) {
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_common_landing_glass"));
	}
}

//Break
unsafe extern "C" fn sound_waluigi_diceblock_Break(fighter: &mut L2CAgentBase) {
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_dolly_superspecial_success"));
	}
}

pub fn install() {
    Agent::new("dolly_diceblock")
    .sound_acmd("sound_regular", sound_waluigi_diceblock_Regular, Low)
    .sound_acmd("sound_bound", sound_waluigi_diceblock_Bound, Low)
    .sound_acmd("sound_break", sound_waluigi_diceblock_Break, Low)
    .install();
}