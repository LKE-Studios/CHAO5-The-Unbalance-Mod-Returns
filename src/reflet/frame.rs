use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_REFLET )]
pub fn frame_reflet(fighter : &mut L2CFighterCommon) {
    unsafe { 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);

        if status_kind == *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_CANCEL {
            if situation_kind == *SITUATION_KIND_AIR {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                    WorkModule::set_int(fighter.module_accessor, *STATUS_KIND_NONE, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
                    ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                }
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_reflet
    );
}