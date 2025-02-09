use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_funky_Wait_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        fighter.sub_wait_common();
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("wait_4"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_Wait_Main as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_WAIT)(fighter)
    }
}

pub fn install() {
    Agent::new("donkey")
    .status(Main, *FIGHTER_STATUS_KIND_WAIT, status_funky_Wait_Main)
    .install();
}