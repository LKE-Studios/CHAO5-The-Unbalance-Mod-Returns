use crate::imports::BuildImports::*;

unsafe extern "C" fn status_bandana_JumpAerial_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        let jump_count_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        if jump_count > 2 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("jump_aerial_f1"), 0.0, 1.0, false, 0.0, false, false);
            fighter.status_JumpAerial()
        }
        else {
            fighter.status_JumpAerial()
        }
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_JUMP_AERIAL)(fighter)
    }
}

pub fn install() {
    Agent::new("edge")
    .status(Main, *FIGHTER_STATUS_KIND_JUMP_AERIAL, status_bandana_JumpAerial_Main)
    .install();
}