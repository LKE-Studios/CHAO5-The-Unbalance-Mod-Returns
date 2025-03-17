use crate::imports::BuildImports::*;
use crate::common::status::Guard::{status_sub_guard_cont, status_guard_common_air, status_guard_main_common};

unsafe extern "C" fn status_koopag_Guard_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_guard_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(koopag_Guard_Main_loop as *const () as _))
}

unsafe extern "C" fn koopag_Guard_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !status_guard_common_air(fighter).get_bool() {
        if !fighter.sub_guard_cont().get_bool() {
            status_guard_main_common(fighter);
        }
    }
    let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    if shield_hp <= 0.0 {
        fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY.into(), false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("koopag")
    .status(Main, *FIGHTER_STATUS_KIND_GUARD, status_koopag_Guard_Main)
    .install();
}