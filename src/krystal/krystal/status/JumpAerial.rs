use crate::imports::BuildImports::*;

unsafe extern "C" fn status_krystal_JumpAerial_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let KRYSTAL = color >= 64 && color <= 71;
    if KRYSTAL {
        fighter.status_JumpAerialSub(false.into(), false.into());
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_JumpAerial_Main as *const () as _))
    }
    else {
        0.into()
    }
}

pub fn install() {
    Agent::new("pitb")
    .status(Main, *FIGHTER_STATUS_KIND_JUMP_AERIAL, status_krystal_JumpAerial_Main)
    .install();
}