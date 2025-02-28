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
	shield_functions(fighter);
	//Enabling certain status changes
	WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_GUARD);
	WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_RUN);
	WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SLIP);
	WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_OTTOTTO);
	WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP);	
	Other_Function(fighter);	
}

pub unsafe extern "C" fn shield_functions(fighter : &mut L2CFighterCommon) {
	let status_kind = StatusModule::status_kind(fighter.module_accessor);
	if status_kind == *FIGHTER_STATUS_KIND_GUARD_ON && MotionModule::frame(fighter.module_accessor) >= 9.0 {
		StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD, true);
	}
	if status_kind == *FIGHTER_STATUS_KIND_GUARD {
		MotionModule::set_frame(fighter.module_accessor, 180.0, true);
		if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
			if WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD) <= WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MIN) {
				StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY, true);
			}
		}
	}
	if status_kind == *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY {
		MotionModule::change_motion(fighter.module_accessor, Hash40::new("damage_fly_top"), 0.0, 1.0, false, 0.0, false, false);
	}
}

pub unsafe extern "C" fn dtilt_input(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) 
	&& ControlModule::get_stick_y(module_accessor) <= -0.25 
	&& (
		ControlModule::get_stick_y(module_accessor) > -0.72 || ControlModule::get_flick_y(module_accessor) > 4
	) 
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
	if stage::get_stage_id() != 0x139 {
		let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
		FighterManager::set_cursor_whole(fighter_manager, false);
		if sv_information::is_ready_go() == false {
			DEAD = false;
			STOP = false;
			DECREASING = false;
			if FighterUtil::is_hp_mode(fighter.module_accessor) {
				INITIAL_STOCK_COUNT = FighterInformation::stock_count(FighterManager::get_fighter_information(fighter_manager, FighterEntryID(ENTRY_ID as i32)));
			}
		}
		if sv_information::is_ready_go() {
			DamageModule::set_reaction_mul(fighter.module_accessor, 0.0);
			DamageModule::set_reaction_mul_2nd(fighter.module_accessor, 0.0);
			DamageModule::set_reaction_mul_4th(fighter.module_accessor, 0.0);
		}
		if is_training_mode() == false {
			if DamageModule::damage(fighter.module_accessor, 0) >= 600.0 && FighterUtil::is_hp_mode(fighter.module_accessor) == false
			&& StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_DEAD {
				StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_DEAD,true);
			}
		}
		// DECREASING FOR STAMINA MODE
		if StatusModule::status_kind(fighter.module_accessor) == 470 || StatusModule::status_kind(fighter.module_accessor) == 181 {
			if FighterUtil::is_hp_mode(fighter.module_accessor) && is_training_mode() == false {
				if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_DEAD {
					if DECREASING && FighterInformation::stock_count(FighterManager::get_fighter_information(fighter_manager, FighterEntryID(ENTRY_ID as i32))) == 0 {
						StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_DEAD, true);
						INITIAL_STOCK_COUNT = 0;
						DECREASING = false;
					}
					if DECREASING && FighterInformation::stock_count(FighterManager::get_fighter_information(fighter_manager, FighterEntryID(ENTRY_ID as i32))) != 0 {
						StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_DEAD, true);
					}
					if FighterInformation::stock_count(FighterManager::get_fighter_information(fighter_manager, FighterEntryID(ENTRY_ID as i32))) < INITIAL_STOCK_COUNT {
						DECREASING = true;
					}
				}
			}
		}
		if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_DEAD && is_training_mode() == false {
			DEAD = true;
		}
		if is_training_mode() == false {
			if DEAD == true {
				if STOP == false {
					if FighterInformation::stock_count(FighterManager::get_fighter_information(fighter_manager, FighterEntryID(ENTRY_ID as i32))) != 0
					&& StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_DEAD {
						StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_DEAD, true);
					}
					if FighterInformation::stock_count(FighterManager::get_fighter_information(fighter_manager, FighterEntryID(ENTRY_ID as i32))) == 0
					&& StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_DEAD {
						StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_DEAD, true);
						STOP = true;
					}
				}
				if STOP == true {
					if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
						StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_STANDBY, true);
					}
				}
			}
		}
	}
}

pub fn install() {
    Agent::new("koopag")
    .on_line(Main, frame_koopag_Main)
    .install();
}