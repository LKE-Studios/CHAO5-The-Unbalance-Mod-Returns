use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Vector3f;
use smash::app::Fighter;

static mut DIDDY_SPECIAL_LW_LAUGH_CHECK : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_DIDDY )]
pub fn frame_diddy(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let frame = MotionModule::frame(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        if status_kind == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_SHOOT {
            if frame > 5.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                }
            }
        };
        if status_kind == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_LW_LAUGH {
            if frame > 0.0 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
        if DIDDY_SPECIAL_LW_LAUGH_CHECK[ENTRY_ID] == true {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_LW_LAUGH, false);
        };

        let boma_opponent1 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
        let status_kind_opponent1 = smash::app::lua_bind::StatusModule::status_kind(boma_opponent1);
        let boma_opponent2 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(2));
        let status_kind_opponent2 = smash::app::lua_bind::StatusModule::status_kind(boma_opponent2);
        let boma_opponent3 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(3));
        let status_kind_opponent3 = smash::app::lua_bind::StatusModule::status_kind(boma_opponent3);
        let boma_opponent4 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(4));
        let status_kind_opponent4 = smash::app::lua_bind::StatusModule::status_kind(boma_opponent4);
        let boma_opponent5 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(5));
        let status_kind_opponent5 = smash::app::lua_bind::StatusModule::status_kind(boma_opponent5);
        let boma_opponent6 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(6));
        let status_kind_opponent6 = smash::app::lua_bind::StatusModule::status_kind(boma_opponent6);
        let boma_opponent7 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(7));
        let status_kind_opponent7 = smash::app::lua_bind::StatusModule::status_kind(boma_opponent7);

        if status_kind_opponent1 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent2 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent3 == *FIGHTER_STATUS_KIND_SLIP 
        || status_kind_opponent4 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent5 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent6 == *FIGHTER_STATUS_KIND_SLIP 
        || status_kind_opponent7 == *FIGHTER_STATUS_KIND_SLIP {
            DIDDY_SPECIAL_LW_LAUGH_CHECK[ENTRY_ID] = true;
        }
        else {
            DIDDY_SPECIAL_LW_LAUGH_CHECK[ENTRY_ID] = false;
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_diddy
    );
}