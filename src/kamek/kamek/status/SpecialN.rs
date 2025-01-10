use crate::imports::BuildImports::*;
use crate::kamek::kamek::frame::*;

unsafe extern "C" fn status_kamek_SpecialN_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {	
        fighter.sub_status_pre_SpecialNCommon();
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_kamek_SpecialN_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let KAMEK = color >= 64 && color <= 71;
	if KAMEK {
        let stop_y_time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("stop_y_time"));
        WorkModule::set_int(fighter.module_accessor, stop_y_time, *FIGHTER_NESS_STATUS_SPECIAL_N_WORK_INT_STOP_Y_TIME);
        sv_kinetic_energy!(clear_speed_ex, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(clear_speed_ex, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        if !StopModule::is_stop(fighter.module_accessor) {
            kamek_SpecialN_Sub_Status(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(kamek_SpecialN_Sub_Status as *const () as _));
        fighter.sub_shift_status_main(L2CValue::Ptr(kamek_SpecialN_Main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn kamek_SpecialN_Sub_Status(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_N_WORK_INT_STOP_Y_TIME);
            let stop_y_time_int = WorkModule::get_int(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_N_WORK_INT_STOP_Y_TIME);
            if stop_y_time_int <= 0 {
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            else {
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn kamek_SpecialN_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
        else if fighter.sub_air_check_stop_ceil().get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_NESS_AIR_STOP_SPECIAL_N);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_start"), -1.0, 1.0, 0.0, false, false);
            SoundModule::play_landing_se(fighter.module_accessor, Hash40::new("se_ness_landing01"));
        }
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_start"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KAMEK_STATUS_KIND_SPECIAL_N_HOLD.into(), false.into());
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn status_kamek_SpecialN_End(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    .status(Main, *FIGHTER_KAMEK_STATUS_KIND_SPECIAL_N_START, status_kamek_SpecialN_Main)
    .status(End, *FIGHTER_KAMEK_STATUS_KIND_SPECIAL_N_START, status_kamek_SpecialN_End)
    .install();
}