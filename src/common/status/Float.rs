use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_Float_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON as u64, (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_STATUS_ATTR_INTO_DOOR) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32, 0);
    0.into()
}

pub unsafe extern "C" fn status_Float_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_x_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_uniq_float"), hash40("speed_x_max"));
    let speed_y_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_uniq_float"), hash40("speed_y_max"));
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x_max, speed_y_max);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x_max, speed_y_max);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let pos_z = PostureModule::pos_z(fighter.module_accessor);
    let result = &mut Vector2f{x: 0.0, y: 0.0};
    let ray_check = GroundModule::ray_check_hit_pos(fighter.module_accessor, &Vector2f{x: pos_x, y: pos_y}, &Vector2f{x: 0.0, y: -6.0}, result, true);
    if ray_check {
        let ray_check_y = result.y;
        let uniq_float_start_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_uniq_float"), hash40("uniq_float_start_y"));
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos_x, y: ray_check_y + uniq_float_start_y, z: pos_z});
    }
    0.into()
}

pub unsafe extern "C" fn status_Float_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);
    let float_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_uniq_float"), hash40("float_frame"));
    let motion_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("param_uniq_float"), hash40("motion_rate"));
    WorkModule::set_int(fighter.module_accessor, float_frame, *FIGHTER_STATUS_FLOAT_WORK_INT_TIME);
    WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_STATUS_FLOAT_WORK_INT_ENABLE_UNIQ);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall"), 0.0, motion_rate, true, 0.0, false, false);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
    EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_flash"), Hash40::new("top"), &Vector3f {x: 0.0, y: 7.0, z: 0.0}, &VECTOR_ZERO, 1.0, true, 0, 0, 0, 0, 0, true, true);
    Float_effect_function(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(Float_Main_loop as *const () as _))
}

unsafe extern "C" fn Float_effect_function(fighter: &mut L2CFighterCommon) {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    if fighter_kind == *FIGHTER_KIND_REFLET {
        SoundModule::play_se(fighter.module_accessor, Hash40::new("se_reflet_appear01"), true, false, false, false, enSEType(0));
        let eff_handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("reflet_catch"), Hash40::new("top"), &Vector3f{x: 0.0, y: -6.0, z: -5.3}, &VECTOR_ZERO, 0.7, true, 0, 0, 0, 0, 0, false, false);
        EffectModule::set_rgb(fighter.module_accessor, eff_handle as u32, 0.0, 1.0, 0.0); 
        let eff_handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_aura_light"), Hash40::new("bookc"), &VECTOR_ZERO, &VECTOR_ZERO, 1.5, true, 0, 0, 0, 0, 0, false, false);
        EffectModule::set_rgb(fighter.module_accessor, eff_handle as u32, 0.0, 1.0, 0.078);
    }
    if fighter_kind == *FIGHTER_KIND_SAMUSD {
        SoundModule::play_se(fighter.module_accessor, Hash40::new("se_samusd_appear01"), true, false, false, false, enSEType(0));
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), &Vector3f{x: -2.0, y: 0.0, z: 0.0}, &VECTOR_ZERO, 2.5, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("colonellm"), &Vector3f{x: 2.0, y: 0.0, z: 0.5}, &VECTOR_ZERO, 2.0, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("colonells"), &Vector3f{x: 2.0, y: 0.0, z: -0.5}, &VECTOR_ZERO, 2.0, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), &Vector3f{x: 0.0, y: 0.0, z: -0.5}, &VECTOR_ZERO, 1.7, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &VECTOR_ZERO, 2.1, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &VECTOR_ZERO, 1.9, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &VECTOR_ZERO, 2.0, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), &Vector3f{x: 0.0, y: 0.0, z: 0.5}, &VECTOR_ZERO, 1.7, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &VECTOR_ZERO, 2.1, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &VECTOR_ZERO, 1.9, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &VECTOR_ZERO, 2.0, true, 0, 0, 0, 0, 0, true, true);
    }
    if fighter_kind == *FIGHTER_KIND_MEWTWO {
        let SILVER = color >= 120 && color <= 127;
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), &Vector3f{x: 1.0, y: 0.0, z: 0.0}, &VECTOR_ZERO, 0.3, true, 0, 0, 0, *EF_FLIP_YZ, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), &Vector3f{x: -1.0, y: 0.0, z: 0.0}, &VECTOR_ZERO, 0.3, true, 0, 0, 0, *EF_FLIP_YZ, 0, true, true);
        if SILVER {
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_mewtwo_special_n09"), true, false, false, false, enSEType(0));
        }
        else {
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_mewtwo_appeal_l01"), true, false, false, false, enSEType(0));
        }
    }
}

