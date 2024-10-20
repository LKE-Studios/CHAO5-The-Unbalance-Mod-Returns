use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_buddy_SpecialSDash_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_NO_REACTION);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn buddy_special_s_dash_check_angle(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        let degree = WorkModule::get_float(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLOAT_GROUND_DEGREE_CURRENT);
        WorkModule::set_float(fighter.module_accessor, degree, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLOAT_GROUND_DEGREE_PREV);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLOAT_GROUND_DEGREE_CURRENT);
    }
    else {
        let touch_normal_x = GroundModule::get_touch_normal_x_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let touch_normal_y = GroundModule::get_touch_normal_y_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let degree_new = touch_normal_x.atan2(touch_normal_y).to_degrees();
        let lr = PostureModule::lr(fighter.module_accessor);
        let degree = WorkModule::get_float(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLOAT_GROUND_DEGREE_CURRENT);
        WorkModule::set_float(fighter.module_accessor, degree, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLOAT_GROUND_DEGREE_PREV);
        WorkModule::set_float(fighter.module_accessor, degree_new * -lr, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLOAT_GROUND_DEGREE_CURRENT);
    }
}

pub unsafe extern "C" fn status_buddy_SpecialSDash_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_CLIFF_CHECK);
    buddy_special_s_dash_check_angle(fighter);
    let degree = WorkModule::get_float(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLOAT_GROUND_DEGREE_CURRENT);
    WorkModule::set_float(fighter.module_accessor, degree, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLOAT_GROUND_DEGREE_PREV);
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    buddy_special_s_dash_kinetic_helper(fighter, false.into());
    buddy_special_s_dash_ground_angle_limit(fighter);
    buddy_special_s_dash_set_speed(fighter);
    fighter.sub_change_motion_by_situation(hash40("special_s_dash").into(), hash40("special_air_s_dash").into(), false.into());
    WorkModule::set_int(fighter.module_accessor, 5, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);
    fighter.sub_shift_status_main(L2CValue::Ptr(buddy_SpecialSDash_Main_loop as *const () as _))
}

unsafe extern "C" fn buddy_SpecialSDash_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    buddy_special_s_set_armor(fighter);
    if MotionModule::frame(fighter.module_accessor) > 15.0 {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
            }
        }
    }
    let frame = fighter.global_table[CURRENT_FRAME].get_f32() as i32;
    let dash_time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("dash_time"));
    if dash_time <= frame {
        fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) && !SlowModule::is_skip(fighter.module_accessor) {
        let touch_flag = if PostureModule::lr(fighter.module_accessor) == 1.0 {
            *GROUND_TOUCH_FLAG_RIGHT
        }
        else {
            *GROUND_TOUCH_FLAG_LEFT
        };
        if GroundModule::is_touch(fighter.module_accessor, touch_flag as u32) {
            fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL.into(), false.into());
            return 0.into();
        }
    }
    buddy_special_s_dash_check_angle(fighter);
    let mut changed_situation = false;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let degree_min = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0x1df7e26482);
        let degree_prev = WorkModule::get_float(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLOAT_GROUND_DEGREE_PREV);
        let degree = WorkModule::get_float(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLOAT_GROUND_DEGREE_CURRENT);
        if degree_min <= degree {
            let idk = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0x17a26fc33e);
            if idk < degree - degree_prev {
                fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL.into(), false.into());
                return 0.into();
            }
        }
        if degree_min <= degree_prev {
            let idk = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0x1757db5b57);
            if idk < -(degree - degree_prev) {
                fighter.set_situation(SITUATION_KIND_AIR.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                buddy_special_s_dash_set_gravity(fighter);
            }
            changed_situation = true;
        }
    }
    fighter.sub_change_motion_by_situation(hash40("special_s_dash").into(), hash40("special_air_s_dash").into(), true.into());
    if fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into()).get_bool() {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            buddy_special_s_dash_set_gravity(fighter);
        }
        buddy_special_s_dash_kinetic_helper(fighter, true.into());
    }
    buddy_special_s_dash_ground_angle_limit(fighter);
    buddy_special_s_dash_set_speed(fighter);
    let change = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_CLIFF_CHECK) && changed_situation;
    fighter.sub_set_ground_correct_by_situation(change.into());
    let limit_param = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        hash40("dash_speed_x_limit_ground")
    }
    else {
        hash40("dash_speed_x_limit_air")
    };
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    let limit = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), limit_param);
    fighter.set_speed_ratio(motion.into(), limit.into(), FIGHTER_BUDDY_STATUS_SPECIAL_S_FLOAT_MOTION_RATE.into());
    let rate = WorkModule::get_float(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLOAT_MOTION_RATE);
    let scale = PostureModule::scale(fighter.module_accessor);
    MotionModule::set_rate(fighter.module_accessor, rate * scale);
    MotionModule::set_rate_2nd(fighter.module_accessor, rate * scale);
    0.into()
}

