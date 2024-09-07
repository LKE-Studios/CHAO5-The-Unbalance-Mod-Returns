use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_pit_SpecialHiFlyEnd_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHiFlyEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_end"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    fighter.sub_shift_status_main(L2CValue::Ptr(pit_SpecialHiFlyEnd_Main_loop as *const () as _))
}

unsafe extern "C" fn pit_SpecialHiFlyEnd_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHiFlyEnd_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("pit")
    .status(Pre, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_END, status_pit_SpecialHiFlyEnd_Pre)
    .status(Main, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_END, status_pit_SpecialHiFlyEnd_Main)
    .status(End, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_END, status_pit_SpecialHiFlyEnd_End)
    .install();
}