unsafe extern "C" fn Float_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if fighter.global_table[CURRENT_FRAME].get_f32() == 3.0 {
        let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_uniq_float"), hash40("air_accel_x"));
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_uniq_float"), hash40("air_accel_y"));
        let accel_x = air_accel_x * stick_x;
        let accel_y = air_accel_y * stick_y;
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, sum_speed_x, 0.0);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, accel_x, accel_y);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::is_enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING) {
        fighter.sub_transition_group_check_air_landing();
    }
    if WorkModule::is_enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL) {
        fighter.sub_transition_group_check_air_special();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO) {    
        if fighter.sub_transition_group_check_air_lasso().get_bool() {
            return 1.into();
        }
    }
    let lr = PostureModule::lr(fighter.module_accessor);
    let mut rot_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_FLOAT_ROT_Y);
    let mut turn_dir = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_FLOAT_TURN_DIR);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_FLAG_TURN) {
        if stick_x * lr < -0.25 && stick_x.abs() > -0.25 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_FLAG_TURN);
            WorkModule::set_float(fighter.module_accessor, -lr, *FIGHTER_STATUS_FLOAT_WORK_FLOAT_TURN_DIR);
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_FLOAT_WORK_FLOAT_ROT_Y);
        }
    }
    else {
        if turn_dir == 0.0 { 
            turn_dir = 1.0
        };
        let target_rot = 180.0;
        let turn_speed_ratio = WorkModule::get_param_float(fighter.module_accessor, hash40("param_uniq_float"), hash40("turn_speed_ratio"));
        rot_y = lerp(&rot_y, &target_rot, &0.0);
        if (rot_y - target_rot).abs() < 2.0 {
            rot_y = target_rot;
            PostureModule::set_lr(fighter.module_accessor, turn_dir);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_FLAG_TURN);
        }
        else {
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f{x: 0.0, y: rot_y, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            WorkModule::set_float(fighter.module_accessor, rot_y, *FIGHTER_STATUS_FLOAT_WORK_FLOAT_ROT_Y);
            if (rot_y > 90.0 && lr != turn_dir) {
                PostureModule::set_lr(fighter.module_accessor, turn_dir);
            }
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR.into(), false.into());
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
    let int_time = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_INT_TIME);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_INT_ENABLE_UNIQ) == 1 {
        if int_time > 0 {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_INT_TIME);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) 
    || WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_FLOAT_WORK_INT_TIME) == 0 
    || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_FLOAT_WORK_INT_ENABLE_UNIQ);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_smash_flash"), Hash40::new("top"), &Vector3f {x: 0.0, y: 5.0, z: 4.0}, &VECTOR_ZERO, 1.0, true, 0, 0, 0, 0, 0, true, true);
        let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        let jump_count_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        let status = if jump_count >= jump_count_max {
            FIGHTER_STATUS_KIND_FALL_AERIAL
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

pub unsafe extern "C" fn status_Float_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_FLOAT);
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    if fighter_kind == *FIGHTER_KIND_REFLET {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("reflet_catch"), false, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_aura_light"), false, true);
    }
    if fighter_kind == *FIGHTER_KIND_SAMUSD {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("samusd_win3_aura"), false, true);
    }
    if fighter_kind == *FIGHTER_KIND_MEWTWO {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("mewtwo_pk_hand"), false, true);
    }
    0.into()
}

pub fn install() {
    Agent::new("fighter")
    .status(Pre, *FIGHTER_STATUS_KIND_FLOAT, status_Float_Pre)
    .status(Init, *FIGHTER_STATUS_KIND_FLOAT, status_Float_Init)
    .status(Main, *FIGHTER_STATUS_KIND_FLOAT, status_Float_Main)
    .status(End, *FIGHTER_STATUS_KIND_FLOAT, status_Float_End)
    .install();
}