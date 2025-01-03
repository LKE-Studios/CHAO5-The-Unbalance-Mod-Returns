use crate::imports::BuildImports::*;

unsafe extern "C" fn status_basyaamo_SpecialLwAirEnd_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_FALL, GROUND_CORRECT_KIND_AIR.into(), GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW.into(), 0);
    return 0.into();
}

unsafe extern "C" fn status_basyaamo_SpecialLwAirEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_end"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(basyaamo_SpecialLwAirEnd_Main_loop as *const () as _))
}

unsafe extern "C" fn basyaamo_SpecialLwAirEnd_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn status_basyaamo_SpecialLwAirEnd_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub fn install() {
    Agent::new("captain")
    .status(Pre, *FIGHTER_BASYAAMO_STATUS_KIND_SPECIAL_LW_AIR_END, status_basyaamo_SpecialLwAirEnd_Pre)
    .status(Main, *FIGHTER_BASYAAMO_STATUS_KIND_SPECIAL_LW_AIR_END, status_basyaamo_SpecialLwAirEnd_Main)
    .status(End, *FIGHTER_BASYAAMO_STATUS_KIND_SPECIAL_LW_AIR_END, status_basyaamo_SpecialLwAirEnd_End)
    .install();
}