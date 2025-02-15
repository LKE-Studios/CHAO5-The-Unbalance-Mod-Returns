use crate::imports::BuildImports::*;

pub static propeller_rate : f32 = 0.5;
pub static fly_frame : f32 = 420.0;
pub static angle_up_max : f32 = 60.0;
pub static angle_down_max : f32 = -50.0;
pub static base_power : f32 = 1.9;
pub static angle_power_rate : f32 = 0.0;
pub static max_speed : f32 = 2.4;
pub static end_speed : f32 = 0.5;
pub static gravity_accel : f32 = 0.03;
pub static gravity_speed_max : f32 = 0.2;
pub static angle_more_power : f32 = -20.0;
pub static down_add_power_max : f32 = 0.04;
pub static angle_accel_up : f32 = 0.55;
pub static angle_accel_down : f32 = 0.7;
pub static angle_speed_max : f32 = 4.5;
pub static angle_speed_stick_add : f32 = 1.0;
pub static glide_landing_frame : f32 = 60.0;
pub static glide_landing_speed : f32 = 1.0;
pub static radial_stick_sensitivity : f32 = 0.25;
pub static glide_touch_decel : f32 = 0.0;
pub static angle_se_pitch_ratio : f32 = -0.002;

pub unsafe extern "C" fn status_funky_SpecialSFly_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
        0.into()
    }
    else {
        0.into()
    }
}

pub unsafe extern "C" fn status_funky_SpecialSFly_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        let lr = PostureModule::lr(fighter.module_accessor);
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::set_float(fighter.module_accessor, base_power, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
        WorkModule::set_float(fighter.module_accessor, -sum_speed_y, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
        let initial_speed = base_power * lr;
        KineticUtility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_FREE, Vector2f{x: initial_speed, y: 0.0}, Vector3f{x: initial_speed, y: 0.0, z: 0.0});
        KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        0.into()
    }
    else {
        0.into()
    }
}

