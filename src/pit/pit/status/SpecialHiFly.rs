use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_pit_SpecialHiFly_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_FREE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHiFly_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_accel_x"));
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_accel_y"));
    let speed_x_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("speed_x_max"));
    let speed_y_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("speed_y_max"));
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, sum_speed_x, sum_speed_y, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x_max, speed_y_max);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x_max, speed_y_max);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, speed_y_max);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, speed_y_max);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
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
    fighter.set_situation(SITUATION_KIND_AIR.into());
    let int_time = WorkModule::get_int(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_TIME);
    let fly_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("fly_frame_max"));
    WorkModule::set_int(fighter.module_accessor, fly_frame_max, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_TIME);
    fighter.sub_shift_status_main(L2CValue::Ptr(pit_SpecialHiFly_Main_loop as *const () as _))
}

unsafe extern "C" fn pit_SpecialHiFly_sound_handler(fighter: &mut L2CFighterCommon) {
    WorkModule::dec_int(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_START_SE);
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_START_SE) <= 0 {
        WorkModule::set_int(fighter.module_accessor, 25, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_START_SE);
        let start_se_counter = WorkModule::get_int(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_START_SE_COUNTER);
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
        WorkModule::inc_int(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_START_SE_COUNTER);                           
    }
}

unsafe extern "C" fn pit_SpecialHiFly_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    pit_SpecialHiFly_sound_handler(fighter);
    let mut rot_y = WorkModule::get_float(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLOAT_ROT_Y);
    let mut turn_dir = WorkModule::get_float(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLOAT_TURN_DIR);
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLAG_TURN) {
        if stick_x * lr < -0.25 && stick_x.abs() > -0.25 {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLAG_TURN);
            WorkModule::set_float(fighter.module_accessor, -lr, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLOAT_TURN_DIR);
            WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLOAT_ROT_Y);
        }
    }
    else {
        if turn_dir == 0.0 { 
            turn_dir = 1.0
        };
        let target_rot = 180.0;
        rot_y = lerp(&rot_y, &target_rot, &0.15);
        if (rot_y - target_rot).abs() < 2.0 {
            rot_y = target_rot;
            PostureModule::set_lr(fighter.module_accessor, turn_dir);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLAG_TURN);
            //MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_fly"), 0.0, 1.0, true, 0.0, false, false);
        }
        else {
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f{x: 0.0, y: rot_y, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            WorkModule::set_float(fighter.module_accessor, rot_y, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLOAT_ROT_Y);
            //MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_fly_turn"), 0.0, 1.0, false, 0.0, false, false);
            if (rot_y > 90.0 && lr != turn_dir) {
                PostureModule::set_lr(fighter.module_accessor, turn_dir);
            }
        } 
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
    let int_time = WorkModule::get_int(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_TIME);
    let fly_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("fly_frame_max"));
    if int_time == 0 {
        WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        return 1.into();
    }
    else if int_time <= 0 + 120 && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLAG_BURN) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLAG_BURN);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_END.into(), false.into());
        return 0.into();
    }
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
        return 0.into();
    }
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn status_pit_SpecialHiFly_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_accel_x"));
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_accel_y"));
    let gravity_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("gravity_speed"));
    if stick_y.abs() > 0.1 {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let mut speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        speed_y = lerp(&speed_y, &0.0, &0.2);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0, speed_y);
    }
    else {
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -gravity_speed);
    }
    let speed_x = air_accel_x * stick_x;
    let speed_y = air_accel_y * stick_y;
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x * lr, speed_y);
    let air_decel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_decel_y"));
    let air_decel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_decel_x"));
    let mut sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    sum_speed_y -= air_decel_y;
    sum_speed_x -= air_decel_x;
    WorkModule::dec_int(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_TIME);
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let speed_x_control = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_y_stop = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingl1"), &Vector3f{x: 1.5, y: 1.5, z: 1.5});
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingr1"), &Vector3f{x: 1.5, y: 1.5, z: 1.5});
    println!("speed_y_stop{ }", speed_y_stop);
    println!("speed_x_control{ }", speed_x_control);
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHiFly_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLAG_BURN);
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingl1"), &Vector3f{x: 1.0, y: 1.0, z: 1.0});
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingr1"), &Vector3f{x: 1.0, y: 1.0, z: 1.0});
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