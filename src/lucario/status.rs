use crate::imports::BuildImports::*;

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn status_lucario_special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64, *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_ENABLE_SPECIAL_HI);
    0.into()
}

#[status_script(agent = "lucario", status = FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn status_lucario_special_hi_rush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_RUSH_FRAME);
    GroundModule::set_passable_check(fighter.module_accessor, true);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_move"), 0.0, 1.0, false, 0.0, false, false);
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_LUCARIO_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    lucario_special_hi_rush_correct(fighter);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(lucario_special_hi_rush_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_hi_rush_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_special_hi_rush_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_RUSH_FRAME);
    }
    0.into()
}

unsafe extern "C" fn lucario_special_hi_rush_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        lucario_special_hi_rush_correct(fighter);
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool()
        || fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        lucario_special_hi_rush_set_end_status(fighter, FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END.into());
    }
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut smash::app::KineticEnergy;
        let speed_x = KineticEnergy::get_speed_x(stop_energy);
        let speed_y = KineticEnergy::get_speed_y(stop_energy);
        let length = sv_math::vec2_length(speed_x, speed_y);
        if length.signum() > 0.0 {
            let normal_x = GroundModule::get_touch_normal_x_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
            let normal_y = GroundModule::get_touch_normal_y_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
            let normalize = sv_math::vec2_normalize(normal_x, normal_y);
            let dot = sv_math::vec2_dot(normalize.x, normalize.y, speed_x, speed_y);
            if 0.00001 < dot {
                fighter.set_situation(SITUATION_KIND_AIR.into());
                GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
        }
    }
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        lucario_special_hi_rush_set_end_status(fighter, FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END.into());
    }
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
    }
    if !GroundModule::is_status_cliff(fighter.module_accessor) {
        if lucario_special_hi_rush_touch_ground(fighter, GROUND_TOUCH_FLAG_DOWN.into(), false.into(), 0.0_f32.into()).get_bool()
        || lucario_special_hi_rush_touch_ground(fighter, GROUND_TOUCH_FLAG_LEFT.into(), true.into(), 1.0_f32.into()).get_bool()
        || lucario_special_hi_rush_touch_ground(fighter, GROUND_TOUCH_FLAG_RIGHT.into(), true.into(), (-1.0_f32).into()).get_bool()
        || lucario_special_hi_rush_touch_ground(fighter, GROUND_TOUCH_FLAG_UP.into(), false.into(), 0.0_f32.into()).get_bool() {
            // nothing lol
        }
    }
    else {
        if GroundModule::can_entry_cliff(fighter.module_accessor) != 0 {
            lucario_special_hi_rush_set_end_status(fighter, FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE.into());
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_STATUS_TRANS)
    && !fighter.global_table[IS_STOP].get_bool() {
        let status = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_NEXT_STATUS);
        StatusModule::change_status_request(fighter.module_accessor, status, false);
        lucario_special_hi_rush_reset_end_status(fighter);
    }
    0.into()
}

unsafe extern "C" fn lucario_special_hi_rush_correct(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
}

unsafe extern "C" fn lucario_special_hi_rush_reset_end_status(fighter: &mut L2CFighterCommon) {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_STATUS_TRANS);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_NEXT_STATUS);
}

unsafe extern "C" fn lucario_special_hi_rush_set_end_status(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_STATUS_TRANS);
    WorkModule::set_int(fighter.module_accessor, param_1.get_i32(), *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_NEXT_STATUS);
}

unsafe extern "C" fn lucario_special_hi_rush_touch_ground(fighter: &mut L2CFighterCommon, flag: L2CValue, some_bool: L2CValue, some_float: L2CValue) -> L2CValue {
    if GroundModule::is_touch(fighter.module_accessor, flag.get_u32()) {
        let normal = Vector2f{x: GroundModule::get_touch_normal_x_consider_gravity(fighter.module_accessor, flag.get_u32()), y: GroundModule::get_touch_normal_y_consider_gravity(fighter.module_accessor, flag.get_u32())};
        let speed = Vector2f{x: KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN), y: KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)};
        let vec_angle = sv_math::vec2_angle(normal.x, normal.y, speed.x, speed.y);
        let crush_angle = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("crush_angle"));
        let ref_angle = (crush_angle + 90.0).to_radians();
        if ref_angle <= vec_angle {
            if !some_bool.get_bool() {
                lucario_special_hi_rush_set_end_status(fighter, FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND.into());
            }
            else {
                if !lucario_special_hi_attach_wall(fighter, some_float).get_bool() {
                    lucario_special_hi_rush_set_end_status(fighter, FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND.into());
                }
                else {
                    lucario_special_hi_rush_set_end_status(fighter, FIGHTER_STATUS_KIND_ATTACH_WALL.into());
                }
            }
            return true.into();
        }
        let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        let stop_speed = Vector2f {x: KineticEnergy::get_speed_x(stop_energy as *mut smash::app::KineticEnergy), y: KineticEnergy::get_speed_y(stop_energy as *mut smash::app::KineticEnergy)};
        let length = sv_math::vec2_length(stop_speed.x, stop_speed.y);
        if 0.0 <= length {
            let normal = Vector2f {x: GroundModule::get_touch_normal_x_consider_gravity(fighter.module_accessor, flag.get_u32()), y: GroundModule::get_touch_normal_y_consider_gravity(fighter.module_accessor, flag.get_u32())};
            let dot = sv_math::vec2_dot(stop_speed.x, stop_speed.y, normal.x, normal.y);
            if dot < 0.0 {
                let angle_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("angle_speed"));
                let rad_speed = angle_speed.to_radians();
                let mut speed_atan = speed.y.atan2(speed.x);
                let mut normal_atan = normal.y.atan2(normal.x);
                let halfpi = PI * 0.5;
                if halfpi <= speed_atan {
                    if normal_atan <= -halfpi {
                        normal_atan += PI * 2.0;
                    }
                    else {
                        if speed_atan <= -halfpi {
                            if halfpi <= normal_atan {
                                speed_atan += PI * 2.0;
                            }
                        }
                    }
                }
                else {
                    if speed_atan <= -halfpi {
                        if halfpi <= normal_atan {
                            speed_atan += PI * 2.0;
                        }
                    }
                }
                let mut diff = speed_atan - normal_atan;
                if rad_speed <= diff {
                    diff = -rad_speed;
                }
                else {
                    if diff <= -rad_speed {
                        diff = rad_speed;
                    }
                    else {
                        diff *= -1.0;
                    }
                }
                let rush_dir = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLOAT_RUSH_DIR);
                let new_dir = rush_dir + diff;
                WorkModule::set_float(fighter.module_accessor, new_dir, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLOAT_RUSH_DIR);
                let vec = lucario_special_hi_get_vec(fighter, new_dir.into(), length.into());
                KineticEnergyNormal::set_speed(stop_energy as *mut smash::app::KineticEnergyNormal, &vec);
            }
        }
    }
    false.into()
}

