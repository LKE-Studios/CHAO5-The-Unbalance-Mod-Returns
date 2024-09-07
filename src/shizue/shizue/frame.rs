use crate::imports::BuildImports::*;
use crate::shizue::shizue_pot::status::Throwed::*;

pub unsafe extern "C" fn frame_shizue_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
            if situation_kind == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
            }
        }
    };
    if status_kind == *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END {
        if MotionModule::frame(fighter.module_accessor) > 3.0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                }
            }
        }
    }
    if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) 
    || !sv_information::is_ready_go() {
        SHIZUE_SHOT_KIND[ENTRY_ID] = 1;
    };
    if ![*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR].contains(&status_kind) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_PICOPICOHAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    };
    if ![*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_CLIFF_ATTACK].contains(&status_kind) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_POMPON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    };
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    if [hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) {
        if MotionModule::frame(fighter.module_accessor) >= 28.0 {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            } 
            else {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
            }
        }
    };
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_POCKET, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_TAKE_OUT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FAILURE, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FIRE].contains(&status_kind) {
        if !fighter.is_in_hitlag() && !StatusModule::is_changing(fighter.module_accessor) && situation_kind == *SITUATION_KIND_AIR {
            fighter.sub_air_check_dive();
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
                if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR || 
                KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                    let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    sv_kinetic_energy::enable(fighter.lua_state_agent);
                    KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                }
            }
        }
    }
}

pub fn install() {
    Agent::new("shizue")
    .on_line(Main, frame_shizue_Main)
    .install();
}