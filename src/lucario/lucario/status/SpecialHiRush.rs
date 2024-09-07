use crate::imports::BuildImports::*;
use crate::lucario::lucario::status::SpecialHiRushEnd::*;

unsafe extern "C" fn status_lucario_SpecialHiRush_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_RUSH_FRAME);
    GroundModule::set_passable_check(fighter.module_accessor, true);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_move"), 0.0, 1.0, false, 0.0, false, false);
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_LUCARIO_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    lucario_SpecialHiRush_correct(fighter);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(lucario_SpecialHiRush_Sub_Status as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_SpecialHiRush_Main_loop as *const () as _))
}

unsafe extern "C" fn lucario_SpecialHiRush_Sub_Status(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_RUSH_FRAME);
    }
    0.into()
}

unsafe extern "C" fn lucario_SpecialHiRush_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        lucario_SpecialHiRush_correct(fighter);
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
        lucario_SpecialHiRush_set_end_status(fighter, FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END.into());
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
        lucario_SpecialHiRush_set_end_status(fighter, FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END.into());
    }
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
    }
    if !GroundModule::is_status_cliff(fighter.module_accessor) {
        if lucario_SpecialHiRush_touch_ground(fighter, GROUND_TOUCH_FLAG_DOWN.into(), false.into(), 0.0_f32.into()).get_bool()
        || lucario_SpecialHiRush_touch_ground(fighter, GROUND_TOUCH_FLAG_LEFT.into(), true.into(), 1.0_f32.into()).get_bool()
        || lucario_SpecialHiRush_touch_ground(fighter, GROUND_TOUCH_FLAG_RIGHT.into(), true.into(), (-1.0_f32).into()).get_bool()
        || lucario_SpecialHiRush_touch_ground(fighter, GROUND_TOUCH_FLAG_UP.into(), false.into(), 0.0_f32.into()).get_bool() {
            // Nothing LOL
        }
    }
    else {
        if GroundModule::can_entry_cliff(fighter.module_accessor) != 0 {
            lucario_SpecialHiRush_set_end_status(fighter, FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE.into());
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_STATUS_TRANS)
    && !fighter.global_table[IS_STOP].get_bool() {
        let status = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_NEXT_STATUS);
        StatusModule::change_status_request(fighter.module_accessor, status, false);
        lucario_SpecialHiRush_reset_end_status(fighter);
    }
    0.into()
}

unsafe extern "C" fn lucario_SpecialHiRush_correct(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
}

unsafe extern "C" fn lucario_SpecialHiRush_reset_end_status(fighter: &mut L2CFighterCommon) {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_STATUS_TRANS);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_NEXT_STATUS);
}

unsafe extern "C" fn lucario_SpecialHiRush_set_end_status(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_STATUS_TRANS);
    WorkModule::set_int(fighter.module_accessor, param_1.get_i32(), *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_NEXT_STATUS);
}

unsafe extern "C" fn lucario_SpecialHiRush_touch_ground(fighter: &mut L2CFighterCommon, flag: L2CValue, some_bool: L2CValue, some_float: L2CValue) -> L2CValue {
    if GroundModule::is_touch(fighter.module_accessor, flag.get_u32()) {
        let normal = Vector2f{x: GroundModule::get_touch_normal_x_consider_gravity(fighter.module_accessor, flag.get_u32()), y: GroundModule::get_touch_normal_y_consider_gravity(fighter.module_accessor, flag.get_u32())};
        let speed = Vector2f{x: KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN), y: KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)};
        let vec_angle = sv_math::vec2_angle(normal.x, normal.y, speed.x, speed.y);
        let crush_angle = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("crush_angle"));
        let ref_angle = (crush_angle + 90.0).to_radians();
        if ref_angle <= vec_angle {
            if !some_bool.get_bool() {
                lucario_SpecialHiRush_set_end_status(fighter, FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND.into());
            }
            else {
                if !lucario_SpecialHi_attach_wall(fighter, some_float).get_bool() {
                    lucario_SpecialHiRush_set_end_status(fighter, FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND.into());
                }
                else {
                    lucario_SpecialHiRush_set_end_status(fighter, FIGHTER_STATUS_KIND_ATTACH_WALL.into());
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
                let half_pi = PI * 0.5;
                if half_pi <= speed_atan {
                    if normal_atan <= -half_pi {
                        normal_atan += PI * 2.0;
                    }
                    else {
                        if speed_atan <= -half_pi {
                            if half_pi <= normal_atan {
                                speed_atan += PI * 2.0;
                            }
                        }
                    }
                }
                else {
                    if speed_atan <= -half_pi {
                        if half_pi <= normal_atan {
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
                let vec = lucario_SpecialHi_get_vec(fighter, new_dir.into(), length.into());
                KineticEnergyNormal::set_speed(stop_energy as *mut smash::app::KineticEnergyNormal, &vec);
            }
        }
    }
    false.into()
}

unsafe extern "C" fn lucario_SpecialHi_get_vec(_fighter: &mut L2CFighterCommon, angle: L2CValue, speed_length: L2CValue) -> Vector2f {
    let mut vec = Vector2f{x: angle.get_f32().cos(), y: angle.get_f32().sin()};
    if vec.y.abs() <= 0.01 {vec.y = 0.0;}
    vec.x *= speed_length.get_f32();
    vec.y *= speed_length.get_f32();
    vec
}

pub fn install() {
    Agent::new("lucario")
    .status(Main, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH, status_lucario_SpecialHiRush_Main)
    .install();
}