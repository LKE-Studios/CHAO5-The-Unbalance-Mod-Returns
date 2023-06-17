use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash_script::*;
use smashline::*;
use smash::app::*;
use smash::phx::Hash40;
use smash::hash40;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::Vector3f;
use smash::phx::Vector2f;
use smash::lib::L2CValue;
use crate::globals::*;

pub fn is_edge_cancel(fighter_kind : i32, status_kind : i32) -> bool {
	let edge_cancel = [
        [*FIGHTER_KIND_YOSHI, *FIGHTER_STATUS_KIND_SPECIAL_N],
        [*FIGHTER_KIND_YOSHI, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_1],
        [*FIGHTER_KIND_YOSHI, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_2],
        [*FIGHTER_KIND_FOX, *FIGHTER_STATUS_KIND_SPECIAL_S],
        [*FIGHTER_KIND_CAPTAIN, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END],
    ];
	for i in &edge_cancel {
		if fighter_kind == i[0] && status_kind == i[1] {
			return true;
		};
	};
	return false;
}

#[skyline::hook(replace = smash::app::lua_bind::StatusModule::init_settings)]
unsafe fn status_init_settings(module_accessor: &mut smash::app::BattleObjectModuleAccessor, situation_kind: i32, arg3: i32, arg4: u64, ground_cliff_check_kind: u64, arg6: bool, arg7: i32, arg8: i32, arg9: i32, arg10: i32) -> u64 {
    let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    if smash::app::utility::get_category(module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
        return original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);
    }
    if is_edge_cancel(fighter_kind, status_kind) && situation_kind == SITUATION_KIND_GROUND {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
    else if [
        *FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_WAIT, *FIGHTER_STATUS_KIND_SQUAT_F, 
        *FIGHTER_STATUS_KIND_SQUAT_B, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, *FIGHTER_STATUS_KIND_ITEM_THROW, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, 
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_DAMAGE
    ].contains(&status_kind) {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
    else if status_kind == *FIGHTER_STATUS_KIND_GUARD_DAMAGE {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);
        KineticModule::clear_speed_all(module_accessor);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL)
    }
    else if fighter_kind == *FIGHTER_KIND_KOOPA && [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G].contains(&status_kind) {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
    else if fighter_kind == *FIGHTER_KIND_GAOGAEN && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
    else if fighter_kind == *FIGHTER_KIND_EDGE
    && ([*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING].contains(&status_kind))
    && (StatusModule::prev_status_kind(module_accessor, 0) != *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH)
    && situation_kind == SITUATION_KIND_GROUND {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);
        KineticModule::clear_speed_all(module_accessor);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL)
    }
    else {
        original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
}

#[skyline::hook(replace = smash::app::lua_bind::GroundModule::correct)]
unsafe fn status_correct(module_accessor: &mut smash::app::BattleObjectModuleAccessor, ground_correct_kind: u32) -> u64 {
    let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let situation_kind = StatusModule::situation_kind(module_accessor);

    if smash::app::utility::get_category(module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
        original!()(module_accessor, ground_correct_kind);
    }
    if [
        *FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_WAIT, *FIGHTER_STATUS_KIND_SQUAT_F, 
        *FIGHTER_STATUS_KIND_SQUAT_B, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, *FIGHTER_STATUS_KIND_ITEM_THROW, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, 
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_DAMAGE
    ].contains(&status_kind) {
        original!()(module_accessor, *GROUND_CORRECT_KIND_GROUND as u32)
    }
    else if status_kind == *FIGHTER_STATUS_KIND_GUARD_DAMAGE {
        original!()(module_accessor, 1 as u32);
        KineticModule::clear_speed_all(module_accessor);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL)
    }
    else if is_edge_cancel(fighter_kind, status_kind) {
        original!()(module_accessor, *GROUND_CORRECT_KIND_GROUND as u32)
    }
	else if [*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_SNAKE, *FIGHTER_KIND_DIDDY, *FIGHTER_KIND_SONIC].contains(&fighter_kind) && status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
		original!()(module_accessor, 1 as u32)
	}
    else if fighter_kind == *FIGHTER_KIND_KOOPA && [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G].contains(&status_kind) {
        original!()(module_accessor, *GROUND_CORRECT_KIND_GROUND as u32)
    }
    else if fighter_kind == *FIGHTER_KIND_GAOGAEN && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        original!()(module_accessor, *GROUND_CORRECT_KIND_KEEP as u32)
    }
    else if fighter_kind == *FIGHTER_KIND_EDGE
    && ([*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING].contains(&status_kind))
    && (StatusModule::prev_status_kind(module_accessor, 0) != *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH)
    && situation_kind == SITUATION_KIND_GROUND {
        original!()(module_accessor, *GROUND_CORRECT_KIND_GROUND as u32);
        KineticModule::clear_speed_all(module_accessor);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL)
    }
    else {
        original!()(module_accessor, ground_correct_kind)
    }
}

extern "C" {
    #[link_name = "\u{1}_ZN3app11FighterUtil33get_ground_correct_kind_air_transERNS_26BattleObjectModuleAccessorEi"]
    fn get_ground_correct_kind_air_trans(module_accessor: &mut smash::app::BattleObjectModuleAccessor, something: i32) -> i32;
}

#[skyline::hook(replace=get_ground_correct_kind_air_trans)]
unsafe fn get_ground_correct_kind_air_trans_hook(_module_accessor: &mut smash::app::BattleObjectModuleAccessor, _something: i32) -> i32 {
    return *GROUND_CORRECT_KIND_AIR;
}

pub fn install() {
	skyline::install_hooks!(
		status_init_settings,
        status_correct,
        get_ground_correct_kind_air_trans_hook
    );
}