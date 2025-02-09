use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_funky_ShoulderStart_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        fighter.change_status(FIGHTER_DONKEY_STATUS_KIND_THROW_F_F.into(), false.into());
        return 0.into()
    }
    else {
        original_status(Main, fighter, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START)(fighter)
    }
}

pub fn install() {
    Agent::new("donkey")
    .status(Main, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START, status_funky_ShoulderStart_Main)
    .install();
}