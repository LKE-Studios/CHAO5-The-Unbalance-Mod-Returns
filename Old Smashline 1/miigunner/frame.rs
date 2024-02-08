use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_MIIGUNNER )]
pub fn frame_miigunner(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        
        if status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL {
            if situation_kind == *SITUATION_KIND_AIR {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                    WorkModule::set_int(fighter.module_accessor, *STATUS_KIND_NONE, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
                }
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_miigunner
    );
}