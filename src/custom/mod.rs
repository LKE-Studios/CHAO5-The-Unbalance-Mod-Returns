use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon /*, L2CFighterBase*/};
use smash_script::*;
use smashline::*;
use smash::hash40;
use smash::phx::Hash40;
//use smash::lua2cpp::L2CAgentBase;
use smash::phx::Vector3f;
use smash::phx::Vector2f;
use smash::lib::L2CValue;
use std::mem;

pub static mut FIGHTER_BOOL_1: [bool;9] = [false;9];
pub static mut FIGHTER_BOOL_2: [bool;9] = [false;9];
pub static mut FIGHTER_BOOL_3: [bool;9] = [false;9];

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let mut globals = fighter.globals_mut().clone();
		let situation_kind = StatusModule::situation_kind(boma);
        let motion_kind = MotionModule::motion_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma) as i32;
		let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
		let fighter_kinetic_energy_gravity = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyGravity>(KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY));
        let attacker_module_accessor = &mut *smash::app::sv_battle_object::module_accessor((WorkModule::get_int(boma, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_OBJECT_ID)) as u32);

        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
        if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
        }
        if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&status_kind) {
            if GroundModule::is_passable_ground(fighter.module_accessor) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) && stick_y <= 0.2 {
                    GroundModule::pass_floor(fighter.module_accessor);
                }
            }
            else {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                    macros::EFFECT(fighter, Hash40::new("sys_kusudama"), Hash40::new("top"), 0, 28, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
                }
            }
        };
        if MotionModule::motion_kind(boma) == hash40("attack_dash") && MotionModule::frame(boma) <= (9.0) {
            if stick_y > 0.45 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }
        }; 
        FIGHTER_BOOL_1[get_player_number(boma)] = false;
        FIGHTER_BOOL_2[get_player_number(boma)] = false;
        FIGHTER_BOOL_3[get_player_number(boma)] = false;
        koopag_stuffs(boma, attacker_module_accessor, fighter_kind, motion_kind, status_kind, situation_kind, fighter_kinetic_energy_motion, fighter_kinetic_energy_gravity, &mut globals, fighter);
    }
}

#[skyline::hook(replace=smash::app::FighterUtil::is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    return true;
}

// Use this for general per-frame weapon-level hooks
// #[weapon_frame_callback]
// pub fn global_weapon_frame(fighter_base : &mut L2CFighterBase) {
//     unsafe {
//         let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
//         let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;

//         if frame % 10 == 0 {
//             println!("[Weapon Hook] Frame : {}", frame);
//         }
//     }
// }

pub unsafe fn get_boma(entry_id: i32) -> *mut smash::app::BattleObjectModuleAccessor {
	let boma = smash::app::sv_battle_object::module_accessor(smash::app::Fighter::get_id_from_entry_id(entry_id));
	return boma;
}

pub unsafe fn get_player_number(module_accessor:  &mut smash::app::BattleObjectModuleAccessor) -> usize {
	let mut player_number = 8;
	if smash::app::utility::get_kind(module_accessor) == *WEAPON_KIND_PTRAINER_PTRAINER {
		player_number = WorkModule::get_int(module_accessor, *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_FIGHTER_ENTRY_ID) as usize;
	}
	else if smash::app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		player_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	}
	else {
		let mut owner_module_accessor = &mut *smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		while smash::app::utility::get_category(owner_module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER { //Keep checking the owner of the boma we're working with until we've hit a boma that belongs to a fighter
			owner_module_accessor = &mut *smash::app::sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		}
		player_number = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	}
	return player_number;
}

pub unsafe fn get_attacker_number(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> usize {
	let attacker_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ATTACKER_COLOR) as usize;
	return attacker_number;
}

