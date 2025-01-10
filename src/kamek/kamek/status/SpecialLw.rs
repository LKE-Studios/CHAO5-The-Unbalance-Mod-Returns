use crate::imports::BuildImports::*;

unsafe extern "C" fn status_kamek_SpecialLw_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_kamek_SpecialLw_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_SE_HANDLE);
        WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_EFFECT_HANDLE);
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_kamek_SpecialLw_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x32de6245ed), *FIGHTER_HAVE_ITEM_WORK_MAIN, true);
        let time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("time"));
        let stop_y_time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("stop_y_time"));
        WorkModule::set_int(fighter.module_accessor, time, *FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_TIME);
        WorkModule::set_int(fighter.module_accessor, stop_y_time, *FIGHTER_NESS_STATUS_SPECIAL_LW_WORK_INT_STOP_Y_TIME);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(kamek_SpecialLw_Main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn kamek_SpecialLw_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            SoundModule::play_landing_se(fighter.module_accessor, Hash40::new("se_ness_landing01"));
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_start"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_start"), -1.0, 1.0, 0.0, false, false);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
            return 0.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_RV.into(),false.into());
            return 0.into();
        }
    }
    0.into()
}

unsafe extern "C" fn status_kamek_SpecialLw_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        0.into()
    }
    else {
        0.into()
    }
}

pub fn install() {
    Agent::new("ness")
    .status(Pre, *FIGHTER_KAMEK_STATUS_KIND_SPECIAL_LW, status_kamek_SpecialLw_Pre)
    .status(Init, *FIGHTER_KAMEK_STATUS_KIND_SPECIAL_LW, status_kamek_SpecialLw_Init)
    .status(Main, *FIGHTER_KAMEK_STATUS_KIND_SPECIAL_LW, status_kamek_SpecialLw_Main)
    .status(End, *FIGHTER_KAMEK_STATUS_KIND_SPECIAL_LW, status_kamek_SpecialLw_End)
    .install();
}