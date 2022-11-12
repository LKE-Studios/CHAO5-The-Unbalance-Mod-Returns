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
use crate::custom::GIGA_GRAB;
use crate::custom::GIGA_GRABBED;
use crate::custom::GIGA_OFFSET;
use crate::custom::GIGA_GRAB_TARGET;
use crate::custom::FIGHTER_BOOL_1;
use crate::custom::FIGHTER_BOOL_2;
use crate::custom::FIGHTER_BOOL_3;
use crate::custom::{get_player_number, get_attacker_number};
use crate::custom::get_boma;
use crate::custom::jump_checker_buffer;
use crate::globals::*;
use crate::koopag::status::*;

#[fighter_frame( agent = FIGHTER_KIND_KOOPAG )]
pub fn koopag_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
		let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = smash::app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let situation_kind = StatusModule::situation_kind(module_accessor);
		let cat = fighter.global_table[CMD_CAT1].get_int() as i32;
		let cat2 = fighter.global_table[CMD_CAT2].get_int() as i32;
		let pad_flag = fighter.global_table[PAD_FLAG].get_int() as i32;
		let mut globals = fighter.globals_mut().clone();
		let GIGA_DTILT = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
		let GIGA_DASH_ATTACK = &mut FIGHTER_BOOL_2[get_player_number(module_accessor)];
		let DTILT_INPUT = &mut FIGHTER_BOOL_3[get_player_number(module_accessor)];

		if fighter_kind == FIGHTER_KIND_KOOPAG {
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
			if sv_information::is_ready_go() == false {
				GIGA_GRABBED[get_player_number(module_accessor)] = 0;
				GIGA_GRAB[get_player_number(module_accessor)] = 0;
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
			//Up Smash Canceling
			if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
				if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
					if jump_checker_buffer(module_accessor, cat) {
						if MotionModule::frame(module_accessor) <= 26.0 && MotionModule::frame(module_accessor) >= 18.0 {
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
						}
					}
				}
			}
			//Grab
			if situation_kind == SITUATION_KIND_GROUND {
				let mut pos = *PostureModule::pos(module_accessor);
				let offset = ModelModule::joint_global_offset_from_top(module_accessor, Hash40{hash: hash40("handl")}, &mut pos);		
				GIGA_OFFSET[get_player_number(module_accessor)] = Vector3f{x: PostureModule::pos_x(module_accessor) + offset.x, y: PostureModule::pos_y(module_accessor) + offset.y + 8.0, z: PostureModule::pos_z(module_accessor) + offset.z};
				
				if ((WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT) 
					|| status_kind == *FIGHTER_STATUS_KIND_WALK || CancelModule::is_enable_cancel(module_accessor)) && (ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) 
					&& ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD)) && (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) == 0) 
				|| globals["giga_guard_grab"].get_bool() {
					globals["giga_lr"] = (PostureModule::lr(module_accessor)).into();
					globals["giga_guard_grab"] = false.into();
					MotionModule::change_motion(module_accessor, Hash40 { hash: hash40("catch") }, 0.0, 1.0, false, 0.0, false, false);
				}
				if motion_kind == hash40("catch") {
					PostureModule::set_lr(module_accessor, globals["giga_lr"].get_num());
					PostureModule::update_rot_y_lr(module_accessor);
					if MotionModule::frame(module_accessor) > 38.0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
					}
					if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT) {
						AttackModule::clear_all(module_accessor);
						GIGA_GRAB[get_player_number(module_accessor)] = 2;
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
					}
				}
				if GIGA_GRAB[get_player_number(module_accessor)] == 2 {
					MotionModule::change_motion(module_accessor, Hash40{hash: hash40("catch_pull")}, 0.0, 1.0, false, 0.0, false, false);
					GIGA_GRAB[get_player_number(module_accessor)] = 1;
				}
				if motion_kind == hash40("catch_pull") {
					PostureModule::set_lr(module_accessor, globals["giga_lr"].get_num());
					PostureModule::update_rot_y_lr(module_accessor);
					if MotionModule::frame(module_accessor) >= 19.0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("catch_wait")}, 0.0, 1.0, true, 0.0, false, false);
					}
				}
				if GIGA_GRAB[get_player_number(module_accessor)] == 9 {
					MotionModule::change_motion(module_accessor, Hash40{hash: hash40("catch_cut")}, 0.0, 1.0, false, 0.0, false, false);
					GIGA_GRAB[get_player_number(module_accessor)] = 0;					
				}
				if motion_kind == hash40("catch_wait") || (motion_kind == hash40("catch_pull") && MotionModule::frame(module_accessor) >= 4.0) {
					PostureModule::set_lr(module_accessor, globals["giga_lr"].get_num());
					PostureModule::update_rot_y_lr(module_accessor);
					if GIGA_GRABBED[get_player_number(module_accessor)] == 0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("catch_cut")}, 0.0, 1.0, false, 0.0, false, false);
					}
					if (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("catch_attack")}, 0.0, 1.0, false, 0.0, false, false);
						GIGA_GRAB[get_player_number(module_accessor)] = 3;
					}
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_F) != 0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("throw_f")}, 0.0, 1.0, false, 0.0, false, false);
						GIGA_GRAB[get_player_number(module_accessor)] = 4;
					}
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B) != 0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("throw_b")}, 0.0, 1.0, false, 0.0, false, false);
						GIGA_GRAB[get_player_number(module_accessor)] = 5;
					}
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_HI) != 0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("throw_hi")}, 0.0, 1.0, false, 0.0, false, false);
						GIGA_GRAB[get_player_number(module_accessor)] = 6;
					}
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_LW) != 0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("throw_lw")}, 0.0, 1.0, false, 0.0, false, false);
						GIGA_GRAB[get_player_number(module_accessor)] = 7;
					}
				}
				if motion_kind == hash40("catch_attack") {
					if GIGA_GRABBED[get_player_number(module_accessor)] == 0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("catch_cut")}, 0.0, 1.0, false, 0.0, false, false);
					}
					if MotionModule::frame(module_accessor) >= 13.0 {
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("catch_wait")}, 0.0, 1.0, true, 0.0, false, false);
					}
				}
				if motion_kind == hash40("throw_f") {
					if MotionModule::frame(module_accessor) >= 58.0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
						GIGA_GRAB_TARGET[get_player_number(module_accessor)] = 8;
					}
				}
				if motion_kind == hash40("throw_b") {
					if MotionModule::frame(module_accessor) >= 46.0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
						GIGA_GRAB_TARGET[get_player_number(module_accessor)] = 8;
					}
				}
				if motion_kind == hash40("throw_hi") {
					if MotionModule::frame(module_accessor) >= 50.0 {
						CancelModule::enable_cancel(module_accessor);
						GIGA_GRAB_TARGET[get_player_number(module_accessor)] = 8;
					}
				}
				if motion_kind == hash40("throw_lw") {
					if MotionModule::frame(module_accessor) >= 9.0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
						GIGA_GRAB[get_player_number(module_accessor)] = 8;
					}
				}
			}
			if motion_kind != hash40("throw_lw") && motion_kind != hash40("special_lw") && motion_kind != hash40("special_air_lw") && motion_kind != hash40("special_lw_land") && GIGA_GRAB[get_player_number(module_accessor)] == 8 {
				GIGA_GRAB[get_player_number(module_accessor)] = 0;
				GIGA_GRAB_TARGET[get_player_number(module_accessor)] = 8;
			}
			if status_kind == *FIGHTER_STATUS_KIND_GUARD_ON && MotionModule::frame(module_accessor) >= 9.0 {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_GUARD, true);
			}
			if status_kind == *FIGHTER_STATUS_KIND_GUARD || status_kind == *FIGHTER_STATUS_KIND_GUARD_ON {
				if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH) || (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) 
				&& ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) && WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME) < 2 
				&& (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) == 0 {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK, true);
					globals["giga_guard_grab"] = true.into();
				}
			}
			if status_kind == *FIGHTER_STATUS_KIND_GUARD {
				MotionModule::set_frame(module_accessor, 0.0, true);
				if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
					if WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD) <= WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MIN) {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY, true);
					}
				}
			}
			if status_kind == *FIGHTER_STATUS_KIND_WIN || status_kind == *FIGHTER_STATUS_KIND_LOSE {
				MotionModule::change_motion(fighter.module_accessor, Hash40::new("wait"), 0.0, 0.0, true, 0.0, false, false);
			}
			//Enabling certain status changes
			WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_GUARD);
			WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_RUN);
			WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SLIP);
			WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_OTTOTTO);
			WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP);		
		}
	}
}

pub fn install() {
    smashline::install_agent_frames!(
        koopag_opff
    );
}