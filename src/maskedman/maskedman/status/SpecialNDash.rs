use crate::imports::BuildImports::*;

pub static speed_x_air : f32 = 5.5;
pub static speed_x_ground : f32 = 6.0;
pub static dash_speed_end_frame : f32 = 15.0;

unsafe extern "C" fn status_maskedman_SpecialNDash_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {	
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_maskedman_SpecialNDash_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {	
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            sv_kinetic_energy!(clear_speed_ex, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x_air);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x_air);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x_ground);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x_ground);
        }
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_maskedman_SpecialNDash_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {
        let float_charge = WorkModule::get_float(fighter.module_accessor, FIGHTER_MASKEDMAN_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE);
        AttackModule::set_power_up(fighter.module_accessor, 1.0 + (float_charge * 0.6));
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_dash"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_dash"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(maskedman_SpecialNDash_Main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn maskedman_SpecialNDash_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_dash"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_dash"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if frame > dash_speed_end_frame {
        KineticModule::suspend_energy_all(fighter.module_accessor);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_N_END.into(), false.into());
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn status_maskedman_SpecialNDash_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_MASKEDMAN_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_MASKEDMAN_STATUS_SPECIAL_N_FLAG_CHARGE);
    0.into()
}

pub fn install() {
    Agent::new("lucas")
    .status(Pre, FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_N_DASH, status_maskedman_SpecialNDash_Pre)
    .status(Init, FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_N_DASH, status_maskedman_SpecialNDash_Init)
    .status(Main, FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_N_DASH, status_maskedman_SpecialNDash_Main)
    .status(End, FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_N_DASH, status_maskedman_SpecialNDash_End)
    .install();
}