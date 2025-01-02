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
	let dice_num = WorkModule::get_int(fighter.module_accessor, *WEAPON_WALUIGI_DICEBLOCK_INSTANCE_WORK_ID_INT_NUMBER);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_dolly_superspecial_success"));
		if dice_num == 0 {
			PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_0_up);
		}
		else if dice_num == 1 {
			PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_2_up);
		}
		else if dice_num == 2 {
			PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_4_up);
		}
		else if dice_num == 3 {
			PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_5_up);
		}
		else if dice_num == 4 {
			PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_7_up);
		}
		else if dice_num == 5 {
			PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_9_up);
		}
		else if dice_num == 6 {
			PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_11_up);
		}
		else if dice_num == 7 {
			PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_12_up);
		}
		else if dice_num == 8 {
			PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_14_up);
		}
		else if dice_num == 9 {
			PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_16_up);
		}
	}
}

pub fn install() {
    Agent::new("dolly_diceblock")
    .sound_acmd("sound_regular", sound_waluigi_diceblock_Regular, Low)
    .sound_acmd("sound_bound", sound_waluigi_diceblock_Bound, Low)
    .sound_acmd("sound_break", sound_waluigi_diceblock_Break, Low)
    .install();
}