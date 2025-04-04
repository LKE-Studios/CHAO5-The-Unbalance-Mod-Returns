use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_pit_SpecialHiFly_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_FALL_FREE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHiFly_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_x_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("speed_x_max"));
    let speed_y_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("speed_y_max"));
    let gravity_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("gravity_speed"));
    let air_decel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_decel_x"));
    let air_decel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_decel_y"));
    let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::KineticEnergy;
    let control = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) as *mut smash::app::KineticEnergy;
    let speed_y = KineticEnergy::get_speed_y(gravity);
    let speed_x = KineticEnergy::get_speed_x(control);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, speed_x, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity_speed);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity_speed);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x_max, speed_y_max);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x_max, speed_y_max);
    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_decel_x, air_decel_y);
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHiFly_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_TREAD_JUMP);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_fly"), 0.0, 1.0, true, 0.0, false, false);
    let motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND);
    let fly_frame_max = WorkModule::get_float(fighter.module_accessor, *FIGHTER_PIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FLY_POWER);
    WorkModule::set_int(fighter.module_accessor, fly_frame_max as i32, *FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_TIME);
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_HI && motion_kind == hash40("special_air_hi_start") {
        let fastest_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("fastest_landing_frame"));
        WorkModule::set_int(fighter.module_accessor, fastest_landing_frame, *FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_LAND_TIME);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(pit_SpecialHiFly_Main_loop as *const () as _))
}

unsafe extern "C" fn pit_SpecialHiFly_sound_handler(fighter: &mut L2CFighterCommon) {
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_START_SE);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_START_SE) <= 0 {
        WorkModule::set_int(fighter.module_accessor, 25, *FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_START_SE);
        let start_se_counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_START_SE_COUNTER);
        let sound = match start_se_counter {
            0 => hash40("se_pit_jump02"), 1 => hash40("se_pit_special_h03"),
            2 => hash40("se_pit_jump02"), 3 => hash40("se_pit_special_h03"),
            4 => hash40("se_pit_jump02"), 5 => hash40("se_pit_special_h03"),
            6 => hash40("se_pit_jump02"), 7 => hash40("se_pit_special_h03"),
            8 => hash40("se_pit_jump02"), 9 => hash40("se_pit_special_h03"),
            10 => hash40("se_pit_jump02"), 11 => hash40("se_pit_special_h03"),
            12 => hash40("se_pit_jump02"), 13 => hash40("se_pit_special_h03"),
            13 => hash40("se_pit_jump02"), 14 => hash40("se_pit_special_h03"),
            15 => hash40("se_pit_jump02"), 16 => hash40("se_pit_special_h03"),
            17 => hash40("se_pit_jump02"), 18 => hash40("se_pit_special_h03"),
            19 => hash40("se_pit_jump02"), 20 => hash40("se_pit_special_h03"),
            21 => hash40("se_pit_jump02"), 22 => hash40("se_pit_special_h03"),
            23 => hash40("se_pit_jump02"), 24 => hash40("se_pit_special_h03"),
            25 => hash40("se_pit_jump02"), _ => hash40("se_pit_special_h03"),
        };
        SoundModule::play_se(fighter.module_accessor, Hash40::new_raw(sound), true, false, false, false, enSEType(0));
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_START_SE_COUNTER);                           
    }
}

unsafe extern "C" fn pit_SpecialHiFly_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    let fly_time = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_TIME);
    let land_time = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_LAND_TIME);
    let frame_remain_burn = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("frame_remain_burn"));
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_TIME);
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_LAND_TIME);
    if fly_time <= 0 {
        let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("landing_frame"));
        WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        return 1.into();
    }
    if fly_time <= frame_remain_burn && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLAG_BURN) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLAG_BURN);
    }
    if land_time <= 0 {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    }
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);    
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if motion_kind == hash40("special_hi_fly_turn") {
        if MotionModule::is_end(fighter.module_accessor) {
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_fly"), 0.0, 1.0, true, 0.0, false, false);
        }
    }
    else if motion_kind == hash40("special_hi_fly") {
        if stick_x * lr < -0.25 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_fly_turn"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    pit_SpecialHiFly_sound_handler(fighter);
    println!("{}", fly_time);
    0.into()
}

unsafe extern "C" fn status_pit_SpecialHiFly_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let prev_stick_x = ControlModule::get_stick_prev_x(fighter.module_accessor);
    if stick_y > 0.0 {
        let motion_rate_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("motion_rate_max"));
        MotionModule::set_rate(fighter.module_accessor, motion_rate_max);
    }
    else {
        let motion_rate_min = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("motion_rate_min"));
        MotionModule::set_rate(fighter.module_accessor, motion_rate_min);
    }
    let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_accel_x"));
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_accel_y"));
    let speed_x_boost = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("speed_x_boost"));
    let mut accel_x = air_accel_x * stick_x;
    let mut accel_y = air_accel_y * stick_y;
    if stick_x > 0.0 && prev_stick_x <= 0.0 {
        accel_x += speed_x_boost;
    }
    if stick_x < 0.0 && prev_stick_x >= 0.0 {
        accel_x -= speed_x_boost;
    }
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, accel_x, accel_y);
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingl1"), &Vector3f{x: 1.5, y: 1.5, z: 1.5});
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingr1"), &Vector3f{x: 1.5, y: 1.5, z: 1.5});
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHiFly_Exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fly_frame_min = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("fly_frame_min"));
    WorkModule::set_float(fighter.module_accessor, fly_frame_min as f32, *FIGHTER_PIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FLY_POWER);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLAG_BURN);
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHiFly_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingl1"), &Vector3f{x: 1.0, y: 1.0, z: 1.0});
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingr1"), &Vector3f{x: 1.0, y: 1.0, z: 1.0});
    0.into()
}

pub fn install() {
    Agent::new("pit")
    .status(Pre, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY, status_pit_SpecialHiFly_Pre)
    .status(Init, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY, status_pit_SpecialHiFly_Init)
    .status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY, status_pit_SpecialHiFly_Main)
    .status(Exec, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY, status_pit_SpecialHiFly_Exec)
    .status(Exit, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY, status_pit_SpecialHiFly_Exit)
    .status(End, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY, status_pit_SpecialHiFly_End)
    .install();
}