unsafe extern "C" fn buddy_special_s_dash_kinetic_helper(fighter: &mut L2CFighterCommon, inherit_speed: L2CValue) {
    let lr = PostureModule::lr(fighter.module_accessor);
    let reset_type;
    let speed_param;
    let accel_param;
    let limit_param;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        reset_type = ENERGY_STOP_RESET_TYPE_GROUND;
        speed_param = hash40("dash_speed_x_ground");
        accel_param = hash40("dash_accel_x_ground");
        limit_param = hash40("dash_speed_x_limit_ground");
    }
    else {
        reset_type = ENERGY_STOP_RESET_TYPE_AIR;
        speed_param = hash40("dash_speed_x_air");
        accel_param = hash40("dash_accel_x_air");
        limit_param = hash40("dash_speed_x_limit_air");
    }
    let speed_x = if !inherit_speed.get_bool() {
        WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), speed_param) * lr
    }
    else {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)
    };
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, reset_type, speed_x, 0.0, 0.0, 0.0, 0.0);
    let dash_accel_x_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), accel_param);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, dash_accel_x_ground * lr, 0.0);
    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    let dash_speed_x_limit_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), limit_param);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, dash_speed_x_limit_ground, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_STOP_RESET_TYPE_AIR, 0.0, speed_y, 0.0, 0.0, 0.0);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let accel_y = sv_kinetic_energy::get_accel_y(fighter.lua_state_agent);
        let dash_gravity_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("dash_gravity_mul"));
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, accel_y * dash_gravity_mul);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let brake_y = sv_kinetic_energy::get_brake_y(fighter.lua_state_agent);
        let dash_gravity_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("dash_gravity_mul"));
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, brake_y * dash_gravity_mul);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let stable_speed_y = sv_kinetic_energy::get_stable_speed_y(fighter.lua_state_agent);
        let dash_gravity_limit_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("dash_gravity_limit_mul"));
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, stable_speed_y * dash_gravity_limit_mul);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
}

unsafe extern "C" fn buddy_special_s_dash_set_speed(fighter: &mut L2CFighterCommon) {
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent).abs();
    let param = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        hash40("dash_speed_x_min_ground")
    }
    else {
        hash40("dash_speed_x_min_air")
    };
    let min_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), param);
    let lr = PostureModule::lr(fighter.module_accessor);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, min_speed.max(speed_x) * lr, 0.0);
}

unsafe extern "C" fn buddy_special_s_dash_set_gravity(fighter: &mut L2CFighterCommon) {
    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    let lr = PostureModule::lr(fighter.module_accessor);
    let degree_prev = WorkModule::get_float(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLOAT_GROUND_DEGREE_PREV);
    let new_speed = speed_x * lr * degree_prev.to_radians().sin();
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        new_speed
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
}

unsafe extern "C" fn buddy_special_s_dash_ground_angle_limit(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let mut dash_accel_x_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("dash_accel_x_ground"));
        let mut dash_speed_x_limit_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("dash_speed_x_limit_ground"));
        let degree = WorkModule::get_float(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLOAT_GROUND_DEGREE_CURRENT).clamp(-45.0, 45.0);
        if 0.0 <= degree {
            let ratio = degree / 45.0;
            let lerp_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0x241d0826e8);
            let accel_lerp = fighter.lerp(1.0_f32.into(), lerp_max.into(), ratio.into()).get_f32();
            dash_accel_x_ground *= accel_lerp;
            let lerp_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0x240b97d6dd);
            let limit_lerp = fighter.lerp(1.0_f32.into(), lerp_max.into(), ratio.into()).get_f32();
            dash_speed_x_limit_ground *= limit_lerp;
        }
        else {
            let ratio = -degree / 45.0;
            let lerp_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0x2630d88437);
            let accel_lerp = fighter.lerp(1.0_f32.into(), lerp_max.into(), ratio.into()).get_f32();
            dash_accel_x_ground *= accel_lerp;
            let lerp_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0x262f54a202);
            let limit_lerp = fighter.lerp(1.0_f32.into(), lerp_max.into(), ratio.into()).get_f32();
            dash_speed_x_limit_ground *= limit_lerp;
        }
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, dash_accel_x_ground * lr, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, dash_speed_x_limit_ground, 0.0);
    }
}

pub unsafe extern "C" fn buddy_special_s_set_armor(fighter: &mut L2CFighterCommon) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR) {
        buddy_special_s_remove_armor(fighter);
    }
    else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP) {
            HitModule::set_total_status_disguise(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);
        }
    }
}

pub unsafe extern "C" fn buddy_special_s_remove_armor(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP) {
        HitModule::set_total_status_disguise(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);
    }
}

pub unsafe extern "C" fn status_buddy_SpecialSDash_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    buddy_special_s_remove_armor(fighter);
    0.into()
}

pub fn install() {
    Agent::new("buddy")
    .status(Pre, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, status_buddy_SpecialSDash_Pre)
    .status(Main, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, status_buddy_SpecialSDash_Main)
    .status(End, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, status_buddy_SpecialSDash_End)
    .install();
}