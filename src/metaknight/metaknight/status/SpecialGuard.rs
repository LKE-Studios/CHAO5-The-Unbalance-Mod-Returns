use crate::imports::BuildImports::*;

unsafe extern "C" fn status_metaknight_SpecialGuard_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let kinetic;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_GUARD);
        kinetic = *FIGHTER_KINETIC_TYPE_FALL;
    }
    else {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_GUARD);
        kinetic = *FIGHTER_KINETIC_TYPE_GROUND_STOP;
    }
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), kinetic, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn status_metaknight_SpecialGuard_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_z"), 0.0, 1.0, false, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_z"), 0.0, 1.0, false, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_SpecialGuard_Main_loop as *const () as _))
}

unsafe extern "C" fn metaknight_SpecialGuard_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_z"), -1.0, 1.0, 0.0, false, false);
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("mantle") as i64, hash40("mantle_normal") as i64);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_z"), -1.0, 1.0, 0.0, false, false);
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("mantle") as i64, hash40("mantle_wing") as i64);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn status_metaknight_SpecialGuard_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("metaknight")
    .status(Pre, FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_GUARD, status_metaknight_SpecialGuard_Pre)
    .status(Main, FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_GUARD, status_metaknight_SpecialGuard_Main)
    .status(End, FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_GUARD, status_metaknight_SpecialGuard_End)
    .install();
}

