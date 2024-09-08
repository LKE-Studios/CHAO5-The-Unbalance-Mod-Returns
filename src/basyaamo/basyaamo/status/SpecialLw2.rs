use crate::imports::BuildImports::*;

unsafe extern "C" fn status_basyaamo_SpecialLw2_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, GROUND_CORRECT_KIND_AIR.into(), GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW.into(), 0);
    return 0.into();
}

unsafe extern "C" fn status_basyaamo_SpecialLw2_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KICK_START_SITUATION) == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_2"), 0.0, 1.0, false, 0.0, false, false);
    } 
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_2"), 14.0, 1.0, false, 0.0, false, false);
    }
    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.sub_shift_status_main(L2CValue::Ptr(basyaamo_SpecialLw2_Main_loop as *const () as _))
}

unsafe extern "C" fn basyaamo_SpecialLw2_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_BASYAAMO_STATUS_KIND_SPECIAL_LW_LOOP.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn status_basyaamo_SpecialLw2_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub fn install() {
    Agent::new("captain")
    .status(Pre, FIGHTER_BASYAAMO_STATUS_KIND_SPECIAL_LW2, status_basyaamo_SpecialLw2_Pre)
    .status(Main, FIGHTER_BASYAAMO_STATUS_KIND_SPECIAL_LW2, status_basyaamo_SpecialLw2_Main)
    .status(End, FIGHTER_BASYAAMO_STATUS_KIND_SPECIAL_LW2, status_basyaamo_SpecialLw2_End)
    .install();
}