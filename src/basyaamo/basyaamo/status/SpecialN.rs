use crate::imports::BuildImports::*;

unsafe extern "C" fn status_basyaamo_SpecialN_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let BASYAAMO = color >= 120 && color <= 127;
    if BASYAAMO {
        fighter.sub_status_pre_SpecialNCommon();
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_AIR, GROUND_CORRECT_KIND_AIR.into(), GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        } else {
            StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, GROUND_CORRECT_KIND_AIR.into(), GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        }
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N.into(), 0);
        return 0.into();
    }
    else {
        original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
    }
}

unsafe extern "C" fn status_basyaamo_SpecialN_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let BASYAAMO = color >= 120 && color <= 127;
    if BASYAAMO {
        GroundModule::set_attach_ground(fighter.module_accessor, false);
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        return 0.into();
    }
    else {
        original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
    }
}

unsafe extern "C" fn status_basyaamo_SpecialN_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let BASYAAMO = color >= 120 && color <= 127;
    if BASYAAMO {
        if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f{ x: 0.0, y: 1.0, z: 1.0 }, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
            
        } else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(basyaamo_SpecialN_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
    }
}

unsafe fn basyaamo_SpecialN_motion_helper(fighter: &mut L2CFighterCommon, situation_change: bool) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !situation_change {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_MOTION);
        if !situation_change {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
        }
    }
}

unsafe extern "C" fn basyaamo_SpecialN_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != fighter.global_table[PREV_SITUATION_KIND].get_i32() {
        basyaamo_SpecialN_motion_helper(fighter, true);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if GroundModule::is_touch(fighter.module_accessor, GROUND_TOUCH_FLAG_DOWN.into()) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn status_basyaamo_SpecialN_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let BASYAAMO = color >= 120 && color <= 127;
    if BASYAAMO {
        return 0.into();
    }
    else {
        original_status(Exec, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
    }
}

unsafe extern "C" fn status_basyaamo_SpecialN_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let BASYAAMO = color >= 120 && color <= 127;
    if BASYAAMO {
        return 0.into();
    }
    else {
        original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
    }
}

pub fn install() {
    Agent::new("captain")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, status_basyaamo_SpecialN_Pre)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, status_basyaamo_SpecialN_Init)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, status_basyaamo_SpecialN_Main)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, status_basyaamo_SpecialN_Exec)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, status_basyaamo_SpecialN_End)
    .install();
}