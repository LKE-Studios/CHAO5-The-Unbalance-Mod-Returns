use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_pit_SpecialHiFly_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHiFly_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_accel_x"));
    let speed_x_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("speed_x_max"));
    let speed_y_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("speed_y_max"));
    sv_kinetic_energy!(controller_set_accel_x_mul, fighter, air_accel_x);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x_max, speed_y_max);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x_max, speed_y_max);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x_max, speed_y_max);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x_max, speed_y_max);
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, sum_speed_x, 0.0);
    sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0);
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHiFly_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_TREAD_JUMP);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_fly"), 0.0, 1.0, true, 0.0, false, false);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.set_situation(SITUATION_KIND_AIR.into());
    let int_time = WorkModule::get_int(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_TIME);
    let fly_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("fly_frame_max"));
    WorkModule::set_int(fighter.module_accessor, fly_frame_max, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_TIME);
    fighter.sub_shift_status_main(L2CValue::Ptr(pit_SpecialHiFly_Main_loop as *const () as _))
}

unsafe extern "C" fn pit_SpecialHiFly_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if stick_x * lr < -0.25 {
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN.into(), true.into());
    }
    let motion_rate_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("motion_rate_max"));
    let motion_rate_min = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("motion_rate_min"));
    if stick_y > 0.0 {
        MotionModule::set_rate(fighter.module_accessor, motion_rate_max);
    }
    else {
        MotionModule::set_rate(fighter.module_accessor, motion_rate_min);
    }
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("landing_frame"));
    if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_END.into(), false.into());
            WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            return 0.into();
        }
    }
    let int_time = WorkModule::get_int(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_TIME);
    let fly_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("fly_frame_max"));
    if int_time == 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn status_pit_SpecialHiFly_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    pit_KineticTransactor_Exec(fighter);
    WorkModule::dec_int(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_TIME);
    0.into()
}

pub unsafe extern "C" fn pit_KineticTransactor_Exec(fighter: &mut L2CFighterCommon) {
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_accel_x"));
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_accel_y"));
    let gravity_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("gravity_speed"));
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -gravity_speed);
    let speed_x = air_accel_x * stick_x;
    let speed_y = air_accel_y * stick_y;
    let mut sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x * lr, speed_y);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x * lr, speed_y);
    let air_decel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_decel_y"));
    let air_decel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_decel_x"));
    sum_speed_y -= air_decel_y;
    sum_speed_x -= air_decel_x;
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
}

pub unsafe extern "C" fn status_pit_SpecialHiFly_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    0.into()
}

pub fn install() {
    Agent::new("pit")
    .status(Pre, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY, status_pit_SpecialHiFly_Pre)
    .status(Init, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY, status_pit_SpecialHiFly_Init)
    .status(Main, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY, status_pit_SpecialHiFly_Main)
    .status(Exec, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY, status_pit_SpecialHiFly_Exec)
    .status(End, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY, status_pit_SpecialHiFly_End)
    .install();
}