use crate::imports::BuildImports::*;
use crate::maskedman::maskedman::status::SpecialHi::*;

pub static accel_y_mul : f32 = 1.5;
pub static accel_x_mul : f32 = 0.05;
pub static special_hi_end_stable_speed_x_mul : f32 = 0.4;

unsafe extern "C" fn status_maskedman_SpecialHiEnd_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32, 0);
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_maskedman_SpecialHiEnd_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("accel_y"));
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, accel_x_mul);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * special_hi_end_stable_speed_x_mul, 0.0);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -accel_y * accel_y_mul);
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_maskedman_SpecialHiEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {
        maskedman_SpecialHi_status_helper(fighter, true, FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_HI_END);
        fighter.sub_shift_status_main(L2CValue::Ptr(maskedman_SpecialHiEnd_Main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn maskedman_SpecialHiEnd_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        maskedman_SpecialHi_status_helper(fighter, false, *FIGHTER_STATUS_KIND_SPECIAL_HI);
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(),false.into());
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn status_maskedman_SpecialHiEnd_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {	
        0.into()
    }
    else {
        0.into()
    }
}

pub fn install() {
    Agent::new("lucas")
    .status(Pre, FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_HI_END, status_maskedman_SpecialHiEnd_Pre)
    .status(Init, FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_HI_END, status_maskedman_SpecialHiEnd_Init)
    .status(Main, FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_HI_END, status_maskedman_SpecialHiEnd_Main)
    .status(End, FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_HI_END, status_maskedman_SpecialHiEnd_End)
    .install();
}
