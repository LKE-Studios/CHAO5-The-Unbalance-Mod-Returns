use crate::imports::BuildImports::*;

pub static mut DEAD : bool = false;
pub static mut STOP : bool = false;
pub static mut ENTRY_ID : usize = 0;
pub static mut DECREASING : bool = false;
pub static mut INITIAL_STOCK_COUNT : u64 = 0;

pub unsafe extern "C" fn frame_koopag_Main(fighter : &mut L2CFighterCommon) {
	let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	let motion_kind = MotionModule::motion_kind(module_accessor);
	let status_kind = StatusModule::status_kind(fighter.module_accessor);
	let situation_kind = StatusModule::situation_kind(module_accessor);
	let mut globals = fighter.globals_mut().clone();
	let GIGA_DTILT = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
	let GIGA_DASH_ATTACK = &mut FIGHTER_BOOL_2[get_player_number(module_accessor)];
	let DTILT_INPUT = &mut FIGHTER_BOOL_3[get_player_number(module_accessor)];
	if let L2CValueType::Void = globals["giga_globals_set"].val_type {
		globals["giga_buffer_timer"] = 0.0.into();
		*GIGA_DTILT = false;
		*GIGA_DASH_ATTACK = false;
		globals["giga_dash"] = false.into();
		globals["giga_neutral_b"] = false.into();
		globals["giga_side_b"] = false.into();
		globals["giga_lr"] = 0.0.into();
		globals["giga_guard_grab"] = false.into();
		globals["giga_hitlag"] = false.into();
		globals["giga_gravity"] = 0.0.into();
		globals["giga_situation"] = 0.into();
		globals["giga_globals_set"] = true.into();
	}
	if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
		if dtilt_input(module_accessor) {
			*DTILT_INPUT = true;
			globals["giga_buffer_timer"] = 6.0.into();
		}
		else {
			*DTILT_INPUT = false;
		}
	}
	if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
		if globals["giga_buffer_timer"].get_num() > 0.0 {
			globals["giga_buffer_timer"] = (globals["giga_buffer_timer"].get_num() - 1.0).into();
		}
		else {
			*DTILT_INPUT = false;
		}
	}
	if situation_kind == SITUATION_KIND_GROUND {
		if *DTILT_INPUT && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3) 
		&& motion_kind != hash40("attack_lw3") 
		&& status_kind != *FIGHTER_STATUS_KIND_ATTACK_S3 
		&& motion_kind != hash40("rebound") {
			StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
			ControlModule::clear_command(module_accessor, true);
			*GIGA_DTILT = true;
		}
	}
	if status_kind == *FIGHTER_STATUS_KIND_RUN {
		if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
			StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
			StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
			*GIGA_DASH_ATTACK = true;
		}
	}
	if motion_kind == hash40("attack_dash") {
		PostureModule::update_rot_y_lr(fighter.module_accessor);
		if MotionModule::frame(fighter.module_accessor) >= 58.0 {
			if *DTILT_INPUT {
				MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw3"), 0.0, 1.0, false, 0.0, false, false);
				*DTILT_INPUT = false;	
			}
			else {
				if ControlModule::get_stick_y(fighter.module_accessor) < -0.66 {
					MotionModule::change_motion(fighter.module_accessor, Hash40::new("squat"), 0.0, 1.0, false, 0.0, false, false);
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SQUAT, true);							
				}
				else {
					MotionModule::change_motion(fighter.module_accessor, Hash40::new("wait"), 0.0, 1.0, false, 0.0, false, false);
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
				}
			}
		}
	}
	if motion_kind == hash40("attack_lw3") {
		PostureModule::update_rot_y_lr(fighter.module_accessor);
		if MotionModule::frame(fighter.module_accessor) >= 36.0 {
			if *DTILT_INPUT {
				MotionModule::change_motion(fighter.module_accessor, Hash40{hash: hash40("attack_lw3")}, 0.0, 1.0, false, 0.0, false, false);
				*DTILT_INPUT = false;
			}
			else {
				if ControlModule::get_stick_y(fighter.module_accessor) < -0.66 {
					MotionModule::change_motion(fighter.module_accessor, Hash40{hash: hash40("squat")}, 0.0, 1.0, false, 0.0, false, false);
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SQUAT, true);							
				}
				else {
					MotionModule::change_motion(fighter.module_accessor, Hash40{hash: hash40("wait")}, 0.0, 1.0, false, 0.0, false, false);
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
				}
				CancelModule::enable_cancel(fighter.module_accessor);
			}
		}
	}
	//Allows Air Neutral and Side B to cancel on landing (and prevents the animations from restarting on landing)
	if motion_kind == hash40("special_s_catch") || motion_kind == hash40("special_s_air_catch") {
		if motion_kind == hash40("special_s_air_catch") {
			globals["giga_side_b"] = true.into();
		}
		if motion_kind == hash40("special_s_catch") && globals["giga_side_b"].get_bool() {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING, true);
		}
	}
	else {
		globals["giga_side_b"] = false.into();
	}
	if motion_kind == hash40("special_air_n_start") || motion_kind == hash40("special_n_start") {
		if motion_kind == hash40("special_air_n_start") {
			globals["giga_neutral_b"] = true.into();
		}
		if motion_kind == hash40("special_n_start") && globals["giga_neutral_b"].get_bool() {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING, true);
		}
	}
	else {
		globals["giga_neutral_b"] = false.into();
	}
	//Enabling certain status changes
	WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_GUARD);
	WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_RUN);
	WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SLIP);
	WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_OTTOTTO);
	WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP);	
	Other_Function(fighter);	
}

