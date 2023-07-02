use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_DIDDY )]
pub fn frame_diddy(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);

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
        let boma_opponent1 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
        let status_kind_opponent1 = smash::app::lua_bind::StatusModule::status_kind(boma_opponent1);
        let frame_opponent1 = MotionModule::frame(boma_opponent1);
        let boma_opponent2 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(2));
        let status_kind_opponent2 = smash::app::lua_bind::StatusModule::status_kind(boma_opponent2);
        let frame_opponent2 = MotionModule::frame(boma_opponent2);
        let boma_opponent3 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(3));
        let status_kind_opponent3 = smash::app::lua_bind::StatusModule::status_kind(boma_opponent3);
        let frame_opponent3 = MotionModule::frame(boma_opponent3);
        let boma_opponent4 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(4));
        let status_kind_opponent4 = smash::app::lua_bind::StatusModule::status_kind(boma_opponent4);
        let frame_opponent4 = MotionModule::frame(boma_opponent4);
        let boma_opponent5 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(5));
        let status_kind_opponent5 = smash::app::lua_bind::StatusModule::status_kind(boma_opponent5);
        let frame_opponent5 = MotionModule::frame(boma_opponent5);
        let boma_opponent6 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(6));
        let status_kind_opponent6 = smash::app::lua_bind::StatusModule::status_kind(boma_opponent6);
        let frame_opponent6 = MotionModule::frame(boma_opponent6);
        let boma_opponent7 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(7));
        let status_kind_opponent7 = smash::app::lua_bind::StatusModule::status_kind(boma_opponent7);
        let frame_opponent7 = MotionModule::frame(boma_opponent7);
        let banana_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_LW_WORK_INT_BANANA_ID);
        
        if status_kind_opponent1 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent2 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent3 == *FIGHTER_STATUS_KIND_SLIP 
        || status_kind_opponent4 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent5 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent6 == *FIGHTER_STATUS_KIND_SLIP 
        || status_kind_opponent7 == *FIGHTER_STATUS_KIND_SLIP {
            WorkModule::set_int(fighter.module_accessor, banana_id, *FIGHTER_DIDDY_STATUS_SPECIAL_LW_WORK_INT_BANANA_ID);
            if (0.0..1.0).contains(&frame_opponent1) || (0.0..1.0).contains(&frame_opponent2) || (0.0..1.0).contains(&frame_opponent3) ||
                (0.0..1.0).contains(&frame_opponent4) || (0.0..1.0).contains(&frame_opponent5) || (0.0..1.0).contains(&frame_opponent6) ||
                (0.0..1.0).contains(&frame_opponent7) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_LW_LAUGH, false);
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_diddy
    );
}