pub unsafe fn jump_checker_buffer(module_accessor: &mut smash::app::BattleObjectModuleAccessor, cat: i32) -> bool {
	if (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 && ControlModule::is_enable_flick_jump(module_accessor){
		return true;
	}
	else if (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
		return true;
	}
	else {
		return false;
	}
}

pub static mut GIGA_GRAB: [i32;9] = [0;9]; //Tracks what phase of a grab or what throw Giga Bowser is in
pub static mut GIGA_GRABBED: [i32;9] = [0;9]; //Tracks remaining frames before the opponent is grab released
pub static mut GIGA_GRAB_TARGET: [usize;9] = [8;9];
pub static mut GIGA_POS: [Vector2f;9] = [Vector2f{x:0.0, y:0.0};9];
pub static mut GIGA_OFFSET: [Vector3f;9] = [Vector3f{x:0.0, y:0.0, z: 0.0};9];

unsafe fn koopag_stuffs(module_accessor: &mut smash::app::BattleObjectModuleAccessor, attacker_module_accessor: &mut smash::app::BattleObjectModuleAccessor, 
    fighter_kind: i32, motion_kind: u64, status_kind: i32, situation_kind: i32, fighter_kinetic_energy_motion: &mut smash::app::FighterKineticEnergyMotion, 
    fighter_kinetic_energy_gravity: &mut smash::app::FighterKineticEnergyGravity, globals: &mut L2CValue, fighter: &mut L2CFighterCommon) {
    if GIGA_GRAB_TARGET[get_attacker_number(module_accessor)] == get_player_number(module_accessor) {
        if GIGA_GRAB[GIGA_GRAB_TARGET[get_player_number(module_accessor)]] == 1 && motion_kind != hash40("capture_wait_hi") {
            MotionModule::change_motion(module_accessor, Hash40{hash: hash40("capture_wait_hi")}, 0.0, 1.0, true, 0.0, false, false);
        }
        if GIGA_GRAB[get_attacker_number(module_accessor)] == 1 {
            if GrabModule::is_rebound(module_accessor) || fighter_kind == *FIGHTER_KIND_KOOPAG {
                GIGA_GRAB[get_attacker_number(module_accessor)] = 9;
                MotionModule::change_motion(module_accessor, Hash40{hash: hash40("catch_cut")}, 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                GIGA_GRABBED[get_attacker_number(module_accessor)] = 90 + ((DamageModule::damage(module_accessor, 0) * 1.7) as i32); 
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, true);
            }
        }
        if GIGA_GRABBED[get_attacker_number(module_accessor)] > 0 {
            if PostureModule::pos_x(module_accessor) > PostureModule::pos_x(get_boma(get_attacker_number(module_accessor) as i32)) {
                PostureModule::set_lr(module_accessor, -1.0);
            }
            else {
                PostureModule::set_lr(module_accessor, 1.0);
            }
            PostureModule::update_rot_y_lr(module_accessor);
            if GIGA_OFFSET[get_attacker_number(module_accessor)].x != 0.0 {
                PostureModule::set_pos(module_accessor, &GIGA_OFFSET[get_attacker_number(module_accessor)]);
            }

            if GIGA_GRAB[get_attacker_number(module_accessor)] == 1 || GIGA_GRAB[get_attacker_number(module_accessor)] == 3 {
                GIGA_GRABBED[get_attacker_number(module_accessor)] -= 1;
            }
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD)
            || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP)
            || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                GIGA_GRABBED[get_attacker_number(module_accessor)] -= 8;
            }
        }
        if GIGA_GRABBED[get_attacker_number(module_accessor)] <= 0 && motion_kind == hash40("capture_wait_hi") && GIGA_GRAB[get_attacker_number(module_accessor)] == 1 {
            GIGA_GRABBED[get_attacker_number(module_accessor)] = 0;
            GIGA_GRAB[get_attacker_number(module_accessor)] = 0;
            GIGA_GRAB_TARGET[get_attacker_number(module_accessor)] = 8;
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, true);
            WorkModule::set_int(module_accessor, 8, *FIGHTER_INSTANCE_WORK_ID_INT_ATTACKER_COLOR);
        }
        if GIGA_GRAB[get_attacker_number(module_accessor)] == 7 {
            StatusModule::set_situation_kind(module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
        }
        if GIGA_GRAB[get_attacker_number(module_accessor)] > 0 && GIGA_GRAB[get_attacker_number(module_accessor)] != 3 && 
        (status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY
        || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR
        || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D 
        || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
        || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
        || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
        || status_kind == *FIGHTER_STATUS_KIND_BURY) {
            GIGA_GRABBED[get_attacker_number(module_accessor)] = 0;
            if GIGA_GRAB[get_attacker_number(module_accessor)] != 8 {
                GIGA_GRAB[get_attacker_number(module_accessor)] = 0;
            }
            WorkModule::set_int(module_accessor, 8, *FIGHTER_INSTANCE_WORK_ID_INT_ATTACKER_COLOR);
        }
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame,
        // global_weapon_frame
    );
    skyline::install_hook!(
        is_valid_just_shield_reflector
    );
}