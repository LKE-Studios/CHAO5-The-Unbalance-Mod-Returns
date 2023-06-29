use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_BAYONETTA )]
pub fn frame_bayonetta(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
        
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME_SUCCESS) {
                DamageModule::heal(fighter.module_accessor, -0.8, 0);
            }
        };
        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME_SUCCESS);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_bayonetta
    );
}