pub unsafe extern "C" fn dtilt_input(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	let stick_y = ControlModule::get_stick_y(module_accessor);
	let flick_y = ControlModule::get_flick_y(module_accessor);
	if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) 
	&& stick_y <= -0.25 && (stick_y > -0.72 || flick_y > 4) 
	&& ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_GUARD) 
	&& ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
		return true;
	}
	else {
		return false;
	}
}

unsafe fn Other_Function(fighter: &mut L2CFighterCommon) {
	ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let fighter_info = FighterManager::get_fighter_information(FighterManager(), FighterEntryID(ENTRY_ID as i32));
	let status_kind = StatusModule::status_kind(fighter.module_accessor);
	if FighterManager::is_final(FighterManager()) {
		WorkModule::enable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STATUS);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_EFFECT_SCREEN_NO_FINAL_BG);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_EFFECT_SCREEN_NO_FINAL_FLASH);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
		FighterManager::set_visible_finalbg(FighterManager(), false);
	}
	let hit_point = FighterInformation::hit_point(fighter_info);
	let stock_count = FighterInformation::stock_count(fighter_info);
	if FighterUtil::is_hp_mode(fighter.module_accessor) && !is_training_mode() {
		if hit_point <= 0.0 && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES) {
			fighter.change_status(FIGHTER_STATUS_KIND_DEAD.into(), false.into());
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES);
		}
		if status_kind != *FIGHTER_STATUS_KIND_DEAD && stock_count == 1 {
			if hit_point <= 0.0 {
				fighter.change_status(FIGHTER_STATUS_KIND_DEAD.into(), false.into());
			}
		}
	}
}

pub unsafe extern "C" fn frame_koopag_breath_Main(weapon : &mut L2CFighterBase) {
	let status_kind = StatusModule::status_kind(weapon.module_accessor);
	if status_kind == *WEAPON_KOOPA_BREATH_STATUS_KIND_MOVE {
		let pos = PostureModule::pos(weapon.module_accessor);
		if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) 
		|| AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
			EffectModule::req(weapon.module_accessor, Hash40::new("brave_fire2_hit"), pos, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 2.0, 0, -1, false, 0);
			SoundModule::play_se(weapon.module_accessor, Hash40::new("se_common_bomb_m"), true, false, false, false, enSEType(0));
			SoundModule::play_se(weapon.module_accessor, Hash40::new("se_koopag_fireball_impact01"), true, false, false, false, enSEType(0));
			notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
		}
	}
}

pub fn install() {
    Agent::new("koopag")
    .on_line(Main, frame_koopag_Main)
    .install();

	Agent::new("koopag_breath")
    .on_line(Main, frame_koopag_breath_Main)
    .install();
}