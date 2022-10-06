use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use crate::utils::*;
use smashline::*;
use smash::app::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lua2cpp::L2CFighterBase;
use smash::phx::Vector3f;
use smash::phx::Hash40;

#[fighter_frame( agent = FIGHTER_KIND_EDGE )]
pub fn edge_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let kind = smash::app::utility::get_kind(boma); 
        if kind == *FIGHTER_KIND_EDGE {
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("swordl1"), &Vector3f{x:1.15, y:1.0, z:1.0});
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("swordr1"), &Vector3f{x:1.15, y:1.0, z:1.0});
        }
        if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL {
            if situation_kind == *SITUATION_KIND_AIR {
                if WorkModule::get_int(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                    WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                    ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                }
            }
        };
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if MotionModule::frame(fighter.module_accessor) > 32.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                if is_grounded(fighter.module_accessor) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                }
            }
        };
        if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH {
            if MotionModule::frame(fighter.module_accessor) > 32.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                if is_grounded(fighter.module_accessor) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                }
            }
        };
        if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH {
            if MotionModule::frame(fighter.module_accessor) > 32.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                if is_grounded(fighter.module_accessor) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                }
            }
        };
        if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END {
            if MotionModule::frame(fighter.module_accessor) > 32.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                if is_grounded(fighter.module_accessor) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                }
            }
        };

    }
}

#[weapon_frame( agent = WEAPON_KIND_EDGE_FIRE )]
pub fn edge_gigaflare_opwf(fighter : &mut L2CFighterBase) {
    unsafe {
        let status = StatusModule::status_kind(fighter.module_accessor);
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        if [*WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S].contains(&status) {
            if ControlModule::check_button_on(owner_module_accessor, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_on(owner_module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *WEAPON_EDGE_FIRE_STATUS_KIND_BURST_S, false);
            }
        }
        if [*WEAPON_EDGE_FIRE_STATUS_KIND_FLY_M].contains(&status) {
            if ControlModule::check_button_on(owner_module_accessor, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_on(owner_module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *WEAPON_EDGE_FIRE_STATUS_KIND_BURST_M, false);
            }
        }
        if [*WEAPON_EDGE_FIRE_STATUS_KIND_FLY_L].contains(&status) {
            if ControlModule::check_button_on(owner_module_accessor, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_on(owner_module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *WEAPON_EDGE_FIRE_STATUS_KIND_BURST_L, false);
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        edge_opff,
        edge_gigaflare_opwf
    );
}