use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
use smash::hash40;
use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smash::lib::L2CValue;
use smash::phx::Vector3f;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::{sv_information};
use smash::lib::{L2CValueType::*, L2CValueType, L2CAgent, L2CTable, L2CTable_meta, L2CInnerFunctionBase, L2CValueInner};
use crate::common::FIGHTER_BOOL_1;
use crate::common::FIGHTER_BOOL_2;
use crate::common::FIGHTER_BOOL_3;
use crate::common::{get_player_number};
use crate::globals::*;
use crate::koopag::status::*;

#[fighter_frame( agent = FIGHTER_KIND_KOOPAG )]
pub fn koopag_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
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
		if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
			if dtilt_input(module_accessor) {
				*DTILT_INPUT = true;
				globals["giga_buffer_timer"] = 6.0.into();
			}
			else {
				*DTILT_INPUT = false;
			}
		}
		if ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
			if globals["giga_buffer_timer"].get_num() > 0.0 {
				globals["giga_buffer_timer"] = (globals["giga_buffer_timer"].get_num() - 1.0).into();
			}
			else {
				*DTILT_INPUT = false;
			}
		}
		if situation_kind == SITUATION_KIND_GROUND {
			if *DTILT_INPUT && WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3) 
			&& motion_kind != hash40("attack_lw3") 
			&& status_kind != *FIGHTER_STATUS_KIND_ATTACK_S3 
			&& motion_kind != hash40("rebound") {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
				ControlModule::clear_command(module_accessor, true);
				*GIGA_DTILT = true;
			}
		}
		if status_kind == *FIGHTER_STATUS_KIND_RUN {
			if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
				StatusModule::change_status_force(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
				*GIGA_DASH_ATTACK = true;
			}
		}
		if motion_kind == hash40("attack_dash") {
			PostureModule::update_rot_y_lr(module_accessor);
			if MotionModule::frame(module_accessor) >= 58.0 {
				if *DTILT_INPUT {
					MotionModule::change_motion(module_accessor, Hash40{hash: hash40("attack_lw3")}, 0.0, 1.0, false, 0.0, false, false);
					*DTILT_INPUT = false;	
				}
				else {
					if ControlModule::get_stick_y(module_accessor) < -0.66 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("squat")}, 0.0, 1.0, false, 0.0, false, false);
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SQUAT, true);							
					}
					else {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("wait")}, 0.0, 1.0, false, 0.0, false, false);
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
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
		//Shield
		if status_kind == *FIGHTER_STATUS_KIND_GUARD_ON && MotionModule::frame(module_accessor) >= 9.0 {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_GUARD, true);
		}
		if status_kind == *FIGHTER_STATUS_KIND_GUARD {
			MotionModule::set_frame(module_accessor, 180.0, true);
			if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
				if WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD) <= WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MIN) {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY, true);
				}
			}
		}
		if status_kind == *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY {
			MotionModule::change_motion(fighter.module_accessor, Hash40::new("damage_fly_top"), 0.0, 1.0, false, 0.0, false, false);
		}
		if status_kind == *FIGHTER_STATUS_KIND_WIN || status_kind == *FIGHTER_STATUS_KIND_LOSE {
			MotionModule::change_motion(fighter.module_accessor, Hash40::new("wait"), 0.0, 1.0, true, 0.0, false, false);
		}
		//Enabling certain status changes
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_GUARD);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_RUN);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SLIP);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_OTTOTTO);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP);		
	}
}

pub fn install() {
    smashline::install_agent_frames!(
        koopag_opff
    );
}