pub unsafe extern "C" fn status_funky_SpecialSFly_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE_LANDING);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_direction"), 90.0, 0.0, true, 0.0, false, false);
        let effect_a = EffectModule::req_follow(fighter.module_accessor, Hash40::new("krool_propeller"), Hash40::new("propeller"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 90.0, z: 0.0}, 0.8, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::set_rate(fighter.module_accessor, effect_a as u32, propeller_rate);
        WorkModule::set_int(fighter.module_accessor, effect_a as i32, *FIGHTER_FUNKY_STATUS_SPECIAL_S_WORK_INT_PROPELLER_EFFECT_HANDLE);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_FUNKY_STATUS_SPECIAL_S_WORK_FLOAT_PROPELLER_EFFECT_ANGLE);
        fighter.sub_shift_status_main(L2CValue::Ptr(funky_SpecialSFly_Main_loop as *const () as _))
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn funky_SpecialSFly_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let frame = MotionModule::frame(fighter.module_accessor);
        if glide_landing_frame <= frame {
            let sum_speed_length = KineticModule::get_sum_speed_length(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            if glide_landing_speed <= sum_speed_length {
                fighter.change_status(FIGHTER_FUNKY_STATUS_KIND_SPECIAL_S_LANDING.into(), false.into());
                return 0.into();
            }
        }
    }
    if fighter.sub_transition_group_check_air_landing().get_bool() {
        return 0.into();
    }
    let angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
    SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_donkey_special_l02"), 1.0 + angle * angle_se_pitch_ratio);
    let fly_frame_float = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GLIDE_FRAME);
    WorkModule::add_float(fighter.module_accessor, 1.0, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GLIDE_FRAME);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
        || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        || (GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) 
        || GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32))
        || fly_frame_float >= fly_frame {
            fighter.change_status(FIGHTER_FUNKY_STATUS_KIND_SPECIAL_S_END.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

pub unsafe extern "C" fn status_funky_SpecialSFly_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        let lr = PostureModule::lr(fighter.module_accessor);
        let energy_stop = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        let mut angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
        let mut angle_speed = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
        let mut stick_angle = ControlModule::get_stick_angle(fighter.module_accessor);
        if lr <= 0.0 {
            let mut above_or_below = -1.0;
            if stick_angle > 0.0 {
                above_or_below = 1.0;
            }
            stick_angle = (180.0 * above_or_below) - (stick_angle * 180.0 / PI);
        }
        else {
            stick_angle = stick_angle * 180.0 / PI;
        }
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        let stick_magnitude = (stick_x.powi(2) + stick_y.powi(2)).sqrt(); //Square Root of Stick X^2 + Stick Y^2
        if stick_magnitude > radial_stick_sensitivity {
            let angle_accel = if stick_angle < 0.0 {
                if stick_angle >= -135.0 {
                    -angle_accel_down
                }
                else {
                    angle_accel_up
                }   
            }
            else {
                if stick_angle >= 45.0 {
                    angle_accel_up
                }
                else {
                    -angle_accel_down
                }
            };
            let scaled_angle_accel = angle_accel * (stick_magnitude - radial_stick_sensitivity) / (1.0 - radial_stick_sensitivity);
            if angle_speed * scaled_angle_accel < 0.0 {
                angle_speed = 0.0;
            }
            let mut new_angle_speed = angle_speed + scaled_angle_accel;
            new_angle_speed = new_angle_speed.clamp(-angle_speed_max, angle_speed_max);
            WorkModule::set_float(fighter.module_accessor, new_angle_speed, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
            angle += new_angle_speed;
        }
        angle = angle.clamp(angle_down_max, angle_up_max);
        let mut power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
        power -= angle * angle_power_rate / 90.0;
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
            power -= glide_touch_decel;
        }
        if power < 0.0 {
            power = 0.0
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_RAPID_FALL) {
            if angle < angle_more_power {
                power += down_add_power_max * (angle_more_power - angle) / (angle_more_power - angle_down_max);
            }
        }
        else if angle > 0.0 {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_RAPID_FALL);
        }
        let gravity = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
        let mut new_gravity = gravity + gravity_accel;
        if new_gravity > gravity_speed_max {
            new_gravity = gravity_speed_max;
        }
        WorkModule::set_float(fighter.module_accessor, new_gravity, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
        let mut angled = Vector2f {x: power * angle.to_radians().cos() * lr, y: power * angle.to_radians().sin()};
        angled.y -= new_gravity;
        let speed = (angled.x * angled.x + angled.y * angled.y).sqrt(); //Square Root of angled X value + angled Y angle
        let ratio = max_speed / speed;
        if speed > max_speed {
            angled.x *= ratio;
            angled.y *= ratio;
        }
        if speed < end_speed || power <= 0.0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP);
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
        }
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, angled.x, angled.y);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, angled.x, angled.y);
        WorkModule::set_float(fighter.module_accessor, power, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
        MotionModule::set_frame(fighter.module_accessor, 90.0 - angle, false);
        WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
        let frame = MotionModule::frame(fighter.module_accessor);
        let effect_handle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_S_WORK_INT_PROPELLER_EFFECT_HANDLE);
        let effect_angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_S_WORK_FLOAT_PROPELLER_EFFECT_ANGLE);
        WorkModule::add_float(fighter.module_accessor, 5.0, *FIGHTER_FUNKY_STATUS_SPECIAL_S_WORK_FLOAT_PROPELLER_EFFECT_ANGLE);
        if frame == 90.0 {
            EffectModule::set_rot(fighter.module_accessor, effect_handle as u32, &Vector3f{x: effect_angle, y: 0.0, z: 0.0});
        }
        else if frame > 90.0 {
            if PostureModule::lr(fighter.module_accessor) == 1.0 {
                EffectModule::set_rot(fighter.module_accessor, effect_handle as u32, &Vector3f{x: effect_angle, y: 0.0, z:((frame - 90.0) / -2.0) - 7.5});
            }
            else {
                EffectModule::set_rot(fighter.module_accessor, effect_handle as u32, &Vector3f{x: effect_angle, y: 0.0, z:((frame - 90.0) / 2.0) + 7.5});
            }
        }
        else if frame < 90.0 {
            if PostureModule::lr(fighter.module_accessor) == 1.0 {
                EffectModule::set_rot(fighter.module_accessor, effect_handle as u32, &Vector3f{x: effect_angle, y: 0.0, z:((frame - 90.0) / -2.0) + 7.5});
            }
            else {
                EffectModule::set_rot(fighter.module_accessor, effect_handle as u32, &Vector3f{x: effect_angle, y: 0.0, z:((frame - 90.0) / 2.0) - 7.5});
            }
        }
        0.into()
    }
    else {
        0.into()
    }
}

pub unsafe extern "C" fn status_funky_SpecialSFly_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        let effect_handle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_S_WORK_INT_PROPELLER_EFFECT_HANDLE);
        EffectModule::set_scale(fighter.module_accessor, effect_handle as u32, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("krool_propeller"), false, false);
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_donkey_special_l02"), 0);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_FUNKY_STATUS_SPECIAL_S_WORK_FLOAT_PROPELLER_EFFECT_ANGLE);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GLIDE_FRAME);
        0.into()
    }
    else {
        0.into()
    }
}

pub fn install() {
    Agent::new("donkey")
    .status(Pre, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_S_FLY, status_funky_SpecialSFly_Pre)
    .status(Init, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_S_FLY, status_funky_SpecialSFly_Init)
    .status(Main, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_S_FLY, status_funky_SpecialSFly_Main)
    .status(Exec, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_S_FLY, status_funky_SpecialSFly_Exec)
    .status(End, *FIGHTER_FUNKY_STATUS_KIND_SPECIAL_S_FLY, status_funky_SpecialSFly_End)
    .install();
}
