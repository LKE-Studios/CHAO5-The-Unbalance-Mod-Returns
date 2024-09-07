use crate::imports::BuildImports::*;

unsafe extern "C" fn status_murabito_SpecialSFall_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_INT_SITUATION);
    let motion = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        hash40("special_air_s_fall")
    }
    else {
        hash40("special_s_fall")
    };
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(murabito_SpecialSFall_Main_loop as *const () as _))
}

unsafe extern "C" fn murabito_SpecialSFall_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("murabito")
    .status(Main, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_FALL, status_murabito_SpecialSFall_Main)
    .install();
}