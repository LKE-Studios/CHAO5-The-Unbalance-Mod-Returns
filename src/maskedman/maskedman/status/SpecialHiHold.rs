use crate::imports::BuildImports::*;
use crate::maskedman::maskedman::status::SpecialHi::*;

pub static speed_x_mul : f32 = 1.0;
pub static speed_y : f32 = 6.5;
pub static air_speed_y_mul : f32 = 1.03;
pub static air_accel_y : f32 = 0.15;
pub static accel_x_mul : f32 = 0.1;
pub static stable_speed_x_mul : f32 = 0.9;
pub static max_frame : i32 = 220;

unsafe extern "C" fn status_maskedman_SpecialHiHold_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MIIGUNNER_SPECIAL_HI2_JUMP, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32, 0);
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_maskedman_SpecialHiHold_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {
        let lr = PostureModule::lr(fighter.module_accessor);
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * lr;
        let speed_x = (((stick_x + 1.0) / 2.0)) * lr;
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x * speed_x_mul);
        sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, accel_x_mul);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * stable_speed_x_mul, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y * air_speed_y_mul);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_maskedman_SpecialHiHold_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {
        let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        WorkModule::set_int(fighter.module_accessor, max_frame, *FIGHTER_MASKEDMAN_STATUS_SPECIAL_HI_HOLD_WORK_INT_STOP_TIME);
        WorkModule::set_int(fighter.module_accessor, max_frame, *FIGHTER_MASKEDMAN_STATUS_SPECIAL_HI_HOLD_WORK_INT_TIME);
        maskedman_SpecialHi_status_helper(fighter, true, *FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_HI_HOLD);
        fighter.sub_shift_status_main(L2CValue::Ptr(maskedman_SpecialHiHold_Main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn maskedman_SpecialHiHold_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into());
        }
    }
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let int_time = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MASKEDMAN_STATUS_SPECIAL_HI_HOLD_WORK_INT_TIME) <= 0;
    if sum_speed_y <= 0.5 || int_time {
        fighter.change_status(FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_HI_END.into(),false.into());
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn status_maskedman_SpecialHiHold_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {
        let lr = PostureModule::lr(fighter.module_accessor);
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * lr;
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_MASKEDMAN_STATUS_SPECIAL_HI_HOLD_WORK_INT_TIME);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if speed_x.abs() > 0.0 
        && speed_x.signum() != lr {
            KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            SET_SPEED_EX(fighter, 0, sum_speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
        0.into()
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn status_maskedman_SpecialHiHold_End(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    .status(Pre, *FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_HI_HOLD, status_maskedman_SpecialHiHold_Pre)
    .status(Init, *FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_HI_HOLD, status_maskedman_SpecialHiHold_Init)
    .status(Main, *FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_HI_HOLD, status_maskedman_SpecialHiHold_Main)
    .status(Exec, *FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_HI_HOLD, status_maskedman_SpecialHiHold_Exec)
    .status(End, *FIGHTER_MASKEDMAN_STATUS_KIND_SPECIAL_HI_HOLD, status_maskedman_SpecialHiHold_End)
    .install();
}
