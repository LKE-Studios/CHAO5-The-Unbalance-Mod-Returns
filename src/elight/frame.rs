use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::hash40;
use smashline::*;
//use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;
use crate::utils::*;

#[fighter_frame( agent = FIGHTER_KIND_ELIGHT )]
pub fn elight_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N || status_kind == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_N_HOLD || status_kind == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_N_END {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -4.0, 0);
            }
        };
        if status_kind == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END {
            if MotionModule::frame(fighter.module_accessor) > 21.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                if is_grounded(fighter.module_accessor) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                }
            }
        }
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_l") {
            if MotionModule::frame(fighter.module_accessor) >= 1.0 && MotionModule::frame(fighter.module_accessor) < 2.0 {
                ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_MAXIMTOMATO), 0, 0, false, false);
            }
        }
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_r") {
            if MotionModule::frame(fighter.module_accessor) >= 1.0 && MotionModule::frame(fighter.module_accessor) < 2.0 {
                ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_MAXIMTOMATO), 0, 0, false, false);
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        elight_opff
    );
}