unsafe extern "C" fn lucario_special_hi_get_vec(_fighter: &mut L2CFighterCommon, angle: L2CValue, speed_length: L2CValue) -> Vector2f {
    let mut vec = Vector2f{x: angle.get_f32().cos(), y: angle.get_f32().sin()};
    if vec.y.abs() <= 0.01 {vec.y = 0.0;}
    vec.x *= speed_length.get_f32();
    vec.y *= speed_length.get_f32();
    vec
}

#[status_script(agent = "lucario", status = FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn status_lucario_special_hi_rush_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if situation != *SITUATION_KIND_GROUND {
        FighterSpecializer_Lucario::set_mach_validity(fighter.module_accessor, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
        GroundModule::set_cliff_check(fighter.module_accessor, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        let end_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("end_brake_x"));
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, end_brake_x, 0.0);
        let end_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("end_accel_y"));
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -end_accel_y);
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y.clamp(-5.0, 2.0));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_end"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_end"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    }
    WorkModule::set_int(fighter.module_accessor, situation, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_RUSH_END_SITUATION);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_hi_rush_end_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_special_hi_rush_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let sit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_RUSH_END_SITUATION);
    if sit != *SITUATION_KIND_GROUND
    && fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if sit != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
            }
            return 0.into();
        }
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_AIR_END_CONTROL_X) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_AIR_END_CONTROL_X);
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
            }
            return 0.into();
        }
    }
    let cont = if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) {
        lucario_special_hi_attach_wall(fighter, (1.0_f32).into()).get_bool()
    }
    else if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) {
        lucario_special_hi_attach_wall(fighter, (-1.0_f32).into()).get_bool()
    }
    else { false };
    if cont {
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACH_WALL, false);
    }
    0.into()
}

unsafe extern "C" fn lucario_special_hi_attach_wall(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    let no_attach = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_ATTACH_WALL_FRAME);
    let wall_jump_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("wall_jump_stick_x"));
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    if no_attach <= 0 {
        if wall_jump_stick_x <= stick_x.abs() {
            let lr = param_1.get_f32();
            if lr * stick_x < 0.0 {
                let vector;
                let some = if 0.0 > lr {
                    vector = Vector2f {
                        x: GroundModule::get_touch_normal_x_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32),
                        y: GroundModule::get_touch_normal_y_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32)
                    };
                    1.0
                }
                else {
                    vector = Vector2f {
                        x: GroundModule::get_touch_normal_x_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32),
                        y: GroundModule::get_touch_normal_y_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32)
                    };
                    -1.0
                };
                let atan = vector.x.atan2(vector.y) * some;
                let deg = atan.to_degrees() + 90.0;
                let attach_wall_slant_max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attach_wall_slant_max"));
                let attach_wall_slant_min = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attach_wall_slant_min"));
                if attach_wall_slant_max > deg
                && deg > attach_wall_slant_min {
                    if !GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) {
                        let attach_on = if 0.0 <= lr {
                            *GROUND_TOUCH_FLAG_LEFT
                        }
                        else { *GROUND_TOUCH_FLAG_RIGHT };
                        if GroundModule::is_attachable(fighter.module_accessor, GroundTouchFlag(attach_on)) {
                            return true.into();
                        }
                    }
                }
            }
        }
    }
    false.into()
}

pub fn install() {
    install_status_scripts!(
        status_lucario_special_hi_pre,
        status_lucario_special_hi_rush_main,
        status_lucario_special_hi_rush_end_main
    );
}

